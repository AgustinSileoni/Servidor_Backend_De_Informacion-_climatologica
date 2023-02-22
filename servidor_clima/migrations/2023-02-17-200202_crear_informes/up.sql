-- Your SQL goes here
CREATE TABLE informes_clima (
  id_informe INT NOT NULL AUTO_INCREMENT,
  fk_estacion INT NOT NULL,
  fecha VARCHAR(10) NOT NULL,
  hora INT NOT NULL,
  nubosidad VARCHAR(100) NOT NULL,
  visibilidad INT(3)  NOT NULL,
  temperatura FLOAT(4)  NOT NULL,
  sensacion_termica FLOAT(4)  NOT NULL,
  humedad INT(3)  NOT NULL,
  viento VARCHAR(45)  NOT NULL,
  presion FLOAT(6)  NOT NULL,
  PRIMARY KEY (id_informe),
  FOREIGN KEY(fk_estacion) REFERENCES estaciones(id_estacion),
  UNIQUE INDEX id_informe_UNIQUE (id_informe ASC) VISIBLE
);