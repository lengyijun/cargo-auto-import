# cargo-auto-import

https://rustcc.cn/article?id=4c8ea6c6-a988-4be0-9632-0678fa91afcf

## install
```
git clone https://github.com/lengyijun/cargo-auto-import
cd cargo-auto-import 
cargo install --path .
```

## How to use

if you meet 
```
help: consider importing this function
     |
1    + use crate::openbsd_compat::bsd_closefrom::closefrom;
     |
```

You can type this to apply the suggestions
```
cargo auto-import
```


## Similar project
https://github.com/m-ou-se/auto-import
