use std::{
    env, fs, io,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::OnceLock,
};

const APP_DIR: &str = "discord-bot";
const AUDIO_DIR: &str = "audio";
const ANIM_DIR: &str = "animations";
const AUDIO_FILE: &str = "tts.wav";
const DEFAULT_ANIM_FILE: &str = "animated_embed.txt";

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

        let ws = Self {
            path_app,
            folder_audio,
            folder_animations,
        };

        ws.ensure_default_animation_file()?;
        println!("Workspace verificado en: {:?}", ws.path_app);
        Ok(ws)
    }

    pub fn get_audio_file(&self) -> Option<PathBuf> {
        let audio_path = self.folder_audio.join(AUDIO_FILE);
        audio_path.is_file().then_some(audio_path)
    }

    pub fn get_animation_file(&self, name: &str) -> PathBuf {
        self.folder_animations.join(name)
    }

    pub fn get_default_animation_file(&self) -> PathBuf {
        self.get_animation_file(DEFAULT_ANIM_FILE)
    }

    fn ensure_default_animation_file(&self) -> io::Result<PathBuf> {
        let path = self.get_default_animation_file();

        if !path.exists() {
            let example = r#"┌───────────────┐
│   frame one   │
│   o     o     │
│       ^       │
└───────────────┘
---
┌───────────────┐
│   frame two   │
│    o   o      │
│      -        │
└───────────────┘
---
┌───────────────┐
│  frame three  │
│   o     o     │
│      ___      │
└───────────────┘
"#;

            fs::write(&path, example)?;
            println!("Archivo de ejemplo creado en: {:?}", path);
        }

        Ok(path)
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

pub fn load_fixed_frames_from_file(
    path: impl AsRef<Path>,
    lines_per_frame: usize,
) -> io::Result<Vec<String>> {
    if lines_per_frame == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "lines_per_frame no puede ser 0",
        ));
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut frames = Vec::new();
    let mut current = String::new();
    let mut line_count = 0usize;

    for line in reader.lines() {
        let line = line?;
        current.push_str(&line);
        current.push('\n');
        line_count += 1;

        if line_count == lines_per_frame {
            frames.push(current.trim_end().to_string());
            current.clear();
            line_count = 0;
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
