use ds::{Frame, Viewport};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::path::Path;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub struct Simulator {
    user_file: String,
    dump_file: String,
    cluster_json: String,
    threshold: f32,
    segment: usize,
    fov_width: usize,
    fov_height: usize,
    path_list: Vec<Vec<Viewport>>,
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
    pub fn new(user_file: &String, dump_file: &String, cluster_json: &String, threshold: f32, segment: usize, fov_width: usize, fov_height: usize) -> Self {
        Simulator {
            user_file: user_file.to_string(),
            dump_file: dump_file.to_string(),
            cluster_json: cluster_json.to_string(),
            threshold,
            segment,
            fov_width,
            fov_height,
            path_list: vec![],
        }
    }

    pub fn parse_tracing_to_path_list(&mut self) {
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
            let start = video_object.from_start;
            let end = video_object.from_end;
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
}