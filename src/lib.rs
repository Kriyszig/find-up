pub mod find_up {
    use std::path;
    use std::env;

    pub fn find(file: &str) -> std::option::Option<String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        return find_in(path, file);
    }

    pub fn find_in(path: &str, file: &str) -> std::option::Option<String> {
        let mut usable_path = path.clone().to_owned();

        loop {
            let mut file_path = usable_path.clone().to_owned();
            file_path.push('/');
            file_path.push_str(file);

            if path::Path::new(&file_path).exists() {
                return Some(file_path);
            } else {
                let index: std::option::Option<usize> = usable_path.rfind('/');
                if !index.is_some() {
                    return None;
                }

                let trim_start = index.unwrap();
                let trim_end = usable_path.len();
                usable_path.replace_range(trim_start..trim_end, "");
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::env;
    use crate::find_up;

    #[test]
    fn cargo_test() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        let toml_path_from_here = find_up::find("Cargo.toml");
        let toml_path = find_up::find_in(path, "Cargo.toml");

        assert_eq!(toml_path_from_here.is_some(), true);
        assert_eq!(toml_path.is_some(), true);

        let toml_file = env::current_dir().unwrap();
        let mut first_toml_path = toml_file.to_string_lossy().into_owned();
        first_toml_path.push_str("/Cargo.toml");

        let toml_path_result = toml_path.unwrap();
        let toml_path_from_here_result = toml_path_from_here.unwrap();

        assert_eq!(toml_path_result, first_toml_path);
        assert_eq!(toml_path_from_here_result, first_toml_path);
    }

    #[test]
    fn outside_test() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();
        let test_path = find_up::find_in(path, "test.txt");
        let test_path_from_here = find_up::find("test.txt");

        let file_dir = env::current_dir().unwrap();
        let mut file_path = String::from(file_dir.to_str().unwrap());
        let index: std::option::Option<usize> = file_path.rfind('/');
        let trim_start = index.unwrap();
        let trim_end = file_path.len();
        file_path.replace_range(trim_start..trim_end, "/test.txt");
        
        let test_path_result = test_path.unwrap();
        let test_path_from_here_result = test_path_from_here.unwrap();
        assert_eq!(test_path_result, test_path_from_here_result);
        assert_eq!(test_path_result, file_path);
    }
}

