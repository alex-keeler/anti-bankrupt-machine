use sqlx::FromRow;
use crate::models::budget::BudgetClassification;

#[derive(Debug, FromRow)]
pub struct SpendingByBudget {
    pub budget_title: Option<String>,
    pub budget_classification: Option<BudgetClassification>,
    pub total_amount: i32,
}