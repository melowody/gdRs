use crate::utils::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct DeleteLevelUserForm<'f> {
    accountID: u32,
    gjp: &'f RawStr,
    levelID: u32,
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>,
}
#[warn(dead_code)]

#[post("/deleteGJLevelUser20.php", data = "<data>")]
pub fn deleteGJLevelUser20(data: Form<DeleteLevelUserForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    if !accounts::verify(data.accountID, data.gjp.url_decode().unwrap()) {
        return "-1".to_string();
    }

    let res: u64 = sql::CONN.lock().unwrap().exec_iter("DELETE FROM levels WHERE levelID=:level_id AND authorID=:author_id AND stars=0 LIMIT 1",
        mysql::params!{
            "level_id" => data.levelID,
            "author_id" => data.accountID
        }
    ).unwrap().affected_rows();

    if res == 0 {
        return "-1".to_string();
    }

    "1".to_string()
}

#[get("/deleteGJLevelUser20.php")]
pub fn deleteGJLevelUser20GET() -> String {
    "-1".to_string()
}