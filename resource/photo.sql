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
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`user_id` VARCHAR ( 32 ) NOT NULL COMMENT '用户id',
	`photo_path` VARCHAR ( 1024 ) COMMENT '图片路径',
	`remark` TEXT COMMENT '备注',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` )
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '图片上传信息表';