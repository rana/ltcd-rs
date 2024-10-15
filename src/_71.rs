pub fn simplify_path(pth: String) -> String {
    // Simplify Path
    // Given a string `pth`.
    // String is a unix file system absolute path.
    // Transform to a simplfied form.
    // Transform rules:
    //  * '.' is current directory.
    //  * ".." is parent directory.
    //  * Multiple '//', '///' equal single '/'.
    //  * "..." three or more dots is a directory name.
    // Return a string as a simplified path.
    // Use a stack to navigate the directory structure.

    // Initialize variable.
    let mut stk: Vec<&str> = Vec::new();

    // Split the absolute path with '/'
    for dir in pth.split('/') {
        match dir {
            // Skip empty and current.
            "" | "." => continue,
            // Go up one directory.
            ".." => {
                stk.pop();
            }
            // Push valid directory.
            _ => {
                stk.push(dir);
            }
        }
    }

    // Build the simplified path string.
    let res: String = format!("/{}", stk.join("/"));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        let path = String::from("/../");
        let res = simplify_path(path);
        assert_eq!(res, "/");
    }

    #[test]
    fn test_multiple_slashes() {
        let path = String::from("/home//foo/");
        let res = simplify_path(path);
        assert_eq!(res, "/home/foo");
    }

    #[test]
    fn test_complex_path() {
        let path = String::from("/a/./b/../../c/");
        let res = simplify_path(path);
        assert_eq!(res, "/c");
    }

    #[test]
    fn test_current_directory() {
        let path = String::from("/./././.");
        let res = simplify_path(path);
        assert_eq!(res, "/");
    }

    #[test]
    fn test_empty_path() {
        let path = String::from("/");
        let res = simplify_path(path);
        assert_eq!(res, "/");
    }
}
