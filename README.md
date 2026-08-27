# Fast Track

Uma **"search engine"** mais rápida que a Pesquisa do Windows. Exibe resultados enquanto você digita e abre arquivos com um único clique. Sem busca na web, sem anúncios. Apenas você, seus arquivos e programas.

Em desenvolvimento com **Tauri** e **Rust** — leve, rápido e nativo.

> ⚠️ Projeto em fase inicial de desenvolvimento (Fase 1). Parte das funcionalidades abaixo ainda estão em construção — o README reflete o que já está decidido/implementado e o que está planejado.

---

## 🛠️ Tecnologias

Rust · Tauri · React (frontend) · `notify` (filesystem events) · `bincode` / `serde_json` (serialização)

---

## 🧠 Como funciona a indexação

A indexação percorre o sistema de arquivos uma vez e constrói uma estrutura em memória (`Vec<EntradaIndexada>`). A busca consulta essa estrutura, **nunca o disco diretamente** — é isso que torna os resultados instantâneos.

```
Boot do app
    └─▶ Indexador percorre o disco (walkdir)
          └─▶ Constrói um Vec<EntradaIndexada> em memória
                └─▶ Busca consulta esse Vec (nunca o disco)
```

## 👀 Monitoramento em tempo real

Após a indexação inicial, o índice ficaria desatualizado a cada criação/remoção/movimentação de arquivo. Solução: monitorar o sistema de arquivos via eventos nativos do SO, usando o crate `notify` (`Create`, `Remove`, `Modify`, `Rename`).

A atualização é **cirúrgica** — só a entrada afetada é adicionada/removida, sem reindexar tudo (uma reindexação completa pode levar segundos; uma atualização pontual é instantânea).

## 🔎 Busca incremental

Resultados aparecem e atualizam a cada tecla pressionada. Isso exige dois cuidados técnicos:

- **Debounce:** aguarda um intervalo de silêncio antes de disparar a busca, evitando 7 buscas para "firefox" digitado em 200ms.
- **Cancelamento de busca obsoleta:** se uma busca demora mais que o esperado e o usuário já digitou outra coisa, o resultado antigo não pode sobrescrever o atual — resolvido com um ID de requisição.

## 🔒 Concorrência

O índice fica em memória compartilhada entre três threads (indexador, monitor de arquivos, e resposta às buscas do frontend), usando `Arc<RwLock<T>>`.

| | Mutex | RwLock |
|---|---|---|
| Leitores simultâneos | ❌ Um por vez | ✅ Vários ao mesmo tempo |
| Escritores | Um por vez | Um por vez |
| Uso ideal | Escrita frequente | Leitura frequente, escrita rara |

No Fast Track a busca (leitura) acontece dezenas de vezes por segundo enquanto o usuário digita, e a escrita é rara (só quando `notify` detecta mudança) — `RwLock` é o encaixe natural. Estado gerenciado via `tauri::State`.

## 💾 Armazenamento do índice

O índice vive em RAM durante a execução (acesso ordens de magnitude mais rápido que disco). Para evitar reindexar tudo a cada boot, o índice é serializado em disco (via crate `dirs` para o local padrão do SO):

```
Boot do app
    ├─▶ Existe cache em disco?
    │       ├─▶ Sim → carrega cache (~100ms) → atualiza em background
    │       └─▶ Não → indexação completa (2–5s)
    └─▶ App disponível
```

Formato: `bincode` para produção (rápido e compacto — ~50ms para 100k entradas vs ~500ms em JSON), `serde_json` só para debug legível.

## 🔌 Comunicação Rust ↔ Frontend (IPC)

**`invoke`** — Frontend chama Backend (request/response), usado quando o usuário espera um resultado (ex.: buscar arquivos, abrir arquivo).

```typescript
const resultados = await invoke<Resultado[]>("buscar", { query: "firefox" });
```

```rust
#[tauri::command]
pub fn buscar(query: String, indice: tauri::State<Indice>) -> Vec<EntradaIndexada> { /* ... */ }
```

**`emit`/`listen`** — Backend notifica Frontend de forma assíncrona (ex.: indexação concluída, progresso de operação longa).

---

## 📊 Métricas planejadas

| Métrica | Meta |
|---|---|
| Tempo até primeiro resultado | < 100ms |
| Tempo de indexação inicial | < 5s |
| Tempo de carregamento do cache | < 200ms |
| Uso de RAM do índice | < 100MB |
| Latência do atalho global | < 50ms |


