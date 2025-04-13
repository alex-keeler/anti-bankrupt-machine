use rocket_db_pools::Connection;
use sqlx::{MySql, MySqlConnection, Transaction};
use crate::models::DB;

pub fn get_executor_conn(conn: &mut Connection<DB>) -> &mut MySqlConnection {
    &mut ***conn
}

pub fn get_executor_txn<'a>(txn: &'a mut Transaction<'_, MySql>) -> &'a mut MySqlConnection {
    &mut **txn
}