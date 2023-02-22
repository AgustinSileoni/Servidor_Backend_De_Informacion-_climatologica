/*
use diesel::{QueryDsl};
use servidor_clima::{models::Estacion, establish_connection, schema::estaciones};
use diesel::prelude::*;


pub fn obtner_estacion_id(id:i32){
 
    let connection = &mut establish_connection();
    let estaciones= estaciones::table.find(1)
                                    .load::<Estacion>(connection)
                                    .expect("Error cargando estaciones");
    
}
*/
/*
        let estacion = estaciones::table.find(1)
                                        .load::<Estacion>(connection)
                                        .expect("Error cargando estaciones")[0]
                                        .nombre
                                        .to_string();
*/