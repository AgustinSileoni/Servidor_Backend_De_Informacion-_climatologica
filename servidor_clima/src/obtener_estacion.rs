use diesel::{QueryDsl};
use servidor_clima::{models::Estacion, establish_connection, schema::estaciones::{self, alias, nombre}};
use diesel::prelude::*;


pub fn obtener_estacion_id(id:i32)-> Vec<Estacion>{

    let connection = &mut establish_connection();
    let estaciones= estaciones::table.find(id)
                                    .load::<Estacion>(connection)
                                    .expect("Error cargando estaciones");
    let mut a = vec![];
    for estacion in estaciones{
      a.push(estacion);
    }
    
    return a;
}  

pub fn obtener_estacion_alias(comparacion:&str){
    let connection = &mut establish_connection();
    let estaciones= estaciones::table.filter(alias.eq(comparacion))
                                    .load::<Estacion>(connection)
                                    .expect("Error cargando estaciones");

    for estacion in estaciones{
      println!("{}",serde_json::to_string(&estacion).unwrap());
    }
}

pub fn obtener_estacion_nombre(comparacion:&str){
    let connection = &mut establish_connection();
    let estaciones= estaciones::table.filter(nombre.eq(comparacion))
                                    .load::<Estacion>(connection)
                                    .expect("Error cargando estaciones");

    for estacion in estaciones{
      println!("{}",serde_json::to_string(&estacion).unwrap());
    }
}
//Problema para retornar el JSON o el vector