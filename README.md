# b2m

lux 12.0 无法以 json 输出结果 ([iawia002/lux#996](https://github.com/iawia002/lux/issues/996)), 请用 `go install github.com/iawia002/lux@HEAD` 或从 github actions ([iawia002/lux#912](https://github.com/iawia002/lux/issues/912)) 获取最新版本

**mpv 自带 [youtube-dl](https://github.com/ytdl-org/youtube-dl) 解析，使用体验好很多**

使用 mpv 播放 bilibili (以及 youtube, iqiqyi)视频  

### 安装

#### cargo

```
cargo install b2m
b2m -h
```

使用命令行选项 `--no-default-features --features <NAME>` 可以选用特定的后端，详见 [Cargo.toml](Cargo.toml)

使用 `cargo install b2m --all-features`  可启用所有后端，并默认使用 [finata](https://github.com/snylonue/finata) (无需 mpv 以外的依赖)

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
