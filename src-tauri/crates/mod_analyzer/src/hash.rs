use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

use blake3::Hasher;
use walkdir::WalkDir;


/// Computes a BLAKE3 hash of a directory.
/// Hash is based on:
/// - Relative file paths (UTF-8 only)
/// - File contents
/// Files are traversed in sorted order for consistency.
pub fn hash_directory<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut hasher = Hasher::new();
    let base_path = path.as_ref();

    let walker = WalkDir::new(base_path)
        .sort_by_file_name()
        .into_iter();

    for entry_result in walker {
        let entry = entry_result?; // walkdir::Error → io::Error

        if !entry.file_type().is_file() {
            continue;
        }

        // Get relative path
        let rel_path = entry.path().strip_prefix(base_path)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "failed to strip prefix"))?;

        let rel_path_str = rel_path.to_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "path is not valid UTF-8"))?;

        // Include relative path in hash
        hasher.update(rel_path_str.as_bytes());
        hasher.update(&[0]); // null separator

        // Read file content
        let file = File::open(entry.path())?;
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 8192];

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
    }

    Ok(hasher.finalize().to_hex().to_string())
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

        assert_ne!(hash1, hash2, "Different content should yield different hash");
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

        assert_ne!(hash1, hash2, "Different structure should yield different hash");
        Ok(())
    }

    #[test]
    fn test_hash_empty_directory() -> Result<()> {
        let dir1 = TempDir::new()?;
        let dir2 = TempDir::new()?;

        let hash1 = hash_directory(dir1.path())?;
        let hash2 = hash_directory(dir2.path())?;

        assert_eq!(hash1, hash2, "Two empty dirs should have same hash (empty hash)");
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