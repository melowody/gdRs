extern crate base64;

use std::time::*;
use base64::encode;

pub struct LevelComment {
    pub comment_id: u32,
    pub comment: std::string::String,
    pub level_id: u32,
    pub dat: u128,
    pub likes: i32,
    pub author_id: u32,
    pub percentage: u32,
    pub is_spam: bool
}

impl PartialEq for LevelComment {
    fn eq(&self, other: &Self) -> bool {
        self.comment_id == other.comment_id && self.level_id == other.level_id && self.comment == other.comment && self.dat == other.dat && self.likes == other.likes && self.author_id == other.author_id
    }
}

impl LevelComment {
    /// Format the date of the Level Comment
    /// 
    /// e.g. 5 months ago
    pub fn format_date(&self) -> String {

        let curr = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let dur: u128 = (curr - self.dat) / 1000;

        if dur < 60 {
            return format!("{} second{}", dur, match dur==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 3600 {
            return format!("{} minute{}", dur / 60, match dur/60==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 86400 {
            return format!("{} hour{}", dur / 3600, match dur/3600==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 31556952 {
            return format!("{} day{}", dur / 86400, match dur/86400==1 { false=> "s", true=> "" }).to_string();
        } else {
            return format!("{} year{}", dur / 2629800, match dur/2629800==1 { false=> "s", true=> "" }).to_string();
        }

    }
}

pub struct AccComment {
    pub comment_id: u32,
    pub comment: std::string::String,
    pub account_id: u32,
    pub dat: u128,
    pub likes: u32,
}

impl PartialEq for AccComment {
    fn eq(&self, other: &Self) -> bool {
        self.comment_id == other.comment_id && self.account_id == other.account_id && self.comment == other.comment
    }
}

impl AccComment {
    /// Format the date of the Account Comment
    /// 
    /// e.g. 5 months ago
    pub fn format_date(&self) -> String {

        let curr = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let dur: u128 = (curr - self.dat) / 1000;

        if dur < 60 {
            return format!("{} second{}", dur, match dur==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 3600 {
            return format!("{} minute{}", dur / 60, match dur/60==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 86400 {
            return format!("{} hour{}", dur / 3600, match dur/3600==1 { false=> "s", true=> "" }).to_string();
        } else if dur < 31556952 {
            return format!("{} day{}", dur / 86400, match dur/86400==1 { false=> "s", true=> "" }).to_string();
        } else {
            return format!("{} year{}", dur / 2629800, match dur/2629800==1 { false=> "s", true=> "" }).to_string();
        }

    }

    /// Formats the account comment into Robtop's format
    pub fn format(&self) -> String {
        format!("2~{}~4~{}~9~{}~6~{}", encode(&self.comment), self.likes, self.format_date(), self.comment_id).to_string()
    }
}

#[derive(FromPrimitive)]
pub enum CommentType {
    LevelComment = 1,
    AccountComment = 2,
}