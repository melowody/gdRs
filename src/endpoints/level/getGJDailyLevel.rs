use crate::utils::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;
use std::time::*;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct GetDailyLevelForm<'f> {
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>,
    accountID: Option<i32>,
    gjp: Option<&'f RawStr>,
    weekly: Option<i32>
}
#[warn(dead_code)]

#[post("/getGJDailyLevel.php", data = "<data>")]
pub fn getGJDailyLevel(data: Form<GetDailyLevelForm>) -> String {

    let typ: i8;
    let to_add: i32;

    let time_until: i64 = match data.weekly {
        Some(_) => {
            typ = 2;
            to_add = 100001;
            utime::time_until_monday_in_millis()
        }
        None => {
            typ = 1;
            to_add = 0;
            utime::time_until_midnight_in_millis()
        }
    };

    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    println!("{}", time);
    println!("{}", typ);

    let mut level_id: Row = match sql::CONN.lock().unwrap().exec_first("SELECT * FROM dailys WHERE time < :curr AND type = :type ORDER BY time DESC",
        mysql::params! {
            "curr" => time,
            "type" => typ
        }
    ).unwrap() {
        Some(r) => r,
        None => {
            return "-1".to_string();
        }
    };

    let level_id: i32 = level_id.take::<i32,_>("levelID").unwrap() + to_add;

    format!("{}|{}", level_id, time_until / 1000).to_string()
    
}

#[get("/getGJDailyLevel.php")]
pub fn getGJDailyLevelGET() -> String {
    "-1".to_string()
}