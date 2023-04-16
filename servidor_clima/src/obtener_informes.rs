use diesel::prelude::*;
use servidor_clima::{models::{InformeClima,Estacion}, establish_connection, schema::{informes_clima::{self, fk_estacion},estaciones::{self, alias}}};

pub fn obtener_informes_alias(alias_estacion:&str, cantidad:i64)-> Vec<InformeClima>{
    
    let connection = &mut establish_connection();

    let estacion = estaciones::table.filter(alias.eq(alias_estacion))
                                                                    .load::<Estacion>(connection)
                                                                    .expect("Mensaje Error") ;

    let results = informes_clima::table
                                    .filter(fk_estacion.eq(estacion[0].id_estacion))
                                    .order(informes_clima::hora.desc())
                                    .limit(cantidad)
                                    .load::<InformeClima>(connection)
                                    .expect("Error cargando informes");

    println!("Mostrando {} informes", results.len());
    let mut vec_informes:Vec<InformeClima>= vec![];
    for informe in results{
        vec_informes.push(informe); 
    }

    return vec_informes;
}