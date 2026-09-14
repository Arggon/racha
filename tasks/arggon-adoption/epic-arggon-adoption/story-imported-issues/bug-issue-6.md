---
type: bug
status: in_progress
id: bug-issue-6
title: "issue #6: Bug: racha add acepta nombres vacíos o de solo espacios"
assignee: Arggon
branch: fix/bug-issue-6
parent: story-imported-issues
labels: [bug]
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:05:13.177Z"
issue: 6
worktree_path: /home/arggon/Projects/racha-bug-issue-6
---
**Encontrado mientras testeaba mi propio código (FASE A), por sondeo manual de edge cases.**

`racha add` no valida el nombre: acepta `""` y `"  "` y crea hábitos que:

1. aparecen en `racha stats` como entradas invisibles (líneas en blanco + métricas sueltas),
2. persisten en el ledger para siempre,
3. no son referenciables de forma ambigua desde el shell (copiar/pegar un nombre vacío).

Reproducción:

```console
$ racha add ""
hábito agregado: 
$ racha stats
<primer bloque: nombre vacío, métricas en 0>
```

Evidencia en el ledger: `"name": ""` persistido.

Fix esperado: rechazar en `add` (y validar en `check`) todo nombre que quede vacío tras `trim()`, con mensaje de error claro. Decide el fix qué hacer con ledgers ya contaminados (sugerido: rechazar y documentar limpieza manual).
> imported from issue #6

## Aceptación

- [x] `racha add` rechaza (exit 1, error en stderr) nombres que quedan vacíos tras `trim()` ("" y "   ").
- [x] `racha add` persiste el nombre trimeado (`"  leer  "` → `leer`).
- [x] `racha check` trimea antes de buscar (`check "  leer  "` encuentra `leer`).
- [x] Ledgers ya contaminados NO se limpian automáticamente; decisión documentada en `docs/FORMAT.md` (limpieza manual).
- [x] Tests de integración en `tests/cli.rs` cubren los 4 casos; suite verde, clippy 0 warnings, fmt limpio.
- [x] PR #10 (squash) merged con `Closes #6`.

### 2026-09-14 @Arggon
Fix en fix/bug-issue-6, PR #10 (squash, Closes #6). src/main.rs: Add trimea el nombre y rechaza con error claro (exit 1) si queda vacío; Check trimea antes de buscar. Decisión de compatibilidad: ledgers ya contaminados con nombres vacíos NO se limpian automáticamente — el fix solo previene nombres nuevos; limpieza manual documentada en docs/FORMAT.md (campo name: no vacío tras trim, se persiste trimeado). Tests: 5 integration nuevos en tests/cli.rs (add vacío falla, add solo espacios falla, add '  leer  ' persiste como 'leer', check con espacios encuentra el hábito, y no-duplicado). Nota: durante el rebase contra master apareció conflicto en tests/cli.rs con la feature list de otro agente; se preservaron ambas suites (12 integration tests totales, todo verde). cargo fmt también normalizó 2 bloques preexistentes en storage.rs/streaks.rs para pasar el gate.
