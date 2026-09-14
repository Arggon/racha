# Formato del ledger

## Ubicación

`${XDG_DATA_HOME:-~/.local/share}/racha/ledger.json`. Override para tests:
flag global `--data-dir <dir>` o variable `RACHA_DATA_DIR` (precedencia:
flag > variable > XDG default).

## Estructura

```json
{
  "habits": [
    {
      "name": "meditar",
      "created": "2026-09-13",
      "checks": ["2026-09-13", "2026-09-12"]
    }
  ]
}
```

| Campo     | Tipo              | Semántica                                        |
| --------- | ----------------- | ------------------------------------------------ |
| `name`    | string            | Identidad del hábito. Único (comparación exacta). No vacío tras `trim()`; se persiste trimeado (`add` rechaza nombres que quedan vacíos, `check` trimea antes de buscar). |
| `created` | fecha `YYYY-MM-DD`| Día de alta.                                     |
| `checks`  | lista de fechas   | Días con check. Una entrada por día (idempotente: re-check del mismo día no duplica). |

- Fechas **locales** del usuario (no UTC), formato ISO `YYYY-MM-DD`.
- El archivo se reescribe entero en cada mutación (pretty-printed, `serde_json`).
- Ledger ausente = estado vacío válido (primer uso). Ledger inválido = error
  claro, nunca se sobreescribe.
- **Compatibilidad**: ledgers previos que ya contienen hábitos con `name`
  vacío o de solo espacios NO se limpian automáticamente; el fix solo previene
  nombres vacíos nuevos. Limpieza manual: editar `ledger.json` (con racha
  cerrado) y borrar o renombrar esas entradas.

## Semántica de rachas

- **Racha actual**: días consecutivos con check terminando en hoy. Si hoy no
  hay check pero sí ayer, la racha se conserva hasta medianoche (el día no
  terminó). Si no hay check ni hoy ni ayer: 0.
- **Mejor racha**: la corrida más larga de días consecutivos de todo el
  historial (los checks se ordenan y deduplican antes de medir; cruza
  límites de mes y de año).
- **Vista semanal**: semana calendario actual, lunes a domingo, `✓`/`·`.

## Compatibilidad

Versión implícita 1 (este documento). Cambios de esquema futuros deben ser
aditivos o escribir un campo de versión y documentar migración acá.
