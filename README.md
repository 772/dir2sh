![Logo](logo.png)

[![License: GPL3](https://img.shields.io/badge/License-GNU%20GPL-blue)](https://opensource.org/license/gpl-3-0)

## What is dir2sh?

This Rust tool simplifies copying a __small__ folder to a remote Linux machine by generating shell commands.

Designed for graphical environments (Linux/Windows), it bypasses the need for manual `scp`/`sftp`/`git` workflows by copying terminal commands directly to your clipboard.  

## Usage

1. Navigate to the folder you want to copy.  
2. Run `dir2sh` (no parameters or configuration needed).  
3. Paste the generated shell commands on the remote machine.  

**Example clipboard output:**
```bash
set +o history
mkdir -p "hello_world"
printf '%s' 'W3BhY2thZ2VdCm5hbWUgPSAiaGVsbG9fd29ybGQiCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyNCIKCltkZXBlbmRlbmNpZXNdCg==' | base64 -d > "hello_world/Cargo.toml"
mkdir -p "hello_world/src"
printf '%s' 'Zm4gbWFpbigpIHsKICAgIHByaW50bG4hKCJIZWxsbywgd29ybGQhIik7Cn0K' | base64 -d > "hello_world/src/main.rs"
set -o history
```

## Install bash command ```dir2sh``` on Linux:

Fedora:
```sudo dnf install xclip -y```

Debian:
```sudo apt install xclip -y```

```bash
sudo tee /usr/local/bin/dir2sh.sh > /dev/null << 'EOF'
#!/bin/bash
generate_commands() {
    local current_dir=$(pwd)
    local dir_name=$(basename "$current_dir")
    local commands="set +o history\n"
    commands+="mkdir -p \"$dir_name\"\n"
    while IFS= read -r -d '' file; do
        local relative_path="${file#$current_dir/}"
        local target_path="$dir_name/$relative_path"
        if [[ -d "$file" ]]; then
            commands+="mkdir -p \"$target_path\"\n"
        elif [[ -f "$file" ]]; then
            local encoded=$(base64 -w 0 "$file" 2>/dev/null || base64 -b 0 "$file")
            commands+="printf '%s' '$encoded' | base64 -d > \"$target_path\"\n"
        fi
    done < <(find "$current_dir" -mindepth 1 -print0 2>/dev/null)
    commands+="set -o history\n"
    echo -e "$commands"
}
if command -v xclip &> /dev/null; then
    generate_commands | xclip -selection clipboard
    echo "Commands copied to clipboard!"
else
    generate_commands
    echo "Error: Install xclip."
fi
EOF
sudo chmod +x /usr/local/bin/dir2sh.sh
```

## Install PowerShell command ```dir2sh``` on Microsoft Windows 11:

```PowerShell
$scriptFolder = "C:\dir2sh"
mkdir $scriptFolder -Force;
@'
function Generate-Commands {
    $currentDir = (Get-Location).Path
    $dirName = (Get-Item $currentDir).Name
    $cmds = @("set +o history")
    $cmds += "mkdir -p `"$dirName`""
    Get-ChildItem -Path . -Recurse | ForEach-Object {
        $relativePath = $_.FullName.Substring($currentDir.Length + 1)
        $targetPath = "$dirName/$relativePath".Replace('\', '/')
        
        if ($_.PSIsContainer) {
            $cmds += "mkdir -p `"$targetPath`""
        } else {
            $base64 = [Convert]::ToBase64String([IO.File]::ReadAllBytes($_.FullName))
            $cmds += "printf '%s' '$base64' | base64 -d > `"$targetPath`""
        }
    }
    $cmds += "set -o history"
    $cmds -join "`n"
}
try {
    Generate-Commands | Set-Clipboard
    Write-Host "Commands copied to clipboard!" -ForegroundColor Green
} catch {
    Write-Host "Fehler: $_" -ForegroundColor Red
}
'@ | Out-File "$scriptFolder/dir2sh.ps1"
[Environment]::SetEnvironmentVariable("PATH", "$env:PATH;$scriptFolder", "Machine")
```
