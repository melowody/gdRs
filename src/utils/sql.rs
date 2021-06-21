use mysql::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use mysql::prelude::*;

lazy_static! {
    pub static ref CONN: Mutex<PooledConn> = {

        // Create a connection to the mysql database
        let url: &str = "mysql://root:9eP36sQvPhQCXwJ8@localhost:3306/rustgdps";
        let pool: Pool = Pool::new(url).unwrap();
        let connection: PooledConn = pool.get_conn().unwrap();

        Mutex::new(connection)
    };
}

pub fn get_next_comment_id() -> i32 {

    // Get the total amount of comments
    let mut count_rows: Row = CONN.lock().unwrap().query_first("SELECT max(commentID)+1 FROM comments;").unwrap().unwrap();

    let num: i32 = count_rows.take("max(commentID)+1").unwrap();

    num
}