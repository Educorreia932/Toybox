use image::{RgbImage, Rgb, ImageBuffer};
use image::imageops::{flip_vertical};
use nalgebra::{Point2};
use rand::Rng;
use std::mem;

mod model;

static WHITE: Rgb<u8> = Rgb([255, 255, 255]);
static RED: Rgb<u8> = Rgb([255, 0, 0]);
static GREEN: Rgb<u8> = Rgb([0, 255, 0]);

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

fn triangle(mut t0: Point2<i32>, mut t1: Point2<i32>, mut t2: Point2<i32>, image: &mut RgbImage, color: Rgb<u8>) {
    // Sort coordinates

    if t0.y > t1.y {
        mem::swap(&mut t0, &mut t1);
    }

    if t0.y > t2.y {
        mem::swap(&mut t0, &mut t2);
    }

    if t1.y > t2.y {
        mem::swap(&mut t1, &mut t2);
    };

    let total_height = t2.y - t0.y;

    for i in 0..total_height {
        let second_half: bool = i > t1.y - t0.y || t1.y == t0.y;
        let segment_height: i32 = if second_half { t2.y-t1.y } else { t1.y - t0.y };
        let alpha: f32 = i as f32 / total_height as f32;
        let beta: f32 = (i as f32 - (if second_half { t1.y as f32 - t0.y as f32} else { 0.0 })) / segment_height as f32;

        let mut a_i = Point2::<i32>::new(t2.x - t0.x, t2.y - t0.y);
        let a_f = Point2::<f32>::new(a_i.x as f32, a_i.y as f32) * alpha;
        a_i = Point2::<i32>::new(t0.x + a_f.x as i32, t0.y + a_f.y as i32);

        let mut b_i;


        if second_half {
            b_i = Point2::<i32>::new(t2.x - t1.x, t2.y - t1.y);
            let b_f = Point2::<f32>::new(b_i.x as f32, b_i.y as f32) * beta;
            b_i = Point2::<i32>::new(t1.x + b_f.x as i32, t1.y + b_f.y as i32);
        }

        else {
            b_i = Point2::<i32>::new(t1.x - t0.x, t1.y - t0.y);
            let b_f = Point2::<f32>::new(b_i.x as f32, b_i.y as f32) * beta;
            b_i = Point2::<i32>::new(t0.x + b_f.x as i32, t0.y + b_f.y as i32);
        }

        if a_i.x > b_i.x {
            mem::swap(&mut a_i, &mut b_i);
        }

        for j in a_i.x..b_i.x {
            if j < 800 {
                (*image).put_pixel(j as u32, (t0.y + i) as u32, color);
            }
        }
    }
}

fn draw_model(model: model::Model, image: &mut RgbImage, width: u32, height: u32) {
    for face in model.faces {
        let mut screen_coordinates: Vec<Point2<i32>> = Vec::new();

        for i in 0..3 {
            let world_coordinates = model.verts.get(face[i] as usize).unwrap();

            screen_coordinates.push(
                Point2::new(
                    ((world_coordinates.x + 1.0) * width as f32 / 2.0) as i32,
                    ((world_coordinates.y + 1.0) * height as f32 / 2.0) as i32
                )
            );
        }

        let mut rng = rand::thread_rng();
        let color =  Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);

        triangle(screen_coordinates[0], screen_coordinates[1], screen_coordinates[2], image, color);
    }
}

fn main() {
    let width: u32 = 800;
    let height: u32 = 800;

    let mut image: RgbImage = ImageBuffer::new(width, height);

    let model = model::Model::from_file("obj/african_head.obj");
    draw_model(model, &mut image, width, height);

    image = flip_vertical(&image);

    image.save("image.png").unwrap();
}
