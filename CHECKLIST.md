# ✅ CHECKLIST — API REST com Rust + Axum

## 🎯 Missão
Construir uma API REST CRUD de tarefas (to-do list) usando **Axum**, **Tokio**, **Serde** e **UUID**, com armazenamento em memória protegido por `Arc<Mutex<HashMap>>`.

---

## 📦 Passo 1 — Inicializar o Projeto Cargo

```bash
# Dentro da pasta SimpleApiRest (já estamos aqui)
cargo init
```

Isso cria:
- `Cargo.toml` — manifesto do projeto
- `src/main.rs` — arquivo principal

> [!IMPORTANT]
> O `cargo init` já vai ter criado um `src/main.rs` padrão com `fn main() {}`. Você vai substituir o conteúdo dele mais pra frente.

- [x] Rodar `cargo init`
- [x] Verificar que `src/main.rs` e `Cargo.toml` foram criados
- [x] Rodar `cargo run` e ver o "Hello, world!" no terminal

---

## 📦 Passo 2 — Adicionar as Dependências no `Cargo.toml`

Edite o `Cargo.toml` para incluir as dependências:

```toml
[package]
name = "simple-api-rest"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tower-http = { version = "0.5", features = ["cors"] }
```

### 📚 O que cada crate faz:

| Crate | Função |
|-------|--------|
| **axum** | Framework web para construir a API |
| **tokio** | Runtime assíncrono (roda o servidor) |
| **serde** | Serialização/deserialização JSON |
| **serde_json** | Manipulação de JSON |
| **uuid** | Geração de IDs únicos (v4) |
| **chrono** | Datas e horários (criado_em, atualizado_em) |
| **tower-http** | Middleware para CORS |

- [x] Editar `Cargo.toml` com as dependências acima
- [x] Rodar `cargo build` para baixar e compilar tudo (pode levar um tempinho na primeira vez)
- [x] ✅ Garantir que compilou sem erros

---

## 📦 Passo 3 — Criar os Modelos de Dados

Vamos definir as structs que representam nossos dados. Crie um arquivo `src/models.rs`:

```rust
// src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa uma tarefa no nosso sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: Uuid,
    pub titulo: String,
    pub descricao: String,
    pub concluida: bool,
    pub criada_em: DateTime<Utc>,
    pub atualizada_em: DateTime<Utc>,
}

/// Dados enviados para CRIAR uma nova tarefa (POST)
#[derive(Debug, Deserialize)]
pub struct CriarTarefaRequest {
    pub titulo: String,
    pub descricao: Option<String>,  // opcional!
}

/// Dados enviados para ATUALIZAR uma tarefa (PUT)
#[derive(Debug, Deserialize)]
pub struct AtualizarTarefaRequest {
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub concluida: Option<bool>,
}
```

### 🧠 Para pensar:

1. Por que `Tarefa` deriva `Clone`, `Serialize` **e** `Deserialize`, mas `CriarTarefaRequest` só deriva `Deserialize`?
2. Por que `descricao` é `Option<String>` no request, mas `String` na `Tarefa`?
3. O que significa `DateTime<Utc>`?

- [x] Criar `src/models.rs`
- [x] Declarar o módulo no `main.rs`: adicione `mod models;` no topo
- [x] Rodar `cargo check` e verificar se compila

---

## 📦 Passo 4 — Criar o Módulo de Erros

Vamos criar um tratamento de erros robusto. Crie `src/error.rs`:

```rust
// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Nosso tipo de erro customizado
#[derive(Debug)]
pub enum AppError {
    /// Tarefa não encontrada (404)
    NaoEncontrada,
    /// Dados inválidos (400)
    DadosInvalidos(String),
    /// Erro interno do servidor (500)
    Interno(String),
}

/// Implementamos IntoResponse para que o Axum saiba
/// como converter nosso erro em uma resposta HTTP
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensagem) = match self {
            AppError::NaoEncontrada => {
                (StatusCode::NOT_FOUND, "Tarefa não encontrada".to_string())
            }
            AppError::DadosInvalidos(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Interno(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        (status, Json(json!({ "erro": mensagem }))).into_response()
    }
}
```

### 🧠 Para pensar:

1. O que é a trait `IntoResponse` do Axum?
2. Por que retornamos `(status, Json(...)).into_response()`?
3. O que a macro `json!()` do `serde_json` faz?

- [x] Criar `src/error.rs`
- [x] Adicionar `mod error;` no `main.rs`
- [x] Rodar `cargo check`

---

## 📦 Passo 5 — Criar o Estado Compartilhado da Aplicação

Em `src/main.rs`, vamos definir o estado que será compartilhado entre todas as rotas:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod models;
mod error;

/// Estado compartilhado da aplicação.
/// Arc permite múltiplos owners (thread-safe).
/// Mutex garante acesso exclusivo (thread-safe).
type TarefasDb = Arc<Mutex<HashMap<Uuid, models::Tarefa>>>;

#[derive(Clone)]
struct AppState {
    db: TarefasDb,
}
```

### 🧠 Para pensar:

1. Por que `Arc<Mutex<HashMap>>` e não apenas `HashMap`?
2. O que aconteceria se dois requests tentassem modificar o HashMap ao mesmo tempo?
3. Por que a struct `AppState` precisa derivar `Clone`?

- [x] Adicionar os `use` e a definição de `AppState` no `main.rs`
- [x] Rodar `cargo check`

---

## 📦 Passo 6 — Implementar os Handlers (CRUD)

Agora a parte principal! Em `src/main.rs`, vamos implementar cada endpoint.

### 6.1 — GET /tarefas (Listar todas)

```rust
async fn listar_tarefas(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::Tarefa>>, error::AppError> {
    let db = state.db.lock().map_err(|e| {
        error::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    let mut tarefas: Vec<models::Tarefa> = db.values().cloned().collect();
    tarefas.sort_by(|a, b| a.criada_em.cmp(&b.criada_em));

    Ok(Json(tarefas))
}
```

### 6.2 — GET /tarefas/:id (Buscar por ID)

```rust
async fn buscar_tarefa(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<models::Tarefa>, error::AppError> {
    let db = state.db.lock().map_err(|e| {
        error::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    db.get(&id)
        .cloned()
        .ok_or(error::AppError::NaoEncontrada)
        .map(Json)
}
```

### 6.3 — POST /tarefas (Criar)

```rust
async fn criar_tarefa(
    State(state): State<AppState>,
    Json(payload): Json<models::CriarTarefaRequest>,
) -> Result<(StatusCode, Json<models::Tarefa>), error::AppError> {
    if payload.titulo.trim().is_empty() {
        return Err(error::AppError::DadosInvalidos(
            "O título não pode estar vazio".to_string(),
        ));
    }

    let agora = Utc::now();
    let tarefa = models::Tarefa {
        id: Uuid::new_v4(),
        titulo: payload.titulo,
        descricao: payload.descricao.unwrap_or_default(),
        concluida: false,
        criada_em: agora,
        atualizada_em: agora,
    };

    let mut db = state.db.lock().map_err(|e| {
        error::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    db.insert(tarefa.id, tarefa.clone());

    Ok((StatusCode::CREATED, Json(tarefa)))
}
```

### 6.4 — PUT /tarefas/:id (Atualizar)

```rust
async fn atualizar_tarefa(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<models::AtualizarTarefaRequest>,
) -> Result<Json<models::Tarefa>, error::AppError> {
    let mut db = state.db.lock().map_err(|e| {
        error::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    let tarefa = db.get_mut(&id).ok_or(error::AppError::NaoEncontrada)?;

    if let Some(titulo) = payload.titulo {
        if titulo.trim().is_empty() {
            return Err(error::AppError::DadosInvalidos(
                "O título não pode estar vazio".to_string(),
            ));
        }
        tarefa.titulo = titulo;
    }
    if let Some(descricao) = payload.descricao {
        tarefa.descricao = descricao;
    }
    if let Some(concluida) = payload.concluida {
        tarefa.concluida = concluida;
    }
    tarefa.atualizada_em = Utc::now();

    Ok(Json(tarefa.clone()))
}
```

### 6.5 — DELETE /tarefas/:id (Deletar)

```rust
async fn deletar_tarefa(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, error::AppError> {
    let mut db = state.db.lock().map_err(|e| {
        error::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    db.remove(&id).ok_or(error::AppError::NaoEncontrada)?;

    Ok(StatusCode::NO_CONTENT)
}
```

### 🧠 Para pensar:

1. O que é `State(state): State<AppState>`? Como o Axum injeta o estado?
2. Por que usamos `lock().map_err(...)` em vez de `lock().unwrap()`?
3. O que `Path(id): Path<Uuid>` faz? Como o Axum extrai o parâmetro da URL?
4. No `criar_tarefa`, retornamos `StatusCode::CREATED` (201). Por que não 200?
5. No `deletar_tarefa`, retornamos `StatusCode::NO_CONTENT` (204). Por que?
6. O que `?` (operador de propagação de erro) faz? Como ele funciona com nosso `AppError`?

- [x] Adicionar todos os handlers no `main.rs`
- [x] Adicionar `use chrono::Utc;` e `use axum::StatusCode;` no topo
- [x] Rodar `cargo check` e corrigir qualquer erro

---

## 📦 Passo 7 — Montar as Rotas e Iniciar o Servidor

No `main.rs`, vamos juntar tudo na função `main`:

```rust
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ... (seus módulos e handlers aqui)

#[tokio::main]
async fn main() {
    // Estado inicial vazio
    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    // Construindo o roteador
    let app = Router::new()
        .route("/tarefas", get(listar_tarefas))
        .route("/tarefas", post(criar_tarefa))
        .route("/tarefas/{id}", get(buscar_tarefa))
        .route("/tarefas/{id}", put(atualizar_tarefa))
        .route("/tarefas/{id}", delete(deletar_tarefa))
        .with_state(state);

    // Iniciar o servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Servidor rodando em http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### 🧠 Para pensar:

1. O que `#[tokio::main]` faz? Por que precisamos dele?
2. O que `Router::new()` retorna?
3. Qual a diferença entre `.route("/tarefas", get(listar_tarefas))` e `.route("/tarefas/{id}", get(buscar_tarefa))`?
4. Por que passamos `.with_state(state)`?
5. O que `tokio::net::TcpListener::bind("0.0.0.0:3000")` significa? Por que `0.0.0.0`?

- [x] Implementar a função `main` completa
- [x] Adicionar todos os `use` necessários no topo
- [x] Rodar `cargo check` e garantir que compila

---

## 📦 Passo 8 — Rodar e Testar!

```bash
# Terminal 1: Rodar o servidor
cargo run
```

Em **outro terminal**, teste cada endpoint com `curl`:

```bash
# Criar uma tarefa
curl -X POST http://localhost:3000/tarefas \
  -H "Content-Type: application/json" \
  -d '{"titulo": "Estudar Rust", "descricao": "Terminar o checklist da API"}'

# Listar todas as tarefas
curl http://localhost:3000/tarefas

# Buscar por ID (substitua pelo ID retornado)
curl http://localhost:3000/tarefas/SEU-ID-AQUI

# Atualizar
curl -X PUT http://localhost:3000/tarefas/SEU-ID-AQUI \
  -H "Content-Type: application/json" \
  -d '{"concluida": true}'

# Deletar
curl -X DELETE http://localhost:3000/tarefas/SEU-ID-AQUI

# Testar erro 404
curl http://localhost:3000/tarefas/00000000-0000-0000-0000-000000000000

# Testar erro 400 (título vazio)
curl -X POST http://localhost:3000/tarefas \
  -H "Content-Type: application/json" \
  -d '{"titulo": "", "descricao": "teste"}'
```

- [x] Rodar `cargo run`
- [x] Testar cada endpoint com `curl`
- [x] Verificar os códigos de status (201, 200, 204, 404, 400)
- [x] Chamar o professor (eu!) para revisar 🎉

---

## 🏁 Passo 9 — Desafios Extras (Opcional)

Se quiser ir além:

- [x] Implementar **filtro por status** (`GET /tarefas?concluida=true`)
- [x] Implementar **paginação** (`GET /tarefas?pagina=1&limite=10`)
- [x] Usar **`tokio::sync::RwLock`** em vez de `Mutex` (por quê seria melhor?)
- [x] Extrair a lógica para um **service layer** (`src/services.rs`)
- [x] Adicionar **testes de integração** com `axum::test`
- [ ] Adicionar **logging** com `tracing` + `tracing-subscriber`
- [ ] Adicionar **validação** com `validator` crate
- [ ] Mudar para **banco de dados real** (SQLite com `sqlx`)
- [ ] Adicionar **autenticação** com JWT

---

## 📚 Conceitos que você vai dominar ao final

| Conceito | Onde usamos |
|----------|-------------|
| Traits (Derive, IntoResponse) | `#[derive(Serialize)]`, `impl IntoResponse` |
| Ownership & Borrowing | `Arc`, `Mutex`, `clone()` |
| Pattern Matching | `match self { ... }` nos erros |
| Option & Result | `Option<String>`, `Result<Json<T>, AppError>` |
| Operador `?` | Propagação de erros |
| Generics | `HashMap<Uuid, Tarefa>`, `Result<Json<T>, E>` |
| Closures | `.map_err(\|e\| ...)` |
| Concorrência | `Arc`, `Mutex`, `tokio` |
| Macros | `#[tokio::main]`, `json!()`, `#[derive(...)]` |
| Módulos | `mod models;`, `mod error;` |

---

**Boa sorte!** 🦀 Quando terminar cada passo, me chame para revisar ou tirar dúvidas!
