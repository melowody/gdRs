use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the deleteGJComment20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct DeleteCommentForm<'f> {
    accountID: u32,
    gjp: &'f RawStr,
    commentID: u32,
    levelID: u32,
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>
}
#[warn(dead_code)]

#[post("/deleteGJComment20.php", data = "<data>")]
pub fn deleteGJComment20(data: Form<DeleteCommentForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    // Make sure the credentials are correct
    let verified = accounts::verify(data.accountID, data.gjp.url_decode().unwrap());
    if !verified {
        return "-1".to_string();
    }

    // Attempt to delete the comment if by the user
    let res: u64 = sql::CONN.lock().unwrap().exec_iter("DELETE FROM comments WHERE commentID=:comment_id AND sourceID=:source_id AND type=:type_id AND authorID=:author_id",
        mysql::params!{
            "comment_id" => data.commentID,
            "source_id" => data.levelID,
            "type_id" => comment::CommentType::LevelComment as i32,
            "author_id" => data.accountID
        }
    ).unwrap().affected_rows();

    // If no comments were deleted
    if res == 0 {

        // Get the author of the level the comment is on
        let authorID: u32 = sql::CONN.lock().unwrap().exec_first("SELECT authorID FROM levels WHERE levelID=:source_id",
            mysql::params!{
                "source_id" => data.levelID,
            }
        ).unwrap().unwrap();

        // Test if the author of the level is deleting, then delete it if they are
        if authorID == data.accountID {
            sql::CONN.lock().unwrap().exec_drop("DELETE FROM comments WHERE commentID=:comment_id AND sourceID=:source_id AND type=:type_id AND authorID=:author_id",
                mysql::params!{
                    "comment_id" => data.commentID,
                    "source_id" => data.levelID,
                    "type_id" => comment::CommentType::LevelComment as i32,
                    "author_id" => data.accountID
                }
            ).unwrap();
        }
    }

    return "1".to_string();
}

#[get("/deleteGJComment20.php")]
pub fn deleteGJComment20GET() -> String {
    "-1".to_string()
}