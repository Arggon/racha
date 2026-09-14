---
type: task
status: in_progress
id: task-issue-2
title: "issue #2: Notificaciones nativas de escritorio — integración Omarchy"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:25:47.653Z"
issue: 2
---
Disparar notificaciones nativas del escritorio (Omarchy: Arch + Hyprland + Wayland) cuando corresponde un recordatorio o se completa una racha notable.

- [ ] exploración documentada: notify-rust (DBus puro) vs notify-send vs canal del compositor
- [ ] ADR en docs/adr/ con la decisión
- [ ] notificación real visible en la sesión del usuario (no basta exit code 0)
- [ ] fallback silencioso cuando no hay sesión gráfica (ej. cron/ssh)

Criterio de aceptación: una notificación visible en Omarchy disparada por `racha remind`.
> imported from issue #2
