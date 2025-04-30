use rocket_db_pools::{sqlx, Connection};
use sqlx::Error;
use crate::models::users::User;
use crate::models::DB;
use crate::repositories::repository_util::get_executor_conn;

pub async fn find_user_by_id(id: i64, conn: &mut Connection<DB>) -> Result<User, Error> {

    let sql = "SELECT u.* FROM users u WHERE u.id = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&id);

    let user = query.fetch_one(get_executor_conn(conn)).await?;

    Ok(user)
}

pub async fn find_user_by_username(username: &str, conn: &mut Connection<DB>) -> Result<Option<User>, Error> {

    let sql = "SELECT u.* FROM users u WHERE u.username = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&username);

    let user = query.fetch_optional(get_executor_conn(conn)).await?;

    Ok(user)
}