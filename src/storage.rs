use std::fs;
use std::path::{Path, PathBuf};

use crate::model::Ledger;

/// Resuelve el directorio de datos: flag `--data-dir` > `$RACHA_DATA_DIR` >
/// `$XDG_DATA_HOME/racha` > `~/.local/share/racha`.
pub fn resolve_dir(flag: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = flag {
        return dir;
    }
    if let Ok(dir) = std::env::var("RACHA_DATA_DIR")
        && !dir.is_empty()
    {
        return PathBuf::from(dir);
    }
    let base = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").expect("HOME no definida");
            PathBuf::from(home).join(".local/share")
        });
    base.join("racha")
}

pub fn ledger_path(dir: &Path) -> PathBuf {
    dir.join("ledger.json")
}

/// Carga el ledger. Archivo inexistente = ledger vacío (primer uso).
pub fn load(dir: &Path) -> Result<Ledger, String> {
    let path = ledger_path(dir);
    if !path.exists() {
        return Ok(Ledger::default());
    }
    let raw =
        fs::read_to_string(&path).map_err(|e| format!("no pude leer {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("ledger corrupto en {}: {e}", path.display()))
}

/// Persiste el ledger creando el directorio si hace falta.
pub fn save(dir: &Path, ledger: &Ledger) -> Result<(), String> {
    save_atomic(dir, ledger)
}

/// Escritura atómica: serializa a un archivo temporal en el mismo directorio y
/// hace rename sobre `ledger.json`. Un crash a mitad de escritura deja el
/// ledger anterior intacto (rename es atómico dentro del mismo filesystem).
pub fn save_atomic(dir: &Path, ledger: &Ledger) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("no pude crear {}: {e}", dir.display()))?;
    let json = serde_json::to_string_pretty(ledger).expect("ledger serializable");
    let target = ledger_path(dir);
    let tmp = dir.join(format!(".ledger.json.tmp-{}", std::process::id()));
    fs::write(&tmp, json + "\n").map_err(|e| format!("no pude escribir {}: {e}", tmp.display()))?;
    fs::rename(&tmp, &target).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        format!(
            "no pude renombrar {} → {}: {e}",
            tmp.display(),
            target.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_wins_over_env() {
        // Safety: tests corren en un solo hilo por test executable... no:
        // cargo test corre los tests en paralelo, pero esta env var solo se
        // usa aquí y ninguna otra test la lee.
        unsafe { std::env::set_var("RACHA_DATA_DIR", "/tmp/env-dir") };
        let dir = resolve_dir(Some(PathBuf::from("/tmp/flag-dir")));
        assert_eq!(dir, PathBuf::from("/tmp/flag-dir"));
        unsafe { std::env::remove_var("RACHA_DATA_DIR") };
    }

    #[test]
    fn missing_ledger_is_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let ledger = load(tmp.path()).unwrap();
        assert!(ledger.habits.is_empty());
    }
}
