use crate::crypto;
use crate::sql;
use mysql::prelude::Queryable;
use mysql::*;

/// Verify an account via credentials
/// 
/// * `account_id` - The account ID of the user
/// * `gjp` - The gjp of the user
pub fn verify(account_id: i32, gjp: String) -> bool {

    // Create a hash using the passed ID and gjp
    let a: String = crypto::hash(gjp, account_id.to_string()).to_lowercase();

    // Get the hashed gjp from the database
    let b: Option<String> = sql::CONN.lock().unwrap().exec_first("SELECT pass FROM accounts WHERE accountID=:account_id", mysql::params!{
        "account_id" => account_id
    }).unwrap();

    if b == None {
        return false;
    }

    a == b.unwrap()
}