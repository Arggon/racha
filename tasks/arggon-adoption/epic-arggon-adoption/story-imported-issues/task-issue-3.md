---
type: task
status: in_progress
id: task-issue-3
title: "issue #3: Recordatorios programados (systemd user timer)"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:25:47.745Z"
issue: 3
---
Recordatorio diario programado con systemd **user** timer + service (unidades instaladas por el proyecto, `systemctl --user`).

- [ ] `racha remind`: consulta vencimientos del día y dispara notificaciones nativas
- [ ] unidades systemd (service oneshot + timer) en el repo, target de instalación documentado
- [ ] runbooks: timer no dispara, notificaciones no aparecen (Wayland/DBus), restore de datos

Criterio de aceptación: timer instalado dispara `racha remind` y produce notificación visible; runbooks probados.
> imported from issue #3
