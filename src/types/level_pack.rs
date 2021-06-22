use mysql::*;
use crate::types::misc::*;

#[derive(Clone)]
pub struct Gauntlet {
    pub gauntlet_id: i32,
    pub levels: Vec<i32>
}

impl PartialEq for Gauntlet {
    fn eq(&self, other: &Self) -> bool {
        self.gauntlet_id == other.gauntlet_id && self.levels == other.levels
    }
}

impl Gauntlet {
    /// Converts a MySQL Row to a LevelComment
    pub fn from_row(mut row: Row) -> Gauntlet {
        Gauntlet {
            gauntlet_id: row.take("gauntletID").unwrap(),
            levels: {
                let value: String = row.take("levels").unwrap();
                value.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
            }
        }
    }
}

#[derive(Clone)]
pub struct MapPack {
    pub pack_id: i32,
    pub name: String,
    pub levels: Vec<i32>,
    pub stars: i32,
    pub coins: i32,
    pub difficulty: Difficulty,
    pub text_color: Color,
    pub bar_color: Color
}

impl PartialEq for MapPack {
    fn eq(&self, other: &Self) -> bool {
        self.pack_id == other.pack_id && self.name == other.name && self.levels == other.levels && self.stars == other.stars && self.coins == other.coins && self.difficulty as i32 == other.difficulty as i32 && self.text_color == other.text_color && self.bar_color == other.bar_color
    }
}

impl MapPack {
    /// Converts a MySQL Row to a Map Pack
    pub fn from_row(mut row: Row) -> MapPack {
        MapPack {
            pack_id: row.take("packID").unwrap(),
            name: row.take("name").unwrap(),
            levels: {
                let value: String = row.take("levels").unwrap();
                value.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
            },
            stars: row.take("stars").unwrap(),
            coins: row.take("coins").unwrap(),
            difficulty: Difficulty::from_int(row.take::<i32,_>("difficulty").unwrap()),
            text_color: Color::from_string(row.take::<String,_>("textColor").unwrap()),
            bar_color: Color::from_string(row.take::<String,_>("barColor").unwrap()),
        }
    }
}