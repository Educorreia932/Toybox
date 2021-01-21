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

    if x0 == x1 || x0 == 800 || y0 == 800 || x1 == 800 || y1 == 800 {
        return;
    }

    for x in x0..=x1 {
        let t: f32 = (x - x0) as f32 / (x1 - x0) as f32;
        let y: i32 = (y0 as f32 * (1.0 - t) + y1 as f32 * t) as i32;

        if steep {
            (*image).put_pixel(y as u32, x as u32, color);
        }

        else {
            (*image).put_pixel(x as u32, y as u32, color);
        }
    }
}

fn main() {
    let width = 800;
    let height = 800;

    let model = model::Model::from_file("obj/african_head.obj");

    let mut image : RgbImage = ImageBuffer::new(width, height);

    for face in model.faces {
        for i in 0..3 {
            let v0 = model.verts.get(face[i] as usize).unwrap();
            let v1 = model.verts.get(face[(i + 1) % 3] as usize).unwrap();
            
            let x0 = ((v0[0] + 1.0) * width as f32 / 2.0) as i32;
            let y0 = ((v0[1] + 1.0) * height as f32 / 2.0) as i32;
            let x1 = ((v1[0] + 1.0) * width as f32 / 2.0) as i32;
            let y1 = ((v1[1] + 1.0) * height as f32 / 2.0) as i32;

            line(x0, y0, x1, y1, &mut image, Rgb([255, 255, 255]));
        }
    }

    image = flip_vertical(&image);
    
    image.save("image.png").unwrap();
}
