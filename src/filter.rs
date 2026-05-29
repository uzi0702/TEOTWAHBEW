use std::path::Path;

const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "c", "cpp", "java", "cs", "js"];

pub fn is_source_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| SOURCE_EXTENSIONS.contains(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_source_extensions_are_recognized() {
        for ext in ["rs", "py", "c", "cpp", "java", "cs", "js"] {
            assert!(is_source_file(&format!("file.{ext}")), ".{ext} should be recognized");
        }
    }

    #[test]
    fn non_source_extensions_are_rejected() {
        assert!(!is_source_file("README.md"));
        assert!(!is_source_file("Cargo.toml"));
        assert!(!is_source_file("script.sh"));
        assert!(!is_source_file("style.css"));
    }

    #[test]
    fn file_without_extension_is_rejected() {
        assert!(!is_source_file("Makefile"));
        assert!(!is_source_file("noextension"));
    }
}
