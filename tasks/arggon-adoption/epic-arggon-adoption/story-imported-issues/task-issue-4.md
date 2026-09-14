---
type: task
status: done
id: task-issue-4
title: "issue #4: Vista web de solo lectura"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
issue: 4
---
Reporte web de solo lectura servido localmente: rachas actuales, históricos, heatmap.

- [ ] genera HTML estático a partir del ledger (sin backend con estado)
- [ ] pura lectura: la escritura sigue siendo CLI
- [ ] cache busting razonable para poder recargar

Criterio de aceptación: `racha web` (o equivalente) sirve la vista y refleja el ledger actual.
> imported from issue #4

### 2026-09-14 @Arggon
Implementado por story-web-static (PR #14): racha web genera HTML estático de solo lectura; escaping testado.
