use chrono::{Duration, NaiveDate};
use im::HashMap;
use ratatui::{style::Style, text::Span};

use crate::helpers::palette::Theme;

pub const WEEKS: usize = 53;
pub const DAYS: usize = 7;

pub fn build_heatmap(
    counts: &HashMap<NaiveDate, usize>,
    today: NaiveDate,
) -> [[usize; WEEKS]; DAYS] {
    let mut grid = [[0usize; WEEKS]; DAYS];

    let start = today - Duration::days((WEEKS * 7) as i64);

    for week in 0..WEEKS {
        for day in 0..DAYS {
            let date = start + Duration::days((week * 7 + day) as i64);
            grid[day][week] = *counts.get(&date).unwrap_or(&0);
        }
    }

    grid
}

pub fn heat_cell(count: usize, theme: &Theme) -> Span {
    let (ch, color) = match count {
        0 => (" ", None),
        1 => ("⠁", Some(theme.COLOR_GREY_800)),
        2 => ("⠃", Some(theme.COLOR_GREY_800)),
        3 => ("⠇", Some(theme.COLOR_GREY_800)),
        4 => ("⠏", Some(theme.COLOR_GREY_800)),
        5 => ("⠟", Some(theme.COLOR_GREY_800)),
        6 => ("⠿", Some(theme.COLOR_GREY_800)),
        7 => ("⡿", Some(theme.COLOR_GREY_800)),
        _ => ("⣿", Some(theme.COLOR_GREY_800)),
    };

    let mut style = Style::default();
    if let Some(c) = color {
        style = style.fg(c);
    }

    Span::styled(ch, style)
}
