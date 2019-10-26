DROP TABLE IF EXISTS `matchups`;
DROP TABLE IF EXISTS `user_teams`;
DROP TABLE IF EXISTS `user_roles`;
DROP TABLE IF EXISTS `game_types`;
DROP TABLE IF EXISTS `teams`;
DROP TABLE IF EXISTS `seasons`;
DROP TABLE IF EXISTS `roles`;
DROP TABLE IF EXISTS `users`;

-- CREATE TABLE `users` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `username` VARCHAR(50) NOT NULL,
--     `first_name` VARCHAR(50) NOT NULL,
--     `last_name` VARCHAR(50) NOT NULL,
--     `nickname` VARCHAR(50) NOT NULL,
--     `password` CHAR(32) DEFAULT NULL,
--     `salt` CHAR(8) DEFAULT NULL,
--     `email_address` VARCHAR(255) DEFAULT NULL
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `roles` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `role` VARCHAR(255) NOT NULL
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `seasons` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `season` CHAR(8) NOT NULL
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `teams` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `nhl_id` INT UNSIGNED NOT NULL,
--     `abbreviation` CHAR(3) NOT NULL,
--     `location` VARCHAR(50) NOT NULL,
--     `name` VARCHAR(50) NOT NULL
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `game_types` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `type` CHAR(3) NOT NULL,
--     `value`  INT UNSIGNED NOT NULL
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `user_roles` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `user_id` INT UNSIGNED NOT NULL,
--     `role_id` INT UNSIGNED NOT NULL,
--     UNIQUE KEY (`user_id`, `role_id`),
--     FOREIGN KEY `fk_ur_user` (`user_id`) REFERENCES `users` (`id`),
--     FOREIGN KEY `fk_ur_role` (`role_id`) REFERENCES `roles` (`id`)
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `user_teams` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `season_id` INT UNSIGNED NOT NULL,
--     `user_id` INT UNSIGNED NOT NULL,
--     `team_id` INT UNSIGNED NOT NULL,
--     UNIQUE KEY (`season_id`, `user_id`),
--     UNIQUE KEY (`season_id`, `team_id`),
--     FOREIGN KEY `fk_ut_season` (`season_id`) REFERENCES `seasons` (`id`),
--     FOREIGN KEY `fk_ut_user` (`user_id`) REFERENCES `users` (`id`),
--     FOREIGN KEY `fk_ut_team` (`team_id`) REFERENCES `teams` (`id`)
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- CREATE TABLE `matchups` (
--     `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
--     `season_id` INT UNSIGNED NOT NULL,
--     `event_time` DATETIME NOT NULL,
--     `game_type_id` INT UNSIGNED NOT NULL,
--     `home_team_id` INT UNSIGNED NOT NULL,
--     `away_team_id` INT UNSIGNED NOT NULL,
--     `home_team_score` INT UNSIGNED DEFAULT NULL,
--     `away_team_score` INT UNSIGNED DEFAULT NULL,
--     FOREIGN KEY `fk_m_season` (`season_id`) REFERENCES `seasons` (`id`),
--     FOREIGN KEY `fk_m_game_type` (`game_type_id`) REFERENCES `game_types` (`id`),
--     FOREIGN KEY `fk_m_home_team` (`home_team_id`) REFERENCES `teams` (`id`),
--     FOREIGN KEY `fk_m_away_team` (`away_team_id`) REFERENCES `teams` (`id`)
-- ) Engine=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- INSERT INTO `users` VALUES
-- (NULL, 'natelenart', 'Nate', 'Lenart', 'Nate', NULL, NULL, 'natelenart@gmail.com');

-- INSERT INTO `roles` VALUES
-- (NULL, 'ROLE_ADMIN'),
-- (NULL, 'ROLE_USER');

-- INSERT INTO `user_roles` VALUES 
-- (
--     NULL,
--     (SELECT `id` FROM `users` WHERE `username` = 'natelenart'),
--     (SELECT `id` FROM `roles` WHERE `role` = 'ROLE_ADMIN')
-- ),
-- (
--     NULL,
--     (SELECT `id` FROM `users` WHERE `username` = 'natelenart'),
--     (SELECT `id` FROM `roles` WHERE `role` = 'ROLE_USER')
-- );