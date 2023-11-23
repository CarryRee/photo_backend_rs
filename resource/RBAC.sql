CREATE DATABASE /*!32312
	IF
	NOT EXISTS */`photo_db` /*!40100 DEFAULT CHARACTER 
SET utf8mb4 COLLATE utf8mb4_bin */;
USE `photo_db`;

DROP TABLE
IF
	EXISTS `user`;
	
	/*如果你的应用有德语、法语或者俄语，请一定使用utf8_unicode_ci。一般用utf8_general_ci就够了*/
CREATE TABLE `user` (
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`uuid` VARCHAR ( 64 ) NOT NULL COMMENT '用户ID', 
	`name` VARCHAR ( 64 ) NOT NULL COMMENT '名称',
	`password` VARCHAR ( 256 ) NOT NULL COMMENT '密码',
	`status` TINYINT(1) NOT NULL COMMENT '状态',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '用户表';

DROP TABLE
IF
	EXISTS `role`;

CREATE TABLE `role` (
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`name` VARCHAR ( 64 ) NOT NULL COMMENT '名称',
	`status` TINYINT(1) COMMENT '状态',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '角色表';

DROP TABLE
IF
	EXISTS `user_role`;

CREATE TABLE `user_role` (
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`user_id` int(12) NOT NULL COMMENT '用户id',
	`role_id` int(12) NOT NULL COMMENT '角色id',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '用户角色对应表';

DROP TABLE
IF
	EXISTS `permission`;

CREATE TABLE `permission` (
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`parent_id` int(12) NOT NULL COMMENT '上级ID，一级菜单为0', 
	`name` VARCHAR ( 64 ) NOT NULL COMMENT '名称',
	`url` VARCHAR ( 512 ) COMMENT '菜单url',
	`type` VARCHAR ( 64 ) COMMENT '类型 0：菜单 1：按钮',
	`method` VARCHAR ( 64 ) COMMENT 'GET,POST,PUT,DELETE',
	`path` VARCHAR( 512 ) COMMENT '路径',
	`permissions` VARCHAR( 256 ) COMMENT '权限标识，如：sys:menu:save',
	`status` TINYINT(1) COMMENT '状态',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '权限表';

DROP TABLE
IF
	EXISTS `role_permission`;

CREATE TABLE `role_permission` (
	`id` int(12) NOT NULL auto_increment COMMENT '自增主键', 
	`role_id` int(12) NOT NULL NOT NULL COMMENT '角色id',
	`permission_id` int(12) NOT NULL COMMENT '权限id',
	`status` TINYINT(1) COMMENT '状态',
	`create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`update_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT '角色权限表';


DROP TABLE IF EXISTS `casbin_rule`;
CREATE TABLE `casbin_rule`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `ptype` VARCHAR(12) NOT NULL,
  `v0` VARCHAR(256) DEFAULT NULL,
  `v1` VARCHAR(128) DEFAULT NULL,
  `v2` VARCHAR(128) DEFAULT '',
  `v3` VARCHAR(128) DEFAULT '',
  `v4` VARCHAR(128) DEFAULT '',
  `v5` VARCHAR(128) DEFAULT '',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `unique_key_sqlx_adapter`(`ptype`, `v0`, `v1`, `v2`, `v3`, `v4`, `v5`) USING BTREE
) ENGINE = INNODB CHARACTER SET = utf8 COLLATE = utf8_general_ci ROW_FORMAT = Dynamic COMMENT 'casbin';