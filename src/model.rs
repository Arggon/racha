use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Un hábito con su historial de checks (fechas locales YYYY-MM-DD).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub name: String,
    pub created: NaiveDate,
    pub checks: Vec<NaiveDate>,
}

impl Habit {
    pub fn new(name: String, created: NaiveDate) -> Self {
        Self {
            name,
            created,
            checks: Vec::new(),
        }
    }

    pub fn has_check(&self, date: NaiveDate) -> bool {
        self.checks.contains(&date)
    }

    /// Registra un check. Devuelve false si ya estaba registrado (idempotente).
    pub fn add_check(&mut self, date: NaiveDate) -> bool {
        if self.has_check(date) {
            return false;
        }
        self.checks.push(date);
        true
    }
}

/// El ledger completo: todos los hábitos con sus checks.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ledger {
    pub habits: Vec<Habit>,
}

impl Ledger {
    pub fn habit(&self, name: &str) -> Option<&Habit> {
        self.habits.iter().find(|h| h.name == name)
    }

    pub fn habit_mut(&mut self, name: &str) -> Option<&mut Habit> {
        self.habits.iter_mut().find(|h| h.name == name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.habit(name).is_some()
    }

    pub fn add_habit(&mut self, name: String, created: NaiveDate) -> bool {
        if self.exists(&name) {
            return false;
        }
        self.habits.push(Habit::new(name, created));
        true
    }
}
