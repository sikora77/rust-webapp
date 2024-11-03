-- Your SQL goes here
create table blogposts (
   id           serial primary key,
   body         text not null,
   publish_date timestamp default current_timestamp not null,
   username     varchar(50) not null,
   image_path   varchar(255),
   avatar_path  varchar(255)
);