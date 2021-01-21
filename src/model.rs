use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Model {
    pub verts: Vec<Vec<f32>>,
    pub faces: Vec<Vec<i32>>,
}

impl Model {
    pub fn from_file(filename: &str) -> Self {
        let mut verts: Vec<Vec<f32>> = Vec::new();
        let mut faces: Vec<Vec<i32>> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if line.chars().count() >= 2 {
                let s = line[0..2].trim();

                if s == "v" {
                    let line_verts = line[2..].trim().split(' ').flat_map(str::parse::<f32>).collect();
                    verts.push(line_verts);
                }   
    
                else if line[0..2].trim() == "f" {
                    let mut face = Vec::new();

                    for f in line[2..].trim().split(' ') {
                        let first: &str = f.split("/").collect::<Vec<&str>>()[0];
                        face.push(first.parse::<i32>().unwrap() - 1);
                    }

                    faces.push(face);
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
