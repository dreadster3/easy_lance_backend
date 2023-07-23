-- Add migration script here
CREATE TABLE tb_job_types (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL
);

CREATE TABLE tb_jobs (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	description TEXT NOT NULL,
	job_type_id INT NOT NULL,
	date DATE NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (job_type_id) REFERENCES tb_job_types(id)
);

