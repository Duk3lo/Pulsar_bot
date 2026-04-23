use std::fs;
use std::sync::OnceLock;

const local_dir : &str = "workspace";

const Config : OnceLock<Workspace> = OnceLock::new();

