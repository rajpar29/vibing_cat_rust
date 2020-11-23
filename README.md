# Vibing Cat Meme Generator using rust

Inspired from :[Meme generator using bash](https://www.reddit.com/r/linuxmemes/comments/jvvcrk/generating_cat_vibing_memes_with_bash_why_not/)

The release build is a single executable file, so no seperate template vedio file needed. 

[Try now. Just unzip it.](https://github.com/rajpar29/vibing_cat_rust/files/5583289/cat_meme.zip)
You will need FFMPEG and rust to run it.
```
USAGE:
    cat_meme [OPTIONS] --cat <cat-msg> --drum <drum-msg> --man <man-msg>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cat <cat-msg>        
    -d, --drum <drum-msg>      
    -m, --man <man-msg>        
    -o, --output <out-path>     [default: meme.mp4]
```

```
cat_meme -c "Rust \n Community" -m "Rust meme \n generator" -d "Rust memes"  
```
![Sample Screenshot of video](https://github.com/rajpar29/vibing_cat_rust/blob/master/sample_screenshot.png?raw=true)
