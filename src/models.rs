use serde::{Deserialize, Serialize};

// 1. Estructuras para Usuarios
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i32,
    pub email: String,
    pub rol: String,
}

#[derive(Debug, Deserialize)]
pub struct RegistroUsuario {
    pub email: String,
    pub password: String,
    pub rol: String, // 'ADMIN', 'EMPRESA', 'MAESTRO'
    pub nombre_o_razonsocial: String,
    pub rut: Option<String>,
    pub telefono: Option<String>,
    pub especialidad: Option<String>,
    pub ciudad: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUsuario {
    pub email: String,
    pub password: String,
}

// 2. Estructuras para Ofertas de Trabajo
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OfertaTrabajo {
    pub id: i32,
    pub empresa_id: i32,
    pub titulo: String,
    pub descripcion: String,
    pub ubicacion: String,
    pub presupuesto: Option<f64>,
    pub es_privada: Option<bool>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevaOferta {
    pub empresa_id: i32,
    pub titulo: String,
    pub descripcion: String,
    pub ubicacion: String,
    pub presupuesto: Option<f64>,
    pub es_privada: Option<bool>,
}

// 3. Estructura para Postulaciones
#[derive(Debug, Deserialize)]
pub struct NuevaPostulacion {
    pub oferta_id: i32,
    pub maestro_id: i32,
    pub precio_cotizado: Option<f64>,
    pub mensaje: Option<String>,
}