fn info_for_path(path: String) -> String {
    format!("Placeholder for info about {}", path)
}

pub fn display_info_for_paths(paths: Vec<String>) {
    for path in paths {
        println!("{}", info_for_path(path));
    }
}
