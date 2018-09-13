create table `instance` (
	`id` integer not null primary key autoincrement,
	`uuid` varchar(64) not null unique,
	`name` varchar(16) not null,
	`enabled` boolean not null default 0
);

create table `log_type` (
	`id` integer not null primary key autoincrement,
	`name` varchar(16) not null,
	`enabled` boolean not null default 0
);

create table `log` (
	`id` integer not null primary key autoincrement,
	`instance_id` integer not null,
	`log_type_id` integer not null,
	`date_time` integer not null,
	`value` real not null,
	`enabled` boolean not null default 0,
	foreign key(`instance_id`) references instance(`id`),
	foreign key(`log_type_id`) references log_type(`id`)
);

create index `idx_instance_id` on `instance` (`id`);
create index `idx_log_type_id` on `log_type` (`id`);
create index `idx_log_id` on `log` (`id`);
