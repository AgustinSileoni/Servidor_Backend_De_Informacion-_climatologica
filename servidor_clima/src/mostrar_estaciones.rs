use diesel::RunQueryDsl;
use servidor_clima::establish_connection;
use servidor_clima::models::Estacion;
use servidor_clima::schema::estaciones;

use serde_json;




pub fn mostrar_estaciones(){
    let connection = &mut establish_connection();

    let estaciones = estaciones::table.load::<Estacion>(connection)
                                                     .expect("Error al cargar las estaciones");
    for estacion in estaciones{
        println!("{}", serde_json::to_string(&estacion).unwrap()); 
    }
}