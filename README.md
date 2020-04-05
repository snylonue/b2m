# mpv-bilibili
**mpv自带 [youtube-dl](https://github.com/ytdl-org/youtube-dl) 解析，使用体验好很多**

使用 mpv 播放 bilibili (以及 youtube, iqiqyi)视频  
you-get 本身支持调用播放器播放，但不完善

### 安装(使用 cargo)
```
cargo install --git https://github.com/snylonue/b2m.git
b2m -h
```

或者 [Python 版(不再维护)](src/bilibili2mpv.py)

### 用法
```
mpv-bilibili 0.16.0
Play bilibili video with mpv

USAGE:
    b2m [FLAGS] <url>

FLAGS:
    -c, --check      Check if all dependencies are installed
    -h, --help       Prints help information
    -i, --info       Print information only
    -j, --json       Print information with json
        --an         Play without audio output
        --vn         Play without video output
    -p, --proxy      Set proxy address
    -V, --version    Prints version information

ARGS:
    <url>    Video url
```

### 依赖
- [you-get](https://github.com/soimort/you-get)  
- [annie](https://github.com/iawia002/annie)  
- [mpv](https://mpv.io)  


**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  