use std::process::Command;
use structopt::StructOpt;


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
        args.y += 58;
    }
    return drawtext;
}

fn main() {
    let args = Cli::from_args();
    let mut text_list: Vec<String> = Vec::new();

    text_list.extend(create_drawtext(args.cat_msg, CAT_ARGS));
    text_list.extend(create_drawtext(args.man_msg, MAN_ARGS));
    text_list.extend(create_drawtext(args.drum_msg, DRUM_ARGS));
    let drawtext = text_list.join(",");

    let status = Command::new("ffmpeg")
        .args(&["-i", "/media/main/RUST/cat_meme/assets/cat.mp4"])
        .args(&["-threads", "16"])
        .args(&["-vf", &drawtext])
        .args(&["-codec:a"])
        .args(&["copy", &args.out_path])
        .status()
        .expect("Failed to execute process");

    match status.code() {
        Some(code) => {
            if code == 0 {
                println!("Your meme is ready")
            } else {
                println!("Exited with status code: {}", code);
            }
        }
        None => println!("Process terminated by signal"),
    }
}
