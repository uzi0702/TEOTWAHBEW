use std::env;

#[derive(Debug)]
pub struct Config {
    pub sort_by_date: bool,
    pub source_only: bool,
    pub color_code: Option<String>,
    pub color_target: Option<String>,
}

pub fn parse_args() -> Result<Config, String> {
    let raw: Vec<String> = env::args().skip(1).collect();
    let refs: Vec<&str> = raw.iter().map(|s| s.as_str()).collect();
    parse_from(&refs)
}

/// Returns true if the code is a valid ANSI color code such as "31" or "34;1".
fn is_valid_color_code(code: &str) -> bool {
    !code.is_empty()
        && code.split(';').all(|part| {
            !part.is_empty() && part.chars().all(|c| c.is_ascii_digit())
        })
}

/// Parse from a slice of string arguments (used in tests).
pub(crate) fn parse_from(args: &[&str]) -> Result<Config, String> {
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
            "--color" => {
                return Err(
                    "'--color' requires '=': use --color=<colorcode> (e.g. --color=31)"
                        .to_string(),
                );
            }
            s if s.starts_with("--color=") => {
                let code = &s["--color=".len()..];
                if !is_valid_color_code(code) {
                    return Err(format!(
                        "invalid color code '{}': use a numeric ANSI code such as 31 (red), 32 (green), 34 (blue)",
                        code
                    ));
                }
                cfg.color_code = Some(code.to_string());
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    cfg.color_target = Some(args[i + 1].to_string());
                    i += 1;
                }
            }
            s if s.starts_with('-') => {
                return Err(format!("unknown option '{}'", s));
            }
            _ => {}
        }
        i += 1;
    }
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_no_args() {
        let cfg = parse_from(&[]).unwrap();
        assert!(!cfg.sort_by_date);
        assert!(!cfg.source_only);
        assert!(cfg.color_code.is_none());
        assert!(cfg.color_target.is_none());
    }

    #[test]
    fn sort_by_date_flag() {
        let cfg = parse_from(&["-sd"]).unwrap();
        assert!(cfg.sort_by_date);
    }

    #[test]
    fn source_only_flag() {
        let cfg = parse_from(&["-c"]).unwrap();
        assert!(cfg.source_only);
    }

    #[test]
    fn color_without_target() {
        let cfg = parse_from(&["--color=31"]).unwrap();
        assert_eq!(cfg.color_code.as_deref(), Some("31"));
        assert!(cfg.color_target.is_none());
    }

    #[test]
    fn color_with_target() {
        let cfg = parse_from(&["--color=31", "Makefile"]).unwrap();
        assert_eq!(cfg.color_code.as_deref(), Some("31"));
        assert_eq!(cfg.color_target.as_deref(), Some("Makefile"));
    }

    #[test]
    fn compound_color_code_is_accepted() {
        let cfg = parse_from(&["--color=34;1"]).unwrap();
        assert_eq!(cfg.color_code.as_deref(), Some("34;1"));
    }

    #[test]
    fn combined_flags() {
        let cfg = parse_from(&["-sd", "-c"]).unwrap();
        assert!(cfg.sort_by_date);
        assert!(cfg.source_only);
    }

    #[test]
    fn unknown_option_is_error() {
        let err = parse_from(&["--unknown", "value"]).unwrap_err();
        assert!(err.contains("unknown option"));
        assert!(err.contains("--unknown"));
    }

    #[test]
    fn color_without_equal_is_error() {
        let err = parse_from(&["--color", "31"]).unwrap_err();
        assert!(err.contains("--color=<colorcode>"));
    }

    #[test]
    fn color_name_is_error() {
        let err = parse_from(&["--color=red"]).unwrap_err();
        assert!(err.contains("invalid color code"));
        assert!(err.contains("red"));
    }

    #[test]
    fn empty_color_code_is_error() {
        let err = parse_from(&["--color="]).unwrap_err();
        assert!(err.contains("invalid color code"));
    }
}
