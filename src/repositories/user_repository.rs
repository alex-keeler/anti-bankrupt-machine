use std::error::Error;
use rocket_db_pools::{sqlx, Connection};
use crate::models::users::User;
use crate::models::DB;

pub async fn find_user_by_id(id: i64, mut conn: Connection<DB>) -> Result<User, Box<dyn Error>> {

    let sql = "SELECT u.* FROM users u WHERE u.id = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&id);

    let user = query.fetch_one(&mut *conn).await?;

    Ok(user)
}

pub async fn find_user_by_username(username: &str, mut conn: Connection<DB>) -> Result<Option<User>, Box<dyn Error>> {

    let sql = "SELECT u.* FROM users u WHERE u.username = ?";

    let query = sqlx::query_as::<_, User>(sql)
        .bind(&username);

    let user = query.fetch_optional(&mut *conn).await?;

    Ok(user)
}