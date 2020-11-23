use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;

use structopt::StructOpt;
use text_io::read;


#[derive(Debug, StructOpt)]
#[structopt(
    name = "Vibing Cat",
    about = "Made in rust to make vibing cat meme. BTW Not an orignal idea."
)]
struct Cli {
    #[structopt(short = "c", long = "cat")]
    cat_msg: String,

    #[structopt(short = "d", long = "drum")]
    drum_msg: String,

    #[structopt(short = "m", long = "man")]
    man_msg: String,

    #[structopt(short = "o", long = "output", default_value = "meme.mp4")]
    out_path: String,
}

struct DrawArgs {
    fontcolor: &'static str,
    fontsize: i32,
    x: i32,
    y: i32,
}

const CAT_ARGS: DrawArgs = DrawArgs {
    fontcolor: "black",
    fontsize: 48,
    x: 180,
    y: 550,
};

const MAN_ARGS: DrawArgs = DrawArgs {
    fontcolor: "white",
    fontsize: 48,
    x: 830,
    y: 400,
};

const DRUM_ARGS: DrawArgs = DrawArgs {
    fontcolor: "white",
    fontsize: 48,
    x: 650,
    y: 600,
};

fn create_drawtext(text: String, mut args: DrawArgs) -> Vec<String> {
    let lines = text.split("\\n");
    let mut drawtext: Vec<String> = Vec::new();
    for line in lines {
        drawtext.push(format!(
            "drawtext=text='{line}': fontcolor={color}: fontsize={size}: x={x}: y={y}",
            line = line,
            color = args.fontcolor,
            size = args.fontsize,
            x = args.x,
            y = args.y
        ));
        args.y += 58; // Make space between two lines
    }
    return drawtext;
}

fn output_file_name(filename: &str) -> String{
    let file_exist: bool = Path::new(&format!("{}", filename)).exists();
    if !file_exist {
        return String::from(filename);
    }
    println!(
        "File with name '{}' already exist. Override it y/n ?",
        filename
    );
    let ans: char = read!();
    if ans == 'y'{
        fs::remove_file(filename).expect("could not remove file");
        return String::from(filename);
    }
    else{
        println!("Enter new output filename:");
        let name: String = read!(); 
        return name;
    }
}

fn main() {
    let mut args = Cli::from_args();
    let mut text_list: Vec<String> = Vec::new();
    let bytes = include_bytes!("../assets/cat.mp4");
    args.out_path = output_file_name(&args.out_path);

    text_list.extend(create_drawtext(args.cat_msg, CAT_ARGS));
    text_list.extend(create_drawtext(args.man_msg, MAN_ARGS));
    text_list.extend(create_drawtext(args.drum_msg, DRUM_ARGS));
    let drawtext = text_list.join(",");
    let mut command = Command::new("ffmpeg");
    command
        .stdin(Stdio::piped())
        .args(&["-i", "-"]) // No file name given as video will be given as input byte stream
        .args(&["-threads", "16"])
        .args(&["-vf", &drawtext])
        .args(&["-codec:a"])
        .args(&["copy", &args.out_path]);

    if let Ok(mut child) = command.spawn() {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(bytes).expect("Failed to write to stdin");
        child.wait().expect("command wasn't running");
        println!("Your meme is ready");
    } else {
        println!("FFMPEG didn't start");
    }
}
