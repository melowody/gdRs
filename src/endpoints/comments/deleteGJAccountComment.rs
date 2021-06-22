use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the deleteGJAccComment20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct DeleteAccCommentForm<'f> {
    accountID: u32,
    gjp: &'f RawStr,
    commentID: u32,
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>
}
#[warn(dead_code)]

#[post("/deleteGJAccComment20.php", data = "<data>")]
pub fn deleteGJAccComment20(data: Form<DeleteAccCommentForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    // Make sure the credentials are correct
    let verified = accounts::verify(data.accountID, data.gjp.url_decode().unwrap());
    if !verified {
        return "-1".to_string();
    }

    // Run the SQL command to delete the comment
    sql::CONN.lock().unwrap().exec_drop("DELETE FROM comments WHERE commentID=:comment_id AND sourceID=:source_id AND type=:type_id",
        mysql::params!{
            "comment_id" => data.commentID,
            "source_id" => data.accountID,
            "type_id" => comment::CommentType::AccountComment as i32,
        }
    ).unwrap();

    return "1".to_string();
}

#[get("/deleteGJAccComment20.php")]
pub fn deleteGJAccComment20GET() -> String {
    "-1".to_string()
}