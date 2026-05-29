mod args;
mod color;
mod filter;
mod format;

use std::fs;
use std::time::SystemTime;

use args::parse_args;
use color::{entry_color, RESET};
use filter::is_source_file;
use format::{format_mode, format_time, human_size};

fn main() {
    let cfg = parse_args();

    let mut items: Vec<(String, fs::Metadata)> = match fs::read_dir(".") {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                let meta = e.metadata().ok()?;
                Some((name, meta))
            })
            .filter(|(name, _)| !cfg.source_only || is_source_file(name))
            .collect(),
        Err(e) => {
            eprintln!("teot: {}", e);
            std::process::exit(1);
        }
    };

    if cfg.sort_by_date {
        items.sort_by(|(a_name, a_meta), (b_name, b_meta)| {
            let a_t = a_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let b_t = b_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            b_t.cmp(&a_t).then_with(|| a_name.cmp(b_name))
        });
    } else {
        items.sort_by(|(a, _), (b, _)| a.cmp(b));
    }

    for (name, meta) in &items {
        let is_dir = meta.is_dir();
        let mode = format_mode(meta);
        let time = meta
            .modified()
            .map(format_time)
            .unwrap_or_else(|_| "????-??-?? ??:??".to_string());
        let size = human_size(meta.len());
        let color = entry_color(name, is_dir, &cfg);
        let suffix = if is_dir { "/" } else { "" };
        let reset = if color.is_empty() { "" } else { RESET };

        println!(
            "{}  {}  {:>8}  {}{}{}{}",
            mode, time, size, color, name, suffix, reset
        );
    }
}
