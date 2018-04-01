CREATE DATABASE packagev_database;

USE packagev_database;
CREATE TABLE packages (
  package_id INT NOT NULL,
  package_name VARCHAR(30) NOT NULL,
  PRIMARY KEY (package_id),
  UNIQUE (package_name)
);

INSERT INTO packages;
    (package_id, package_name) 
VALUES 
    (1,"0.1"),
    (2,"0.0.1"),
    (3,"0.0.1"),
    (4,"0.1"),
    (5,"0.0.1),
    (6,"0.1");
