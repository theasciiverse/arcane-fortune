// Where Arcane Fortune finds its data and keeps the player's files.
//
// The game reads config/ and nn/ through paths relative to the working
// folder. Started from anywhere else (a launcher, or a package that puts the
// binary on PATH), it first moves to the folder its binary is in, if that is
// where config/ is.
//
// Saved games and the language preference are the player's, so they live in
// the player's state folder ($XDG_STATE_HOME/arcane-fortune, else
// ~/.local/state/arcane-fortune, ~/Library/Application Support/arcane-fortune
// on macOS, %LocalAppData%\arcane-fortune on Windows), not beside the game,
// where an update or a read-only install would lose or refuse them.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_data_dir() {
	if Path::new("config").is_dir() {return;}
	if let Ok(exe) = env::current_exe() {
		let exe = fs::canonicalize(&exe).unwrap_or(exe);
		if let Some(dir) = exe.parent() {
			if dir.join("config").is_dir() {
				let _ = env::set_current_dir(dir);
			}
		}
	}
}

fn state_dir() -> PathBuf {
	if let Some(d) = env::var_os("XDG_STATE_HOME").filter(|d| !d.is_empty()) {
		return PathBuf::from(d).join("arcane-fortune");
	}
	if cfg!(target_os = "windows") {
		if let Some(d) = env::var_os("LocalAppData") {
			return PathBuf::from(d).join("arcane-fortune");
		}
	}
	if let Some(h) = env::var_os("HOME").filter(|h| !h.is_empty()) {
		let h = PathBuf::from(h);
		if cfg!(target_os = "macos") {
			return h.join("Library").join("Application Support").join("arcane-fortune");
		}
		return h.join(".local").join("state").join("arcane-fortune");
	}
	PathBuf::from(".")
}

// the saved games folder, made if it is missing
pub fn save_dir() -> String {
	let d = state_dir().join("saves");
	let _ = fs::create_dir_all(&d);
	d.to_string_lossy().into_owned()
}

// the language preference the player chose; until they choose one, the
// default shipped in config/
pub fn lang_pref_read() -> String {
	let p = state_dir().join("current_lang_pref.txt");
	if p.is_file() {p.to_string_lossy().into_owned()} else {"config/current_lang_pref.txt".to_string()}
}

pub fn lang_pref_write() -> String {
	let d = state_dir();
	let _ = fs::create_dir_all(&d);
	d.join("current_lang_pref.txt").to_string_lossy().into_owned()
}
