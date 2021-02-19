create table events (
    id serial primary key,
    event_type_id serial references event_types
);

create table event_types (
    id serial primary key,
);

create table loner (
    id serial primary key
);
