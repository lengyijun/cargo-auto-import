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
                    prepend_file(suggestion.as_bytes(), &pf)?;
                    // println!("\x1b[1;32m   Injecting\x1b[m {}", suggestion.trim());
                }
            }
        }
    }
    Ok(())
}

fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> Result<(), std::io::Error> {
    let mut f = File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}
