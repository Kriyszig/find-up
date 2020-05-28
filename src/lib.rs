pub mod find_up {
    use std::fs;
    use std::path;
    use std::env;

    pub fn find(file: &str) -> std::option::Option<String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();
        
        return find_up(path, file);
    }

    pub fn find_dir(dir: &str) -> std::option::Option<String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        return find_up_dir(path, dir);
    }

    pub fn find_symlink(link: &str) -> std::option::Option<String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        return find_up_symlink(path, link);
    }

    pub fn find_up_symlink(path: &str, link: &str) -> std::option::Option<String> {
        let mut usable_path = path.clone().to_owned();

        loop {
            let mut link_path = usable_path.clone().to_owned();
            
            if cfg!(windows) {
                link_path.push('\\');
            } else {
                link_path.push('/');
            }

            link_path.push_str(link);
            let read_link = fs::read_link(link_path.clone().to_owned());
            let is_link: bool = match read_link {
                Ok(_link) => true,
                Err(_error) => false,
            };

            if path::Path::new(&link_path).exists() && is_link {
                return Some(link_path);
            } else {
                let index: std::option::Option<usize> = if cfg!(windows) {
                    usable_path.rfind('\\')
                } else {
                    usable_path.rfind('/')
                };

                if !index.is_some() {
                    return None;
                }

                let trim_start = index.unwrap();
                let trim_end = usable_path.len();
                usable_path.replace_range(trim_start..trim_end, "");
            }
        }
    }

    pub fn find_up_dir(path: &str, dir: &str) -> std::option::Option<String> {
        let mut usable_path = path.clone().to_owned();

        loop {
            let mut dir_path = usable_path.clone().to_owned();
            
            if cfg!(windows) {
                dir_path.push('\\');
            } else {
                dir_path.push('/');
            }

            dir_path.push_str(dir);

            if path::Path::new(&dir_path).exists() && path::Path::new(&dir_path).is_dir() {
                return Some(dir_path);
            } else {
                let index: std::option::Option<usize> = if cfg!(windows) {
                    usable_path.rfind('\\')
                } else {
                    usable_path.rfind('/')
                };

                if !index.is_some() {
                    return None;
                }

                let trim_start = index.unwrap();
                let trim_end = usable_path.len();
                usable_path.replace_range(trim_start..trim_end, "");
            }
        }
    }

    pub fn find_up(path: &str, file: &str) -> std::option::Option<String> {
        let mut usable_path = path.clone().to_owned();

        loop {
            let mut file_path = usable_path.clone().to_owned();
            
            if cfg!(windows) {
                file_path.push('\\');
            } else {
                file_path.push('/');
            }

            file_path.push_str(file);

            if path::Path::new(&file_path).exists() {
                return Some(file_path);
            } else {
                let index: std::option::Option<usize> = if cfg!(windows) {
                    usable_path.rfind('\\')
                } else {
                    usable_path.rfind('/')
                };

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
        let toml_path = find_up::find_up(path, "Cargo.toml");

        assert_eq!(toml_path_from_here.is_some(), true);
        assert_eq!(toml_path.is_some(), true);

        let toml_file = env::current_dir().unwrap();
        let mut first_toml_path = toml_file.to_string_lossy().into_owned();
        if cfg!(windows) {
            first_toml_path.push_str("\\Cargo.toml");
        } else {
            first_toml_path.push_str("/Cargo.toml");
        }

        let toml_path_result = toml_path.unwrap();
        let toml_path_from_here_result = toml_path_from_here.unwrap();

        assert_eq!(toml_path_result, first_toml_path);
        assert_eq!(toml_path_from_here_result, first_toml_path);
    }

    #[test]
    fn outside_test() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();
        let test_path = find_up::find_up(path, "test.txt");
        let test_path_from_here = find_up::find("test.txt");

        let file_dir = env::current_dir().unwrap();
        let mut file_path = String::from(file_dir.to_str().unwrap());
        let index: std::option::Option<usize> = if cfg!(windows) {
            file_path.rfind('\\')
        } else {
            file_path.rfind('/')
        };

        let trim_start = index.unwrap();
        let trim_end = file_path.len();

        if cfg!(windows) {
            file_path.replace_range(trim_start..trim_end, "\\test.txt");
        } else {
            file_path.replace_range(trim_start..trim_end, "/test.txt");
        }

        let test_path_result = test_path.unwrap();
        let test_path_from_here_result = test_path_from_here.unwrap();
        assert_eq!(test_path_result, test_path_from_here_result);
        assert_eq!(test_path_result, file_path);
    }

    #[test]
    fn none_test() { 
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        let no_path_from_here = find_up::find("Cargo.rofl");
        let no_path = find_up::find_up(path, "Cargo.rofl");

        assert_eq!(no_path.is_none(), true);
        assert_eq!(no_path_from_here.is_none(), true);
    }

    #[test]
    fn dir_test() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();

        let dir_path_from_here = find_up::find_dir("src");
        let dir_path = find_up::find_up_dir(path, "src");

        assert_eq!(dir_path_from_here.is_some(), true);
        assert_eq!(dir_path.is_some(), true);

        let dir_file = env::current_dir().unwrap();
        let mut first_dir_path = dir_file.to_string_lossy().into_owned();
        if cfg!(windows) {
            first_dir_path.push_str("\\src");
        } else {
            first_dir_path.push_str("/src");
        }

        let dir_path_result = dir_path.unwrap();
        let dir_path_from_here_result = dir_path_from_here.unwrap();

        assert_eq!(dir_path_result, first_dir_path);
        assert_eq!(dir_path_from_here_result, first_dir_path);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn symlink_test() {
        let link_path = find_up::find_up_symlink("", "bin");
        let link_path_from_here = find_up::find_symlink("bin");

        assert_eq!(link_path.is_some(), true);
        assert_eq!(link_path_from_here.is_some(), true);

        let link_path_result = link_path.unwrap();
        let link_path_from_here_result = link_path_from_here.unwrap();

        assert_eq!(link_path_result, "/bin");
        assert_eq!(link_path_from_here_result, "/bin");
    }
}

