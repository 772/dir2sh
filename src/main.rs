use base64::Engine;
use std::{fs, io};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let commands = generate_commands()?;
    
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        clipboard.set_text(commands).unwrap();
    }
    Ok(())
}

fn generate_commands() -> io::Result<String> {
    let mut commands = String::new();
    let current_dir = std::env::current_dir()?;
    
    if let Some(dir_name) = current_dir.file_name() {
        commands.push_str(&format!("mkdir -p \"{}\"\n", dir_name.to_string_lossy()));
    }
    
    process_directory(&current_dir, &mut commands)?;
    Ok(commands)
}

fn process_directory(dir: &PathBuf, commands: &mut String) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(dir.parent().unwrap_or(dir)).unwrap();
        let target_path = relative_path.to_string_lossy().replace("\\", "/");

        if path.is_dir() {
            commands.push_str(&format!("mkdir -p \"{}\"\n", target_path));
            process_directory(&path, commands)?;
        } else if path.is_file() {
            let content = fs::read(&path)?;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&content);
            commands.push_str(&format!(
                "printf '%s' '{}' | base64 -d > \"{}\"\n",
                encoded, target_path
            ));
        }
    }
    Ok(())
}
