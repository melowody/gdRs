use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;
use base64::encode;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct DownloadLevelForm<'f> {
    levelID: i32,
    secret: &'f RawStr,
    inc: Option<u32>,
    extras: Option<u32>,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>,
    accountID: Option<i32>,
    gjp: Option<&'f RawStr>,
    udid: Option<&'f RawStr>,
    uuid: Option<&'f RawStr>,
    rs: Option<&'f RawStr>,
    chk: Option<&'f RawStr>
}
#[warn(dead_code)]

#[post("/downloadGJLevel22.php", data = "<data>")]
pub fn downloadGJLevel22(data: Form<DownloadLevelForm>) -> String {

    let mut row: Row = match sql::CONN.lock().unwrap().exec_first("SELECT * FROM levels WHERE levelID=:level_id",
        mysql::params! {
            "level_id" => data.levelID
        }
    ).unwrap() {
        Some(r) => r,
        None => {
            return "-1".to_string();
        }
    };

    let level: level::Level = level::Level::from_row(&mut row);

    let xorpass: String = encode(crypto::xor_cipher(level.pass.as_bytes(), "26364".as_bytes()));
    
    let mut out: String = format!("1:{}:2:{}:3:{}:4:{}:5:{}:6:{}:8:{}:9:{}:10:{}:12:{}:13:{}:14:{}:17:{}:43:{}:25:{}:18:{}:19:{}:42:{}:45:{}:15:{}:30:{}:31:{}:28:{}:29:{}:35:{}:36:{}:37:{}:38:{}:39:{}:46:{}:47:{}:40:{}:27:{}", level.level_id, level.name, level.description, level.level_string, level.version, level.author_id, level.difficulty_denom, level.difficulty_num, level.downloads, level.official_song, level.game_version, level.likes, level.demon as u8, level.demon_difficulty, level.auto as u8, level.stars, level.feature_score, level.epic as u8, level.objects, level.length, level.copied_id, level.two_player as u8, level.upload_date, level.update_date, level.custom_song_id, level.extra_string, level.coins, level.verified_coins as u8, level.stars_requested, level.editor_time, level.editor_time_w_copies, level.ldm as u8, xorpass).to_string();

    out = format!("{}#{}", out, crypto::hash_level_string(level.level_string));

    out = format!("{}#{}", out, crypto::sha1_salt(format!("{},{},{},{},{},{},{},{}", level.author_id, level.stars, level.demon as u8, level.level_id, level.verified_coins as u8, level.feature_score, level.pass, 0), "xI25fpAapCQg"));

    out
}

#[get("/downloadGJLevel22.php")]
pub fn downloadGJLevel22GET() -> String {
    "-1".to_string()
}