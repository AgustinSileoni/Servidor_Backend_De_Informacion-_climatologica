// @generated automatically by Diesel CLI.

diesel::table! {
    estaciones (id_estacion) {
        id_estacion -> Integer,
        nombre -> Varchar,
        alias -> Varchar,
        provincia -> Varchar,
    }
}

diesel::table! {
    informes_clima (id_informe) {
        id_informe -> Integer,
        fk_estacion -> Integer,
        fecha -> Varchar,
        hora -> Integer,
        nubosidad -> Varchar,
        visibilidad -> Integer,
        temperatura -> Float,
        sensacion_termica -> Float,
        humedad -> Integer,
        viento -> Varchar,
        presion -> Float,
    }
}

diesel::joinable!(informes_clima -> estaciones (fk_estacion));

diesel::allow_tables_to_appear_in_same_query!(
    estaciones,
    informes_clima,
);
