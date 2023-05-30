use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() -> Result<(), std::io::Error> {
    let out = Command::new("cargo")
        .arg("build")
        .arg("--message-format=json")
        .output()
        .unwrap();
    if out.status.success() {
        exit(0);
    }

    let mut mp: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    for line in std::str::from_utf8(&out.stdout)
        .unwrap()
        .lines()
        .filter(|l| l.starts_with('{'))
    {
        if let Ok(d) = json::parse(line) {
            for c in d["message"]["children"].members() {
                let suggestion = c["spans"][0]["suggested_replacement"]
                    .as_str()
                    .unwrap_or_default();
                let mut pf = PathBuf::from(d["manifest_path"].as_str().unwrap());
                pf.pop();
                pf.push(c["spans"][0]["file_name"].as_str().unwrap());
                if suggestion.starts_with("use ") {
                    match mp.get_mut(&pf) {
                        Some(x) => {
                            x.insert(suggestion.to_string());
                        }
                        None => {
                            let mut y: BTreeSet<String> = BTreeSet::new();
                            y.insert(suggestion.to_string());
                            mp.insert(pf, y);
                        }
                    }
                }
            }
        }
    }

    for (pf, use_list) in mp.into_iter() {
        let mut v: Vec<u8> = Vec::new();
        for s in use_list.into_iter() {
            v.extend_from_slice(s.as_bytes());
        }
        prepend_file(v, &pf)?;
    }

    Ok(())
}

fn prepend_file<P: AsRef<Path> + ?Sized>(
    mut content: Vec<u8>,
    path: &P,
) -> Result<(), std::io::Error> {
    let mut f = File::open(path)?;
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}
