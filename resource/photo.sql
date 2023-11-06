CREATE DATABASE /*!32312
	IF
	NOT EXISTS */`photo_db` /*!40100 DEFAULT CHARACTER 
SET utf8mb4 COLLATE utf8mb4_bin */;
USE `photo_db`;

DROP TABLE
IF
	EXISTS `photo`;
	
	/*如果你的应用有德语、法语或者俄语，请一定使用utf8_unicode_ci。一般用utf8_general_ci就够了*/
CREATE TABLE `photo` (
	`id` int(12) NOT NULL auto_increment,
	`user_id` VARCHAR ( 32 ) NOT NULL,
	`photo_path` VARCHAR ( 1024 ),
	`remark` TEXT,
	`create_time` DATETIME,
	`update_time` DATETIME,
	PRIMARY KEY ( `id` )
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic;
