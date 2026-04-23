use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

const APP_DIR: &str = "discord-bot";
const AUDIO_DIR: &str = "audio";
const AUDIO_FILE: &str = "tts.wav";

pub static WORKSPACE: OnceLock<Workspace> = OnceLock::new();

pub struct Workspace {
    pub path_app: PathBuf,
    pub folder_audio: PathBuf,
}

impl Workspace {
    pub fn global() -> &'static Workspace {
        WORKSPACE.get().expect("Workspace no ha sido inicializado")
    }

    pub fn load_workspace() -> io::Result<Self> {
        let path_app = PathBuf::from(APP_DIR);
        let folder_audio = path_app.join(AUDIO_DIR);
        fs::create_dir_all(&folder_audio)?;
        println!("Workspace verificado en: {:?}", path_app);
        Ok(Self {
            path_app,
            folder_audio,
        })
    }

    pub fn get_audio_file(&self) -> Option<PathBuf> {
        let audio_path = self.folder_audio.join(AUDIO_FILE);
        if audio_path.is_file() {
            Some(audio_path)
        } else {
            None
        }
    }
}