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
        let mut x = other_viewport.x + ((other_viewport.width as i32 - width as i32) / 2);
        let y = other_viewport.y + ((other_viewport.height as i32 - height as i32) / 2);
        if x < 0 {
            x = 3840 + x;
        }
        Viewport {
            conf: other_viewport.conf,
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_cover_result(&self, user_fov: &Viewport) -> f32 {
        let mut total_x = 0;
        if (user_fov.x + user_fov.width as i32) <= 3840 && (self.x + self.width as i32) <= 3840 {
            let left = i32::max(self.x, user_fov.x);
            let right = i32::min(self.x + self.width as i32, user_fov.x + user_fov.width as i32);
            total_x = right - left;
        } else if user_fov.x + user_fov.width as i32 > 3840 && (self.x + self.width as i32) <= 3840 {
            if self.x < (user_fov.x + user_fov.width as i32 - 3840) {
                let left_1 = i32::max(self.x, 0);
                let right_1 = i32::min(self.x + self.width as i32, user_fov.x + user_fov.width as i32 - 3840);
                total_x += (right_1 - left_1);
            }
            if self.x + self.width as i32 > user_fov.x {
                let left_2 = user_fov.x;
                let right_2 = self.x + self.width as i32;
                total_x += (right_2 - left_2);
            }
        } else if self.x + self.width as i32 > 3840 {
            let left_1 = i32::max(0, user_fov.x);
            let right_1 = i32::min(self.x + self.width as i32 - 3840, user_fov.x + user_fov.width as i32);
            let left_2 = i32::max(self.x, user_fov.x);
            let right_2 = i32::min(3840, user_fov.x + user_fov.width as i32);
            if right_1 - left_1 > 0 {
                total_x += (right_1 - left_1);
            }
            if right_2 - left_2 > 0 {
                total_x += (right_2 - left_2);
            }
        }

        let bottom = i32::max(self.y, user_fov.y);
        let top = i32::min(self.y + self.height as i32, user_fov.y + user_fov.height as i32);
        let total_y = if i32::abs(top - bottom) > user_fov.height as i32 {
            user_fov.height as i32
        } else {
            i32::abs(top - bottom)
        };
        let total_x = i32::abs(total_x);
        let ratio: f32 = ((total_x * total_y) as f32 / (user_fov.width * user_fov.height) as f32);
        if ratio > 1.0 {
            println!("total_x {}", total_x);
            println!("self {:?}", self);
            println!("user {:?}", user_fov);
            assert!(false);
        }
        return ratio;
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