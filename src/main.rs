#![feature(decl_macro)]
#![feature(allocator_api)]
#[macro_use] extern crate rocket;

extern crate num;
#[macro_use] extern crate num_derive;

extern crate hmac;
extern crate sha1;
extern crate sha2;
extern crate lazy_static;
extern crate hex;

#[macro_use] mod utils;

mod types;
mod endpoints;

use crate::endpoints::comments::*;
use crate::endpoints::level_packs::*;
use crate::endpoints::level::*;
use crate::utils::*;

fn main(){
    rocket::ignite()
    .mount("/database/", routes![
        deleteGJAccountComment::deleteGJAccComment20, deleteGJAccountComment::deleteGJAccComment20GET,
        deleteGJComment::deleteGJComment20, deleteGJComment::deleteGJComment20GET,
        getGJAccountComments::getGJAccountComments20, getGJAccountComments::getGJAccountComments20GET,
        getGJCommentHistory::getGJCommentHistory, getGJCommentHistory::getGJCommentHistoryGET,
        getGJComments::getGJComments21, getGJComments::getGJComments21GET,
        uploadGJAccComment::uploadGJAccComment20, uploadGJAccComment::uploadGJAccComment20GET,
        uploadGJComment::uploadGJComment21, uploadGJComment::uploadGJComment21GET,
        getGJGauntlets::getGJGauntlets21, getGJGauntlets::getGJGauntlets21GET,
        getGJMapPacks::getGJMapPacks21, getGJMapPacks::getGJMapPacks21GET,
        deleteGJLevelUser::deleteGJLevelUser20, deleteGJLevelUser::deleteGJLevelUser20GET
    ])
    .launch();
}