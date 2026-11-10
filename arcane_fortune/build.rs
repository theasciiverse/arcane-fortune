use std::env::var;
use std::process::Command;

fn main() {
	// the commit shown in the version line: AF_COMMIT_ID when the build sets
	// it, else git's HEAD, else "unknown" (it was read from
	// .git/refs/heads/v0.4.0, which only a clone with that branch checked out has)
	let commit = std::env::var("AF_COMMIT_ID").ok().filter(|c| !c.is_empty()).or_else(|| {
		Command::new("git").args(&["rev-parse", "HEAD"]).output().ok()
			.filter(|o| o.status.success())
			.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
	}).unwrap_or_else(|| "unknown".to_string());
	println!("cargo:rustc-env=AF_COMMIT_ID={}", commit);
	println!("cargo:rerun-if-env-changed=AF_COMMIT_ID");
	
	//println!(r"cargo:rustc-link-search=link_dir/windows");
	//println!(r"cargo:rustc-link-search=link_dir/linux_musl");
	
	// attempt to get the binary to search in the current directory for the library
	//println!("cargo:rustc-flags=-Wl,-rpath,$ORIGIN");
	
	println!("cargo:rustc-env=TARGET={}", var("TARGET").unwrap());
	println!("cargo:rustc-env=PROFILE={}", var("PROFILE").unwrap());
	println!("cargo:rustc-env=HOST={}", var("HOST").unwrap());
	println!("cargo:rustc-env=OPT_LEVEL={}", var("OPT_LEVEL").unwrap());

	///// set rust version as environmental variable
	let mut rust_v = Command::new("rustc");
	rust_v.arg("-V");
	let txt = rust_v.output().expect("process failed to execute rustc");
	
	println!("cargo:rustc-env=RUSTV={}", String::from_utf8(txt.stdout).unwrap());
	
	//// build time
	let mut d = Command::new("date");
	let txt = d.output().expect("process failed to execute date");
	
	println!("cargo:rustc-env=COMPILE_DATE={}", String::from_utf8(txt.stdout).unwrap());
}

