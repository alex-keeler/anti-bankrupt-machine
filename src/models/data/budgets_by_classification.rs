use crate::models::budget::{Budget, BudgetClassification};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetsByClassification {
    pub classification: BudgetClassification,
    pub budgets: Vec<Budget>,
}