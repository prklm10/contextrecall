use std::path::{Path, PathBuf};

// We don't need 'pub' for this constant since it's only used inside this file
const PROJECT_MARKERS: &[&str] = &[
    ".git",
    "Gemfile",
    "docker-compose.yml",
    "package.json",
    "Cargo.toml",
    "pom.xml",
];

// Added 'pub' here so main.rs can see it!
// function that takes a starting directory,
// checks for our markers, and if it doesn't find them,
// moves up to the parent directory and tries again.
// if it reaches root directly without findng any marker it returns None
pub fn find_project_root(starting_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = starting_dir;

    loop {
        for marker in PROJECT_MARKERS {
            let marker_path = current_dir.join(marker);
            if marker_path.exists() {
                return Some(current_dir.to_path_buf());
            }
        }

        match current_dir.parent() {
            Some(parent) => {
                current_dir = parent;
            }
            None => {
                return None;
            }
        }
    }
}
