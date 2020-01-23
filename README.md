# mpv-bilibili
使用mpv播放B站视频  
you-get本身支持调用播放器播放，但不完善

### 安装(使用cargo)
```
git clone https://github.com/snylonue/b2m
cd b2m
cargo build --release
```
编译好的文件位于`target/release/b2m.exe`

如果无法编译rust程序，可以使用[Python版(不再维护)](src/bilibili2mpv.py)

### 用法
```
mpv-bilibili 0.10.3
Play bilibili video with mpv

USAGE:
    b2m.exe [FLAGS] <url>

FLAGS:
    -c, --check      Check if all dependencies are installed
    -h, --help       Prints help information
    -j, --json       Print stdout in json
        --an         Play without audio output
    -u, --url        Print url only
    -V, --version    Prints version information

ARGS:
    <url>    Video url

```

**依赖**: [you-get](https://github.com/soimort/you-get), [mpv](https://mpv.io)  
**思路**: [Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  