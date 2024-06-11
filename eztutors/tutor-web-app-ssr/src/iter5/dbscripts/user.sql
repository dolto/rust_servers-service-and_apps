drop table if exists ezyweb_user;

create table ezyweb_user(
    username varchar(20) primary key,
    tutor_id int,
    user_password char(100) not null
);
