# Mission: Evoluir a SimpleApiRest de CRUD didático para uma API de produção em Rust

## Why
Transformar a todo list em uma API completa e idiomática — aprendendo arquitetura real de serviços Rust (axum/sqlx) no processo, não só sintaxe.

## Success looks like
- Implementar autenticação JWT (único item pendente do CHECKLIST.md)
- Evoluir arquitetura: melhor separação de camadas, error handling robusto, logging estruturado
- Adicionar recursos de API real: rate limiting, paginação avançada, testes de propriedade
- Código passando `cargo clippy -- -D warnings` sem warnings

## Constraints
- Você escreve o código (mão na massa), eu guio e reviso
- Sessões longas (2h+)
- Nível avançado: sem re-explicar ownership/borrow checker básico
- Projeto local, sem deploy (por enquanto)

## Out of scope
- Frontend/interface web
- Deploy e infraestrutura (Docker, CI/CD)
- Outros bancos além de SQLite
