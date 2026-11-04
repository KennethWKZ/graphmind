# graphmind

Local-first code intelligence CLI with MCP server.

## Tech Stack
- **Rust** — full Cargo workspace
- **Parsing**: tree-sitter via napi-rs (`crates/graphmind-core/`)
- **Graph**: SQLite + FTS5 (`crates/graphmind-db/`)
- **Memory**: JSONL store + cross-project inference (`crates/graphmind-memory/`)
- **Embeddings**: cosine search + RRF (`crates/graphmind-embeddings/`)
- **MCP**: rmcp SDK v1.5, stdio transport (`crates/graphmind-mcp/`)
- **CLI**: clap (`crates/graphmind-cli/`)

## Commands
```bash
cargo check --workspace       # check all crates
cargo build --release -p graphmind-cli  # build release binary
cargo clippy --workspace -- -D warnings # lint
cargo test --workspace        # run tests
```

## User-facing setup
```bash
graphmind setup               # global one-time: hooks + MCP + skill
graphmind init                # per-project: register + git hooks + build
graphmind init --skip-build   # per-project without building
```

## Project Structure
- `crates/graphmind-core/` — Rust parsing engine (tree-sitter, napi-rs)
- `crates/graphmind-db/` — SQLite graph (schema, builder, queries, FTS5, cache)
- `crates/graphmind-memory/` — JSONL memory store + cross-project links
- `crates/graphmind-embeddings/` — embedding store, cosine search, RRF fusion
- `crates/graphmind-mcp/` — MCP server (rmcp SDK, 27 tools)
- `crates/graphmind-cli/` — CLI (clap, all commands)

## Installation
```bash
curl -fsSL https://raw.githubusercontent.com/aouicher/graphmind/main/scripts/install.sh | bash
```

## Git & GitHub conventions
- Never mention Claude, Claude Code or Anthropic in commits, PRs or GitHub comments.
- No `Co-Authored-By: Claude …` or `Claude-Session:` trailers in commit messages.
- No "Generated with Claude Code" line or session link in PR descriptions or comments.

<!-- GM:START -->
<!-- GM:VERSION:0.3.7 -->

## Mandatory: Code exploration via graphmind

In any graphmind-registered project, ALWAYS use `/gm` as the FIRST step for ANY code exploration, architecture understanding, or symbol lookup. This applies to:
- Answering questions about how something works
- Finding symbols, callers, dependencies, outlines
- Understanding code before making changes
- Exploring architecture or blast radius

**Do NOT** spawn Explore agents, use grep/find/Glob for code structure, or delegate code search to sub-agents until `/gm` has been tried first.

To **read** a symbol's source code, use `/gm-fn <symbol>` — it returns full source + callers + callees in one call. Prefer it over Read when you already know the symbol name.

Only fall back to grep for: string literals, config values, env vars, non-code patterns.

Invoke: `Skill(skill: "gm", args: "<your query>")`

Outside a registered project (`graphmind which` prints nothing and exits with code 1), use `/gm` for questions about a registered project: it lists the registered projects (`graphmind list`) and targets the one the question is about with `--slug`. For a question about the current folder when it is not registered, use your normal tools (and suggest `graphmind init` to index it).

**Auto-memory — mandatory behavior**: You MUST proactively save knowledge to graphmind memory **during** and after every session, without waiting for the user to ask. Save immediately when something is decided or discovered — do not wait for the end of the session.

Categories and commands:
- Architectural decisions → `graphmind memory add "<decision>" --type decision`
- Patterns/conventions → `--type pattern` or `--type convention`
- Bugs found or fixed → `--type bug`
- Critical always-needed facts → add `--priority` (injected every session)
- Cross-project or user-level facts → add `--global`

In a registered project: save without `--global` by default (project-scoped).
Outside a registered project: save with `--slug <project>` when the fact is about that project; use `--global` only for cross-project or personal facts.

Be selective — only facts useful in a future session. Skip task details and temporary state.

## graphmind

Last build: 2026-11-04 | 2624 symbols | 7420 edges | 221 files
Languages: rust (175), typescript (22), toml (10), markdown (5), bash (4), python (2), ruby (1), html (1), go (1)
MCP: `graphmind mcp` (stdio)


| Need | MCP tool | CLI equivalent |
|------|----------|----------------|
| Find symbol | `gm_fn` | `graphmind fn <name>` |
| Search by intent | `gm_search` | `graphmind search "<query>"` |
| File dependencies | `gm_deps` | `graphmind deps <file>` |
| Symbol resolution | `gm_query` | `graphmind query <name>` |
| Blast radius | `gm_fn_impact` | `graphmind fn-impact <name>` |
| Git diff impact | `gm_diff_impact` | `graphmind diff-impact` |
| Project overview | `gm_map` | `graphmind map` |
| File outline | `gm_outline` | `graphmind outline <file>` |
| Raw file content | `gm_file` | `graphmind file <file>` |
| Who calls chain | `gm_who_calls_chain` | — |
| Dead code | `gm_dead_code` | — |
| Cross-project | `gm_cross_query` | `graphmind cross query` |
| Memory search | `gm_memory_search` | `graphmind memory search` |

Only fall back to grep/find for: string literals, config values, non-code patterns.

### Rebuild when
Structural changes, new modules, after merge.
Command: `graphmind build`
<!-- GM:END -->
