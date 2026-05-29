use std::env;

pub struct Config {
    pub sort_by_date: bool,
    pub source_only: bool,
    pub color_code: Option<String>,
    pub color_target: Option<String>,
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cfg = Config {
        sort_by_date: false,
        source_only: false,
        color_code: None,
        color_target: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-sd" => cfg.sort_by_date = true,
            "-c" => cfg.source_only = true,
            s if s.starts_with("--color=") => {
                cfg.color_code = Some(s["--color=".len()..].to_string());
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    cfg.color_target = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    cfg
}
