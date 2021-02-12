create table organizations (
  id serial primary key,
  organization varchar(64) -- orgs have a limit of 39 currently
);

create table event_types (
    id serial primary key, 
    event_type varchar(128)
);

create table repositories (
    id serial primary key,
    repo text,
);

create table events (
    id SERIAL primary key,
    event_type integer references event_types,
    org_id integer references organizations,
    repo_id integer references repositories,
);

create table push_event_payload (
    id serial primary key,
    before varchar(64),
    head varchar(64)
);

create table push_event_payload_commit (
    id serial primary key,
    author_name text not null,
    message text,
    sha varchar(256), 
    push_event_payload_id integer references push_event_payload
);
