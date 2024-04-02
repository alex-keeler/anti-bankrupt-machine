use rocket_db_pools::{sqlx, Connection};
use sqlx::{Error, MySql};
use sqlx::pool::PoolConnection;
use crate::models::users::User;
use crate::models::DB;

pub async fn find_user_by_id(id: i64, conn: &mut PoolConnection<MySql>) -> Result<User, Error> {

    let sql = "SELECT u.* FROM users u WHERE u.id = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&id);

    let user = query.fetch_one(conn).await?;

    Ok(user)
}

pub async fn find_user_by_username(username: &str, conn: &mut PoolConnection<MySql>) -> Result<Option<User>, Error> {

    let sql = "SELECT u.* FROM users u WHERE u.username = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&username);

    let user = query.fetch_optional(conn).await?;

    Ok(user)
}