# Lição 1: Revisão de código sênior — contratos de dados e atomicidade

O aluno (nível avançado) demonstrou: (1) identificou sozinho um bug real de desalinhamento migration/model (`descricao NOT NULL` no banco vs `Option<String>` na API, costurado por `unwrap_or_default()`); (2) foi honesto ao admitir que copiou a query de filtro sem analisar; (3) diagnosticou corretamente a falta de metadados de paginação; (4) desconhece o padrão TOCTOU e a atomicidade de instruções SQL únicas.

## Implicações para as próximas lições
- Zona de desenvolvimento proximal: contratos de dados (fonte única de verdade), construção condicional de queries (sqlx::QueryBuilder), envelope de paginação, optimistic concurrency.
- Ensinar: por que `unwrap_or_default()` é uma costura frágil; TOCTOU aparecerá na implementação de JWT/ownership.
- Decisão pendente do aluno: contrato da `descricao` (obrigatória vs opcional vs nullable).
