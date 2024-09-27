use sqlx::Error;
use rocket_db_pools::Connection;
use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_accounts::BankAccount;
use crate::models::DB;
use crate::models::users::User;
use crate::repositories::repository_util::get_executor_conn;

pub async fn find_accounts_for_user(user: User, restrict_to_roles: Option<Vec<BankAccountUserRole>>, conn: &mut Connection<DB>) -> Result<Vec<BankAccount>, Error> {

    let mut sql = String::from("\
            SELECT ba.* \
            FROM users u \
                JOIN bank_account_user bau on bau.user_id = u.id \
                JOIN bank_account ba on ba.id = bau.bank_account_id \
            WHERE u.id = ? \
        ");

    if let Some(roles) = &restrict_to_roles {
        sql = sql + format!("AND bau.role IN ({})", vec!["?"; roles.len()].join(",")).as_str();
    }

    let mut query = sqlx::query_as::<_, BankAccount>(sql.as_str())
        .bind(user.id.unwrap());

    if let Some(roles) = &restrict_to_roles {
        for role in roles {
            query = query.bind(role);
        }
    }

    let bank_accounts = query.fetch_all(get_executor_conn(conn)).await?;

    Ok(bank_accounts)
}

pub async fn find_account_by_id(id: i64, conn: &mut Connection<DB>) -> Result<BankAccount, Error> {

    let sql = "\
            SELECT ba.* \
            FROM bank_account ba \
            WHERE ba.id = ? \
        ";

    let query = sqlx::query_as::<_, BankAccount>(sql)
        .bind(id);

    let bank_account = query.fetch_one(get_executor_conn(conn)).await?;

    Ok(bank_account)
}