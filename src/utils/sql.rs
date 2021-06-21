use mysql::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref CONN: Mutex<PooledConn> = {

        // Create a connection to the mysql database
        let url: &str = "mysql://root:9eP36sQvPhQCXwJ8@localhost:3306/rustgdps";
        let pool: Pool = Pool::new(url).unwrap();
        let connection: PooledConn = pool.get_conn().unwrap();

        Mutex::new(connection)
    };
}