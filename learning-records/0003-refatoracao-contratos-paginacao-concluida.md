# Tarefas 1-3 concluídas: contrato de dados, QueryBuilder e envelope de paginação

O aluno executou a refatoração completa guiada da Lição 1: descricao nullable, WHERE condicional e envelope de paginação. 8/8 testes verdes, `cargo clippy -D warnings` limpo.

## O que foi aprendido (evidências)
- **Contracts:** migration/model alinhados (`Option<String>` + `TEXT NULL`), morte do `unwrap_or_default`; entendido que `SELECT 1`+`fetch_one` conta existência, não total.
- **QueryBuilder:** montagem condicional de WHERE, função `aplicar_filtro_de_concluida` compartilhada entre listagem e COUNT — um lugar só para a regra.
- **Semântica Option:** `Some(false)` é filtro real (pendentes); `None` = sem filtro. Tri-state (Option<Option<T>>) adiado por YAGNI — decisão consciente do aluno.
- **Envelope:** `PaginaDeTarefas { dados, paginacao }` com `tem_mais` calculado no servidor (fonte da verdade, pós-clamp). `total_paginas` cortado — YAGNI, volta se houver UI de páginas numeradas.
- **Type hygiene:** `as i64` eliminado (params i64 fim a fim); derives mínimos — `Deserialize` removido de tipos de saída (aluno detectou o smell sozinho).
- **Debug:** erro "Handler not implemented" do axum aponta pro `.route()`, não pro handler — causa real é retorno sem `IntoResponse` (faltava `Serialize`). Padrão: `#[axum::debug_handler]` para desmascarar.

## Implicações
- Próximas lições: JWT/autenticação (item pendente do CHECKLIST), TOCTOU/optimistic concurrency quando houver ownership por usuário, e testes para criação sem `descricao` (buraco de cobertura).
- Nota para VPS: COUNT é barato em SQLite local; se a tabela crescer, resposta é índice, não `COUNT(1)`.
- Aluno preparando frontend HTMX: envelope JSON fica para consumidores de API; paginação HTMX viverá em renderização de fragmento HTML.
