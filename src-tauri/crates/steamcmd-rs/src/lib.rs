use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

const STEAMCMD_NAME: &str = "steamcmd.exe";

pub struct SteamCMD {
    home_dir: PathBuf,
}

impl SteamCMD {
    pub fn new(home_dir: PathBuf) -> SteamCMD {
        SteamCMD { home_dir }
    }

    pub fn set_steamcmd_path(&mut self, path: PathBuf) {
        self.home_dir = path;
    }
    fn steamcmd_full_path(&self) -> PathBuf {
        self.home_dir.join(STEAMCMD_NAME)
    }

    pub async fn run_script_from_path(&self, script: &PathBuf) -> Result<(), ()> {
        let mut cmd = tokio::process::Command::new(self.steamcmd_full_path());
        cmd.current_dir(self.home_dir.clone())
            .arg("+runscript")
            .arg(script)
            .output()
            .await
            .map(|_| ())
            .map_err(|_| ())
    }

    pub async fn run_script(&self, script: &str) -> Result<(), ()> {
        let mut temp_file: NamedTempFile<File> = tempfile::Builder::default()
            .suffix(".txt")
            .tempfile()
            .map_err(|_| ())?;
        writeln!(temp_file, "{}", script).map_err(|_| ())?;
        self.run_script_from_path(&temp_file.path().to_path_buf())
            .await
    }

    fn download_item(game_id: usize, mod_id: usize) -> String {
        format!("workshop_download_item {} {}", game_id, mod_id)
    }

    fn login(username: Option<&str>, password: Option<&str>) -> String {
        match (username, password) {
            (Some(username), Some(password)) => format!("login {} {}", username, password),
            (Some(username), None) => format!("login {}", username),
            (None, Some(password)) => format!("login anonymous {}", password),
            (None, None) => "login anonymous".to_string(),
        }
    }
    pub async fn download_mod(&self, game_id: usize, mod_ids: Vec<usize>) -> Result<(), ()> {
        // gen all mod down scripts
        let script = {
            let mut script = String::new();
            script += &SteamCMD::login(None, None);
            script += "\n";
            script += &mod_ids
                .into_iter()
                .map(|mod_id| SteamCMD::download_item(game_id, mod_id))
                .collect::<String>();
            script
        };
        self.run_script(&script).await
    }

    pub async fn download_mod_par(
        &self,
        game_id: usize,
        mod_ids: Vec<usize>,
        n: usize,
    ) -> Result<(), ()> {
        let chunk_size = mod_ids.len() + 1 / n;
        let scripts = mod_ids
            .chunks(chunk_size)
            .map(|mod_ids| {
                let mut script = String::new();
                script += &SteamCMD::login(None, None);
                script += "\n";
                script += &mod_ids
                    .into_iter()
                    .map(|mod_id| SteamCMD::download_item(game_id, mod_id.clone()))
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
        Err(())
    }
}
