use ds::{Frame, Viewport};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::f32;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Copy, Clone)]
enum CacheLevel {
    LevelOne,
    LevelTwo,
    LevelThree,
}

#[derive(Debug, Copy, Clone)]
struct Hit {
    index: usize,
    ratio: f32,
    path: usize,
    cache_level: CacheLevel,
    width: usize,
    height: usize,
}

pub struct Simulator {
    user_file: String,
    dump_file: String,
    cluster_json: String,
    threshold: f32,
    segment: usize,
    fov_width: usize,
    fov_height: usize,
    path_list: Vec<Vec<Viewport>>,
    user_fov_list: Vec<Viewport>,
    current_cache_level: CacheLevel,
    level_two_width: usize,
    level_two_height: usize,
    hit_list: Vec<Hit>,
}

#[derive(Deserialize, Debug)]
struct VideoObject {
    from_start: usize,
    from_end: usize,
    size: usize,
    cluster: Vec<usize>,
}

fn read_json_cluster_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<VideoObject>, Box<Error>> {
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `Vec[VideoObject]`.
    let u = serde_json::from_reader(file)?;

    // Return the `VideoObject`.
    Ok(u)
}


impl Simulator {
    pub fn new(user_file: &String, dump_file: &String, cluster_json: &String, threshold: f32, segment: usize, fov_width: usize, fov_height: usize, level_two_width: usize, level_two_height: usize) -> Self {
        let mut sim = Simulator {
            user_file: user_file.to_string(),
            dump_file: dump_file.to_string(),
            cluster_json: cluster_json.to_string(),
            threshold,
            segment,
            fov_width,
            fov_height,
            path_list: vec![],
            user_fov_list: vec![],
            current_cache_level: CacheLevel::LevelOne,
            level_two_width,
            level_two_height,
            hit_list: vec![],
        };
        sim.parse_tracing_to_path_list();
        sim.parse_user_data();
        sim
    }

    // update self.path_list
    fn parse_tracing_to_path_list(&mut self) {
        let file = File::open(&self.dump_file).unwrap();
        let buf_reader = BufReader::new(&file);
        let mut traces: Vec<Viewport> = vec![];
        let mut frame_id = 0;
        let mut frame_list: Vec<Frame> = vec![];

        for line in buf_reader.lines() {
            let line = match line {
                Ok(T) => T,
                Err(_) => return (),
            };
            let id_vec: Vec<&str> = line.split(" ").collect();
            frame_id = (&id_vec[0]).parse::<i32>().unwrap();
            let object_id = (&id_vec[1]).parse::<i32>().unwrap();

            let coord: Vec<&str> = id_vec[2].split(",").collect();
            let x = (&coord[0]).parse::<i32>().unwrap();
            let y = (&coord[1]).parse::<i32>().unwrap();
            let width = (&coord[2]).parse::<usize>().unwrap();
            let height = (&coord[3]).parse::<usize>().unwrap();
            let viewport = Viewport::new(100, x, y, width, height);

            if object_id == 0 {
                if frame_id != 1 {
                    frame_list.push(Frame::new(frame_id, &traces));
                }
                traces.clear();
            }
            traces.push(viewport);
        }
        // viewport in frame_list is not normalized using our fov size yet
        frame_list.push(Frame::new(frame_id, &traces));

        // integrate cluster_json and trace dump
        let video_objects = read_json_cluster_from_file(&self.cluster_json).unwrap();
        for video_object in video_objects {
            // -1 is for normalize the start id in trace dump file to 0
            // so that we have the same start id as we get from user_view_port_result
            let start = video_object.from_start - 1;
            let end = video_object.from_end - 1;
            let mut frame_id = start;
            let mut path: Vec<Viewport> = vec![];

            // iterate all the frames from dumping data
            for frame in frame_list[start..end].iter() {
                for cluster in &video_object.cluster {
                    let v = frame.traces[*cluster];
                    path.push(Viewport::create_new_with_size(&v, self.fov_width, self.fov_height));
                }
                self.path_list.push(path.clone());
                path.clear();
            }
//            println!("{}: {:?}", start, self.path_list[start]);
        }
    }

    // update user_fov_list
    fn parse_user_data(&mut self) {
        let file = File::open(&self.user_file).unwrap();
        let buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let line_split: Vec<&str> = line.split(" ").collect();
            let key = (&line_split[0]).parse::<usize>().unwrap();
            let conf = (&line_split[1]).parse::<i32>().unwrap();

            let extract: Vec<&str> = line_split[2].split(",").collect();
            let x = (&extract[0]).parse::<i32>().unwrap();
            let y = (&extract[1]).parse::<i32>().unwrap();
            let width = (&extract[2]).parse::<usize>().unwrap();
            let height = (&extract[3]).parse::<usize>().unwrap();
            let u_fov = Viewport::new(conf, x, y, width, height);
            // assume user_viewport file has key start from 0 and add one consecutively
            self.user_fov_list.push(u_fov);
        }
//        println!("{:?}", self.user_fov_list);
    }

    fn compare_from_level_one(&self, fov: &Viewport, user_fov: &Viewport, index: usize, path: usize, width: usize, height: usize) -> (Hit, CacheLevel) {
        let ratio = fov.get_cover_result(user_fov);
        let hit: Hit;
        if ratio >= self.threshold {
            println!("L1 hit {} at {}", index, ratio);
            hit = Hit {
                index,
                ratio,
                cache_level: CacheLevel::LevelOne,
                path,
                width,
                height,
            };
            (hit, CacheLevel::LevelOne)
        } else {
            println!("L1 miss {} at {}", index, ratio);
            self.compare_from_level_two(&fov, &user_fov, index, path)
        }
    }

    fn compare_from_level_two(&self, fov: &Viewport, user_fov: &Viewport, index: usize, path: usize) -> (Hit, CacheLevel) {
        let level_one_ratio = fov.get_cover_result(user_fov);
        let hit: Hit;
        let level_two_viewport = Viewport::create_new_with_size(&fov, self.level_two_width, self.level_two_height);
        let level_two_ratio = level_two_viewport.get_cover_result(user_fov);
        if level_two_ratio >= self.threshold {
            println!("L2 hit {} at {}", index, level_two_ratio);
            hit = Hit {
                index,
                ratio: level_two_ratio,
                cache_level: CacheLevel::LevelTwo,
                path,
                width: self.level_two_width,
                height: self.level_two_height,
            };
            (hit, CacheLevel::LevelTwo)
        } else {
            println!("L2 miss {} at {}", index, level_two_ratio);
            if level_two_ratio < level_one_ratio {
                println!("l1 {:?}", fov);
                println!("l2 {:?}", level_two_viewport);
                println!("user {:?}", user_fov);
                assert!(false);
            }
            self.compare_from_level_three(index, path)
        }
    }

    fn compare_from_level_three(&self, index: usize, path: usize) -> (Hit, CacheLevel) {
        println!("L3 hit {} at {}", index, 1);
        let hit: Hit;
        hit = Hit {
            index,
            ratio: 1.0,
            cache_level: CacheLevel::LevelThree,
            path,
            width: 3840,
            height: 2160,
        };
        (hit, CacheLevel::LevelThree)
    }

    pub fn hierarchical_simulate(&mut self) {
        let mut current_path: Option<usize> = None;
        let mut hit_cache_pair: (Hit, CacheLevel) = (Hit {
            index: 0,
            ratio: 0.0,
            cache_level: CacheLevel::LevelOne,
            path: 0,
            width: 0,
            height: 0,
        }, CacheLevel::LevelOne);
        for (k, user_fov) in self.user_fov_list.iter().enumerate() {
            let mut max_ratio: f32 = f32::NEG_INFINITY;
            let mut max_ratio_path: Option<usize> = None;
            let mut temp_viewport: Option<&Viewport> = None;
            let width = self.fov_width;
            let height = self.fov_height;

            for (path, path_viewport) in self.path_list[k].iter().enumerate() {
                let current_ratio = path_viewport.get_cover_result(user_fov);
                if max_ratio < current_ratio {
                    max_ratio = current_ratio;
                    max_ratio_path = Some(path);
                    temp_viewport = Some(path_viewport);
                }
            }

            if k % self.segment == 0 {
                current_path = max_ratio_path;
                hit_cache_pair = self.compare_from_level_one(&temp_viewport.unwrap(), &user_fov, k, max_ratio_path.unwrap(), width, height);
                self.hit_list.push(hit_cache_pair.0.clone());
            } else {
//                println!("k: {}, path: {}, ratio: {}", k, max_ratio_path.unwrap(), max_ratio);
                match hit_cache_pair.1 {
                    CacheLevel::LevelOne => {
                        if current_path == max_ratio_path {
                            hit_cache_pair = self.compare_from_level_one(&temp_viewport.unwrap(), &user_fov, k, max_ratio_path.unwrap(), width, height);
                        } else {
                            hit_cache_pair = self.compare_from_level_three(k, max_ratio_path.unwrap());
                        }
                    }
                    CacheLevel::LevelTwo => {
                        if current_path == max_ratio_path {
                            hit_cache_pair = self.compare_from_level_two(&temp_viewport.unwrap(), &user_fov, k, max_ratio_path.unwrap());
                        } else {
                            hit_cache_pair = self.compare_from_level_three(k, current_path.unwrap());
                        }
                    }
                    CacheLevel::LevelThree => {
                        hit_cache_pair = self.compare_from_level_three(k, current_path.unwrap());
                    }
                }
                self.hit_list.push(hit_cache_pair.0.clone());
            }
        }
        assert_eq!(self.hit_list.len(), self.user_fov_list.len());
//        println!("{:?}", self.hit_list);
    }
}