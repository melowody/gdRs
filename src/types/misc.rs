#[derive(FromPrimitive, Clone, Copy)]
pub enum IconType {
    CUBE = 1,
    SHIP = 2,
    BALL = 3,
    UFO = 4,
    WAVE = 5,
    ROBOT = 6,
    SPIDER = 7
}

impl IconType {
    pub fn from_string(t: String) -> IconType {
        let st: &str = &t;
        match st {
            "cube" => IconType::CUBE,
            "ship" => IconType::SHIP,
            "ball" => IconType::BALL,
            "ufo" => IconType::UFO,
            "wave" => IconType::WAVE,
            "robot" => IconType::ROBOT,
            "spider" => IconType::SPIDER,
            _ => IconType::CUBE
        }
    }
}