ALTER TABLE budget ADD COLUMN spending_type int(11) NOT NULL;
UPDATE budget SET spending_type = budget_type;
ALTER TABLE budget DROP COLUMN budget_type;

ALTER TABLE budget ADD COLUMN cumulative bool NOT NULL DEFAULT false;
ALTER TABLE budget ADD COLUMN hidden bool NOT NULL DEFAULT false;

ALTER TABLE budget_history ADD COLUMN start_date date NOT NULL DEFAULT '2022-01-01';
ALTER TABLE budget_history ADD COLUMN amount_monthly int(11) NOT NULL;
ALTER TABLE budget_history ADD COLUMN months_until_maturity int(11) unsigned;
ALTER TABLE budget_history ADD COLUMN months_until_completion int(11) unsigned;

UPDATE budget_history SET start_date = DATE_SUB(payment_date, INTERVAL time_months MONTH);
UPDATE budget_history SET amount_monthly = amount DIV time_months;
UPDATE budget_history SET months_until_maturity = time_months;
UPDATE budget_history SET months_until_completion = time_months;

ALTER TABLE budget_history DROP COLUMN amount;
ALTER TABLE budget_history DROP COLUMN time_months;
ALTER TABLE budget_history DROP COLUMN payment_date;
