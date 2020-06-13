# mpv-bilibili
**mpv自带 [youtube-dl](https://github.com/ytdl-org/youtube-dl) 解析，使用体验好很多**

使用 mpv 播放 bilibili (以及 youtube, iqiqyi)视频  
you-get 本身支持调用播放器播放，但不完善

### 安装(使用 cargo)
```
cargo install b2m
b2m -h
```

或者 [Python 版(不再维护)](src/b2m.py)

### 用法
```
b2m 0.19.0
Play bilibili or other videos with mpv

USAGE:
    b2m [FLAGS] [OPTIONS] <url>

FLAGS:
    -c, --check      Check if all dependencies are installed
    -h, --help       Prints help information
    -i, --info       Print information only
    -j, --json       Print information with json
        --an         Play without audio output
        --vn         Play without video output
    -V, --version    Prints version information

OPTIONS:
    -p, --proxy <proxy>    Set proxy address [default: 127.0.0.1:1080]

ARGS:
    <url>    Video url
```

### 依赖
- [you-get](https://github.com/soimort/you-get)  
- [annie](https://github.com/iawia002/annie)  
- [mpv](https://mpv.io)  


**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  