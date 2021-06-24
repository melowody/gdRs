use crate::utils::*;
use crate::types::misc;
use mysql::*;
use mysql::prelude::*;

pub struct Account {
    pub username: String,
    pub account_id: u32,
    pub pass: String,
    pub icon_id: u32,
    pub player_color: u32,
    pub player_color_2: u32,
    pub icon_type: misc::IconType,
    pub special: u32
}

impl Account {
    pub fn new() -> Account {
        Account {
            username: "".to_string(),
            account_id: 0,
            pass: "".to_string(),
            icon_id: 0,
            player_color: 0,
            player_color_2: 0,
            icon_type: misc::IconType::CUBE,
            special: 0
        }
    }

    pub fn get_by_id(&mut self, account_id: u32) {
        let mut row: Row = sql::CONN.lock().unwrap().exec_first("SELECT userName, pass, iconID, playerColor, playerColor2, iconType+0, special FROM accounts WHERE accountID=:id",
            mysql::params! {
                "id" => account_id
            }
        ).unwrap().unwrap();

        self.username = row.take("userName").unwrap();
        self.account_id = account_id;
        self.pass = row.take("pass").unwrap();
        self.icon_id = row.take("iconID").unwrap();
        self.player_color = row.take("playerColor").unwrap();
        self.player_color_2 = row.take("playerColor2").unwrap();
        self.icon_type = misc::IconType::from_string(row.take::<String, _>("iconType").unwrap());
        self.special = row.take("special").unwrap();
    }
}