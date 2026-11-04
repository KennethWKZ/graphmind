use graphmind_config::{paths, Registry};
use super::resolve_project_slug;
use colored::Colorize;
use graphmind_db::queries::GraphQueries;
use graphmind_db::schema::init_database;

const MARKER_START: &str = "<!-- GM:START -->";
const MARKER_END: &str = "<!-- GM:END -->";

/// The per-project section's guidance: pick the cheapest tool for the question, and keep memory
/// small and current. Deliberately not setup's global block (`GM_INSTRUCTIONS`), which puts `/gm`
/// first for every lookup; measured on real repos, `rg` + Read find code as accurately, and
/// graph-first routing only added calls and context. The graph tools earn their place on
/// callers, callees and blast radius, and on searches with no names to grep for.
const SYNC_INSTRUCTIONS: &str = r#"## Code exploration and memory (graphmind)

Use the cheapest tool that answers the question; no tool has to come first.

1. "Why is it like this / did we already decide this": search memory (`graphmind memory search "<terms>"` or `gm_memory_search`) and the repo's ADRs before reading code.
2. Exact strings, config values, env vars, routes, migrations: `rg` (or the Grep tool), then Read.
3. A vague "where is the thing that does X", with no names to search for: `gm_search` (or `/gm <query>`), then Read.
4. Callers, callees or blast radius of one symbol: `gm_fn` (`/gm-fn <symbol>`) or `gm_who_calls_chain`, then Read what you'll edit.
5. Memory and code disagree: say so and show both; don't silently pick one.

Pass `project=<slug>` to `gm_*` tools, spelled as `graphmind list` shows it; in a git worktree, pass the checkout's absolute path. Graph answers reflect the last build: run `graphmind build` before querying uncommitted changes.

Memory: save when something is decided or discovered, and only what helps a future session (no task details or temporary state). Facts about this repo stay project-scoped; `--global` is for cross-project facts. One fact per entry, under about 600 characters, with the exact identifiers someone would search for (paths, function, config and command names, ticket ids) in the text and the tags:
`graphmind memory add "<fact>" --type decision|pattern|convention|bug --tags <tag> <tag>` (the fact goes before `--tags`, which takes several values).
When a fact changes, rewrite it (`graphmind memory update <id> --content "<new text>"`) or delete it; never add a contradicting one next to it. Use `--priority` only for the few facts every session needs. A decision recorded in the repo (an ADR) needs no memory copy."#;

pub fn sync(slug: Option<&str>, all: bool, dir: Option<&str>) {
    if all {
        let projects = Registry::list();
        if projects.is_empty() {
            println!("{}", "No projects registered.".dimmed());
            return;
        }
        for p in &projects {
            sync_single(&p.slug, dir);
        }
        return;
    }

    let slug = match resolve_project_slug(&[slug]) {
        Some(s) => s,
        None => {
            eprintln!(
                "{} No project specified and none could be resolved",
                "Error:".red().bold()
            );
            std::process::exit(1);
        }
    };

    sync_single(&slug, dir);
}

fn sync_single(slug: &str, dir: Option<&str>) {
    let project = match Registry::get(slug) {
        Some(p) => p,
        None => {
            eprintln!("{} Project {} not found", "Error:".red().bold(), slug);
            return;
        }
    };

    let target_dir = dir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(&project.path));

    let claude_md_path = target_dir.join("CLAUDE.md");

    let section = build_section(slug);

    let existing = std::fs::read_to_string(&claude_md_path).unwrap_or_default();

    // Also handle old graphmind:start marker for migration
    let existing = if existing.contains("<!-- graphmind:start -->") {
        existing
            .replace("<!-- graphmind:start -->", MARKER_START)
            .replace("<!-- graphmind:end -->", MARKER_END)
    } else {
        existing
    };

    let new_content = if existing.contains(MARKER_START) && existing.contains(MARKER_END) {
        let start_idx = existing.find(MARKER_START).unwrap();
        let end_idx = existing.find(MARKER_END).unwrap() + MARKER_END.len();
        format!(
            "{}{}{}",
            &existing[..start_idx],
            section,
            &existing[end_idx..]
        )
    } else if existing.is_empty() {
        section
    } else {
        format!("{existing}\n{section}")
    };

    if let Err(e) = std::fs::write(&claude_md_path, new_content) {
        eprintln!(
            "{} Failed to write CLAUDE.md for {}: {}",
            "Error:".red().bold(),
            slug,
            e
        );
        return;
    }
    println!(
        "{} Synced CLAUDE.md for {} at {}",
        "OK".green().bold(),
        slug.cyan(),
        claude_md_path.display().to_string().dimmed()
    );
}

fn build_section(slug: &str) -> String {
    let version = env!("CARGO_PKG_VERSION");

    // Build project-specific stats header
    let stats_block = build_stats_block(slug);

    format!(
        r#"{MARKER_START}
<!-- GM:VERSION:{version} -->

{SYNC_INSTRUCTIONS}

## graphmind

{stats_block}
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
{MARKER_END}"#
    )
}

fn build_stats_block(slug: &str) -> String {
    let db_path = paths::graph_db_path(slug);
    if !db_path.exists() {
        return "Graph not built yet. Run `graphmind build`.\n\n".to_string();
    }
    let db_path_str = db_path.to_string_lossy().to_string();
    let Ok(db) = init_database(&db_path_str) else {
        return "Graph not built yet. Run `graphmind build`.\n\n".to_string();
    };
    let q = GraphQueries::new(&db);
    let stats = q.stats();
    let langs = q.language_breakdown();
    let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let lang_str: Vec<String> = langs
        .iter()
        .map(|l| format!("{} ({})", l.language, l.count))
        .collect();

    let mut out = format!(
        "Last build: {} | {} symbols | {} edges | {} files\n",
        now, stats.symbols, stats.edges, stats.files
    );
    if !lang_str.is_empty() {
        out.push_str(&format!("Languages: {}\n", lang_str.join(", ")));
    }
    out.push_str("MCP: `graphmind mcp` (stdio)\n\n");
    out
}
