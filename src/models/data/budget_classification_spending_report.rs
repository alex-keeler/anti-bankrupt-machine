use serde::Serialize;
use crate::models::budget::BudgetClassification;
use crate::models::data::budget_spending_report::BudgetSpendingReport;

#[derive(Debug, Serialize)]
pub struct BudgetClassificationSpendingReport {
    pub classification: BudgetClassification,
    pub total_spent: i32,
    pub budgets: Vec<BudgetSpendingReport>,
}