-- Your SQL goes here
CREATE TABLE master(
	id SERIAL PRIMARY KEY,
	username VARCHAR (35),
	password VARCHAR (100)
);

CREATE TABLE projects(
	id SERIAL PRIMARY KEY, 
	app_type VARCHAR(255), 
	deployed_url VARCHAR(255), 
	description VARCHAR(255), 
	game_file VARCHAR(255), 
	git_url VARCHAR(255), 
	icon_file VARCHAR(255), 
	style_file VARCHAR(255), 
	title VARCHAR(255)
);