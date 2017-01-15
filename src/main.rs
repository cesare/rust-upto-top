use std::env::current_dir;
use std::io::Write;
use std::path::PathBuf;

fn is_top_directory(path: &PathBuf) -> bool {
    let mut target_path = PathBuf::from(path);
    target_path.push(".git");
    return target_path.is_dir();
}

fn visit(path: &PathBuf) -> Option<PathBuf> {
    if is_top_directory(path) {
        Some(path.clone())
    } else {
        path.parent().and_then(|parent| visit(&PathBuf::from(parent)))
    }
}

fn main() {
    match current_dir() {
        Ok(path) => {
            if let Some(parent) = path.parent() {
                let dir = PathBuf::from(parent);
                visit(&dir).map(|path| println!("{}", path.display()));
            }
        }
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).unwrap();
        }
    }
}
