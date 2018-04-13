#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    conf: i32,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
}

impl Viewport {
    pub fn new(conf: i32, x: i32, y: i32, width: usize, height: usize) -> Self {
        Viewport {
            conf,
            x,
            y,
            width,
            height,
        }
    }

    pub fn create_new_with_size(other_viewport: &Viewport, width: usize, height: usize) -> Viewport {
        let x = other_viewport.x + ((other_viewport.width as i32 - width as i32) / 2);
        let y = other_viewport.y + ((other_viewport.height as i32 - height as i32) / 2);
        Viewport {
            conf: other_viewport.conf,
            x,
            y,
            width,
            height,
        }
    }
}

// each frame has multiple viewport
#[derive(Debug)]
pub struct Frame {
    pub index: i32,
    pub traces: Vec<Viewport>,
}

impl Frame {
    pub fn new(index: i32, traces: &Vec<Viewport>) -> Self {
        let t = traces.clone();
        Frame {
            index,
            traces: t,
        }
    }
}

#[derive(Debug)]
pub struct Path {
    dump: String,
    frame_list: Vec<Option<Frame>>,
}


#[test]
fn test_trace() {
    let t_1 = Viewport::new(100, 700, 700, 1200, 1200);
    let t_2 = Viewport::create_new_with_size(&t_1, 2000, 2000);
}