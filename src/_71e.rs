pub fn simplify_path(pth: String) -> String {
    // Simplify Path
    // Given a string `pth`.
    // String is a unix file system absolute path.
    // Transform to a simplified form.
    // Transform rules:
    //  * Starts with '/'
    //  * '.' is current directory.
    //  * ".." is parent directory.
    //  * "//", "///" multiple slashes equal single slash.
    //  * "...", "...." three or more dots a directory name.
    // Return a string as a simplified form.
    // Use a stack to keep track of the directory path.

    // Initialize variable.
    let mut stk: Vec<&str> = Vec::new();

    // Loop through all directories.
    // Split directories with separator "/".
    for dir in pth.split("/") {
        match dir {
            // Skip empty or current directory.
            "" | "." => continue,
            // Traverse up one directory
            ".." => {
                stk.pop();
            }
            // Add directory.
            _ => stk.push(dir),
        }
    }

    // Build simplified path.
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
