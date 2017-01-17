use std::env::current_dir;
use std::path::PathBuf;

fn is_top_directory(target_path: &PathBuf) -> bool {
    let mut path = PathBuf::from(target_path);
    path.push(".git");
    path.exists()
}

fn list_directories(maybe_path: Option<PathBuf>) -> Vec<PathBuf> {
    let mut paths = vec![];
    match maybe_path {
        Some(path) => {
            if is_top_directory(&path) {
                paths.push(PathBuf::from(&path));
            }

            let ancestors = traverse(&path);
            paths.extend_from_slice(&ancestors);
            paths
        }
        None => paths,
    }
}

fn traverse(target_path: &PathBuf) -> Vec<PathBuf> {
    let parent = target_path.parent().map(|path| PathBuf::from(path));
    list_directories(parent)
}

fn main() {
    let dir = current_dir().unwrap();
    let paths = traverse(&dir);
    for path in paths {
        println!("{}", path.display());
    }
}
