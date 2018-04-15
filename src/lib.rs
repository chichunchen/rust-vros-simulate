#[macro_use]
extern crate serde_derive;

mod simulator;
mod ds;

pub use simulator::{Simulator, PowerConstants};
pub use ds::{Viewport, Frame};

use std::env;
use std::path::Path;
use std::fs::{self};
use std::io;
use std::fs::DirEntry;
use std::fs::File;
use std::error::Error;

extern crate serde;
extern crate serde_json;

fn read_power_consumption_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<PowerConstants>, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let object_result = args[1].clone();
    let dump_file: String = args[2].clone();
    let cluster_json: String = args[3].clone();
    let threshold = args[4].parse::<f64>().unwrap();
    let segment = args[5].parse::<usize>().unwrap();
    let width = args[6].parse::<usize>().unwrap();
    let height = args[7].parse::<usize>().unwrap();
    let l2_width = args[8].parse::<usize>().unwrap();
    let l2_height = args[9].parse::<usize>().unwrap();

    let power_constant: Vec<PowerConstants> = read_power_consumption_from_file(Path::new("power.json")).unwrap();

    let mut user_paths: Vec<_> = fs::read_dir(&object_result).unwrap().map(|r| r.unwrap()).collect();
    user_paths.sort_by_key(|dir| dir.path());
    for path in user_paths {
        let user_file = path.path().to_str().unwrap().to_string();
        println!("{}", user_file);

        let mut simulator = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, width, height, l2_width, l2_height, power_constant.clone());
        simulator.simulate();
        simulator.get_hit_ratios();
        simulator.power_consumption();
        let mut simulator_base = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, width, height, width, height, power_constant.clone());
        simulator_base.simulate();
        simulator_base.power_consumption();
        println!("hier: {:?}, baseline: {:?}", simulator.get_hit_ratios(), simulator_base.get_hit_ratios());
    }
}
