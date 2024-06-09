drop table if exists ezy_course_c6 cascade;
drop table if exists ezy_tutor_c6;

create table ezy_tutor_c6 (
    tutor_id serial primary key,
    tutor_name varchar(100) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(200) not null
);


create table ezy_course_c6 (
    course_id serial primary key,
    tutor_id int not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price int,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now() not null,
    constraint fk_tutor
        foreign key(tutor_id)
        references ezy_tutor_c6(tutor_id)
    on delete cascade
);


grant all privileges on table ezy_tutor_c6 to truuser;
grant all privileges on table ezy_course_c6 to truuser;

insert into ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile) values ('Dolto', 'http://s3.amazon.aws.com/pic1', 'Blorin Compannys Good');
insert into ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile) values ('Frank', 'http://s3.amazon.aws.com/pic2', 'Frank is an expert nuclear engineer');

insert into ezy_course_c6 (tutor_id, course_name, course_level, course_price) values (1,'First course', 'Beginner', 1);
insert into ezy_course_c6 (tutor_id, course_name, course_level, course_price) values (1,'Second course', 'Sinier', 5);
insert into ezy_course_c6 (tutor_id, course_name, course_level, course_price) values (2,'test course', 'Sinier', 1);
