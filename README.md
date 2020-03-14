# mpv-bilibili
**mpv自带[youtube-dl](https://github.com/ytdl-org/youtube-dl)解析，使用体验好很多**

使用mpv播放bilibili(以及youtube, iqiqyi)视频  
you-get本身支持调用播放器播放，但不完善

### 安装(使用cargo)
```
cargo install --git https://github.com/snylonue/b2m.git
b2m -h
```

或者[Python版(不再维护)](src/bilibili2mpv.py)

### 用法
```
mpv-bilibili 0.15.0
Play bilibili video with mpv

USAGE:
    b2m [FLAGS] [OPTIONS] <url>

FLAGS:
    -c, --check      Check if all dependencies are installed
    -h, --help       Prints help information
    -i, --info       Print information only
    -j, --json       Print stdout in json
        --an         Play without audio output
        --vn         Play without video output
    -V, --version    Prints version information

OPTIONS:
    -p, --proxy <proxy>    Set proxy address

ARGS:
    <url>    Video url

```

**依赖**: [you-get](https://github.com/soimort/you-get), [annie](https://github.com/iawia002/annie) [mpv](https://mpv.io)  
**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  
