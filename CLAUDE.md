# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

TEOTWAHBEW は Rust 製の CLI ツール。`ls` に似た動作で、カレントディレクトリのファイルを表示し、特定ファイル（`Makefile`、`.gitignore` など）を色付きでハイライトする。

仕様は `.github/assets/spec.md` に記載されている。

## コマンド

```bash
# ビルド（リリース）
cargo build --release

# Lint（警告をエラーとして扱う）
cargo clippy -- -D warnings

# テスト実行
cargo test

# カバレッジ計測（llvm-cov が必要）
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
cargo llvm-cov --lcov --output-path coverage.lcov
```

## アーキテクチャ

エントリポイントは `src/main.rs` のみ。現状は scaffold 段階（`Hello, world!` のみ）。

実装予定の CLI インターフェース：

```
teot [OPTIONS]
teot [--color=<colorcode>] [file_name]
```

オプション（`.github/assets/spec.md` より）：
- `-sd` : 最終更新日時の降順でソート。タイムスタンプ同一なら名前順
- `-c` : ソースコードファイルのみ表示（対象拡張子: `.rs .py .c .cpp .java .cs .js`）
- `--color=<colorcode> [file_name]` : 指定ファイルの表示色を設定。ファイル未指定なら全ファイルに適用

出力情報：最終更新時刻・サイズ（human-readable）・ファイルモード など

## CI / リリースフロー

| ワークフロー | トリガー | 内容 |
|---|---|---|
| `build.yaml` | 全ブランチへの push | `cargo clippy` + `cargo build --release` + カバレッジ（ubuntu のみ） |
| `update_version.yaml` | `releases/v*` ブランチへの push | `Cargo.toml` のバージョンを自動更新してコミット |
| `public.yaml` | `releases/v*` → `main` の PR マージ | GitHub Release を作成してアセットをアップロード |

リリース手順：`releases/vX.Y.Z` ブランチを作成して push → バージョン自動更新 → `main` へ PR → マージで公開。

# テスト方針
- 単体テスト（Sテスト）は、ソースコー
ドと同じファイルに書く。
- 結合テスト、システムテスト（M, Lテス
ト）はtestsディレクトリに置く。
- examplesディレクトリには使用例とな
るプログラムを置く。

# docコメント
docコメント
•///でのコメントはcargo docで生成さ
れるドキュメントに利用される。
• 関数や変数の前に書く。
•モジュール（ファイル）全体のコメント
は、ファイル冒頭に//!で始まるコメン
トに書く。

# docker
配布用のdocker fileの作成
- ../20260626.pdfの26ページの画像のようなファイルを作成
- ビルドファイルの作成（just）
    - .github/scripts/build_docker.sh
    - github actionsにも追加（同資料34ページ）

# 補完ファイルを出力する
• コマンドライン引数にオプションが指定されれば、
補完ファイルを出力する。
• 対応するすべてのシェルに対応したファイルを出力する。
1. Cargo.toml に clap_complete を追加する。
• cargo add clap_complete
2. コマンドラインオプションに complete を追加する。
• #[arg(long, help="generate completion files",
 default_value_t = false)]
pub completions: bool,
•下のようなファイルを gencomp.rsに置く。
• main.rsで引数解析後、completions オプションが指定されていれば、
generate("completions") を呼び出す。
```
use std::path::Path;
use clap::{Command, CommandFactory};
fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
 let destfile = outdir.join(file);
 std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
 if let Ok(mut dest) = std::fs::File::create(destfile) {
 clap_complete::generate(s, app, appname, &mut dest);
 }
}
pub(super) fn generate(outdir: &Path) {
 use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
 let appname = "lis";
 let mut app = crate::Args::command();
 app.set_bin_name(appname);
 generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
 generate_impl(Elvish, &mut app, appname, outdir, format!("elvish/{appname}"));
 generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
 generate_impl(PowerShell, &mut app, appname, outdir, format!("powershell/{appname}"));
 generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}
```