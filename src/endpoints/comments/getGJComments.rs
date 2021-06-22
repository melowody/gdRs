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
pub struct GetCommentsForm<'f> {
    levelID: u32,
    page: u32,
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>,
    mode: Option<u32>,
    total: Option<u32>,
    count: Option<u32>
}
#[warn(dead_code)]

#[post("/getGJComments21.php", data = "<data>")]
pub fn getGJComments21(data: Form<GetCommentsForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    let mode: String = match remove!(data.mode, 0) {
        1 => "likes".to_string(),
        _ => "commentID".to_string()
    };
    let count: u32 = remove!(data.count, 10);

    // Get a vector of Account Comments from the user id
    let comments: Vec<comment::LevelComment> = sql::CONN.lock().unwrap().exec_map(format!("SELECT * FROM comments WHERE sourceID=:source_id AND type=:type_id ORDER BY {} DESC LIMIT {} OFFSET {}", mode, count, data.page * count), //  DESC LIMIT :res_count OFFSET :offset_num;
        mysql::params!{
            "source_id" => data.levelID,
            "type_id" => comment::CommentType::LevelComment as u32
        },
        comment::LevelComment::from_row
    ).unwrap();

    let mut out: String = comments
        .into_iter()
        .map(|comment: comment::LevelComment| {
            let mut acc = account::Account::new();
            acc.get_by_id(comment.author_id);
            format!("2~{}~3~{}~4~{}~7~{}~10~{}~9~{}~6~{}:{}", encode(&comment.comment), comment.author_id, comment.likes, comment.is_spam as u32, comment.percentage, comment.format_date(), comment.comment_id,
                format!("1~{}~9~{}~10~{}~11~{}~14~{}~15~{}~16~{}", acc.username, acc.icon_id, acc.player_color, acc.player_color_2, acc.icon_type as u32, acc.special, acc.account_id)
            )
        })
        .collect::<Vec<String>>()
        .join("|");
    
    // Get the total amount of comments
    let mut count_rows: Row = sql::CONN.lock().unwrap().exec_first("SELECT count(*) FROM comments WHERE sourceID=:source_id AND type=:type_id",
        mysql::params! {
            "source_id" => data.levelID,
            "type_id" => comment::CommentType::AccountComment as u32
        }
    ).unwrap().unwrap();

    let num: u32 = count_rows.take("count(*)").unwrap();
    
    out = format!("{}{}", out, format!("#{}:{}:{}", num, data.page, count));

    out
}

#[get("/getGJComments21.php")]
pub fn getGJComments21GET() -> String {
    "-1".to_string()
}