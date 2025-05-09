use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BudgetSpendingReport {
    pub title: String,
    pub total_spent: i32,
}