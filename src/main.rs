use std::{process::Command, env};
use crossterm::{
    style::{Color, SetForegroundColor, ResetColor},
    ExecutableCommand,
};
use ansi_term::Color as OtherColor;
use image::{GenericImageView, ImageBuffer, Rgba, open};
use image::Pixel;
use image::io::Reader as ImageReader;

fn main() {
    println!("{}■{}■{}■{}",
    SetForegroundColor(Color::Cyan),
    SetForegroundColor(Color::Green),
    SetForegroundColor(Color::Yellow),
    ResetColor);
    // 入力ファイル名と出力ファイル名を指定
    // let input_file = "input.mp4";
    // let output_pattern = "output_frames\\image_%03d.png";

    // let ffmpegPath = "ffmpeg\\ffmpeg\\bin\\ffmpeg.exe";
    
    // let path = env::current_dir();
    // println!("{:?}", path);

    // let MovieToBMP = Command::new(ffmpegPath)
    //     .arg("-i")
    //     .arg(input_file)
    //     .arg("-vcodec")
    //     .arg("bmp")
    //     .arg(output_pattern)
    //     .output()
    //     .expect("Failed is Codec");

    // if MovieToBMP.status.success()
    // {
    //     println!("Success");
    // }
    // else {
    //     println!("Error");
    // }

    println!("{:?}", env::current_dir());

    let image_path = "output_frames\\input.png";
    let result = open(image_path);

    match result
    {
        Ok(image) =>
        {
            let (width, height) = image.dimensions();

            // 画像の各ピクセルに対して色情報を取得し、出力する
            for y in 0..height {
                for x in 0..width {
                    // ピクセルの色情報を取得する
                    let pixel_color: Rgba<u8> = image.get_pixel(x, y).into();
        
                    // R, G, B の値を取得する
                    let (red, green, blue, _) = pixel_color.channels4();
                    // ANSIカラーを使ってテキストを出力する
                    print!("{}■", SetForegroundColor(Color::Rgb{r:red, g:green, b:blue}));
                }
                println!();
            }
        
        }

        Err(err) =>
        {
            println!("{}", err);
        }
    }

}
