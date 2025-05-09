use serde::Serialize;
use crate::models::data::budget_classification_spending_report::BudgetClassificationSpendingReport;

#[derive(Debug, Serialize)]
pub struct SpendingReport {
    pub total_spent: i32,
    pub total_spent_unbudgeted: i32,
    pub budget_classifications: Vec<BudgetClassificationSpendingReport>,
}