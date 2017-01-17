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

            let ancestors = list_directories(path.parent().map(|path| PathBuf::from(&path)));
            paths.extend_from_slice(&ancestors);
            paths
        }
        None => paths,
    }
}

fn main() {
    let dir = current_dir().unwrap();
    let paths = list_directories(dir.parent().map(|path| PathBuf::from(path)));
    for path in paths {
        println!("{}", path.display());
    }
}
