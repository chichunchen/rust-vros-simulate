#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod simulator;
mod ds;
mod constants;

pub use simulator::{Simulator, PowerConstants};
pub use ds::{Viewport, Frame};

use std::env;
use std::path::Path;
use std::fs::{self};
use std::fs::File;
use std::error::Error;
use std::fs::DirEntry;

fn read_power_consumption_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<PowerConstants>, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

fn single_simulate_pc(user_paths: &Vec<DirEntry>, dump_file: &String, cluster_json: &String,
                      threshold: f64, segment: usize, fov_width: usize, fov_height: usize,
                      level_two_width: usize, level_two_height: usize,
                      power_constants_4k_360: &Vec<PowerConstants>, power_constant_1080p: &Vec<PowerConstants>, opt: ds::Optimization) {
    let mut pc_tuple: (f64, f64) = (0.0, 0.0);
    let mut hit_ratios: (f64, f64, f64) = (0.0, 0.0, 0.0);
    let mut count = 0;
    let mut no_resend_segment_rate = 0.0;

    for path in user_paths {
        let user_file = path.path().to_str().unwrap().to_string();
        let mut simulator = Simulator::new(&user_file, &dump_file, &cluster_json, threshold, segment, fov_width, fov_height,
                                           level_two_width, level_two_height, power_constants_4k_360.clone(),
                                           power_constant_1080p.clone(), opt);
        simulator.simulate();
        pc_tuple.0 += simulator.get_wifi_pc();
        pc_tuple.1 += simulator.get_soc_pc();

        let x = simulator.get_hit_ratios();
        hit_ratios.0 += x[0];
        hit_ratios.1 += x[1];
        hit_ratios.2 += x[2];

        no_resend_segment_rate += 1.0 - simulator.get_segment_resend_cnt() as f64 / simulator.get_segment_count() as f64;

        count += 1;
//        simulator.print_power_consumption();
    }

    hit_ratios.0 /= count as f64;
    hit_ratios.1 /= count as f64;
    hit_ratios.2 /= count as f64;
    no_resend_segment_rate /= count as f64;
    println!("{} {} {} {} {} {} {}", pc_tuple.0 / count as f64, pc_tuple.1 / count as f64, threshold, hit_ratios.0, hit_ratios.1, hit_ratios.2, no_resend_segment_rate);
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
    let mode = args[10].clone();    // pc or hit rate
    let opt_flag = {
        match args[11].as_ref() {
            "O0" => ds::Optimization::O0,
            "O1" => ds::Optimization::O1,
            _ => panic!("opt parse failed!")
        }
    };

    let power_constant_4k_360: Vec<PowerConstants> = read_power_consumption_from_file(Path::new("power_4k_360.json")).unwrap();
    let power_constant_1080p: Vec<PowerConstants> = read_power_consumption_from_file(Path::new("power_1080p.json")).unwrap();

//    compare_each_simulation(&object_result, &dump_file, &cluster_json, threshold, segment, width, height, l2_width, l2_height, &power_constant);
//    batch_simulation(&object_result, &dump_file, &cluster_json, threshold, segment, &power_constant);

// for auto.sh
    let mut user_paths: Vec<DirEntry> = fs::read_dir(&object_result).unwrap().map(|r| r.unwrap()).collect();
    user_paths.sort_by_key(|dir| dir.path());
    match mode.as_ref() {
        "power" => {
            single_simulate_pc(&user_paths, &dump_file, &cluster_json, threshold, segment,
                               width, height, l2_width,
                               l2_height, &power_constant_4k_360, &power_constant_1080p, opt_flag);
        }
        "hit" => {
            panic!("hit deprecated");
        }
        _ => assert!(false),
    }
}
