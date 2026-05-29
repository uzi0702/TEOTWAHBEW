use crate::args::Config;

pub const RESET: &str = "\x1b[0m";
const DIR_COLOR: &str = "\x1b[34;1m";
const SPECIAL_COLORS: &[(&str, &str)] = &[
    ("Makefile", "\x1b[33m"),
    (".gitignore", "\x1b[36m"),
];

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
