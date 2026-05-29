use std::env;

pub struct Config {
    pub sort_by_date: bool,
    pub source_only: bool,
    pub color_code: Option<String>,
    pub color_target: Option<String>,
}

pub fn parse_args() -> Config {
    let raw: Vec<String> = env::args().skip(1).collect();
    let refs: Vec<&str> = raw.iter().map(|s| s.as_str()).collect();
    parse_from(&refs)
}

/// Parse from a slice of string arguments (used in tests).
pub(crate) fn parse_from(args: &[&str]) -> Config {
    let mut cfg = Config {
        sort_by_date: false,
        source_only: false,
        color_code: None,
        color_target: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "-sd" => cfg.sort_by_date = true,
            "-c" => cfg.source_only = true,
            s if s.starts_with("--color=") => {
                cfg.color_code = Some(s["--color=".len()..].to_string());
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    cfg.color_target = Some(args[i + 1].to_string());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_no_args() {
        let cfg = parse_from(&[]);
        assert!(!cfg.sort_by_date);
        assert!(!cfg.source_only);
        assert!(cfg.color_code.is_none());
        assert!(cfg.color_target.is_none());
    }

    #[test]
    fn sort_by_date_flag() {
        let cfg = parse_from(&["-sd"]);
        assert!(cfg.sort_by_date);
    }

    #[test]
    fn source_only_flag() {
        let cfg = parse_from(&["-c"]);
        assert!(cfg.source_only);
    }

    #[test]
    fn color_without_target() {
        let cfg = parse_from(&["--color=31"]);
        assert_eq!(cfg.color_code.as_deref(), Some("31"));
        assert!(cfg.color_target.is_none());
    }

    #[test]
    fn color_with_target() {
        let cfg = parse_from(&["--color=31", "Makefile"]);
        assert_eq!(cfg.color_code.as_deref(), Some("31"));
        assert_eq!(cfg.color_target.as_deref(), Some("Makefile"));
    }

    #[test]
    fn combined_flags() {
        let cfg = parse_from(&["-sd", "-c"]);
        assert!(cfg.sort_by_date);
        assert!(cfg.source_only);
    }

    #[test]
    fn unknown_args_are_ignored() {
        let cfg = parse_from(&["--unknown", "value"]);
        assert!(!cfg.sort_by_date);
        assert!(!cfg.source_only);
        assert!(cfg.color_code.is_none());
    }
}
