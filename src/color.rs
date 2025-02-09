use color_print::cformat;

pub enum Color {
    Clear,
    Yellow,
    Red,
}

/// Colorizes a string into an alarm color, given a value and a function determining the color.
pub fn colorize<C, F>(s: String, v: C, f: F) -> String
where
    F: Fn(C) -> Color,
{
    match f(v) {
        Color::Clear => s,
        Color::Yellow => {
            cformat!("<rgb(250,193,73)>{}</>", s)
        }
        Color::Red => {
            cformat!("<rgb(251,82,69)>{}</>", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize() {
        println!(
            "{}",
            colorize("test".to_string(), 0.5, |_| { Color::Yellow })
        )
    }
}
