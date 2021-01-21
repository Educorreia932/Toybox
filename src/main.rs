use image::{RgbImage, Rgb, ImageBuffer};
use image::imageops::{flip_vertical};
use std::mem;
mod model;

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut RgbImage, color: Rgb<u8>) {
    let mut steep = false;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror2 = (dy * 2).abs();
    let mut error2 =  0;
    let mut y = 0;

    for x in x0..=x1 {
        if steep {
            (*image).put_pixel(y as u32, x as u32, color);
        }

        else {
            (*image).put_pixel(x as u32, y as u32, color);
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 }; 
            error2 -= dx * 2;
        }
    }
}

fn main() {
    let width = 100;
    let height = 100;

    let model = model::Model::from_file("obj/african_head.obj");

    let mut image : RgbImage = ImageBuffer::new(width, height);

    line(0, 0, 50, 50, &mut image, Rgb([255, 255, 255])); 
    line(0, 0, 10, 50, &mut image, Rgb([255, 0, 0])); 

    image = flip_vertical(&image);
    
    image.save("image.png").unwrap();
}
