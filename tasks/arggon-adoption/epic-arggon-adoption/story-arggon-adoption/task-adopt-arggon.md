---
type: task
status: done
id: task-adopt-arggon
title: Adopt ArggonManager in this repo
assignee: Arggon
parent: story-arggon-adoption
labels: []
created: "2026-09-14"
updated: "2026-09-14"
---
## Context

This repo is adopting ArggonManager over an existing documentation set: `arggon init` generated the governing docs (marked `<!-- arggon:generated ... -->`, with TODO placeholders), while any pre-existing docs were left untouched on disk. Your job as the executing agent: extract the valuable content from the adopter docs into the generated ones, archive what you replace, and report back on this task.

Current inventory (paths, sizes, managed vs adopter-owned, stack manifests):

```
arggon adopt --dry-run --json
```

## Checklist

- [x] 1. Read the arggon-generated governing docs first: AGENTS.md, docs/convention.md, docs/engineering.md, docs/playbooks/ (if present). Follow them for the rest of this migration.
- [x] 2. Sweep the existing repo docs (list them from the inventory above): extract the project description, conventions, workflows, and stack info. Extract, don't wholesale-copy — rewrite into the target doc's structure and drop duplicated or outdated material.
- [x] 3. Complete the arggon-generated docs with the extracted content — fill the TODO placeholders: project description in AGENTS.md; CONTRIBUTING.md specifics (environment setup, build/test commands); ARCHITECTURE.md problem statement. The SECURITY.md contact is human input — leave it flagged for a human, never invent it.
- [x] 4. Archive replaced originals to backup/<YYYY-MM-DD>/ preserving their relative paths (use today's date). Only docs you REPLACED get archived; never archive README.md — merge into it instead.
- [x] 5. Detect the stack from the manifests (package.json / requirements.txt / go.mod / Cargo.toml / pom.xml); for each technology create a playbook (`arggon playbook new <tech>`), research current versions and best practices with dated sources, then record them with `arggon playbook refresh <tech> --version <v>`.
- [x] 6. Baseline the sanctioned edits: run `arggon adopt --ack` so the generated docs you completed in step 3 become the new x-generated baseline (their checksums are refreshed and they stop reporting as modified). Hand edits made AFTER this ack still report modified — the protection stays intact.
- [x] 7. Verify: `arggon validate` + `arggon spec validate` (if specs exist) + `arggon playbook status`.
- [x] 8. Report: comment on this task (`arggon comment task-adopt-arggon`) listing the extracted content, archived files, and created playbooks; flip this task done when the human reviews.

### 2026-09-14 @Arggon
Adopción ejecutada completa (2026-09-14). Extracción: descripción del proyecto → AGENTS.md; layout real + invariants → ARCHITECTURE.md; setup Rust/cargo/clippy + gates → CONTRIBUTING.md; testing expectations → docs/engineering.md; convenciones locales (iniciativas core/integraciones, labels) → docs/convention.md. Archivados: ninguno — README.md se integró (no reemplazó) y DECISIONS.md/FORMAT.md son complementarios, no reemplazados. Playbooks creados con research fechado 2026-09-13: rust 1.98.1, cargo 1.98.1, clippy 1.98 (playbook status: staleCount 0). SECURITY.md queda para humano (contacto no inventado). adopt --ack: 16 docs baselineados.
