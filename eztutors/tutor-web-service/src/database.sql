drop table if exists ezy_course_c4;

create table ezy_course_c4 (
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c4
    (course_id, tutor_id, course_name)
    values (1,1,'First course');

insert into ezy_course_c4
    (course_id, tutor_id, course_name)
    values (2,1,'Second course');


drop table if exists ezy_course_c5;

create table ezy_course_c5(
    course_id serial primary key,
    tutor_id int not null,
    course_name varchar(140) not null,
    posted_time timestamp default now()
);

insert into ezy_course_c5
    (course_id, tutor_id, course_name) values (1,1,'First course');
insert into ezy_course_c5
    (course_id, tutor_id, course_name) values (2,1,'Second course');
