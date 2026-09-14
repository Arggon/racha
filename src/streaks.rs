use chrono::{Datelike, Duration, NaiveDate};

/// Racha actual contando hacia atrás desde `today` (o desde ayer si hoy no
/// hay check: el día todavía no terminó, la racha no se rompe hasta medianoche).
pub fn current_streak(checks: &[NaiveDate], today: NaiveDate) -> u32 {
    let set: std::collections::HashSet<NaiveDate> = checks.iter().copied().collect();
    let start = if set.contains(&today) {
        today
    } else if set.contains(&(today - Duration::days(1))) {
        today - Duration::days(1)
    } else {
        return 0;
    };

    let mut streak = 0u32;
    let mut day = start;
    while set.contains(&day) {
        streak += 1;
        day -= Duration::days(1);
    }
    streak
}

/// Mejor racha histórica: la corrida más larga de días consecutivos.
pub fn best_streak(checks: &[NaiveDate]) -> u32 {
    let mut dates = checks.to_vec();
    dates.sort_unstable();
    dates.dedup();

    let mut best = 0u32;
    let mut run = 0u32;
    for (i, d) in dates.iter().enumerate() {
        if i > 0 && *d - dates[i - 1] == Duration::days(1) {
            run += 1;
        } else {
            run = 1;
        }
        best = best.max(run);
    }
    best
}

/// Vista semanal: los 7 días de la semana calendario (lunes a domingo)
/// que contiene `today`. true = con check.
pub fn week_view(checks: &[NaiveDate], today: NaiveDate) -> [bool; 7] {
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let set: std::collections::HashSet<NaiveDate> = checks.iter().copied().collect();
    let mut week = [false; 7];
    for (i, day) in week.iter_mut().enumerate() {
        *day = set.contains(&(monday + Duration::days(i as i64)));
    }
    week
}

pub fn total_checks(checks: &[NaiveDate]) -> usize {
    checks.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn current_streak_counts_back_from_today() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 11), d(2026, 9, 12), today];
        assert_eq!(current_streak(&checks, today), 3);
    }

    #[test]
    fn current_streak_survives_until_midnight_when_today_missing() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 11), d(2026, 9, 12)];
        assert_eq!(current_streak(&checks, today), 2);
    }

    #[test]
    fn current_streak_zero_when_yesterday_also_missing() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 10)];
        assert_eq!(current_streak(&checks, today), 0);
    }

    #[test]
    fn best_streak_ignores_order_and_duplicates() {
        let checks = vec![
            d(2026, 1, 3),
            d(2026, 1, 1),
            d(2026, 1, 2),
            d(2026, 1, 2),
            d(2026, 3, 8),
        ];
        assert_eq!(best_streak(&checks), 3);
    }

    #[test]
    fn best_streak_crosses_month_boundary() {
        let checks = vec![d(2026, 8, 30), d(2026, 8, 31), d(2026, 9, 1)];
        assert_eq!(best_streak(&checks), 3);
    }

    #[test]
    fn week_view_maps_monday_to_sunday() {
        // 2026-09-13 es domingo; la semana es 7..13 de septiembre.
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 7), d(2026, 9, 13)];
        assert_eq!(week_view(&checks, today), [true, false, false, false, false, false, true]);
    }

    #[test]
    fn empty_checks_give_zeroes() {
        let today = d(2026, 9, 13);
        assert_eq!(current_streak(&[], today), 0);
        assert_eq!(best_streak(&[]), 0);
        assert_eq!(week_view(&[], today), [false; 7]);
    }
}
