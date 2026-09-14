//! Export/import portable del ledger (issue #5).
//!
//! Formatos:
//! - JSON canónico: `{"version":1,"habits":[{"name":...,"created":...,"checks":[...]}]}`
//!   (mismo shape que el ledger, más el campo `version`).
//! - CSV: header `habit,date`, una fila por check, ordenado por (nombre, fecha).
//!
//! El import valida TODO antes de tocar el ledger: fechas ISO válidas, nombres
//! no vacíos tras trim, sin checks duplicados dentro del archivo. La escritura
//! es atómica (temp + rename) vía `storage::save_atomic`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::{Habit, Ledger};

/// Versión del formato portable (coincide con docs/FORMAT.md).
pub const FORMAT_VERSION: u64 = 1;

// ---------------------------------------------------------------- export

#[derive(Serialize)]
struct ExportHabit<'a> {
    name: &'a str,
    created: NaiveDate,
    checks: &'a [NaiveDate],
}

#[derive(Serialize)]
struct ExportLedger<'a> {
    version: u64,
    habits: Vec<ExportHabit<'a>>,
}

/// Exporta el ledger a JSON canónico (hábitos por nombre, checks ordenados).
pub fn to_json(ledger: &Ledger) -> String {
    let mut habits = ledger.habits.clone();
    habits.sort_by(|a, b| a.name.cmp(&b.name));
    for h in &mut habits {
        h.checks.sort();
        h.checks.dedup();
    }
    let doc = ExportLedger {
        version: FORMAT_VERSION,
        habits: habits
            .iter()
            .map(|h| ExportHabit {
                name: &h.name,
                created: h.created,
                checks: &h.checks,
            })
            .collect(),
    };
    let mut out = serde_json::to_string_pretty(&doc).expect("export serializable");
    out.push('\n');
    out
}

/// Escapa un campo CSV: comillas si contiene coma, comilla, \n o \r.
fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Exporta el ledger a CSV: header `habit,date`, una fila por check,
/// ordenado por (nombre, fecha). Hábitos sin checks no generan filas.
pub fn to_csv(ledger: &Ledger) -> String {
    let mut rows: BTreeMap<&str, Vec<NaiveDate>> = BTreeMap::new();
    for h in &ledger.habits {
        let mut dates = h.checks.clone();
        dates.sort();
        dates.dedup();
        rows.entry(&h.name).or_default().extend(dates);
    }
    let mut out = String::from("habit,date\n");
    for (name, dates) in rows {
        for d in dates {
            let _ = writeln!(out, "{},{}", csv_field(name), d);
        }
    }
    out
}

// ---------------------------------------------------------------- import

/// Archivo portable a importar (ya validado, salvo colisiones con el ledger).
#[derive(Clone, Debug, Deserialize)]
pub struct ImportHabit {
    name: String,
    created: NaiveDate,
    checks: Vec<NaiveDate>,
}

#[derive(Deserialize)]
pub struct ImportLedger {
    #[serde(default = "default_version")]
    #[allow(dead_code)]
    version: u64,
    habits: Vec<ImportHabit>,
}

fn default_version() -> u64 {
    FORMAT_VERSION
}

/// Error de validación del import.
#[derive(Debug)]
pub struct ImportError(pub String);

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parsea y valida un archivo portable, detectando el formato por contenido
/// (JSON si el contenido recortado arranca con `{`; si no, CSV).
pub fn parse_portable(raw: &str) -> Result<Vec<ImportHabit>, ImportError> {
    if raw.trim_start().starts_with('{') {
        parse_json(raw)
    } else {
        parse_csv(raw)
    }
}

fn validate_habits(habits: Vec<ImportHabit>) -> Result<Vec<ImportHabit>, ImportError> {
    let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    for h in &habits {
        let name = h.name.trim();
        if name.is_empty() {
            return Err(ImportError(
                "nombre de hábito vacío (o solo espacios) en el archivo a importar".into(),
            ));
        }
        if !seen_names.insert(name.to_string()) {
            return Err(ImportError(format!(
                "hábito duplicado en el archivo: {name}"
            )));
        }
        let mut seen_checks: std::collections::HashSet<NaiveDate> =
            std::collections::HashSet::new();
        for c in &h.checks {
            if !seen_checks.insert(*c) {
                return Err(ImportError(format!(
                    "check duplicado para '{name}' en el archivo: {c}"
                )));
            }
        }
    }
    Ok(habits)
}

fn parse_json(raw: &str) -> Result<Vec<ImportHabit>, ImportError> {
    let doc: ImportLedger =
        serde_json::from_str(raw).map_err(|e| ImportError(format!("JSON inválido: {e}")))?;
    validate_habits(doc.habits)
}

/// Parsea una línea CSV respetando campos entrecomillados (`""` = comilla).
fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else {
            match c {
                '"' if cur.is_empty() => in_quotes = true,
                ',' => {
                    fields.push(std::mem::take(&mut cur));
                }
                _ => cur.push(c),
            }
        }
    }
    fields.push(cur);
    fields
}

fn parse_date(field: &str, what: &str) -> Result<NaiveDate, ImportError> {
    NaiveDate::parse_from_str(field.trim(), "%Y-%m-%d").map_err(|_| {
        ImportError(format!(
            "fecha inválida ({what}): '{field}' (se espera YYYY-MM-DD)"
        ))
    })
}

fn parse_csv(raw: &str) -> Result<Vec<ImportHabit>, ImportError> {
    let mut lines = raw.lines().peekable();
    let header = lines.peek().copied().unwrap_or("");
    let header_fields: Vec<String> = split_csv_line(header)
        .into_iter()
        .map(|f| f.trim().to_lowercase())
        .collect();
    if header_fields != vec!["habit", "date"] {
        return Err(ImportError(format!(
            "CSV inválido: el header debe ser `habit,date` (encontrado: {header:?})"
        )));
    }
    lines.next();
    let mut by_name: BTreeMap<String, ImportHabit> = BTreeMap::new();
    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = split_csv_line(line);
        if fields.len() != 2 {
            return Err(ImportError(format!(
                "línea {} del CSV inválida: se esperaban 2 campos, hay {}",
                i + 2,
                fields.len()
            )));
        }
        let name = fields[0].trim().to_string();
        if name.is_empty() {
            return Err(ImportError(format!(
                "línea {} del CSV: nombre de hábito vacío",
                i + 2
            )));
        }
        let date = parse_date(&fields[1], &format!("línea {} del CSV", i + 2))?;
        let entry = by_name.entry(name.clone()).or_insert_with(|| ImportHabit {
            // `created` no viaja en CSV: se fija a la primera check importada.
            name: name.clone(),
            created: date,
            checks: Vec::new(),
        });
        entry.created = entry.created.min(date);
        entry.checks.push(date);
    }
    validate_habits(by_name.into_values().collect())
}

/// Fusiona los hábitos validados en el ledger: los nuevos se agregan; los
/// existentes reciben los checks que les falten (dedup por nombre+fecha, el
/// `created` existente gana). Devuelve true si algo cambió.
pub fn merge_into(ledger: &mut Ledger, habits: Vec<ImportHabit>) -> bool {
    let mut changed = false;
    for h in habits {
        let name = h.name.trim().to_string();
        if let Some(existing) = ledger.habit_mut(&name) {
            for c in h.checks {
                changed |= existing.add_check(c);
            }
        } else {
            ledger.habits.push(Habit {
                name,
                created: h.created,
                checks: h.checks,
            });
            changed = true;
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Ledger {
        let mut l = Ledger::default();
        let mut h = Habit::new(
            "meditar".into(),
            NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        );
        for d in ["2026-09-02", "2026-09-01"] {
            h.add_check(NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap());
        }
        l.habits.push(h);
        let mut h2 = Habit::new("agua".into(), NaiveDate::from_ymd_opt(2026, 8, 30).unwrap());
        h2.add_check(NaiveDate::from_ymd_opt(2026, 8, 31).unwrap());
        l.habits.push(h2);
        l
    }

    #[test]
    fn json_round_trip_is_identical() {
        let l = sample();
        let exported = to_json(&l);
        let habits = parse_portable(&exported).unwrap();
        let mut restored = Ledger::default();
        merge_into(&mut restored, habits);
        assert_eq!(to_json(&restored), exported);
    }

    #[test]
    fn csv_has_header_and_is_sorted() {
        let out = to_csv(&sample());
        assert_eq!(
            out,
            "habit,date\nagua,2026-08-31\nmeditar,2026-09-01\nmeditar,2026-09-02\n"
        );
    }

    #[test]
    fn csv_round_trip_preserves_checks() {
        let l = sample();
        let imported = parse_portable(&to_csv(&l)).unwrap();
        let mut restored = Ledger::default();
        merge_into(&mut restored, imported);
        assert_eq!(to_csv(&restored), to_csv(&l));
    }

    #[test]
    fn rejects_invalid_date() {
        let err = parse_portable("habit,date\nmeditar,2026-13-40\n").unwrap_err();
        assert!(err.0.contains("fecha inválida"), "{}", err.0);
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_portable("habit,date\n   ,2026-09-01\n").unwrap_err();
        assert!(err.0.contains("vacío"), "{}", err.0);
    }

    #[test]
    fn rejects_duplicate_checks_in_file() {
        let err =
            parse_portable("habit,date\nmeditar,2026-09-01\nmeditar,2026-09-01\n").unwrap_err();
        assert!(err.0.contains("duplicado"), "{}", err.0);
    }

    #[test]
    fn rejects_bad_csv_header() {
        let err = parse_portable("name,day\nmeditar,2026-09-01\n").unwrap_err();
        assert!(err.0.contains("header"), "{}", err.0);
    }

    #[test]
    fn json_detection_by_content_not_extension() {
        let l = sample();
        let habits = parse_portable(&to_json(&l)).unwrap();
        assert_eq!(habits.len(), 2);
    }

    #[test]
    fn merge_is_idempotent() {
        let l = sample();
        let habits = parse_portable(&to_json(&l)).unwrap();
        let mut target = Ledger::default();
        assert!(merge_into(&mut target, habits.clone()));
        assert_eq!(to_json(&target), to_json(&l));
        // Segunda pasada: nada nuevo, nada duplicado.
        assert!(!merge_into(&mut target, habits));
        assert_eq!(to_json(&target), to_json(&l));
    }

    #[test]
    fn csv_field_quotes_names_with_commas() {
        let mut l = Ledger::default();
        let mut h = Habit::new(
            "café, té".into(),
            NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        );
        h.add_check(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap());
        l.habits.push(h);
        let out = to_csv(&l);
        assert_eq!(out, "habit,date\n\"café, té\",2026-09-01\n");
        let mut restored = Ledger::default();
        merge_into(&mut restored, parse_portable(&out).unwrap());
        assert_eq!(restored.habits[0].name, "café, té");
    }
}
