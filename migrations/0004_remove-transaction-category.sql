ALTER TABLE bank_transaction ADD COLUMN budget_id bigint(20);
ALTER TABLE bank_transaction ADD FOREIGN KEY fk_bt_b (budget_id) REFERENCES budget (id);

UPDATE bank_transaction bt
    JOIN transaction_category tc on bt.category_id = tc.id
    JOIN budget_category_mapping bcm on tc.id = bcm.category_id
    JOIN budget b ON b.id = bcm.budget_id
SET bt.budget_id = b.id;

ALTER TABLE bank_transaction DROP FOREIGN KEY transaction_FK_1;
ALTER TABLE bank_transaction DROP INDEX transaction_FK_1;
ALTER TABLE bank_transaction DROP COLUMN category_id;

ALTER TABLE bank_transaction DROP FOREIGN KEY fk_bt_cb;
ALTER TABLE bank_transaction DROP INDEX fk_bt_cb;
ALTER TABLE bank_transaction DROP COLUMN contribution_budget_id;

ALTER TABLE bank_transaction DROP COLUMN hide_from_calculations;

DROP TABLE budget_category_mapping;
DROP TABLE transaction_category;
DROP TABLE transaction_category_group;