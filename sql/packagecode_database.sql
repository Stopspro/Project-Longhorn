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
