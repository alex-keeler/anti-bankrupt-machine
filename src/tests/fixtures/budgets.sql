set @budget_type_fixed = 1;
set @budget_type_variable = 2;

set @classification_needs = 1;
set @classification_savings = 2;
set @classification_wants = 3;
set @classification_income = 4;

insert into budget (id, title, budget_type, classification, user_id, sort_order) values
    (1,  "Budget 1 - User 1, Indefinite Recurring", @budget_type_fixed, @classification_needs, 1, 1),
    (2,  "Budget 2 - User 1, Modified & Current", @budget_type_fixed, @classification_savings, 1, 2),
    (3,  "Budget 3 - User 1, Past", @budget_type_fixed, @classification_wants, 1, 3),
    (4,  "Budget 4 - User 1, Future", @budget_type_variable, @classification_wants, 1, 4),
    (5,  "Budget 5 - User 1, On First Payment Month", @budget_type_variable, @classification_wants, 1, 5),
    (6,  "Budget 6 - User 1, In First Payment Period", @budget_type_variable, @classification_wants, 1, 6),
    (7,  "Budget 7 - User 1, On Third Payment Month", @budget_type_variable, @classification_wants, 1, 7),
    (8,  "Budget 8 - User 1, In Third Payment Period", @budget_type_variable, @classification_wants, 1, 8),
    (9,  "Budget 9 - User 1, Month Before", @budget_type_variable, @classification_wants, 1, 9),
    (10, "Budget 10 - User 1, Month After", @budget_type_variable, @classification_wants, 1, 10),
    (11, "Budget 11 - User 2, Current", @budget_type_variable, @classification_wants, 2, 1),
    (12, "Budget 12 - User 2, Current", @budget_type_fixed, @classification_savings, 2, 2),
    (13, "Budget 13 - User 3, Current", @budget_type_fixed, @classification_needs, 3, 1);

insert into budget_history (id, budget_id, payment_date, time_months, recur_count, amount) values
    (1,  1,  '2022-01-01', 1, null, 25000),
    (2,  2,  '2022-01-01', 3, 3, 30000),
    (3,  2,  '2022-06-01', 1, 5, 50000),
    (4,  2,  '2023-01-01', 4, null, 60000),
    (5,  3,  '2019-01-01', 1, 12, 10000),
    (6,  4,  '2024-01-01', 1, 12, 10000),
    (7,  5,  '2023-10-01', 3, 1, 50000),
    (8,  6,  '2023-12-01', 3, 1, 60000),
    (9,  7,  '2023-04-01', 3, null, 100000),
    (10, 8,  '2023-06-01', 3, null, 120000),
    (11, 9,  '2023-09-01', 3, 0, 30000),
    (12, 10, '2024-01-01', 3, 1, 30000),
    (13, 11, '2022-01-01', 1, null, 1000000),
    (14, 12, '2022-01-01', 12, null, 12000000),
    (15, 13, '2023-08-01', 6, 1, 50000);
