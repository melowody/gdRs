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
pub struct UploadAccCommentForm<'f> {
    accountID: i32,
    gjp: &'f RawStr,
    comment: &'f RawStr,
    secret: &'f RawStr,
    gameVersion: Option<i32>,
    binaryVersion: Option<i32>,
    gdw: Option<i32>,
    ctype: Option<i32>,
    chk: Option<&'f RawStr>
}
#[warn(dead_code)]

#[post("/uploadGJAccComment20.php", data = "<data>")]
pub fn uploadGJAccComment20(data: Form<UploadAccCommentForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    let comment: String = data.comment.url_decode().unwrap();
    let gjp: String = data.gjp.url_decode().unwrap();

    if !accounts::verify(data.accountID, gjp) {
        return "-1".to_string();
    }

    let next_id: i32 = sql::get_next_comment_id();

    // Attempt to delete the comment if by the user
    let res: u64 = sql::CONN.lock().unwrap().exec_iter("INSERT INTO comments (commentID, comment, sourceID, type, authorID, date, likes) VALUES (:next_id, :comment_str, :source_id, :type_id, :author_id, :date_num, 0)",
        mysql::params!{
            "next_id" => next_id,
            "comment_str" => comment,
            "source_id" => data.accountID,
            "type_id" => comment::CommentType::AccountComment as i32,
            "author_id" => data.accountID,
            "date_num" => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
        }
    ).unwrap().affected_rows();

    if res == 0 {
        return "-1".to_string();
    }

    "1".to_string()
}

#[get("/uploadGJAccComment20.php")]
pub fn uploadGJAccComment20GET() -> String {
    "-1".to_string()
}