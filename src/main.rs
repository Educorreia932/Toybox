use image::{RgbImage, Rgb, ImageBuffer};

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut RgbImage) {
    for t in 0..100 { 
        let t = t as f32 * 0.01;

        let x = (x0 as f32 + (x1 as f32 - x0 as f32) * t) as u32; 
        let y = (y0 as f32 + (y1 as f32 - y0 as f32) * t) as u32; 

        (*image).put_pixel(x, y, Rgb([255, 0, 0]));
    } 
}

fn main() {
    let width = 100;
    let height = 100;

    let mut image : RgbImage = ImageBuffer::new(width, height);

    line(13, 20, 80, 40, &mut image); 

    image.save("image.png").unwrap();
}
