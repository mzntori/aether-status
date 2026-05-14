#[derive(Debug)]
pub enum Color {
    Clear,
    Yellow,
    Red,
}

/// Colorizes a string into an alarm color, given a value `v` and a tuple `range` a thresholds.
/// If `v < range.0` returns red if `v < range.1` returns yellow.
/// Otherwise returns clear.
pub fn colorize_range<C: PartialOrd>(s: &str, v: C, range: (C, C)) -> String {
    colorize(s, v, move |x| {
        if x < range.0 {
            Color::Red
        } else if x < range.1 {
            Color::Yellow
        } else {
            Color::Clear
        }
    })
}

/// Colorizes a string into an alarm color, given a value and a function determining the color.
pub fn colorize<C, F>(s: &str, v: C, f: F) -> String
where
    F: Fn(C) -> Color,
{
    match f(v) {
        Color::Clear => format!("<span foreground=\"#E2CEAD\">{}</span>", s),
        Color::Yellow => format!("<span foreground=\"#DD9C55\">{}</span>", s),
        Color::Red => format!("<span foreground=\"#AD4549\">{}</span>", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize() {
        println!("{}", colorize("test", 0.5, |_| { Color::Yellow }))
    }

    #[test]
    fn test_colorize_range() {
        let s = "test";
        let range = (0.1, 0.3);

        dbg!(colorize_range(s, 0.01, range));
        dbg!(colorize_range(s, 0.1, range));
        dbg!(colorize_range(s, 0.2, range));
        dbg!(colorize_range(s, 0.3, range));
        dbg!(colorize_range(s, 0.4, range));
    }
}
