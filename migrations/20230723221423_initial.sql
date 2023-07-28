-- Add migration script here
CREATE TABLE tb_users (
	id SERIAL PRIMARY KEY,
	username VARCHAR(255) UNIQUE NOT NULL,
	email VARCHAR(255) UNIQUE NOT NULL,
	password VARCHAR(255) NOT NULL,
	modified_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tb_job_types (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,

	user_id INT NOT NULL,

	modified_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (user_id) REFERENCES tb_users(id)
);

CREATE TABLE tb_jobs (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	description TEXT NOT NULL,
	job_type_id INT NOT NULL,
	start_date TIMESTAMPTZ NOT NULL,
	end_date TIMESTAMPTZ NOT NULL,

	user_id INT NOT NULL,

	modified_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (job_type_id) REFERENCES tb_job_types(id),
	FOREIGN KEY (user_id) REFERENCES tb_users(id)
);

