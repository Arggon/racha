---
type: story
status: in_progress
id: story-notify-exploration
title: "Exploración + ADR: canal de notificaciones nativas"
assignee: Arggon
branch: feat/story-notify-exploration
parent: epic-desktop-notifications
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T01:55:18.422Z"
worktree_path: /home/arggon/Projects/racha-story-notify-exploration
---
<!--
  Placement (v0): tasks/integraciones/epic-desktop-notifications/story-notify-exploration/story-notify-exploration.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# "Exploración + ADR: canal de notificaciones nativas"

## Context

Issue #2 necesita un canal de notificaciones nativas. Decisión cross-cutting
(dependencia nueva vs proceso externo vs canal del compositor): pipeline completo
exploración → ADR → implementación. No se escribe código de producción acá.

## Acceptance

- [ ] docs/explorations/ con candidatos (notify-rust / notify-send / canal del compositor), criterios, hallazgos con fuentes fechadas 2026-09-13 y recomendación
- [ ] ADR en docs/adr/ (0001-*) con estado Proposed, consecuencias y alternativas descartadas
- [ ] Spike en vivo: una notificación real visible en la sesión Omarchy del usuario (no basta exit 0), evidencia registrada en la exploración
- [ ] Hallazgos y decisión volcados en este story vía `arggon comment`

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
