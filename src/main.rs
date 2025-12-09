use std::fs;
use std::thread;
use std::time::Duration;
use std::env::home_dir;

use copypasta::{ClipboardContext, ClipboardProvider};

type Result<T> = std::result::Result<T, ()>;

fn main() -> Result<()> {
    let secret_path = match home_dir() {
	Some(home) => home.join("Secret/very_secret_password.txt"),
	None => {
	    eprintln!("ERROR: Home directory not found.");
	    std::process::exit(69);
	}
    };
    
    let content = fs::read_to_string(&secret_path)
	.map_err(|e| eprintln!("ERROR: Couldn't read {}: {}", secret_path.display(), e))?;
    let mut ctx = ClipboardContext::new()
	.map_err(|e| eprintln!("ERROR: Clipboard error: {e}"))?;
    ctx.set_contents(content.clone())
	.map_err(|e| eprintln!("ERROR: Clipboard copying error: {e}"))?;

    thread::sleep(Duration::from_secs(5));
    
    Ok(())
}
