use logger::info;
use std::path::Path;

pub mod hash;

pub fn link_directory(src: String, dst: String) -> Result<(), String> {
    info!("Creating soft link from {} to {}", src, dst);
    let src = Path::new(src.as_str());
    let dst = Path::new(dst.as_str());

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(src, dst).map_err(|e| {
        format!(
            "Failed to create symbolic link: {} -> {}.\n\
             Error: {}. \n\
             Note: You may need admin rights or to enable Developer Mode on Windows.",
            src.display(),
            dst.display(),
            e
        )
    })?;
    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dst)
        .map_err(|e| format!("{}, failed to create soft link.", e))?;

    Ok(())
}

pub use hash::*;
