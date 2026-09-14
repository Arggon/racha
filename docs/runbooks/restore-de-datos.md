# Restore de datos: recuperar el ledger desde un export

## Cuándo correrlo (trigger)

- `racha` (o cualquier comando) muestra `ledger corrupto en ...: ...` al arrancar.
- `~/.local/share/racha/ledger.json` no existe (borrado accidental, sync con
  conflictos, disco lleno a mitad de escritura) y hay datos que recuperar.
- El ledger existe pero perdió hábitos o checks (comparado con lo esperado).

## Prerrequisitos

- Un export previo: salida de `racha export --format json --out <backup>`
  (o CSV). Sin export no hay restore: esta es la razón para exportar con
  regularidad (ej.: cron semanal).
- Acceso de escritura al data dir: `${XDG_DATA_HOME:-~/.local/share}/racha/`.
- Un binario `racha` funcionando (cualquier versión entiende el formato
  `version: 1`).

## Diagnóstico

```bash
# 1. ¿El ledger existe y es JSON válido?
cat "${XDG_DATA_HOME:-~/.local/share}/racha/ledger.json" | jq empty && echo OK || echo ROTO

# 2. ¿El export que vamos a usar está completo? (debe ser JSON válido y
#    terminar en '}' de cierre, no en medio de un objeto)
cat backup.json | jq -e '.version == 1 and (.habits | type == "array")' && echo EXPORT_OK
```

Si el export **no pasa** el paso 2, va truncado o corrupto → ver sección
"Export truncado" más abajo antes de seguir.

## Mitigación (restore propiamente dicho)

```bash
# 1. Preservar el ledger roto por si hay que forensiarlo (NUNCA borrarlo primero)
mv ~/.local/share/racha/ledger.json ~/.local/share/racha/ledger.json.corrupt-$(date +%Y%m%d-%H%M%S)

# 2. Importar el export al data dir limpio. El import valida TODO el archivo
#    antes de escribir y la escritura es atómica (temp + rename): si algo del
#    export es inválido, falla con exit 1 sin dejar un ledger a medio escribir.
racha import backup.json

# 3. Verificar que volvió todo:
racha list                  # los hábitos están
racha stats                 # rachas y totales coherentes con el export
```

Alternativa sin `import` (restore manual, mismo resultado): el export JSON es
el ledger más `"version": 1`:

```bash
jq '.habits' backup.json > /tmp/habits.json
printf '{\n  "habits": ' > ~/.local/share/racha/ledger.json
cat /tmp/habits.json >> ~/.local/share/racha/ledger.json
printf '\n}\n' >> ~/.local/share/racha/ledger.json
jq empty ~/.local/share/racha/ledger.json && echo OK
```

### Qué hacer si el export mismo está truncado

1. **No importar un export truncado a mano editándolo a ciegas**: el import
   rechaza JSON inválido, pero un truncamiento puede dejar JSON sintácticamente
   válido con datos perdidos (hábito entero afuera).
2. Buscar backups anteriores y elegir el más reciente que pase el diagnóstico:
   ```bash
   for f in backup-*.json; do jq -e '.version == 1' "$f" >/dev/null 2>&1 && echo "OK: $f"; done
   ```
3. Restaurar del más reciente válido y aceptar la pérdida incremental (checks
   posteriores a ese backup se re-cargan con `racha check` a mano, o desde un
   export CSV parcial si existe: el import de CSV hace merge sin duplicar).
4. Si hay un export CSV más nuevo que el JSON válido: importar primero el JSON
   y después el CSV — el merge es aditivo por (nombre, fecha).
5. Si no hay NINGÚN export válido: los datos son irrecuperables desde racha.
   Revisar `ledger.json.corrupt-*` con `jq` — a veces el daño es de una sola
   línea y se puede reparar editando a mano, validando con `jq empty` antes de
   moverlo de vuelta. Escalar si el archivo importa (ver abajo).

## Escalation

- Si el restore falla con `import inválido` sobre un export que `jq` da por
  válido, o `save_atomic` reporta errores de rename: abrir bug en el tracker
  (`arggon create bug "..." --parent epic-data-portability`) adjuntando el
  mensaje de error exacto y la versión de `racha` (`racha --version`).
- Pérdida de datos de un usuario real: capturar el `ledger.json.corrupt-*`
  antes de cualquier otra acción.

## Rollback

- El ledger roto original queda en `ledger.json.corrupt-<timestamp>`:
  ```bash
  mv ~/.local/share/racha/ledger.json.corrupt-<timestamp> ~/.local/share/racha/ledger.json
  ```
- Si el import dejó un estado indeseado (datos válidos pero equivocados),
  repetir la mitigación desde otro export; no hay operación destructiva en el
  camino: importar solo agrega/mergea, nunca borra hábitos existentes.
