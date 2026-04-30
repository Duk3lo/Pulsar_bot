use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    sync::OnceLock,
};

const APP_DIR: &str = "discord-bot";
const AUDIO_DIR: &str = "audio";
const ANIM_DIR: &str = "animations";
const AUDIO_FILE: &str = "tts.wav";

pub static WORKSPACE: OnceLock<Workspace> = OnceLock::new();

pub struct Workspace {
    pub path_app: PathBuf,
    pub folder_audio: PathBuf,
    pub folder_animations: PathBuf,
}

impl Workspace {
    pub fn load_workspace() -> io::Result<Self> {
        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "No se pudo obtener el directorio del ejecutable",
            )
        })?;

        let path_app = exe_dir.join(APP_DIR);
        let folder_audio = path_app.join(AUDIO_DIR);
        let folder_animations = path_app.join(ANIM_DIR);

        fs::create_dir_all(&folder_audio)?;
        fs::create_dir_all(&folder_animations)?;

        println!("Workspace verificado en: {:?}", path_app);

        Ok(Self {
            path_app,
            folder_audio,
            folder_animations,
        })
    }

    pub fn get_audio_file(&self) -> Option<PathBuf> {
        let audio_path = self.folder_audio.join(AUDIO_FILE);
        audio_path.is_file().then_some(audio_path)
    }

    pub fn get_animation_file(&self, name: &str) -> PathBuf {
        self.folder_animations.join(name)
    }
}

pub fn load_frames_from_file(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let mut frames = Vec::new();
    let mut current = String::new();

    for line in content.lines() {
        if line.trim() == "---" {
            if !current.trim().is_empty() {
                frames.push(current.trim_end().to_string());
                current.clear();
            }
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }

    if !current.trim().is_empty() {
        frames.push(current.trim_end().to_string());
    }

    if frames.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "El archivo no tiene frames",
        ));
    }

    Ok(frames)
}