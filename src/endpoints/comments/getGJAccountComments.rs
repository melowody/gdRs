use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct GetAccCommentForm<'f> {
    accountID: i32,
    page: i32,
    secret: &'f RawStr,
    gameVersion: Option<i32>,
    binaryVersion: Option<i32>,
    gdw: Option<i32>
}
#[warn(dead_code)]

#[post("/getGJAccountComments20.php", data = "<data>")]
pub fn getGJAccountComments20(data: Form<GetAccCommentForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    // Get a vector of Account Comments from the user id
    let comments: Vec<comment::AccComment> = sql::CONN.lock().unwrap().exec_map("SELECT * FROM comments WHERE sourceID=:source_id AND type=:type_id ORDER BY date DESC LIMIT 10 OFFSET :page",
        mysql::params!{
            "source_id" => data.accountID,
            "type_id" => comment::CommentType::AccountComment as i32,
            "page" => data.page * 10,
        },
        comment::AccComment::from_row
    ).unwrap();

    // Format the vector of Strings and join them with the pipe character
    let mut out: String = (&comments)
        .into_iter()
        .map(|comment| comment.format())
        .collect::<Vec<String>>()
        .join("|");
    
    
    // Get the total amount of comments
    let num: u32 = sql::CONN.lock().unwrap().exec_first("SELECT count(*) FROM comments WHERE sourceID=:source_id AND type=:type_id",
        mysql::params! {
            "source_id" => data.accountID,
            "type_id" => comment::CommentType::AccountComment as i32
        }
    ).unwrap().unwrap();

    // Add the ending check
    out = format!("{}{}", out, format!("#{}:{}:10", num, data.page)).to_string();

    out
}

#[get("/getGJAccountComments20.php")]
pub fn getGJAccountComments20GET() -> String {
    "-1".to_string()
}