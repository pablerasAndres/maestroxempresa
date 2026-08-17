use axum::{extract::Path, extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;
use crate::models::{LoginUsuario, NuevaOferta, NuevaPostulacion, OfertaTrabajo, RegistroUsuario};

// Endpoint: Registrar usuario y perfil (Con contraseña encriptada)
pub async fn registrar_usuario(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegistroUsuario>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error al procesar contraseña".to_string()))?;

    let res = sqlx::query!(
        "INSERT INTO usuarios (email, password, rol) VALUES (?, ?, ?)",
        payload.email,
        password_hash,
        payload.rol
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al crear usuario: {}", e)))?;

    let usuario_id = res.last_insert_id() as i32;

    sqlx::query!(
        "INSERT INTO perfiles (usuario_id, nombre_o_razonsocial, rut, telefono, especialidad, ciudad) VALUES (?, ?, ?, ?, ?, ?)",
        usuario_id,
        payload.nombre_o_razonsocial,
        payload.rut,
        payload.telefono,
        payload.especialidad,
        payload.ciudad
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al crear perfil: {}", e)))?;

    Ok((StatusCode::CREATED, format!("Usuario y perfil registrados con ID {}", usuario_id)))
}

// Endpoint: Iniciar Sesión / Login (Validando hash de contraseña)
pub async fn login_usuario(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginUsuario>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let usuario = sqlx::query!(
        "SELECT id, email, password, rol FROM usuarios WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error en la consulta: {}", e)))?;

    match usuario {
        Some(user) => {
            let es_valida = verify(&payload.password, &user.password).unwrap_or(false);
            if es_valida {
                Ok((StatusCode::OK, format!("Login exitoso. Bienvenido usuario ID {} ({})", user.id, user.rol)))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Contraseña incorrecta".to_string()))
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Usuario no encontrado".to_string())),
    }
}

// Endpoint: Crear nueva oferta de trabajo
pub async fn crear_oferta(
    State(pool): State<MySqlPool>,
    Json(payload): Json<NuevaOferta>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let es_privada = payload.es_privada.unwrap_or(false);

    let res = sqlx::query!(
        "INSERT INTO ofertas_trabajo (empresa_id, titulo, descripcion, ubicacion, presupuesto, es_privada) VALUES (?, ?, ?, ?, ?, ?)",
        payload.empresa_id,
        payload.titulo,
        payload.descripcion,
        payload.ubicacion,
        payload.presupuesto,
        es_privada
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al crear oferta: {}", e)))?;

    Ok((StatusCode::CREATED, format!("Oferta creada con ID {}", res.last_insert_id())))
}

// Endpoint: Obtener todas las ofertas publicadas (GET)
pub async fn obtener_ofertas(
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rows = sqlx::query!(
        "SELECT id, empresa_id, titulo, descripcion, ubicacion, CAST(presupuesto AS DOUBLE) as presupuesto, es_privada, estado FROM ofertas_trabajo WHERE estado = 'PUBLICADA' OR estado = 'PENDIENTE'"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al obtener ofertas: {}", e)))?;

    let ofertas: Vec<OfertaTrabajo> = rows
        .into_iter()
        .map(|r| OfertaTrabajo {
            id: r.id,
            empresa_id: r.empresa_id,
            titulo: r.titulo,
            descripcion: r.descripcion,
            ubicacion: r.ubicacion,
            presupuesto: r.presupuesto,
            es_privada: r.es_privada.map(|v| v != 0),
            estado: r.estado,
        })
        .collect();

    Ok((StatusCode::OK, Json(ofertas)))
}
// Endpoint: Obtener el perfil de un usuario específico por ID (GET)
pub async fn obtener_perfil(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let perfil = sqlx::query!(
        "SELECT u.id, u.email, u.rol, p.nombre_o_razonsocial, p.rut, p.telefono, p.especialidad, p.ciudad 
         FROM usuarios u 
         JOIN perfiles p ON u.id = p.usuario_id 
         WHERE u.id = ?",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al obtener perfil: {}", e)))?;

    match perfil {
        Some(p) => Ok((StatusCode::OK, Json(serde_json::json!({
            "id": p.id,
            "email": p.email,
            "rol": p.rol,
            "nombre": p.nombre_o_razonsocial,
            "rut": p.rut,
            "telefono": p.telefono,
            "especialidad": p.especialidad,
            "ciudad": p.ciudad
        })))),
        None => Err((StatusCode::NOT_FOUND, "Perfil no encontrado".to_string())),
    }
}

// Endpoint: Postular a una oferta de trabajo
pub async fn postular_a_oferta(
    State(pool): State<MySqlPool>,
    Json(payload): Json<NuevaPostulacion>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let res = sqlx::query!(
        "INSERT INTO postulaciones (oferta_id, maestro_id, precio_cotizado, mensaje) VALUES (?, ?, ?, ?)",
        payload.oferta_id,
        payload.maestro_id,
        payload.precio_cotizado,
        payload.mensaje
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al postular: {}", e)))?;

    Ok((StatusCode::CREATED, format!("Postulación enviada con ID {}", res.last_insert_id())))
}