# dir2sh

This Rust tool simplifies copying a folder to a remote machine by generating ready-to-use shell commands. Designed for graphical environments (Linux/Windows), it bypasses the need for manual `scp`/`sftp`/`git` workflows by copying terminal commands directly to your clipboard.  

## Usage

1. Navigate to the folder you want to copy.  
2. Run `dir2sh` (no parameters or configuration needed).  
3. Paste the generated shell commands on the remote machine.  

**Example clipboard output:**
```bash
mkdir -p "src"
printf '%s' 'dXNlIGJhc2U2NDo6RW5naW5lOwp1cCB9CiAgICBPaygoKSkKfQo=' | base64 -d > "src/main.rs"
```
