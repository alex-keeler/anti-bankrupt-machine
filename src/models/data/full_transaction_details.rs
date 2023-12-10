use serde::{Serialize, Deserialize};
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::query::transaction_details::TransactionDetails;

#[derive(Debug, Serialize, Deserialize)]
pub struct FullTransactionDetails {
    pub transaction: TransactionDetails,
    pub budget_mappings: Vec<BudgetTransactionMappingDetails>,
}