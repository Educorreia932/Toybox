use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra::{Vector3};

pub struct Model {
    pub verts: Vec<Vector3<f32>>,
    pub faces: Vec<Vector3<i32>>,
}

impl Model {
    pub fn from_file(filename: &str) -> Self {
        let mut verts: Vec<Vector3<f32>> = Vec::new();
        let mut faces: Vec<Vector3<i32>> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if line.chars().count() >= 2 {
                let s = line[0..2].trim();

                if s == "v" {
                    let line_verts: Vec<f32> = line[2..].trim().split(' ').flat_map(str::parse::<f32>).collect();

                    verts.push(Vector3::new(line_verts[0], line_verts[1], line_verts[2]));
                }   
    
                else if line[0..2].trim() == "f" {
                    let mut face = Vec::new();

                    for f in line[2..].trim().split(' ') {
                        let first: &str = f.split("/").collect::<Vec<&str>>()[0];
                        face.push(first.parse::<i32>().unwrap() - 1);
                    }

                    faces.push(Vector3::new(face[0], face[1], face[2]));
                }
            }
        }

        println!("Read model with success");
        println!("{} vertices", verts.len());
        println!("{} faces", faces.len());

        return Model {
            verts,
            faces
        }
    }
}
