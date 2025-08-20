use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::{Path, PathBuf};

use blake3::Hasher;
// You can replace this with Sha256, etc.
use rayon::prelude::*;
use walkdir::WalkDir;

/// Computes a **reproducible cryptographic hash** of an entire directory's contents.
///
/// This includes:
/// - The **relative path** of each file (relative to `path`)
/// - The **full content** of each file
///
/// The resulting hash is deterministic: two directories with the same file structure
/// and contents will produce the same hash, regardless of file system order.
///
/// # How it works
///
/// 1. **Enumerate**: Recursively walk the directory, collecting all file paths.
/// 2. **Sort**: Sort file paths lexicographically to ensure consistent ordering.
/// 3. **Parallel Hashing**: Each file’s content (and its path) is hashed in parallel using `rayon`.
/// 4. **Merge**: Final hash is computed by combining `(relative_path, content_hash)` pairs in sorted order.
///
/// This approach ensures:
/// - ✅ **Performance**: File I/O and hashing are parallelized.
/// - ✅ **Reproducibility**: Output is the same across runs and platforms.
/// - ✅ **Correctness**: Paths and contents are both included in the hash.
///
/// # Errors
///
/// Returns `Err(std::io::Error)` if:
/// - The directory cannot be traversed (permissions, I/O error).
/// - A file path contains invalid UTF-8 (required for hashing as string).
/// - A file cannot be read (e.g., deleted mid-operation).
///
/// # Performance Notes
///
/// - Best suited for directories with **many or large files**.
/// - Uses 8 KiB read buffers; adjust if processing very large files.
/// - Memory usage scales with number of files (stores path strings and hashes).
pub fn hash_directory<P: AsRef<Path>>(path: P) -> Result<String> {
    let base_path = path.as_ref();

    // Step 1: Recursively collect all file entries, sorted by path
    let file_entries: Result<Vec<PathBuf>> = WalkDir::new(base_path)
        .sort_by_file_name() // Ensure consistent traversal order
        .into_iter()
        .filter_map(|entry_result| {
            let entry = match entry_result {
                Ok(entry) if entry.file_type().is_file() => entry,
                Ok(_) => return None,                 // Skip directories
                Err(e) => return Some(Err(e.into())), // Propagate walkdir error as io::Error
            };

            // Get path relative to base directory
            entry
                .path()
                .strip_prefix(base_path)
                .map(|p| p.to_owned())
                .map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "failed to strip prefix (invalid directory structure)",
                    )
                })
                .map(Some)
                .transpose()
        })
        .collect();

    let relative_paths = file_entries?;

    // Step 2: Process each file in parallel and compute (path_str, content_hash)
    let path_hash_pairs: Result<Vec<(String, [u8; 32])>> = relative_paths
        .par_iter()
        .map(|rel_path| {
            // Convert path to UTF-8 string (required for hashing)
            let rel_path_str = rel_path
                .to_str()
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "file path contains invalid UTF-8 characters",
                    )
                })?
                .to_owned();

            // Open the full file path
            let full_path = base_path.join(rel_path);
            let file = File::open(&full_path)?;
            let mut reader = BufReader::new(file);
            let mut buffer = [0; 8192];
            let mut hasher = Hasher::new();

            // Include the relative path in the hash (to prevent collisions)
            hasher.update(rel_path_str.as_bytes());
            hasher.update(&[0]); // Null byte as separator

            // Read and hash file content
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }

            let hash: [u8; 32] = hasher.finalize().into();
            Ok((rel_path_str, hash))
        })
        .collect(); // Collect all results or return first error

    let mut path_hash_pairs = path_hash_pairs?;

    // Step 3: Sort by relative path to ensure deterministic output
    path_hash_pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Step 4: Final hash: combine all (path, hash) pairs in sorted order
    let mut final_hasher = Hasher::new();
    for (rel_path_str, content_hash) in path_hash_pairs {
        final_hasher.update(rel_path_str.as_bytes());
        final_hasher.update(&[0]);
        final_hasher.update(&content_hash);
    }

    Ok(final_hasher.finalize().to_hex().to_string())
}

// ================
// 🧪 UNIT TESTS
// ================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    // dev-dependency for safe temp dirs

    // Add this to Cargo.toml under [dev-dependencies]
    // tempfile = "3"

    /// Helper: create a test directory structure
    fn setup_test_dir() -> Result<TempDir> {
        let dir = tempfile::TempDir::new()?;

        // Create files:
        // dir/a.txt → "hello"
        // dir/sub/b.txt → "world"

        fs::create_dir(dir.path().join("sub"))?;

        fs::write(dir.path().join("a.txt"), b"hello")?;
        fs::write(dir.path().join("sub").join("b.txt"), b"world")?;

        Ok(dir)
    }

    #[test]
    fn test_hash_directory_identical_dirs() -> Result<()> {
        let dir1 = setup_test_dir()?;
        let dir2 = setup_test_dir()?; // same content

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_eq!(hash1, hash2, "Identical directories should have same hash");
        Ok(())
    }

    #[test]
    fn test_hash_directory_different_content() -> Result<()> {
        let dir1 = setup_test_dir()?;
        let dir2 = setup_test_dir()?;

        // Modify dir2: change a.txt
        fs::write(dir2.path().join("a.txt"), b"hello!")?; // changed

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_ne!(
            hash1, hash2,
            "Different content should yield different hash"
        );
        Ok(())
    }

    #[test]
    fn test_hash_directory_different_structure() -> Result<()> {
        let dir1 = setup_test_dir()?;
        let dir2 = TempDir::new()?;

        // dir2: only has root file
        fs::write(dir2.path().join("a.txt"), b"hello")?;

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_ne!(
            hash1, hash2,
            "Different structure should yield different hash"
        );
        Ok(())
    }

    #[test]
    fn test_hash_empty_directory() -> Result<()> {
        let dir1 = TempDir::new()?;
        let dir2 = TempDir::new()?;

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_eq!(
            hash1, hash2,
            "Two empty dirs should have same hash (empty hash)"
        );
        Ok(())
    }

    #[test]
    fn test_hash_single_file() -> Result<()> {
        let dir1 = TempDir::new()?;
        let dir2 = TempDir::new()?;

        fs::write(dir1.path().join("data.txt"), b"rust")?;
        fs::write(dir2.path().join("data.txt"), b"rust")?;

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_eq!(hash1, hash2);
        Ok(())
    }
}
