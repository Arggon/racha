---
type: task
status: in_progress
id: task-issue-1
title: "issue #1: Motor de rachas y estadísticas"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:25:47.564Z"
issue: 1
---
El motor ya existe en versión mínima (racha actual, mejor racha, vista semanal) pero falta completar el dominio:

- [ ] estadísticas agregadas por período (mes, año)
- [ ] porcentaje de cumplimiento semanal
- [ ] manejo de zonas horarias documentado y testeado en fronteras de día
- [ ] API interna (crate lib) estable para consumos externos (reminders, web)

Criterio de aceptación: tests unitarios de cada regla de racha + documentación actualizada en docs/FORMAT.md.
> imported from issue #1

### 2026-09-14 @Arggon
Implementado por story-stats-engine (PR #11, squash 30aa7f1); issue #1 cerrado por el PR.
