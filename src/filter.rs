use std::path::Path;

const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "c", "cpp", "java", "cs", "js"];

pub fn is_source_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| SOURCE_EXTENSIONS.contains(&e))
}
