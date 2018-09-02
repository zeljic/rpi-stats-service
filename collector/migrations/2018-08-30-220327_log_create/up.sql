create table log (
    id integer primary key autoincrement,
    instance_id integer not null,
    log_type_id integer not null,
    `date_time` integer not null,
    `value` real not null,
    foreign key(instance_id) references instance(id),
    foreign key(log_type_id) references log_type(id)
);

create index idx_log_id on log (id);