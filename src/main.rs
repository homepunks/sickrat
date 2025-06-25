use std::fs;
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::env::home_dir;

use copypasta::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), Box<dyn Error>> {
    let path = home_dir()
	.ok_or("home directory not found!")?
	.join("Programming/token.txt");
    
    let content = fs::read_to_string(&path)
	.map_err(|e| format!("couldn't read {}: {}", path.display(), e))?;

    let mut ctx = ClipboardContext::new()
	.map_err(|e| format!("clipboard error: {e}"))?;
    ctx.set_contents(content.clone())
	.map_err(|e| format!("clipboard copying error: {e}"))?;

    thread::sleep(Duration::from_secs(5));
    
    Ok(())
}
