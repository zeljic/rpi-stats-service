create table "user" (
    "id" serial not null primary key,
    "name" varchar(64) not null,
    "email" varchar(320) not null,
    "password" varchar(64) not null,
    "enabled" boolean not null default 'f'
);

create table "instance" (
	"id" serial not null primary key,
	"uuid" varchar(64) not null unique,
	"name" varchar(16) not null,
	"description" text,
	"enabled" boolean not null default 'f'
);

create table "log_type" (
	"id" serial not null primary key,
	"user_id" integer not null references "user"("id"),
	"name" varchar(32) not null,
	"description" text,
	"enabled" boolean not null default 'f',
	unique("user_id", "name")
);

create table "log" (
	"id" serial not null primary key,
	"instance_id" integer not null references "instance"("id"),
	"log_type_id" integer not null references "log_type"("id"),
	"date_time" integer not null,
	"value" real not null,
	"enabled" boolean not null default 'f'
);

create table "mesh" (
    "id" serial not null primary key,
    "name" varchar(16) not null,
    "description" text,
    "enabled" boolean not null default 'f'
);

create table "mesh_instance" (
    "id" serial not null primary key,
    "mesh_id" integer not null references "mesh"("id"),
    "instance_id" integer not null references "instance"("id"),
    "enabled" boolean not null default 'f'
);

create table "user_mesh" (
    "id" serial not null primary key,
    "user_id" integer not null references "user"("id"),
    "mesh_id" integer not null references "mesh"("id"),
    "enabled" boolean not null default 'f'
);

create index "idx_user_id" on "user"("id");
create index "idx_instance_id" on "instance" ("id");
create index "idx_log_type_id" on "log_type" ("id");
create index "idx_log_id" on "log" ("id");
create index "idx_mesh_id" on "mesh" ("id");
create index "idx_mesh_instance_id" on "mesh_instance" ("id");
create index "idx_user_mesh_id" on "user_mesh" ("id");
