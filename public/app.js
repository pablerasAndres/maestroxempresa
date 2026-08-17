const API_URL = 'http://127.0.0.1:3000/api';

// Cambiar pestaña (Login vs Registro)
function mostrarFormulario(tipo) {
  const loginForm = document.getElementById('login-form');
  const regForm = document.getElementById('registro-form');
  const tabLogin = document.getElementById('tab-login');
  const tabRegistro = document.getElementById('tab-registro');
  const mensaje = document.getElementById('auth-mensaje');

  mensaje.textContent = '';

  if (tipo === 'login') {
    loginForm.classList.remove('hidden');
    regForm.classList.add('hidden');
    tabLogin.classList.add('active');
    tabRegistro.classList.remove('active');
  } else {
    loginForm.classList.add('hidden');
    regForm.classList.remove('hidden');
    tabLogin.classList.remove('active');
    tabRegistro.classList.add('active');
  }
}

// Adaptar etiqueta según si es Empresa o Maestro
function toggleCamposEmpresa() {
  const rol = document.getElementById('reg-rol').value;
  const lblNombre = document.getElementById('lbl-nombre');
  const inputNombre = document.getElementById('reg-nombre');

  if (rol === 'EMPRESA') {
    lblNombre.textContent = 'Nombre de la Empresa / Razón Social';
    inputNombre.placeholder = 'Constructora S.A.';
  } else {
    lblNombre.textContent = 'Nombre Completo';
    inputNombre.placeholder = 'Juan Pérez';
  }
}

// Petición de LOGIN
document.getElementById('login-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const email = document.getElementById('login-email').value;
  const password = document.getElementById('login-password').value;
  const mensaje = document.getElementById('auth-mensaje');

  try {
    const res = await fetch(`${API_URL}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password })
    });

    const texto = await res.text();
    if (res.ok) {
      mensaje.style.color = '#16a34a';
      mensaje.textContent = texto;
    } else {
      mensaje.style.color = '#dc2626';
      mensaje.textContent = texto;
    }
  } catch (err) {
    mensaje.style.color = '#dc2626';
    mensaje.textContent = 'Error al conectar con el backend.';
  }
});

// Petición de REGISTRO
document.getElementById('registro-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const mensaje = document.getElementById('auth-mensaje');

  const datos = {
    email: document.getElementById('reg-email').value,
    password: document.getElementById('reg-password').value,
    rol: document.getElementById('reg-rol').value,
    nombre_o_razonsocial: document.getElementById('reg-nombre').value,
    ciudad: document.getElementById('reg-ciudad').value || null,
    rut: null,
    telefono: null,
    especialidad: null
  };

  try {
    const res = await fetch(`${API_URL}/registro`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(datos)
    });

    const texto = await res.text();
    if (res.ok) {
      mensaje.style.color = '#16a34a';
      mensaje.textContent = '¡Cuenta creada con éxito! Ahora puedes iniciar sesión.';
      setTimeout(() => mostrarFormulario('login'), 1500);
    } else {
      mensaje.style.color = '#dc2626';
      mensaje.textContent = texto;
    }
  } catch (err) {
    mensaje.style.color = '#dc2626';
    mensaje.textContent = 'Error al conectar con el backend.';
  }
});