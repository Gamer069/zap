#[inline]
pub fn filename_from_path(path: &str) -> Option<&str> {
    // Handle both Unix and Windows separators
    path.rsplit(&['/', '\\'][..])
        .next()
        .filter(|s| !s.is_empty())
}

// Join paths as strings - single allocation
#[inline]
pub fn join_path_str(base: &str, name: &str) -> String {
    let mut result = String::with_capacity(base.len() + name.len() + 1);
    result.push_str(base);
    
    // Add separator if needed
    if !base.ends_with('/') && !base.ends_with('\\') {
        #[cfg(unix)]
        result.push('/');
        
        #[cfg(windows)]
        result.push('\\');
    }
    
    result.push_str(name);

    result
}

// Check if path exists - uses std::path only for the check
#[inline]
pub fn path_exists_str(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
