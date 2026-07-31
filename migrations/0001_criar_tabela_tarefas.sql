CREATE TABLE IF NOT EXISTS tarefas (
	id				TEXT PRIMARY KEY NOT NULL,
	titulo			TEXT NOT NULL,
	descricao		TEXT NOT NULL,
	concluida		BOOLEAN NOT NULL DEFAULT 0,
	criada_em		TEXT NOT NULL,
	atualizada_em	TEXT NOT NULL
)
