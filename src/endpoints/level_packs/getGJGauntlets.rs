use crate::utils::*;
use crate::types::*;
use mysql::*;
use mysql::prelude::*;
use rocket::request::Form;
use rocket::http::RawStr;

// Create the form used for the getGJAccountComments20 endpoint
#[allow(dead_code)]
#[derive(FromForm)]
pub struct GetGauntletsForm<'f> {
    secret: &'f RawStr,
    gameVersion: Option<u32>,
    binaryVersion: Option<u32>,
    gdw: Option<i32>,
}
#[warn(dead_code)]

#[post("/getGJGauntlets21.php", data = "<data>")]
pub fn getGJGauntlets21(data: Form<GetGauntletsForm>) -> String {

    // If the secret is wrong, return -1
    if data.secret != "Wmfd2893gb7" {
        return "-1".to_string();
    }

    let mut outdata: String = String::new();

    // Get a vector of all Gauntlets
    let gauntlets: Vec<level_pack::Gauntlet> = sql::CONN.lock().unwrap().query_map(format!("SELECT * FROM gauntlets"),
        | row: Row | {
            let gauntlet: level_pack::Gauntlet = level_pack::Gauntlet::from_row(row);

            outdata.push_str(&gauntlet.gauntlet_id.to_string());
            outdata.push_str(&gauntlet.clone().levels.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

            gauntlet
        }
    ).unwrap();

    let mut out: String = gauntlets
        .into_iter()
        .map(|g: level_pack::Gauntlet| {
            format!("1:{}:3:{}", g.gauntlet_id, g.levels.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
        })
        .collect::<Vec<String>>()
        .join("|");
    
    out.push('#');
    out.push_str(&crypto::sha1_salt(outdata, "xI25fpAapCQg"));

    out
}

#[get("/getGJGauntlets21.php")]
pub fn getGJGauntlets21GET() -> String {
    "-1".to_string()
}