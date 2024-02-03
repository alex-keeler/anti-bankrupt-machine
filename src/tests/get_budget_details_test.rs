use chrono::NaiveDate;
use rocket::http::ext::IntoCollection;
use sqlx::MySql;
use sqlx::mysql::MySqlDatabaseError;
use sqlx::pool::PoolConnection;

use crate::models::budget::BudgetType;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;
use crate::repositories::budget_repository;
use crate::util::{compare_util, date_util};

#[sqlx::test(fixtures("users", "budgets"))]
async fn test_get_budget_details_list_matches_expected(mut conn: PoolConnection<MySql>) -> sqlx::Result<()> {

    let user = user_1();
    let (month, year) = (10, 2023);
    let (begin_date, end_date) = date_util::get_date_range(year, month).unwrap();

    let budget_details_list = budget_repository::get_details_list_for_user_and_time_period(&user, begin_date, end_date, &mut conn).await?;

    let expected_list = vec![
        budget_details_1(), budget_details_2(), budget_details_5(),
        budget_details_6(), budget_details_7(), budget_details_8()
    ];
    assert!(compare_util::_vecs_are_equal_ordered(&budget_details_list, &expected_list),
            "Budget details to add does not match expected!\n\nExpected: {:?}\n\nReceived: {:?}\n\n",
            expected_list, budget_details_list);

    Ok(())
}

fn user_1() -> User {
    User {
        id: Some(1),
        username: "user1".to_string(),
        password_hash: "".to_string(),
        is_active: true,
        email_address: "user1@example.com".to_string(),
        first_name: "User".to_string(),
        last_name: "1".to_string(),
    }
}

fn user_2() -> User {
    User {
        id: Some(2),
        username: "user2".to_string(),
        password_hash: "".to_string(),
        is_active: true,
        email_address: "user2@example.com".to_string(),
        first_name: "User".to_string(),
        last_name: "2".to_string(),
    }
}

fn budget_details_1() -> BudgetDetails {
    BudgetDetails {
        id: 1,
        title: "Budget 1 - User 1, Indefinite Recurring".to_string(),
        classification: CategoryClassification::Needs,
        budget_type: BudgetType::Fixed,
        amount: 25000,
        time_months: 1,
        payment_date: NaiveDate::from_ymd_opt(2022, 01, 01).unwrap(),
        recur_count: None,
    }
}

fn budget_details_2() -> BudgetDetails {
    BudgetDetails {
        id: 2,
        title: "Budget 2 - User 1, Modified & Current".to_string(),
        classification: CategoryClassification::Savings,
        budget_type: BudgetType::Fixed,
        amount: 60000,
        time_months: 4,
        payment_date: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        recur_count: None,
    }
}

fn budget_details_5() -> BudgetDetails {
    BudgetDetails {
        id: 5,
        title: "Budget 5 - User 1, On First Payment Month".to_string(),
        classification: CategoryClassification::Wants,
        budget_type: BudgetType::Variable,
        amount: 50000,
        time_months: 3,
        payment_date: NaiveDate::from_ymd_opt(2023, 10, 01).unwrap(),
        recur_count: Some(1),
    }
}

fn budget_details_6() -> BudgetDetails {
    BudgetDetails {
        id: 6,
        title: "Budget 6 - User 1, In First Payment Period".to_string(),
        classification: CategoryClassification::Wants,
        budget_type: BudgetType::Variable,
        amount: 60000,
        time_months: 3,
        payment_date: NaiveDate::from_ymd_opt(2023, 12, 01).unwrap(),
        recur_count: Some(1),
    }
}

fn budget_details_7() -> BudgetDetails {
    BudgetDetails {
        id: 7,
        title: "Budget 7 - User 1, On Third Payment Month".to_string(),
        classification: CategoryClassification::Wants,
        budget_type: BudgetType::Variable,
        amount: 100000,
        time_months: 3,
        payment_date: NaiveDate::from_ymd_opt(2023, 04, 01).unwrap(),
        recur_count: None,
    }
}

fn budget_details_8() -> BudgetDetails {
    BudgetDetails {
        id: 8,
        title: "Budget 8 - User 1, In Third Payment Period".to_string(),
        classification: CategoryClassification::Wants,
        budget_type: BudgetType::Variable,
        amount: 120000,
        time_months: 3,
        payment_date: NaiveDate::from_ymd_opt(2023, 06, 01).unwrap(),
        recur_count: None,
    }
}
