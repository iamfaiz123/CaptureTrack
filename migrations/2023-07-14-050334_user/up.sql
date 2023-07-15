-- Your SQL goes here
create table client(
    id uuid,
    first_name varchar(128),
    last_name varchar(128),
    email varchar(128) NOT NULL,
    hash_password varchar(256) NOT NULL


);

alter table client add constraint Pk_CLIENT primary key (id);
create unique index ui_email_is_lwr on client (Lower(email));