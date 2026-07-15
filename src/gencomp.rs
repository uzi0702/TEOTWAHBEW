//! Shell completion file generation, invoked with the `--completions` option.
//!
//! The CLI itself is parsed by the hand-written parser in `args.rs` (the
//! `-sd` flag cannot be expressed with clap), so this module keeps a clap
//! `Command` definition of the interface purely for `clap_complete`.

use std::path::Path;

use clap::{Arg, ArgAction, Command};
use clap_complete::Shell;

/// clap definition of the CLI, used only for generating completion files.
fn build_command() -> Command {
    Command::new("teot")
        .about("TEOTWAHBEW is a CLI command like 'ls'")
        .arg(
            Arg::new("sort_date")
                .long("sd")
                .action(ArgAction::SetTrue)
                .help("Sort files by most recently updated"),
        )
        .arg(
            Arg::new("source_only")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Display only source code files"),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .require_equals(true)
                .value_name("colorcode")
                .help("Choose display color (numeric ANSI code such as 31)"),
        )
        .arg(
            Arg::new("file")
                .value_name("file_name")
                .help("Apply --color only to this file"),
        )
        .arg(
            Arg::new("completions")
                .long("completions")
                .action(ArgAction::SetTrue)
                .help("generate completion files"),
        )
}

fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    if let Ok(mut dest) = std::fs::File::create(destfile) {
        clap_complete::generate(s, app, appname, &mut dest);
    }
}

pub(super) fn generate(outdir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
    let appname = "teot";
    let mut app = build_command();
    app.set_bin_name(appname);
    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
    generate_impl(Elvish, &mut app, appname, outdir, format!("elvish/{appname}"));
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
    generate_impl(
        PowerShell,
        &mut app,
        appname,
        outdir,
        format!("powershell/{appname}"),
    );
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}
