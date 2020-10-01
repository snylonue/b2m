# b2m
**mpv 自带 [youtube-dl](https://github.com/ytdl-org/youtube-dl) 解析，使用体验好很多**

使用 mpv 播放 bilibili (以及 youtube, iqiqyi)视频  
you-get 本身支持调用播放器播放，但不完善

### 安装(使用 cargo)
```
cargo install --git https://github.com/snylonue/b2m.git
b2m -h
```
由于使用了未发布至 crates.io 的依赖，目前无法通过 `cargo install b2m` 安装最新版

使用命令行选项 `--no-default-features --features <NAME>` 可以选用特定的解析器，具体见 [Cargo.toml](Cargo.toml)

或者 [Python 版(不再维护)](src/b2m.py)

### 用法
```
b2m 0.20.0
Play bilibili or other videos with mpv

USAGE:
    b2m [FLAGS] [OPTIONS] <url>

FLAGS:
        --check        Check if all dependencies are installed
    -h, --help         Prints help information
    -i, --info         Print information only
    -j, --json         Print information with json
        --an           Play without audio output
        --no-cookie    Don't use any cookie
        --vn           Play without video output
    -V, --version      Prints version information

OPTIONS:
    -c, --cookie <cookie>    Set cookie
    -p, --proxy <proxy>      Set proxy address [default: http://127.0.0.1:1080]

ARGS:
    <url>    Video url
```

### 依赖
- [you-get](https://github.com/soimort/you-get)  
- [annie](https://github.com/iawia002/annie)  
- [mpv](https://mpv.io)  


**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  