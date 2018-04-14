#[macro_use]
extern crate serde_derive;

mod simulator;
mod ds;

pub use simulator::Simulator;
pub use ds::{Viewport, Frame};

use std::env;
use std::path::Path;
use std::fs::{self};
use std::io;
use std::fs::DirEntry;

fn iterate_userfile(p: &Path) -> io::Result<()> {
    if p.is_dir() {
        for entry in fs::read_dir(p)? {
            let dir = entry?;
            println!("{:?}", dir.path());
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let object_result = args[1].clone();
    let dump_file: String = args[2].clone();
    let cluster_json: String = args[3].clone();
    let threshold = args[4].parse::<f32>().unwrap();
    let segment = args[5].parse::<usize>().unwrap();
    let width = args[6].parse::<usize>().unwrap();
    let height = args[7].parse::<usize>().unwrap();
    let l2_width = args[8].parse::<usize>().unwrap();
    let l2_height = args[9].parse::<usize>().unwrap();

    for entry in fs::read_dir(Path::new(&object_result)).unwrap() {
        let dir: DirEntry = entry.unwrap();
        let user_file = dir.path().to_str().unwrap().to_string();
        println!("{}", user_file);
        let mut simulator = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, width, height, l2_width, l2_height);
        simulator.hierarchical_simulate();
    }
}
