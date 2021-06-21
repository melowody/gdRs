use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;
use std::time::*;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct UploadCommentForm<'f> {
    accountID: i32,
    gjp: &'f RawStr,
    userName: &'f RawStr,
    comment: &'f RawStr,
    secret: &'f RawStr,
    levelID: i32,
    percent: f32,
    chk: &'f RawStr,
    gameVersion: Option<i32>,
    binaryVersion: Option<i32>,
    gdw: Option<i32>,
}
#[warn(dead_code)]

#[post("/uploadGJComment21.php", data = "<data>")]
pub fn uploadGJComment21(data: Form<UploadCommentForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    if data.chk.url_decode().unwrap() != crypto::generate_chk(vec![data.userName.url_decode().unwrap(), data.comment.url_decode().unwrap(), data.levelID.to_string(), data.percent.to_string()], "29481", "0xPT6iUrtws0J") {
        return "-1".to_string();
    }

    let comment: String = data.comment.url_decode().unwrap();
    let gjp: String = data.gjp.url_decode().unwrap();

    if !accounts::verify(data.accountID, gjp) {
        return "-1".to_string();
    }

    let next_id: i32 = sql::get_next_comment_id();

    // Attempt to delete the comment if by the user
    let res: u64 = sql::CONN.lock().unwrap().exec_iter("INSERT INTO comments (commentID, comment, sourceID, type, authorID, date, likes, percent, isSpam) VALUES (:next_id, :comment_str, :source_id, :type_id, :author_id, :date_num, 0, :percent_num, 0)",
        mysql::params!{
            "next_id" => next_id,
            "comment_str" => comment,
            "source_id" => data.accountID,
            "type_id" => comment::CommentType::LevelComment as i32,
            "author_id" => data.accountID,
            "date_num" => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            "percent_num" => data.percent
        }
    ).unwrap().affected_rows();

    if res == 0 {
        return "-1".to_string();
    }

    next_id.to_string()
}

#[get("/uploadGJComment21.php")]
pub fn uploadGJComment21GET() -> String {
    "-1".to_string()
}