use termion::terminal_size;
use image::{GenericImageView, imageops::FilterType};
use opencv::{core, imgcodecs, prelude::*, types::VectorOfu8, videoio, Error};
fn to_ascii(image: &image::DynamicImage) -> String {
    let ascii_chars = "    $@B%8WM#*oahkbdpqwmZO0QCJYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
    let ascii_chars = ascii_chars.chars().rev().collect::<Vec<_>>();
    let ascii_scale = ascii_chars.len() as f32;

    let mut ascii_art = String::new();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let brightness = (pixel[0] as f32 * 0.299) + (pixel[1] as f32 * 0.587) + (pixel[2] as f32 * 0.114);
            let brightness = brightness / 255.0;
            let index = (brightness * ascii_scale) as usize;
            ascii_art.push(ascii_chars[index.min(ascii_chars.len() - 1)]);
        }
        ascii_art.push('\n');
    }
    ascii_art
}

fn main() ->Result<(),Error>{
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let mut i:u8 = 0;
    loop {
        cam.read(&mut frame)?;
        
        // Convert the frame to a byte vector
        let mut buf =VectorOfu8::new();
        imgcodecs::imencode(".jpg", &frame, &mut buf, &core::Vector::new()).unwrap();
        let buf = buf.to_vec();

        // Create a DynamicImage from the byte vector
        let img = image::load_from_memory(&buf).unwrap();

        let img = img.resize(terminal_size().unwrap().0 as u32, terminal_size().unwrap().1 as u32, FilterType::Nearest);
        let ascii_art = to_ascii(&img);
        print!("{}", termion::clear::All);
        print!("{}", ascii_art);
        i+=1;
        if i>128{
            break;
        }
        
    }
    println!("{}", termion::clear::All);
    println!("Goodbye!");
    Ok(())
}