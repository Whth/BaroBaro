use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

const STEAMCMD_NAME: &str = "steamcmd.exe";

#[derive(Debug, Default)]
pub struct SteamCMD {
    home_dir: Option<PathBuf>,
}

impl SteamCMD {
    const WORKSHOP_DIR_BASE: &'static str = r"steamapps\workshop\content";

    pub fn new(home_dir: PathBuf) -> SteamCMD {
        SteamCMD {
            home_dir: Some(home_dir),
        }
    }

    pub fn workshop_item_dir(&self, app_id: u64, mod_id: u64) -> Result<PathBuf, String> {
        let base = self
            .home_dir
            .as_ref()
            .ok_or("SteamCMD home directory not set")?;
        Ok(base
            .join(Self::WORKSHOP_DIR_BASE)
            .join(app_id.to_string())
            .join(mod_id.to_string()))
    }

    pub fn set_steamcmd_home(&mut self, path: PathBuf) -> &mut Self {
        self.home_dir = Some(path);
        self
    }

    fn steamcmd_full_path(&self) -> Result<PathBuf, String> {
        let base = self
            .home_dir
            .as_ref()
            .ok_or("SteamCMD home directory not set")?;
        Ok(base.join(STEAMCMD_NAME))
    }

    pub async fn run_script_from_path(&self, script: &PathBuf) -> Result<(), String> {
        let steamcmd_path = self.steamcmd_full_path()?;
        let output = tokio::process::Command::new(steamcmd_path)
            .arg("+runscript")
            .arg(script)
            .output()
            .await
            .map_err(|e| format!("Failed to run steamcmd: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "SteamCMD failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub async fn run_script(&self, script: &str) -> Result<(), String> {
        let mut file =
            NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(script.as_bytes())
            .map_err(|e| format!("Failed to write script: {}", e))?;
        self.run_script_from_path(&file.path().to_path_buf()).await
    }

    fn download_item(game_id: u64, mod_id: u64) -> String {
        format!("workshop_download_item {} {}", game_id, mod_id)
    }

    fn login(username: Option<&str>, password: Option<&str>) -> String {
        let mut s = String::from("+login anonymous");
        if let Some(u) = username {
            s = format!("+login {} {}", u, password.unwrap_or(""));
        }
        s
    }

    pub async fn download_mod(&self, game_id: u64, mod_ids: Vec<u64>) -> Result<(), String> {
        let script = {
            let mut script = String::new();
            script += &SteamCMD::login(None, None);
            script += "\n";
            script += &mod_ids
                .iter()
                .map(|mod_id| SteamCMD::download_item(game_id, *mod_id))
                .collect::<Vec<String>>()
                .join("\n");
            script += "\n";
            script += "quit";
            script
        };
        self.run_script(&script).await
    }

    pub async fn download_mod_par(
        &self,
        game_id: u64,
        mod_ids: Vec<u64>,
        n: usize,
    ) -> Result<(), String> {
        let n = n.min(mod_ids.len()).max(1);
        let chunk_size = (mod_ids.len() + n - 1) / n;
        let scripts = mod_ids
            .chunks(chunk_size)
            .map(|mod_ids| {
                let mut script = String::new();
                script += &SteamCMD::login(None, None);
                script += "\n";
                script += &mod_ids
                    .iter()
                    .map(|mod_id| SteamCMD::download_item(game_id, *mod_id))
                    .collect::<Vec<String>>()
                    .join("\n");
                script += "\n";
                script += "quit";
                script
            })
            .collect::<Vec<_>>();

        let fut = scripts
            .into_iter()
            .map(|script| async move { self.run_script(script.as_str()).await })
            .collect::<Vec<_>>();

        if futures::future::join_all(fut)
            .await
            .into_iter()
            .all(|x| x.is_ok())
        {
            return Ok(());
        }
        Err("Failed to download some of the mods".to_string())
    }
}
