use ansi_term::Color as OtherColor;
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use image::io::Reader as ImageReader;
use image::Pixel;
use image::{open, GenericImageView, ImageBuffer, Rgba};
use std::{io::Write, fmt::format};
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::{env, process::Command};

fn main() {
    const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
    let input_file = "input.mp4";
    let output_dir = Path::new("output_frames");
    let output_pattern = "output_frames\\image_%d.png";
    let tmp_dir = "tmp";

    let ffmpegPath = "ffmpeg\\ffmpeg\\bin\\ffmpeg.exe";

    let (mWidth, mHeight) = crossterm::terminal::size().unwrap();

    println!("{}<<>>{}", mWidth, mHeight);

    // let MinMovie = Command::new(ffmpegPath)
    //     .arg("-y")
    //     .arg("-i")
    //     .arg(input_file)
    //     .arg("-vf")
    //     .arg(format!("scale={}:{}", mWidth, -1))
    //     .arg("tmp\\tmp.mp4")
    //     .output()
    //     .expect("Failed is Convert Movie");

    // println!("Min>>{:?}", MinMovie);

    // let MovieToBMP = Command::new(ffmpegPath)
    //     .arg("-i")
    //     .arg("tmp\\tmp.mp4")
    //     .arg("-vcodec")
    //     .arg("png")
    //     .arg(output_pattern)
    //     .output()
    //     .expect("Failed is Codec");

    // println!("Conv >> {:?}", MovieToBMP);

    let entry = output_dir.read_dir().expect("read_dir call failed");
    let entry_count = entry.count();

    for entry in 1..entry_count {
        let path = format!("output_frames\\image_{}.png", entry);
        let result = open(path);

        match result {
            Ok(image) => {
                let (width, height) = image.dimensions();

                // 画像の各ピクセルに対して色情報を取得し、出力する
                for y in 0..height {
                    for x in 0..width {
                        // ピクセルの色情報を取得する
                        let pixel_color: Rgba<u8> = image.get_pixel(x, y).into();

                        // R, G, B の値を取得する
                        let (red, green, blue, _) = pixel_color.channels4();
                        print!(
                            "{}A ",
                            SetForegroundColor(Color::Rgb {
                                r: red,
                                g: green,
                                b: blue
                            })
                        );
                    }
                    println!("");
                }
                thread::sleep(Duration::from_millis(33));
                println!("{}", CLEAR_SCREEN);
            }

            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
