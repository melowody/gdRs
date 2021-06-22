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

#[derive(Clone, Copy)]
pub enum Difficulty {
    EASY = 1,
    NORMAL = 2,
    HARD = 3,
    HARDER = 4,
    INSANE = 5,
    DEMON = 6
}

impl Difficulty {
    pub fn from_int(i: i32) -> Difficulty {
        match i {
            1 => Difficulty::EASY,
            2 => Difficulty::NORMAL,
            3 => Difficulty::HARD,
            4 => Difficulty::HARDER,
            5 => Difficulty::INSANE,
            6 => Difficulty::DEMON,
            _ => Difficulty::EASY,
        }
    }
}

#[derive(Clone)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Color {
    pub fn from_string(s: String) -> Color {
        let colors: Vec<i32> = s.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        Color {
            r: *colors.get(0).unwrap(),
            g: *colors.get(1).unwrap(),
            b: *colors.get(2).unwrap()
        }
    }

    pub fn format(&self) -> String {
        format!("{},{},{}", self.r, self.b, self.g)
    }
}