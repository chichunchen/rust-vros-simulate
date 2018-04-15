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

    pub fn get_cover_result(&self, user_fov: &Viewport) -> f64 {
        let mut total_x = 0;
        let self_rightmost = self.x + self.width as i32;
        let user_rightmost = user_fov.x + user_fov.width as i32;
        match self_rightmost {
            n if n > 3840 => {
                match user_rightmost {
                    m if m > 3840 => {
                        let left_1 = 0;
                        let right_1 = i32::min(self_rightmost - 3840, user_rightmost - 3840);
                        total_x += right_1 - left_1;
                        let left_2 = i32::max(self.x, user_fov.x);
                        let right_2 = 3840;
                        total_x += right_2 - left_2;
                    },
                    m if m <= 3840 && m >= 0 => {
                        if self.x + self.width as i32 - 3840 > user_fov.x {
                            let left = user_fov.x;
                            let right = i32::min(self_rightmost - 3840, user_rightmost);
                            total_x += right - left;
                        }
                        if (user_fov.x + user_fov.width as i32) > self.x {
                            let left = i32::max(self.x, user_fov.x);
                            let right = i32::min(3840, user_rightmost);
                            total_x += right - left;
                        }
                    },
                    _ => assert!(false),
                }
            },
            n if n <= 3840 && n >= 0 => {
                match user_rightmost {
                    m if m > 3840 => {
                        if self.x < (user_rightmost - 3840) {
                            let left_1 = i32::max(self.x, 0);
                            let right_1 = i32::min(self_rightmost, user_rightmost - 3840);
                            total_x += right_1 - left_1;
                        }
                        if self.x + self.width as i32 > user_fov.x {
                            let left_2 = user_fov.x;
                            let right_2 = self_rightmost;
                            total_x += right_2 - left_2;
                        }
                    },
                    m if m <= 3840 && m >= 0 => {
                        let left = i32::max(self.x, user_fov.x);
                        let right = i32::min(self_rightmost, user_rightmost);
                        if right - left > 0 {
                            total_x = right - left;
                        }
                    },
                    _ => assert!(false),
                }
            },
            _ => assert!(false),
        }

        let bottom = i32::max(self.y, user_fov.y);
        let top = i32::min(self.y + self.height as i32, user_fov.y + user_fov.height as i32);
        let total_y = if i32::abs(top - bottom) > user_fov.height as i32 {
            user_fov.height as i32
        } else {
            i32::abs(top - bottom)
        };
        let total_x = i32::abs(total_x);
        let ratio: f64 = ((total_x * total_y) as f64 / (user_fov.width * user_fov.height) as f64);
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


#[test]
fn test_trace() {
    let t_1 = Viewport::new(100, 700, 700, 1200, 1200);
    let t_2 = Viewport::create_new_with_size(&t_1, 2000, 2000);
}
