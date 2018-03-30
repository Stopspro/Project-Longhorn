CREATE DATABASE packages_database;

CREATE DATABASE packagev_database;

CREATE DATABASE packagecode_database;

CREATE DATABASE ioncode_database;

CREATE DATABASE electroncode_database;

CREATE DATABASE neutroncode_database;

USE packages_database;
CREATE TABLE packages (
  package_id INT NOT NULL,
  package_name VARCHAR(30) NOT NULL,
  PRIMARY KEY (recipe_id),
  UNIQUE (recipe_name)
);

INSERT INTO packages;
    (package_id, package_name) 
VALUES 
    (1,"ion"),
    (2,"neutron"),
    (3,"electron"),
    (4,"molecule"),
    (5,"chess"),
    (6,"unicorn");

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
     
USE packagecode_database;
CREATE TABLE molecule (
  package_id INT NOT NULL,
  line_contents VARCHAR(30) NOT NULL,
  PRIMARY KEY (package_id),
  UNIQUE (line_contents)
);

INSERT INTO molecule;
    (package_id, line_contents)
VALUES 
    (1,"lib.rs"),
    (2,""),
    (3,"load.rs");

CREATE TABLE chess (
  package_id INT NOT NULL,
  line_contents VARCHAR(30) NOT NULL,
  PRIMARY KEY (package_id),
  UNIQUE (line_contents)
);

INSERT INTO chess;
    (package_id, line_contents) 
VALUES 
    (1,"lib.rs"),
    (2,""),
    (3,"");
     
CREATE TABLE unicorn (
  package_id INT NOT NULL,
  line_contents VARCHAR(30) NOT NULL,
  PRIMARY KEY (package_id),
  UNIQUE (line_contents)
);

INSERT INTO unicorn;
    (package_id, line_contents) 
VALUES 
    (1,"lib.rs"),
    (2,""),
    (3,"");
