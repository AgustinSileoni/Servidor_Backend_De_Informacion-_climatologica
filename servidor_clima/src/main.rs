pub mod obtener_informes;
pub mod mostrar_estaciones;
pub mod obtener_estacion;

fn main() {

    //mostrar_estaciones::mostrar_estaciones();
    //let a = obtener_estacion::obtener_estacion_id(1);
    //for b in a {
    //    println!("{}",serde_json::to_string(&b).unwrap());
    //}
    //obtener_estacion::obtener_estacion_alias("azul");
    //obtener_estacion::obtener_estacion_nombre("AZUL");
    // let a = obtener_informes::obtener_informes_alias("azul",10);
    // for b in a {
        // println!("{}",serde_json::to_string_pretty(&b).unwrap());
    // }
    let b = obtener_estacion::obtener_estacion_alias("azul");

}