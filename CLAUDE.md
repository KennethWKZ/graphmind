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

## Code exploration and memory (graphmind)

Use the cheapest tool that answers the question; no tool has to come first.

1. "Why is it like this / did we already decide this": search memory (`graphmind memory search "<terms>"` or `gm_memory_search`) and the repo's ADRs before reading code.
2. Exact strings, config values, env vars, routes, migrations: `rg` (or the Grep tool), then Read.
3. A vague "where is the thing that does X", with no names to search for: `gm_search` (or `/gm <query>`), then Read.
4. Callers, callees or blast radius of one symbol: `gm_fn` (`/gm-fn <symbol>`) or `gm_who_calls_chain`, then Read what you'll edit.
5. Memory and code disagree: say so and show both; don't silently pick one.

Pass `project=<slug>` to `gm_*` tools, spelled as `graphmind list` shows it; in a git worktree, pass the checkout's absolute path. Graph answers reflect the last build: run `graphmind build` before querying uncommitted changes.

Outside a registered project (`graphmind which` prints nothing and exits with code 1), use `/gm` for questions about a registered project: it lists the registered projects (`graphmind list`) and targets the one the question is about with `--slug`. For a question about the current folder when it is not registered, use your normal tools (and suggest `graphmind init` to index it).

Memory: save when something is decided or discovered, and only what helps a future session (no task details or temporary state). In a registered project, facts about it stay project-scoped (no `--global`); outside one, save with `--slug <project>` when the fact is about that project; `--global` is for cross-project facts. One fact per entry, under about 600 characters, with the exact identifiers someone would search for (paths, function, config and command names, ticket ids) in the text and the tags:
`graphmind memory add "<fact>" --type decision|pattern|convention|bug --tags <tag> <tag>` (the fact goes before `--tags`, which takes several values).
When a fact changes, rewrite it (`graphmind memory update <id> --content "<new text>"`) or delete it; never add a contradicting one next to it. Use `--priority` only for the few facts every session needs. A decision recorded in the repo (an ADR) needs no memory copy.

## graphmind

Last build: 2026-11-05 | 2631 symbols | 7438 edges | 221 files
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

### Rebuild when
Structural changes, new modules, after merge.
Command: `graphmind build`
<!-- GM:END -->
