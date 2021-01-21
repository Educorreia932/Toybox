use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Model {
    verts: Vec<i32>,
    faces: Vec<i32>,
}

impl Model {
    pub fn from_file(filename: &str) -> Self {
        let faces = Vec::<i32>::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
        }

        return Model {
            verts: Vec::new(),
            faces
        }
    }
}
