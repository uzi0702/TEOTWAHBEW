use crate::args::Config;

pub const RESET: &str = "\x1b[0m";
const DIR_COLOR: &str = "\x1b[34;1m";
const SPECIAL_COLORS: &[(&str, &str)] = &[
    ("Makefile", "\x1b[33m"),
    (".gitignore", "\x1b[36m"),
];

/// Returns the ANSI escape sequence to use for the given entry, or an empty string for no color.
pub fn entry_color(name: &str, is_dir: bool, cfg: &Config) -> String {
    if let Some(ref code) = cfg.color_code {
        let applies = cfg.color_target.as_deref().is_none_or(|t| t == name);
        if applies {
            return format!("\x1b[{}m", code);
        }
    }
    if is_dir {
        return DIR_COLOR.to_string();
    }
    for &(special, color) in SPECIAL_COLORS {
        if name == special {
            return color.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Config;

    fn no_color() -> Config {
        Config {
            sort_by_date: false,
            source_only: false,
            color_code: None,
            color_target: None,
        }
    }

    #[test]
    fn directory_gets_dir_color() {
        assert_eq!(entry_color("foo", true, &no_color()), DIR_COLOR);
    }

    #[test]
    fn regular_file_no_color() {
        assert_eq!(entry_color("foo.txt", false, &no_color()), "");
    }

    #[test]
    fn makefile_gets_yellow() {
        assert_eq!(entry_color("Makefile", false, &no_color()), "\x1b[33m");
    }

    #[test]
    fn gitignore_gets_cyan() {
        assert_eq!(entry_color(".gitignore", false, &no_color()), "\x1b[36m");
    }

    #[test]
    fn custom_color_applies_to_all_when_no_target() {
        let cfg = Config {
            color_code: Some("31".to_string()),
            color_target: None,
            sort_by_date: false,
            source_only: false,
        };
        assert_eq!(entry_color("any_file.txt", false, &cfg), "\x1b[31m");
    }

    #[test]
    fn custom_color_applies_to_matching_target() {
        let cfg = Config {
            color_code: Some("31".to_string()),
            color_target: Some("Makefile".to_string()),
            sort_by_date: false,
            source_only: false,
        };
        assert_eq!(entry_color("Makefile", false, &cfg), "\x1b[31m");
    }

    #[test]
    fn custom_color_skips_non_matching_target() {
        let cfg = Config {
            color_code: Some("31".to_string()),
            color_target: Some("Makefile".to_string()),
            sort_by_date: false,
            source_only: false,
        };
        assert_eq!(entry_color("other.txt", false, &cfg), "");
    }
}
