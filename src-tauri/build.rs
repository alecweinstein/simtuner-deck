use std::fs;
use std::path::Path;

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
	fs::create_dir_all(dst)?;
	for entry in fs::read_dir(src)?.flatten() {
		let to = dst.join(entry.file_name());
		if entry.file_type()?.is_dir() {
			copy_dir(&entry.path(), &to)?;
		} else {
			fs::copy(entry.path(), &to)?;
			#[cfg(unix)]
			{
				use std::os::unix::fs::PermissionsExt;
				let name = entry.file_name().to_string_lossy().to_string();
				let text =
					name.ends_with(".json") || name.ends_with(".png") || name.ends_with(".svg") || name.ends_with(".html") || name.ends_with(".css") || name.ends_with(".md") || name.ends_with(".txt");
				if !text {
					let mut perms = fs::metadata(&to)?.permissions();
					perms.set_mode(perms.mode() | 0o755);
					let _ = fs::set_permissions(&to, perms);
				}
			}
		}
	}
	Ok(())
}

fn install_prebuilt(src: &Path, name: &str) -> std::io::Result<()> {
	let out_dir = Path::new("target").join("plugins").join(name);
	if out_dir.exists() {
		fs::remove_dir_all(&out_dir)?;
	}
	copy_dir(src, &out_dir)
}

fn official_build() -> bool {
	matches!(
		std::env::var("SIMDECK_OFFICIAL").ok().as_deref().map(str::trim),
		Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
	)
}

fn main() {
	println!("cargo:rerun-if-changed=../plugins");
	println!("cargo:rerun-if-changed=../simtuner/plugin");
	println!("cargo:rerun-if-changed=../simtuner/akp153");
	println!("cargo:rerun-if-env-changed=SIMDECK_OFFICIAL");

	let source_plugins = (|| -> Result<(), std::io::Error> {
		for entry in fs::read_dir("../plugins")?.flatten() {
			if !entry.file_type()?.is_dir() || !entry.path().join("build.ts").exists() {
				continue;
			}
			let out_dir = Path::new("target").join("plugins").join(entry.file_name());
			fs::create_dir_all(&out_dir)?;
			let status = std::process::Command::new("deno")
				.args([
					"run",
					"--lock=target/deno.lock",
					"--allow-all",
					"build.ts",
					fs::canonicalize(&out_dir)?.to_string_lossy().as_ref(),
					&std::env::var("TARGET").unwrap(),
				])
				.current_dir(entry.path())
				.status()?;
			if !status.success() {
				panic!("Failed to build plugin {}: status code {}", entry.file_name().to_string_lossy(), status.code().unwrap());
			}
		}
		Ok(())
	})();

	if let Err(error) = source_plugins {
		#[cfg(debug_assertions)]
		eprintln!("Failed to build builtin plugins: {error}");
		#[cfg(not(debug_assertions))]
		panic!("Failed to build builtin plugins: {error}");
	}

	// The strip plugin is part of the private pack. A public/plain build omits it.
	// Official installers overlay it first and set SIMDECK_OFFICIAL=1 so a missing
	// copy cannot slip through.
	let official = official_build();
	let optional_prebuilt = [(
		"../simtuner/plugin/com.simtuner.golfdeck.strip.sdPlugin",
		"com.simtuner.golfdeck.strip.sdPlugin",
	)];
	for (src, name) in optional_prebuilt {
		if !Path::new(src).is_dir() {
			if official {
				panic!("Official build is missing required plugin {name} at {src}");
			}
			println!("cargo:warning=Skipping optional plugin {name}; {src} is not present");
			continue;
		}
		install_prebuilt(Path::new(src), name).unwrap_or_else(|error| {
			panic!("Failed to bundle builtin plugin {name}: {error}");
		});
	}

	// Prebuilt device plugin. CI builds the AKP153 Windows binary from the commit in
	// simtuner/akp153/PIN and places it in the folder below before this script runs.
	let required_prebuilt = [(
		"../simtuner/akp153/st.lynx.plugins.opendeck-akp153.sdPlugin",
		"st.lynx.plugins.opendeck-akp153.sdPlugin",
	)];
	for (src, name) in required_prebuilt {
		if let Err(error) = install_prebuilt(Path::new(src), name) {
			#[cfg(debug_assertions)]
			eprintln!("Failed to bundle builtin plugin {name}: {error}");
			#[cfg(not(debug_assertions))]
			panic!("Failed to bundle builtin plugin {name}: {error}");
		}
	}

	built::write_built_file().expect("failed to acquire build-time information");
	tauri_build::build();
}
