use std::fs;
use std::thread;
use std::time::Duration;
use std::env::home_dir;

use copypasta::{ClipboardContext, ClipboardProvider};
use anyhow::{anyhow, Context, Result};
    

fn main() -> Result<()> {
    let secret_path = home_dir()
	.context("[ERROR] Home directory not found!")?
	.join("Secret/very_secret_password.txt");

    let content = fs::read_to_string(&secret_path)
	.with_context(|| format!("[ERROR] Failed to read {}", secret_path.display()))?;
    
    let mut ctx = ClipboardContext::new()
	.map_err(|e| anyhow!("[ERROR] Failed to access clipboard: {}", e))?;
    
    ctx.set_contents(content.clone())
	.map_err(|e| anyhow!("[ERROR] Failed to copy to clipboard: {}", e))?;

    thread::sleep(Duration::from_secs(5));
    
    Ok(())
}
