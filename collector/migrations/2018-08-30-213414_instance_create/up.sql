create table instance (
    id integer primary key autoincrement,
    uuid varchar(36) not null,
    name varchar(16) not null,
    enabled boolean not null default 0
);

create index idx_instance_id on instance (id);