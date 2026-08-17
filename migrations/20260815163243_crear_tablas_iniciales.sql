-- 1. USUARIOS
CREATE TABLE IF NOT EXISTS usuarios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    rol ENUM('ADMIN', 'EMPRESA', 'MAESTRO') NOT NULL,
    creado_en TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 2. PERFILES (Información de la persona o empresa)
CREATE TABLE IF NOT EXISTS perfiles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT NOT NULL UNIQUE,
    nombre_o_razonsocial VARCHAR(150) NOT NULL,
    rut VARCHAR(20),
    telefono VARCHAR(30),
    especialidad VARCHAR(100), -- Solo para maestros (ej: Vidriería, Electricidad)
    ciudad VARCHAR(100),
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- 3. OFERTAS DE TRABAJO
CREATE TABLE IF NOT EXISTS ofertas_trabajo (
    id INT AUTO_INCREMENT PRIMARY KEY,
    empresa_id INT NOT NULL,
    titulo VARCHAR(150) NOT NULL,
    descripcion TEXT NOT NULL,
    ubicacion VARCHAR(150) NOT NULL,
    presupuesto DECIMAL(10, 2),
    es_privada BOOLEAN DEFAULT FALSE,
    estado ENUM('PENDIENTE', 'PUBLICADA', 'FINALIZADA', 'RECHAZADA') DEFAULT 'PENDIENTE',
    creado_en TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (empresa_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- 4. POSTULACIONES
CREATE TABLE IF NOT EXISTS postulaciones (
    id INT AUTO_INCREMENT PRIMARY KEY,
    oferta_id INT NOT NULL,
    maestro_id INT NOT NULL,
    precio_cotizado DECIMAL(10, 2),
    mensaje TEXT,
    estado ENUM('PENDIENTE', 'ACEPTADO', 'RECHAZADO') DEFAULT 'PENDIENTE',
    creado_en TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (oferta_id) REFERENCES ofertas_trabajo(id) ON DELETE CASCADE,
    FOREIGN KEY (maestro_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- 5. CALIFICACIONES
CREATE TABLE IF NOT EXISTS calificaciones (
    id INT AUTO_INCREMENT PRIMARY KEY,
    oferta_id INT NOT NULL,
    evaluador_id INT NOT NULL,
    evaluado_id INT NOT NULL,
    estrellas TINYINT NOT NULL, -- De 1 a 5
    comentario TEXT,
    FOREIGN KEY (oferta_id) REFERENCES ofertas_trabajo(id),
    FOREIGN KEY (evaluador_id) REFERENCES usuarios(id),
    FOREIGN KEY (evaluado_id) REFERENCES usuarios(id)
);