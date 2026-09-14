use chrono::NaiveDate;

use crate::model::Habit;

/// Hábitos vencidos: los que NO tienen check en `today`. Función pura
/// (sin I/O); el orden es el del ledger.
pub fn due_habits(habits: &[Habit], today: NaiveDate) -> Vec<&Habit> {
    habits.iter().filter(|h| !h.has_check(today)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Habit;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn habit(name: &str, checks: &[NaiveDate]) -> Habit {
        let mut h = Habit::new(name.to_string(), d(2026, 9, 1));
        h.checks = checks.to_vec();
        h
    }

    #[test]
    fn due_excludes_habits_checked_today() {
        let today = d(2026, 9, 13);
        let habits = vec![
            habit("meditar", &[d(2026, 9, 13)]),
            habit("leer", &[d(2026, 9, 12)]),
        ];
        let due = due_habits(&habits, today);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].name, "leer");
    }

    #[test]
    fn due_includes_all_when_nobody_checked_today() {
        let today = d(2026, 9, 13);
        let habits = vec![habit("a", &[d(2026, 9, 12)]), habit("b", &[])];
        let due = due_habits(&habits, today);
        let names: Vec<&str> = due.iter().map(|h| h.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn due_empty_when_everyone_checked_today() {
        let today = d(2026, 9, 13);
        let habits = vec![habit("a", &[d(2026, 9, 13)])];
        assert!(due_habits(&habits, today).is_empty());
    }

    #[test]
    fn due_empty_with_no_habits() {
        assert!(due_habits(&[], d(2026, 9, 13)).is_empty());
    }

    #[test]
    fn due_preserves_ledger_order() {
        let today = d(2026, 9, 13);
        let habits = vec![
            habit("zeta", &[]),
            habit("alfa", &[]),
            habit("checkeado", &[today]),
            habit("miga", &[]),
        ];
        let due = due_habits(&habits, today);
        let names: Vec<&str> = due.iter().map(|h| h.name.as_str()).collect();
        assert_eq!(names, vec!["zeta", "alfa", "miga"]);
    }
}
