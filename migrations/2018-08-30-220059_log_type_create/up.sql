create table log_type (
    id integer not null primary key autoincrement,
    name varchar(16) not null,
    enabled boolean not null default 0
);

create index idx_log_type_id on log_type (id);