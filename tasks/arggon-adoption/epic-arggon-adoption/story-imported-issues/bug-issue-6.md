---
type: bug
status: todo
id: bug-issue-6
title: "issue #6: Bug: racha add acepta nombres vacíos o de solo espacios"
parent: story-imported-issues
labels: [bug]
created: "2026-09-14"
updated: "2026-09-14"
issue: 6
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
