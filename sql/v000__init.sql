create extension "uuid-ossp";

create table if not exists app
(
	id uuid default uuid_generate_v4() not null
		constraint app_pk
			primary key,
	name text,
	created_at timestamp with time zone default now()
);

create table if not exists "user"
(
	id uuid default uuid_generate_v4() not null
		constraint user_pk
			primary key,
	app_id uuid not null
		constraint user_app_id_fk
			references app,
	name text not null,
	created_at timestamp with time zone default now() not null
);

create table score
(
	id uuid default uuid_generate_v4() not null
		constraint score_pk
			primary key,
	user_id uuid not null
		constraint score_user_id_fk
			references "user",
	app_id uuid not null
		constraint score_app_id_fk
			references app,
	score double precision,
	created_at timestamp with time zone default now() not null
);