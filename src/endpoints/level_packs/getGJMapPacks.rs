use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct GetMapPacksForm<'f> {
    secret: &'f RawStr,
    gameVersion: Option<i32>,
    binaryVersion: Option<i32>,
    gdw: Option<i32>,
    page: Option<i32>,
    count: Option<i32>,
}
#[warn(dead_code)]

#[post("/getGJMapPacks21.php", data = "<data>")]
pub fn getGJMapPacks21(data: Form<GetMapPacksForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    let count: i32 = remove!(data.count, 10);
    let page: i32 = remove!(data.page, 0);
    let mut ids: Vec<i32> = vec![];

    // Get a vector of all Gauntlets
    let packs: Vec<level_pack::MapPack> = sql::CONN.lock().unwrap().exec_map("SELECT * FROM mappacks LIMIT :limit_num OFFSET :offset_num",
        mysql::params! {
            "limit_num" => count,
            "offset_num" => page * count
        },
        level_pack::MapPack::from_row
    ).unwrap();

    for pack in packs.clone() {
        ids.push(pack.pack_id);
    }

    let mut out: String = packs
        .into_iter()
        .map(|m: level_pack::MapPack| {
            format!("1:{}:2:{}:3:{}:4:{}:5:{}:6:{}:7:{}:8:{}", m.pack_id, m.name, m.levels.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","), m.stars, m.coins, m.difficulty as i32, m.text_color.format(), m.bar_color.format())
        })
        .collect::<Vec<String>>()
        .join("|");

    // Get the total amount of map packs
    let mut count_rows: Row = sql::CONN.lock().unwrap().query_first("SELECT count(*) FROM mappacks").unwrap().unwrap();

    let num: u32 = count_rows.take("count(*)").unwrap();

    out.push_str(&format!("#{}:{}:{}", num, page * count, count));

    out.push_str(&format!("#{}", crypto::hash_pack(ids)));

    out
}

#[get("/getGJMapPacks21.php")]
pub fn getGJMapPacks21GET() -> String {
    "-1".to_string()
}