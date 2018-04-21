#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod simulator;
mod ds;

pub use simulator::{Simulator, PowerConstants};
pub use ds::{Viewport, Frame};

use std::env;
use std::path::Path;
use std::fs::{self};
use std::fs::File;
use std::error::Error;
use std::fs::DirEntry;

struct PythonFor(isize, isize, isize);

impl Iterator for PythonFor {
    type Item = isize;

    #[inline]
    fn next(&mut self) -> Option<isize> {
        if self.0 < self.1 {
            let v = self.0;
            self.0 = v + self.2;
            Some(v)
        } else {
            None
        }
    }
}

fn read_power_consumption_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<PowerConstants>, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

#[allow(dead_code)]
fn compare_each_simulation(object_result: &String, dump_file: &String, cluster_json: &String, threshold: f64, segment: usize, fov_width: usize, fov_height: usize, level_two_width: usize, level_two_height: usize, power_constants: Vec<PowerConstants>) {
    let mut user_paths: Vec<DirEntry> = fs::read_dir(&object_result).unwrap().map(|r| r.unwrap()).collect();
    user_paths.sort_by_key(|dir| dir.path());

    for path in user_paths {
        let user_file = path.path().to_str().unwrap().to_string();
//        println!("{}", user_file);
        let mut simulator = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, fov_width, fov_height, level_two_width, level_two_height, power_constants.clone(), false);
        simulator.simulate();
        simulator.power_consumption();
        let mut simulator_base = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, fov_width, fov_height, fov_width, fov_height, power_constants.clone(), false);
        simulator_base.simulate();
        simulator_base.power_consumption();
        let mut simulator_opt = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, fov_width, fov_height, level_two_width, level_two_height, power_constants.clone(), true);
        simulator_opt.simulate();
        simulator_opt.power_consumption();
        println!("l1-l2-hier: {:?}, l1-only: {:?}, l1-l2-opt-hier: {:?}", simulator.get_hit_ratios(), simulator_base.get_hit_ratios(), simulator_opt.get_hit_ratios());
    }
}

fn single_simulation(user_paths: &Vec<DirEntry>, dump_file: &String, cluster_json: &String, threshold: f64, segment: usize, fov_width: usize, fov_height: usize, level_two_width: usize, level_two_height: usize, power_constants: Vec<PowerConstants>) {
    let mut pc_tuple: (f64, f64) = (0.0, 0.0);
    let mut count = 0;
    for path in user_paths {
        let user_file = path.path().to_str().unwrap().to_string();
        let mut simulator_opt = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, fov_width, fov_height, level_two_width, level_two_height, power_constants.clone(), true);
        simulator_opt.simulate();
        pc_tuple.0 += simulator_opt.get_wifi_pc();
        pc_tuple.1 += simulator_opt.get_soc_pc();
        count += 1;
//        println!("{} {} {}", simulator_opt.get_wifi_pc(), simulator_opt.get_soc_pc(), count);
    }
    // wifi soc screen level_2
    println!("{} {} {} {}", pc_tuple.0 / count as f64, pc_tuple.1 / count as f64, fov_width, level_two_width);
}

#[allow(dead_code)]
fn batch_simulation(object_result: &String, dump_file: &String, cluster_json: &String, threshold: f64, segment: usize, power_constants: Vec<PowerConstants>) {
    let mut user_paths: Vec<DirEntry> = fs::read_dir(&object_result).unwrap().map(|r| r.unwrap()).collect();
    user_paths.sort_by_key(|dir| dir.path());

    for screen in PythonFor(1200, 2001, 100) {
        for level_2 in PythonFor(2200, 3401, 100) {
            single_simulation(&user_paths, &dump_file, &cluster_json, threshold, segment, screen as usize, screen as usize, level_2 as usize, level_2 as usize, power_constants.clone());
        }
    }
}

#[allow(dead_code)]
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

//    compare_each_simulation(&object_result, &dump_file, &cluster_json, threshold, segment, width, height, l2_width, l2_height, power_constant);
//    batch_simulation(&object_result, &dump_file, &cluster_json, threshold, segment, power_constant);

    // for auto.sh
    let mut user_paths: Vec<DirEntry> = fs::read_dir(&object_result).unwrap().map(|r| r.unwrap()).collect();
    user_paths.sort_by_key(|dir| dir.path());
    single_simulation(&user_paths, &dump_file, &cluster_json, threshold, segment, width, height, l2_width, l2_height, power_constant);
}
