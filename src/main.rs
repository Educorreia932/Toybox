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

fn triangle(t0: &Vec<i32>, t1 : &Vec<i32>, t2: &Vec<i32>, image: &mut RgbImage, color: Rgb<u8>) {
    line(t0[0], t0[1], t1[0], t1[1], image, color);
    line(t1[0], t1[1], t2[0], t2[1], image, color);
    line(t2[0], t2[1], t0[0], t0[1], image, color);
}   

fn draw_model(model: model::Model, image: &mut RgbImage, width: u32, height: u32) {
    for face in model.faces {
        for i in 0..3 {
            let v0 = model.verts.get(face[i] as usize).unwrap();
            let v1 = model.verts.get(face[(i + 1) % 3] as usize).unwrap();
            
            let x0 = ((v0[0] + 1.0) * width as f32 / 2.0) as i32;
            let y0 = ((v0[1] + 1.0) * height as f32 / 2.0) as i32;
            let x1 = ((v1[0] + 1.0) * width as f32 / 2.0) as i32;
            let y1 = ((v1[1] + 1.0) * height as f32 / 2.0) as i32;

            line(x0, y0, x1, y1, image, Rgb([255, 255, 255]));
        }
    }
}

fn main() {
    let width: u32 = 800;
    let height: u32 = 800;

    let mut image : RgbImage = ImageBuffer::new(width, height);

    let red = Rgb([255, 0, 0]);
    let white = Rgb([255, 255, 255]);
    let green = Rgb([0, 255, 0]);
   
    let model = model::Model::from_file("obj/african_head.obj");
    draw_model(model, &mut image, width, height);
    
    // let t0 = vec![vec![10, 70], vec![50, 160], vec![70, 80]];
    // let t1 = vec![vec![180, 50], vec![150, 1], vec![70, 180]];
    // let t2 = vec![vec![180, 150], vec![120, 160], vec![130, 180]];

    // triangle(&t0[0], &t0[1], &t0[2], &mut image, red); 
    // triangle(&t1[0], &t1[1], &t1[2], &mut image, white); 
    // triangle(&t2[0], &t2[1], &t2[2], &mut image, green);

    image = flip_vertical(&image);
    
    image.save("image.png").unwrap();
}
