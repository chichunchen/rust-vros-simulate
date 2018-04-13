#[macro_use] extern crate serde_derive;

mod simulator;
mod ds;

pub use simulator::Simulator;
pub use ds::{Viewport, Frame};

use std::env;
use std::path::Path;
use std::fs::{self};
use std::io;

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
    let user_file: String = args[1].clone();
    let dump_file: String = args[2].clone();
    let cluster_json: String = args[3].clone();
    let threshold = args[4].parse::<f32>().unwrap();
    let segment = args[5].parse::<usize>().unwrap();

//    let p: &Path = Path::new(&object_result);
//    iterate_userfile(p);
    let mut simulator = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, 1800, 1800);
    simulator.parse_tracing_to_path_list();
    simulator.parse_user_data();
}
