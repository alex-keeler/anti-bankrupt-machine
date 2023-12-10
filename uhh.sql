CREATE TABLE `users` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `username` varchar(50) NOT NULL,
    `email_address` varchar(100) NOT NULL,
    `first_name` varchar(100) NOT NULL,
    `last_name` varchar(100) NOT NULL,
    `password_hash` varchar(255) NOT NULL,
    `active` tinyint(1) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_username_UN` (`username`),
    UNIQUE KEY `users_email_UN` (`email_address`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `user_role` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `user_id` bigint(20) NOT NULL,
    `role` varchar(255) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `user_role_FK` (`user_id`),
    CONSTRAINT `user_role_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `bank_account` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `title` varchar(255) NOT NULL,
    `account_type` int(11) NOT NULL,
    `bank` int(11) NOT NULL,
    `balance` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `bank_account_title_UN` (`title`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `bank_account_user` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `user_id` bigint(20) NOT NULL,
    `bank_account_id` bigint(20) NOT NULL,
    `role` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `bank_account_user_FK` (`user_id`),
    KEY `bank_account_user_FK_1` (`bank_account_id`),
    CONSTRAINT `bank_account_user_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
    CONSTRAINT `bank_account_user_FK_1` FOREIGN KEY (`bank_account_id`) REFERENCES `bank_account` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `bank_transaction` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `date` date NOT NULL,
    `bank_description` varchar(255) DEFAULT NULL,
    `user_description` varchar(255) DEFAULT NULL,
    `comment` varchar(255) DEFAULT NULL,
    `amount` int(11) NOT NULL,
    `bank_account_id` bigint(20) NOT NULL,
    `category_id` bigint(20) DEFAULT NULL,
    `contribution_budget_id` bigint(20) DEFAULT NULL,
    `hide_from_calculations` tinyint(1) NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `transaction_FK` (`bank_account_id`),
    KEY `transaction_FK_1` (`category_id`),
    KEY `fk_bt_cb` (`contribution_budget_id`),
    CONSTRAINT `fk_bt_cb` FOREIGN KEY (`contribution_budget_id`) REFERENCES `budget` (`id`),
    CONSTRAINT `transaction_FK` FOREIGN KEY (`bank_account_id`) REFERENCES `bank_account` (`id`),
    CONSTRAINT `transaction_FK_1` FOREIGN KEY (`category_id`) REFERENCES `transaction_category` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `transaction_category_group` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `title` varchar(100) NOT NULL,
    `classification` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tcg_t` (`title`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `transaction_category` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `title` varchar(100) DEFAULT NULL,
    `category_group_id` bigint(20) NOT NULL,
    `user_id` bigint(20) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tc_tcg_t` (`category_group_id`,`title`),
    KEY `fk_tc_u` (`user_id`),
    CONSTRAINT `fk_tc_u` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
    CONSTRAINT `transaction_category_FK` FOREIGN KEY (`category_group_id`) REFERENCES `transaction_category_group` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `budget` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `title` varchar(255) NOT NULL,
    `budget_type` int(11) NOT NULL,
    `classification` int(11) NOT NULL,
    `user_id` bigint(20) NOT NULL,
    `sort_order` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_b_u` (`user_id`),
    CONSTRAINT `fk_b_u` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `budget_history` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `amount` int(11) NOT NULL,
    `time_months` int(10) unsigned NOT NULL,
    `budget_id` bigint(20) NOT NULL,
    `recur_count` int(10) unsigned DEFAULT NULL,
    `payment_date` date NOT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_bh_b` (`budget_id`),
    CONSTRAINT `fk_bh_b` FOREIGN KEY (`budget_id`) REFERENCES `budget` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `budget_category_mapping` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `budget_id` bigint(20) NOT NULL,
    `category_id` bigint(20) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_bcm_b` (`budget_id`),
    KEY `fk_bcm_c` (`category_id`),
    CONSTRAINT `fk_bcm_b` FOREIGN KEY (`budget_id`) REFERENCES `budget` (`id`),
    CONSTRAINT `fk_bcm_c` FOREIGN KEY (`category_id`) REFERENCES `transaction_category` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;