use diesel::prelude::*;
use servidor_clima::{models::{InformeClima,Estacion}, establish_connection, schema::{informes_clima::{self, fk_estacion},estaciones::{self, alias}}};

pub fn mostrar_informes(){
    
    let connection = &mut establish_connection();

    let estacion = estaciones::table.filter(alias.eq("azul"))
                                                                    .load::<Estacion>(connection)
                                                                    .expect("Mensaje Error") ;

    let results = informes_clima::table
                                    .filter(fk_estacion.eq(estacion[0].id_estacion))
                                    .order(informes_clima::hora.desc())
                                    .limit(3)
                                    .load::<InformeClima>(connection)
                                    .expect("Error cargando informes");

    println!("Mostrando {} informes", results.len());
    for informe in results{
        println!("{}", serde_json::to_string(&informe).unwrap()); 
    }
}