# b2m
**mpv 自带 [youtube-dl](https://github.com/ytdl-org/youtube-dl) 解析，使用体验好很多**

使用 mpv 播放 bilibili (以及 youtube, iqiqyi)视频  
you-get 本身支持调用播放器播放，但不完善

### 安装

#### cargo
```
cargo install b2m
b2m -h
```

使用命令行选项 `--no-default-features --features <NAME>` 可以选用特定的解析器，详见 [Cargo.toml](Cargo.toml)

#### [AUR](https://aur.archlinux.org/packages/b2m/  )

感谢 @MrAru 打包

#### [Python 版(不再维护)](src/b2m.py)

### 用法
```
b2m 0.25.0
Play bilibili or other videos with mpv

USAGE:
    b2m.exe [FLAGS] [OPTIONS] <url> [-- <mpv-args>...]

FLAGS:
        --check        Check if all dependencies are installed
    -h, --help         Prints help information
    -i, --info         Print information only
    -j, --json         Print information with json
        --an           Play without audio output
        --no-cookie    Don't use any cookie
        --no-merge     Don't pass --merge-files to mpv
        --vn           Play without video output
    -V, --version      Prints version information

OPTIONS:
    -c, --cookie <cookie>    Load cookie [env: B2M_COOKIES=]
        --parser <parser>    Choose a parser [possible values: lux, fina]
    -p, --proxy <proxy>      Set proxy address [env: HTTP_PROXY=]  [default: 127.0.0.1:1080]

ARGS:
    <url>            Video url
    <mpv-args>...    args to pass to mpv, may have some limitations

ARGS:
    <url>            Video url
    <mpv-args>...    args to pass to mpv, may have some limitations
```

从`0.23.0`起，可以使用 `--parser` 选项选择特定的后端

### 依赖
- ~~[you-get](https://github.com/soimort/you-get)~~  
- [lux](https://github.com/iawia002/annie)  
- [mpv](https://mpv.io)  

### ToDo
- [ ] 支持播放列表
- [ ] 支持(toml)配置文件
- [ ] 减少对 you-get, lux 的依赖
- [x] 重构解析器

**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  
