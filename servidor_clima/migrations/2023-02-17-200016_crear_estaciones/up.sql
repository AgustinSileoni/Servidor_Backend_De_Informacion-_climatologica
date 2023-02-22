-- Your SQL goes here
CREATE TABLE estaciones (
  id_estacion INT NOT NULL AUTO_INCREMENT,
  nombre VARCHAR(100) NOT NULL,
  alias VARCHAR(100) NOT NULL,
  provincia VARCHAR(100) NOT NULL,
  PRIMARY KEY (id_estacion)
);
