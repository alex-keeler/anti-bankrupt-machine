use std::collections::HashMap;
use chrono::Months;
use rocket_db_pools::Connection;

use crate::models::data::application_error::ApplicationError;

use crate::models::budget::BudgetHistory;
use crate::models::budget_transaction_mapping::TransactionBehavior;
use crate::models::query::budgets_by_classification::BudgetsByClassification;
use crate::models::DB;
use crate::models::query::budget_state::BudgetState;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::users::User;
use crate::repositories::{budget_history_repository, budget_repository, budget_transaction_mapping_repository};
use crate::util::date_util;

pub async fn find_budget_states_by_user_and_month(user: &User, year: i32, month: u32, conn: &mut Connection<DB>) -> Result<Vec<BudgetState>, ApplicationError> {

    let mut budget_states = Vec::new();

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let budgets = budget_repository::find_all_visible_by_user(user, conn).await?;

    for budget in budgets {
        let histories = budget_history_repository::find_applicable_by_budget(
            &budget, &begin_date, &end_date, conn).await?;

        if histories.is_empty() {
            continue;
        }

        let mut earliest_history: Option<&BudgetHistory> = None;
        let mut latest_history: Option<&BudgetHistory> = None;

        let mut amount_current = 0;
        let mut amount_total = None;

        for index in 0..histories.len() {

            let history = &histories[index];

            if earliest_history.is_none() || history.start_date.lt(&earliest_history.as_ref().unwrap().start_date) {
                earliest_history = Some(&history);
            }

            if latest_history.is_none() || history.start_date.gt(&latest_history.as_ref().unwrap().start_date) {
                latest_history = Some(&history);
            }
        }

        let earliest_history = earliest_history.unwrap();
        let latest_history = latest_history.unwrap();

        let cycle_begin_date = if budget.cumulative || latest_history.months_until_completion.is_none() {
            earliest_history.start_date
        } else {
            let months_until_completion = latest_history.months_until_completion.unwrap();
            let months_since_start_date = date_util::months_between(latest_history.start_date, begin_date) as u32;
            let cycle_num = months_since_start_date / months_until_completion;
            latest_history.start_date + Months::new(cycle_num * months_until_completion)
        };

        for index in 0..histories.len() {

            let history = &histories[index];

            match history.months_until_maturity {
                Some(months_until_maturity) => {
                    if end_date.ge(&(cycle_begin_date + Months::new(months_until_maturity))) {
                        amount_current += history.amount_monthly * months_until_maturity as i32;
                    } else {
                        amount_current += history.amount_monthly * date_util::months_between(cycle_begin_date, end_date);
                    }
                    amount_total = Some(amount_total.unwrap_or(0) + history.amount_monthly * history.months_until_maturity.unwrap() as i32);
                }
                None => {
                    amount_current += history.amount_monthly * date_util::months_between(history.start_date, end_date);
                    amount_total = None;
                }
            }
        }

        let cycle_transaction_mappings =
            budget_transaction_mapping_repository::find_by_budget_and_time_period(
                &budget, Some(&cycle_begin_date), Some(&end_date), conn).await?;

        let mut amount_spent = 0;
        let mut amount_contributed = 0;

        for transaction_mapping in cycle_transaction_mappings {
            if TransactionBehavior::Contribution.eq(&transaction_mapping.behavior) {
                amount_contributed += transaction_mapping.amount;
            } else {
                amount_spent -= transaction_mapping.amount;
            }
        }

        let mut amount_spent_this_month = 0;
        let mut amount_contributed_this_month = 0;

        let month_transaction_mappings =
            budget_transaction_mapping_repository::find_by_budget_and_time_period(
                &budget, Some(&begin_date), Some(&end_date), conn).await?;

        for transaction_mapping in month_transaction_mappings {
            if TransactionBehavior::Contribution.eq(&transaction_mapping.behavior) {
                amount_contributed_this_month += transaction_mapping.amount;
            } else {
                amount_spent_this_month -= transaction_mapping.amount;
            }
        }

        budget_states.push(
            BudgetState {
                id: budget.id.unwrap(),
                title: budget.title,
                classification: budget.classification,
                spending_type: budget.spending_type,
                cycle_start_date: cycle_begin_date,
                months_until_maturity: latest_history.months_until_maturity,
                amount_monthly: latest_history.amount_monthly,
                amount_current,
                amount_total,
                amount_spent,
                amount_spent_this_month,
                amount_contributed,
                amount_contributed_this_month,
            }
        );
    }

    Ok(budget_states)
}

pub async fn find_budgets_by_user_grouped_by_classification(
    user: &User, conn: &mut Connection<DB>
) -> Result<Vec<BudgetsByClassification>, ApplicationError> {

    let budgets = budget_repository::find_all_by_user(user, conn).await?;

    let mut classification_to_budget_map = HashMap::new();
    for budget in budgets {
        classification_to_budget_map.entry(budget.classification.clone())
            .or_insert(Vec::new())
            .push(budget);
    }

    let mut budgets_by_classification_list =
        Vec::with_capacity(classification_to_budget_map.len());

    for entry in classification_to_budget_map {
        budgets_by_classification_list.push(BudgetsByClassification {
            classification: entry.0,
            budgets: entry.1,
        });
    }

    budgets_by_classification_list.sort_by(|a, b| a.classification.cmp(&b.classification));

    Ok(budgets_by_classification_list)
}

pub async fn find_budget_details_by_user_and_month(user: &User, year: i32, month: u32, conn: &mut Connection<DB>) -> Result<Vec<BudgetDetails>, ApplicationError> {
    let (begin_date, end_date) = date_util::get_date_range(year, month)?;
    let budget_details_list = budget_repository::get_details_list_for_user_and_time_period(user, begin_date, end_date, conn).await?;
    Ok(budget_details_list)
}