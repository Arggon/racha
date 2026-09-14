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

- [x] docs/explorations/ con candidatos (notify-rust / notify-send / canal del compositor), criterios, hallazgos con fuentes fechadas 2026-09-13 y recomendación
- [x] ADR en docs/adr/ (0001-*) con estado Proposed, consecuencias y alternativas descartadas
- [x] Spike en vivo: una notificación real visible en la sesión Omarchy del usuario (no basta exit 0), evidencia registrada en la exploración
- [x] Hallazgos y decisión volcados en este story vía `arggon comment`

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Exploración desktop-notifications-001 + ADR 0001 mergeados en PR #8 (squash ca39243). Recomendación: notify-rust 4.18.0 con backend zbus (D-Bus 100% Rust, sin libdbus/libnotify en runtime) — habla el estándar org.freedesktop.Notifications que el shell Quickshell de Omarchy ya registra en el bus de sesión (busctl verificado, PID 1355). Alternativas: notify-send 0.8.8 (dependencia de runtime no garantizable, fallback documentado), canal del compositor (acoplamiento a Omarchy). Spikes en vivo verificados: notify-send mostró la notificación en pantalla (/tmp/racha-evidence/exploracion-spike-notify-send.png, 866 KB); proyecto mínimo con notify-rust compiló en 15.22s debug y mostró la notificación (/tmp/racha-evidence/exploracion-spike-notify-rust.png, 843 KB). Riesgo documentado: ~25 crates transitivas de zbus; show() devuelve error sin daemon → degradar suave.
