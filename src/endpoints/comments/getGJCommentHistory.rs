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
pub struct GetCommentHistForm<'f> {
    userID: i32,
    page: i32,
    secret: &'f RawStr,
    gameVersion: Option<i32>,
    binaryVersion: Option<i32>,
    gdw: Option<i32>,
    mode: Option<i32>,
    total: Option<i32>,
    count: Option<i32>
}
#[warn(dead_code)]

#[post("/getGJCommentHistory.php", data = "<data>")]
pub fn getGJCommentHistory(data: Form<GetCommentHistForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    let mode: String = match remove!(data.mode, 0) {
        1 => "likes".to_string(),
        _ => "commentID".to_string()
    };
    let count: i32 = remove!(data.count, 10);

    // Get a vector of Account Comments from the user id
    let comments: Vec<comment::LevelComment> = sql::CONN.lock().unwrap().exec_map(format!("SELECT * FROM comments WHERE authorID=:source_id AND type=:type_id ORDER BY {} DESC LIMIT {} OFFSET {}", mode, count, data.page * count), //  DESC LIMIT :res_count OFFSET :offset_num;
        mysql::params!{
            "source_id" => data.userID,
            "type_id" => comment::CommentType::LevelComment as i32
        },
        comment::LevelComment::from_row
    ).unwrap();

    let mut out: String = comments
        .into_iter()
        .map(|comment: comment::LevelComment| {
            let mut acc = account::Account::new();
            acc.get_by_id(comment.author_id);
            format!("2~{}~1~{}~3~{}~4~{}~10~{}~9~{}~6~{}:{}", encode(&comment.comment), comment.level_id, comment.author_id, comment.likes, comment.percentage, comment.format_date(), comment.comment_id,
                format!("1~{}~9~{}~10~{}~11~{}~14~{}~15~{}~16~{}", acc.username, acc.icon_id, acc.player_color, acc.player_color_2, acc.icon_type as u32, acc.special, acc.account_id)
            )
        })
        .collect::<Vec<String>>()
        .join("|");
    
    // Get the total amount of comments
    let num: Vec<Row> = sql::CONN.lock().unwrap().exec("SELECT count(*) FROM comments WHERE sourceID=:source_id AND type=:type_id",
        mysql::params! {
            "source_id" => data.userID,
            "type_id" => comment::CommentType::AccountComment as i32
        }
    ).unwrap();
    
    out = format!("{}{}", out, format!("#{}:{}:{}", num.len(), data.page, count));

    out
}

#[get("/getGJCommentHistory.php")]
pub fn getGJCommentHistoryGET() -> String {
    "-1".to_string()
}