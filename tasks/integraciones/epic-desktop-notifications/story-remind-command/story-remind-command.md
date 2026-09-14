---
type: story
status: todo
id: story-remind-command
title: racha remind — vencimientos del día y notificación nativa
parent: epic-desktop-notifications
labels: []
created: "2026-09-14"
updated: "2026-09-14"
depends_on: [story-notify-exploration]
---
<!--
  Placement (v0): tasks/integraciones/epic-desktop-notifications/story-remind-command/story-remind-command.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# racha remind — vencimientos del día y notificación nativa

## Context

Issue #2: `racha remind` consulta qué hábitos no tienen check hoy y dispara una
notificación nativa por cada uno, usando el canal decidido en el ADR-0001
(story-notify-exploration). Es el comando que el timer de systemd va a invocar.

## Acceptance

- [ ] `racha remind` lista vencidos (sin check hoy) y dispara una notificación nativa por hábito con el canal del ADR
- [ ] Sin hábitos vencidos: mensaje claro, sin notificaciones, exit 0
- [ ] Sin sesión gráfica/DBus disponible: comportamiento degradado claro y exit code documentado
- [ ] Notificación real visible en Omarchy verificada (captura adjunta en la exploración o comentario del story)
- [ ] Tests de la lógica de vencimientos (pura) + integration del flujo

Closes #2.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
