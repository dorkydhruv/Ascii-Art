use termion::{terminal_size};
use image::{GenericImageView, imageops::FilterType};

fn to_ascii(image: &image::DynamicImage) -> String {
    let ascii_chars = "$@B%8WM#*oahkbdpqwmZO0QCJYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
    let ascii_chars = ascii_chars.chars().rev().collect::<Vec<_>>();
    let ascii_scale = ascii_chars.len() as f32;

    let mut ascii_art = String::new();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let brightness = (pixel[0] as f32 * 0.299) + (pixel[1] as f32 * 0.587) + (pixel[2] as f32 * 0.114);
            let brightness = brightness / 255.0; // Normalize to 0.0 - 1.0
            let index = (brightness * ascii_scale) as usize;
            ascii_art.push(ascii_chars[index.min(ascii_chars.len() - 1)]);
        }
        ascii_art.push('\n');
    }
    ascii_art
}

fn main() {
    let img = image::open("cat.jpeg").unwrap();
    let (width,height)=terminal_size().unwrap();
    let img = img.resize(width as u32, height as u32, FilterType::CatmullRom);
    let ascii_art = to_ascii(&img);
    println!("{}", ascii_art);
}