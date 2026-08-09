# Aluno demonstra verificação rigorosa; Tarefa 1 concluída

O aluno corrigiu o professor: ao afirmar o estado de `models.rs`/`services.rs` sem reler, o aluno já havia aplicado as mudanças (`Option<String>`, bind direto). Lição para o processo: sempre reler antes de afirmar estado.

## Evidência
- `descricao: Option<String>` em models.rs, `descricao: payload.descricao` em services.rs, migration com `TEXT NULL`, banco zerado (escolha consciente: pré-produção).
- `cargo test` → 8/8 passando após a mudança.
- Aluno optou pela abordagem A (não limpar campos via PATCH) por YAGNI, ciente do tri-state.

## Implicações
- ZDP avança: aluno verifica código de forma independente e questiona o professor — sinal de autonomia crescente.
- Tarefa 2 (QueryBuilder condicional) e Tarefa 3 (envelope de paginação) pendentes; teste `limite=-5` quebra na Tarefa 3 (formato array → objeto).
- Pendência: teste de criação sem `descricao` (buraco de cobertura revelado pela lição).
