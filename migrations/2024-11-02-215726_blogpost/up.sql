-- Your SQL goes here
create table blogposts (
   id           serial primary key,
   body         text not null,
   publish_date timestamp not null,
   username     varchar(50) not null,
   image_oid    oid,
   avatar_oid   oid
);