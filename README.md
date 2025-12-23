# 🦀 Products Microservice - Rust GraphQL API

Microservicio de gestión de productos construido con Rust, GraphQL y MongoDB, siguiendo arquitectura hexagonal (Clean Architecture).

## 🚀 Características

- **GraphQL API** con async-graphql
- **MongoDB** como base de datos
- **Arquitectura Hexagonal** (Domain, Application, Infrastructure, Presentation)
- **Async/Await** con Tokio
- **Type-safe** con el sistema de tipos de Rust

## 📋 Requisitos Previos

- Rust 1.92.0 o superior
- MongoDB 2.8 o superior (local o remoto)
- Cargo (incluido con Rust)

## 🛠️ Instalación

1. **Clonar el repositorio:**

```bash
git clone <tu-repositorio>
cd products_microservice
```

2. **Configurar variables de entorno:**

Crea un archivo `.env` en la raíz del proyecto:

```env
MONGO_URI=mongodb://localhost:27017
```

3. **Instalar dependencias y compilar:**

```bash
cargo build --release
```

## ▶️ Ejecutar el Proyecto

```bash
cargo run
```

El servidor estará disponible en: `http://127.0.0.1:4000`

## 🎮 Uso de la API

### Interfaz GraphQL Playground

Abre tu navegador en:

```
http://127.0.0.1:4000/
```

### Endpoint GraphQL

```
POST http://127.0.0.1:4000/graphql
Content-Type: application/json
```

## 📝 Ejemplos de Queries

### 1. Health Check

**GraphQL:**

```graphql
{
  healthCheck
}
```

**Postman (JSON):**

```json
{
  "query": "{ healthCheck }"
}
```

**Respuesta:**

```json
{
  "data": {
    "healthCheck": "¡El servicio está vivo! 🚀"
  }
}
```

---

### 2. Obtener Todos los Productos

**GraphQL:**

```graphql
{
  products {
    id
    name
    price
    stock
    category
  }
}
```

**Postman (JSON):**

```json
{
  "query": "{ products { id name price stock category } }"
}
```

**Respuesta:**

```json
{
  "data": {
    "products": [
      {
        "id": "507f1f77bcf86cd799439011",
        "name": "Laptop Dell",
        "price": 15999.99,
        "stock": 25,
        "category": "Electrónica"
      }
    ]
  }
}
```

---

### 3. Crear un Producto

**GraphQL:**

```graphql
mutation {
  createProduct(
    name: "Laptop Dell"
    price: 15999.99
    stock: 25
    category: "Electrónica"
  ) {
    id
    name
    price
    stock
    category
  }
}
```

**Postman (JSON):**

```json
{
  "query": "mutation { createProduct(name: \"Laptop Dell\", price: 15999.99, stock: 25, category: \"Electrónica\") { id name price stock category } }"
}
```

**Respuesta:**

```json
{
  "data": {
    "createProduct": {
      "id": "507f1f77bcf86cd799439011",
      "name": "Laptop Dell",
      "price": 15999.99,
      "stock": 25,
      "category": "Electrónica"
    }
  }
}
```

---

### 4. Crear Producto con Variables

**GraphQL Query:**

```graphql
mutation CreateProduct(
  $name: String!
  $price: Float!
  $stock: Int!
  $category: String!
) {
  createProduct(
    name: $name
    price: $price
    stock: $stock
    category: $category
  ) {
    id
    name
    price
    stock
    category
  }
}
```

**Variables:**

```json
{
  "name": "Mouse Logitech",
  "price": 299.99,
  "stock": 100,
  "category": "Accesorios"
}
```

**Postman (JSON):**

```json
{
  "query": "mutation CreateProduct($name: String!, $price: Float!, $stock: Int!, $category: String!) { createProduct(name: $name, price: $price, stock: $stock, category: $category) { id name price stock category } }",
  "variables": {
    "name": "Mouse Logitech",
    "price": 299.99,
    "stock": 100,
    "category": "Accesorios"
  }
}
```

## 🏗️ Arquitectura del Proyecto

```
src/
├── domain/              # Capa de dominio (entidades y contratos)
│   ├── entity/
│   │   └── product_entity.rs
│   └── repository/
│       └── product_repository.rs
├── application/         # Casos de uso
│   ├── create_product.rs
│   └── get_product.rs
├── infrastructure/      # Implementaciones técnicas
│   ├── mongo_connection.rs
│   └── mongo_product_repository.rs
├── presentation/        # Capa de presentación (GraphQL)
│   └── graphql_schema.rs
├── lib.rs
└── main.rs
```

### Capas de la Arquitectura

- **Domain**: Lógica de negocio pura, sin dependencias externas
- **Application**: Casos de uso que orquestan la lógica de dominio
- **Infrastructure**: Implementaciones concretas (MongoDB, etc.)
- **Presentation**: Interfaz GraphQL para exponer la API

## 🔧 Tecnologías Utilizadas

| Tecnología    | Versión | Propósito                |
| ------------- | ------- | ------------------------ |
| Rust          | 1.92.0  | Lenguaje de programación |
| Tokio         | 1.0     | Runtime asíncrono        |
| Axum          | 0.6     | Framework web            |
| async-graphql | 6.0     | Servidor GraphQL         |
| MongoDB       | 2.8     | Base de datos            |
| async-trait   | 0.1     | Traits asíncronos        |

## 📦 Dependencias Principales

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.6"
async-graphql = { version = "6.0", features = ["bson","chrono"] }
async-graphql-axum = "6.0"
mongodb = "2.8"
serde = { version = "1.0", features = ["derive"] }
async-trait = "0.1"
```

## 🧪 Testing

```bash
# Ejecutar tests
cargo test

# Ejecutar tests con output
cargo test -- --nocapture
```

## 🐛 Troubleshooting

### Error: "Cannot parse the unexpected character '<'"

- **Causa**: Usando GET en lugar de POST
- **Solución**: Asegúrate de usar método POST en Postman

### Error: "expected executable_definition"

- **Causa**: Content-Type incorrecto o body mal formateado
- **Solución**: Verifica que el header sea `Content-Type: application/json`

### Error de conexión a MongoDB

- **Causa**: MongoDB no está corriendo o URI incorrecta
- **Solución**: Verifica que MongoDB esté activo y la variable `MONGO_URI` en `.env`

## 📄 Licencia

Este proyecto está bajo la licencia MIT.

## 👨‍💻 Autor

Samuel Edrei Uc Angulo

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request
