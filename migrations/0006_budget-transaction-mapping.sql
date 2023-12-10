CREATE TABLE budget_transaction_mapping (
    id bigint(20) NOT NULL AUTO_INCREMENT,
    budget_id bigint(20) NOT NULL,
    bank_transaction_id bigint(20) NOT NULL,
    amount int(11) NOT NULL,
    behavior int(11) NOT NULL,
    PRIMARY KEY (id),
    KEY fk_btm_b (budget_id),
    KEY fk_btm_bt (bank_transaction_id),
    CONSTRAINT fk_btm_b FOREIGN KEY (budget_id) REFERENCES budget (id),
    CONSTRAINT fk_btm_bt FOREIGN KEY (bank_transaction_id) REFERENCES bank_transaction (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

INSERT INTO budget_transaction_mapping (budget_id, bank_transaction_id, amount, behavior)
SELECT b.id, bt.id, bt.amount, CASE WHEN bt.amount < 0 THEN 1 ELSE 2 END
FROM bank_transaction bt JOIN budget b on bt.budget_id = b.id;

ALTER TABLE bank_transaction DROP FOREIGN KEY bank_transaction_ibfk_1;
ALTER TABLE bank_transaction DROP COLUMN budget_id;