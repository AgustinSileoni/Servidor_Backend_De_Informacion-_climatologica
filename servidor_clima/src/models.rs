use diesel::{prelude::*};

use serde::{Serialize, Deserialize};


#[derive(Queryable,Serialize, Deserialize)]
pub struct InformeClima{
    pub id_informe: i32,
    pub fk_estacion: i32,
    pub fecha : String,
    pub hora: i32,
    pub nubosidad: String,
    pub visibilidad: i32,   
    pub temperatura: f32,
    pub sensacion_termica: f32,
    pub humedad: i32,
    pub viento: String,
    pub presion: f32,
}

#[derive(Queryable,Serialize, Deserialize)]
pub struct Estacion{
    pub id_estacion: i32,
    pub nombre: String,
    pub alias: String,
    pub provincia: String
}