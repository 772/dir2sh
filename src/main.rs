use base64::Engine;
use std::path::Path;
use std::{fs, io};

fn main() -> io::Result<()> {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        clipboard.set_text(generate_commands()?).unwrap();
    }
    Ok(())
}

fn generate_commands() -> io::Result<String> {
    let current_dir = std::env::current_dir()?;
    let dir_name = current_dir.file_name().unwrap().to_string_lossy();
    let mut commands = String::from("set +o history\n");
    commands.push_str(&format!("mkdir -p \"{dir_name}\"\n"));
    process_directory(&current_dir, &current_dir, &mut commands)?;
    commands.push_str("set -o history\n");
    Ok(commands)
}

fn process_directory(base_dir: &Path, current_dir: &Path, commands: &mut String) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_dir).unwrap();
        let target_path = format!(
            "{}/{}",
            base_dir.file_name().unwrap().to_string_lossy(),
            relative_path.display().to_string().replace("\\", "/")
        );
        if path.is_dir() {
            commands.push_str(&format!("mkdir -p \"{target_path}\"\n"));
            process_directory(base_dir, &path, commands)?;
        } else if path.is_file() {
            let encoded = base64::engine::general_purpose::STANDARD.encode(fs::read(&path)?);
            commands.push_str(&format!(
                "printf '%s' '{encoded}' | base64 -d > \"{target_path}\"\n"
            ));
        }
    }
    Ok(())
}
