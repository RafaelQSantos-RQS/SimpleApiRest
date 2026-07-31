use axum::http::StatusCode;
use axum_test::{TestResponse, TestServer};
use serde_json::json;
use simplerestapi::{AppState, criar_router};
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

async fn criar_servidor_teste() -> TestServer {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Falha ao conectar no bando de dados");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Falha ao rodar as migrations");
    let state = AppState { db: pool };

    let app = criar_router(state);

    TestServer::new(app)
}

#[tokio::test]
async fn teste_listar_tarefas_vazia() {
    // Inicialiando o servidor
    let server: TestServer = criar_servidor_teste().await;

    // Efeutando a requisição de teste
    let resposta: TestResponse = server.get("/tarefas").await;

    // Verificando
    resposta.assert_status(StatusCode::OK);
    resposta.assert_json(&json!([]));
}

#[tokio::test]
async fn teste_criar_tarefa_com_sucesso() {
    // Inicializando o servidor de teste
    let server: TestServer = criar_servidor_teste().await;

    // Criando o body
    let body: serde_json::Value = json!({
      "titulo": "Estudar Rust",
      "descricao": "Terminar o checklist da API"
    });

    // Efeutnado a requisição
    let resposta: TestResponse = server.post("/tarefas").json(&body).await;

    // Verificando o status code
    resposta.assert_status(StatusCode::CREATED);

    // Deserializadno o json para análise
    let tarefa_criada: serde_json::Value = resposta.json();

    // Verificando os campos
    assert_eq!(tarefa_criada["titulo"], "Estudar Rust");
    assert_eq!(tarefa_criada["descricao"], "Terminar o checklist da API");
    assert_eq!(tarefa_criada["concluida"], false);
    assert!(tarefa_criada["id"].is_string());
    assert!(tarefa_criada["criada_em"].is_string());
    assert!(tarefa_criada["atualizada_em"].is_string());
}

#[tokio::test]
async fn teste_criar_tarefa_com_titulo_vazio() {
    let server = criar_servidor_teste().await;

    let body = json!({
        "titulo": "",
        "descricao": "Tarefa sem titulo"
    });

    let resposta = server.post("/tarefas").json(&body).await;

    resposta.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn teste_buscar_tarefa_inexistente() {
    let server = criar_servidor_teste().await;

    let id_teste = Uuid::new_v4();

    let resposta = server.get(format!("/tarefas/{}", id_teste).as_str()).await;

    resposta.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn teste_atualizar_tarefa_com_sucesso() {
    let server = criar_servidor_teste().await;

    let body = json!({
        "titulo": "Estudar Rust",
        "descricao": "Terminar o checklist da API"
    });

    let req_criacao = server.post("/tarefas").json(&body).await;

    // Deserializadno o json para análise
    let tarefa_criada: serde_json::Value = req_criacao.json();

    // Verificando os campos
    assert_eq!(tarefa_criada["titulo"], "Estudar Rust");
    assert_eq!(tarefa_criada["descricao"], "Terminar o checklist da API");
    assert_eq!(tarefa_criada["concluida"], false);
    assert!(tarefa_criada["id"].is_string());
    assert!(tarefa_criada["criada_em"].is_string());
    assert!(tarefa_criada["atualizada_em"].is_string());

    let payload_para_atualizar = json!({
        "descricao": "Descrição Atualizada",
        "concluida": true,
    });

    let id = tarefa_criada["id"].as_str().unwrap();
    let req_atualizacao = server
        .put(&format!("/tarefas/{}", id))
        .json(&payload_para_atualizar)
        .await;

    req_atualizacao.assert_status(StatusCode::OK);

    let tarefa_atualizada: serde_json::Value = req_atualizacao.json();

    assert_eq!(tarefa_atualizada["titulo"], "Estudar Rust");
    assert_eq!(tarefa_atualizada["descricao"], "Descrição Atualizada");
    assert_eq!(tarefa_atualizada["concluida"], true);
}

#[tokio::test]
async fn teste_deletar_tarefa_com_sucesso() {
    let server = criar_servidor_teste().await;

    let body = json!({
        "titulo": "Estudar Rust",
        "descricao": "Terminar o checklist da API"
    });

    let req_criacao = server.post("/tarefas").json(&body).await;

    let tarefa_criada: serde_json::Value = req_criacao.json();

    assert_eq!(tarefa_criada["titulo"], "Estudar Rust");
    assert_eq!(tarefa_criada["descricao"], "Terminar o checklist da API");
    assert_eq!(tarefa_criada["concluida"], false);
    assert!(tarefa_criada["id"].is_string());
    assert!(tarefa_criada["criada_em"].is_string());
    assert!(tarefa_criada["atualizada_em"].is_string());

    let id_para_deletar = tarefa_criada["id"].as_str().unwrap();

    let req_deletar = server
        .delete(&format!("/tarefas/{}", id_para_deletar))
        .await;

    req_deletar.assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn teste_criar_tarefa_com_titulo_de_espacos() {
    let server = criar_servidor_teste().await;

    let body = json!({
        "titulo": "    ",
        "descricao": "Teste de espaços vazios",
    });

    let resposta = server.post("/tarefas").json(&body).await;

    assert_eq!(resposta.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn teste_listar_tarefas_com_limite_invalido() {
    let server = criar_servidor_teste().await;

    for i in 0..3 {
        let body = json!({
            "titulo": format!("Tarefa {}", i),
            "descricao": "Teste de limite"
        });
        server.post("/tarefas").json(&body).await;
    }

    let resposta = server.get("/tarefas?limite=-5").await;
    resposta.assert_status(StatusCode::OK);

    let tarefas: serde_json::Value = resposta.json();
    assert_eq!(tarefas.as_array().unwrap().len(), 1);
}
