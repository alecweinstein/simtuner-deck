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

fn main() {
	println!("cargo:rerun-if-changed=../plugins");
	println!("cargo:rerun-if-changed=../simtuner/plugin");
	println!("cargo:rerun-if-changed=../simtuner/akp153");

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

	// Prebuilt plugins. The AKP153 binaries are the v0.11.1 release of
	// alecweinstein/simtuner-deck-akp153 @ fc4116ab (same commit as 4ndv/opendeck-akp153 v0.11.1).
	let prebuilt = [
		("../simtuner/plugin/com.simtuner.golfdeck.strip.sdPlugin", "com.simtuner.golfdeck.strip.sdPlugin"),
		("../simtuner/akp153/st.lynx.plugins.opendeck-akp153.sdPlugin", "st.lynx.plugins.opendeck-akp153.sdPlugin"),
	];
	for (src, name) in prebuilt {
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
