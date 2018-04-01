CREATE DATABASE packages_database;

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
