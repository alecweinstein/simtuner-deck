//! Seed Golf Deck profiles the first time an Ajazz AKP153 / AKP153E is seen.

use crate::shared::{DeviceInfo, config_dir};

use std::fs;
use std::path::Path;

use anyhow::Context;
use tauri::Manager;

const GOLF_HOME: &str = "Golf Home";

/// Ajazz AKP153, AKP153E (0300:1010), and AKP153E rev. 2 (0300:3010).
/// Names come from the bundled opendeck-akp153 plugin (`Kind::human_name`).
pub fn is_golf_deck(device: &DeviceInfo) -> bool {
	device.id.starts_with("99-") && matches!(device.name.as_str(), "Ajazz AKP153" | "Ajazz AKP153E" | "Ajazz AKP153E (rev. 2)")
}

pub fn seed_if_needed(device: &DeviceInfo) {
	if !is_golf_deck(device) {
		return;
	}
	if let Err(error) = seed(device) {
		log::warn!("Golf Deck profile seed skipped for {}: {error:#}", device.id);
	}
}

fn seed(device: &DeviceInfo) -> Result<(), anyhow::Error> {
	if device.id.contains('/') || device.id.contains('\\') || device.id.contains("..") {
		anyhow::bail!("refusing to seed profiles for unsafe device id");
	}

	let profiles_root = config_dir().join("profiles");
	let dest = profiles_root.join(&device.id);
	if directory_has_profiles(&dest) {
		log::info!("Golf Deck profiles already present for {}; leaving them untouched", device.id);
		return Ok(());
	}

	let bundled = bundled_profile_dir()?;
	let staging = profiles_root.join(format!(".{}.golfdeck-seed", device.id));
	let _ = fs::remove_dir_all(&staging);
	fs::create_dir_all(&staging)?;

	let mut copied = 0u32;
	for entry in fs::read_dir(&bundled).with_context(|| format!("reading {}", bundled.display()))?.flatten() {
		let name = entry.file_name();
		let name_str = name.to_string_lossy();
		if !name_str.ends_with(".json") || !entry.file_type().is_ok_and(|kind| kind.is_file()) {
			continue;
		}
		let bytes = fs::read(entry.path())?;
		let output = match serde_json::from_slice::<serde_json::Value>(&bytes) {
			Ok(mut value) => {
				if rewrite_device_ids(&mut value, &device.id) {
					serde_json::to_vec_pretty(&value)?
				} else {
					bytes
				}
			}
			Err(error) => {
				log::warn!("Bundled profile {name_str} is not JSON ({error}); copying bytes unchanged");
				bytes
			}
		};
		fs::write(staging.join(&name), output)?;
		copied += 1;
	}

	if copied == 0 {
		let _ = fs::remove_dir_all(&staging);
		anyhow::bail!("bundled Golf Deck profiles directory was empty");
	}

	// Re-check so a profile saved while we were copying is not replaced.
	if directory_has_profiles(&dest) {
		let _ = fs::remove_dir_all(&staging);
		log::info!("Golf Deck profiles appeared for {} during seed; leaving them untouched", device.id);
		return Ok(());
	}
	if dest.exists() {
		fs::remove_dir_all(&dest)?;
	}
	fs::rename(&staging, &dest)?;
	select_golf_home(&profiles_root, &device.id, &dest)?;
	log::info!("Seeded {copied} Golf Deck profiles for {} and selected {GOLF_HOME}", device.id);
	Ok(())
}

fn select_golf_home(profiles_root: &Path, device_id: &str, dest: &Path) -> Result<(), anyhow::Error> {
	let selection = profiles_root.join(format!("{device_id}.json"));
	if !selection.exists() {
		let body = serde_json::to_string_pretty(&serde_json::json!({ "selected_profile": GOLF_HOME }))?;
		fs::write(&selection, body + "\n")?;
		return Ok(());
	}

	let Ok(bytes) = fs::read(&selection) else {
		return Ok(());
	};
	let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
		return Ok(());
	};
	let current = value.get("selected_profile").and_then(|v| v.as_str()).unwrap_or("");
	if dest.join(format!("{current}.json")).exists() {
		return Ok(());
	}
	value["selected_profile"] = serde_json::Value::String(GOLF_HOME.to_owned());
	fs::write(&selection, serde_json::to_string_pretty(&value)? + "\n")?;
	Ok(())
}

fn bundled_profile_dir() -> Result<std::path::PathBuf, anyhow::Error> {
	let app = crate::APP_HANDLE.get().context("application handle is not set")?;
	let path = app
		.path()
		.resolve("golfdeck-profiles", tauri::path::BaseDirectory::Resource)
		.context("Golf Deck profiles are not bundled")?;
	if !path.is_dir() {
		anyhow::bail!("bundled Golf Deck profiles missing at {}", path.display());
	}
	Ok(path)
}

fn directory_has_profiles(dir: &Path) -> bool {
	let Ok(entries) = fs::read_dir(dir) else {
		return false;
	};
	for entry in entries.flatten() {
		let name = entry.file_name().to_string_lossy().to_lowercase();
		if name.ends_with(".json") || name.ends_with(".json.bak") || name.ends_with(".json.temp") {
			return true;
		}
		if entry.path().is_dir() && directory_has_profiles(&entry.path()) {
			return true;
		}
	}
	false
}

fn looks_like_device_id(value: &str) -> bool {
	let Some((ns, rest)) = value.split_once('-') else {
		return false;
	};
	ns.len() == 2 && ns.chars().all(|c| c.is_ascii_hexdigit()) && !rest.is_empty() && !value.contains("--") && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Portable profile files use `Keypad.0.0`. A context copied from a live device is
/// `device.profile.controller.position.index`. Rewrite only that device segment.
fn rewrite_context(context: &str, device_id: &str) -> Option<String> {
	let parts: Vec<&str> = context.split('.').collect();
	if parts.len() != 5 || !matches!(parts[2], "Keypad" | "Encoder" | "Infobar") {
		return None;
	}
	if parts[0] == device_id || !looks_like_device_id(parts[0]) {
		return None;
	}
	Some(format!("{device_id}.{}.{}.{}.{}", parts[1], parts[2], parts[3], parts[4]))
}

fn rewrite_device_ids(value: &mut serde_json::Value, device_id: &str) -> bool {
	let mut changed = false;
	match value {
		serde_json::Value::Object(map) => {
			if let Some(context) = map.get("context").and_then(|v| v.as_str()).map(str::to_owned)
				&& let Some(rewritten) = rewrite_context(&context, device_id)
			{
				map.insert("context".to_owned(), serde_json::Value::String(rewritten));
				changed = true;
			}
			if let Some(device) = map.get("device").and_then(|v| v.as_str()).map(str::to_owned)
				&& looks_like_device_id(&device)
				&& device != device_id
			{
				map.insert("device".to_owned(), serde_json::Value::String(device_id.to_owned()));
				changed = true;
			}
			for child in map.values_mut() {
				changed |= rewrite_device_ids(child, device_id);
			}
		}
		serde_json::Value::Array(items) => {
			for child in items {
				changed |= rewrite_device_ids(child, device_id);
			}
		}
		_ => {}
	}
	changed
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rewrites_full_context_and_device_setting() {
		let mut value = serde_json::json!({
			"context": "99-355499441494-153E.Golf Home.Keypad.0.0",
			"settings": { "device": "99-OLD", "profile": "GSPro" },
			"children": [{ "context": "99-355499441494-153E.GSPro 2.Encoder.1.0" }]
		});
		assert!(rewrite_device_ids(&mut value, "99-NEW"));
		assert_eq!(value["context"], "99-NEW.Golf Home.Keypad.0.0");
		assert_eq!(value["settings"]["device"], "99-NEW");
		assert_eq!(value["settings"]["profile"], "GSPro");
		assert_eq!(value["children"][0]["context"], "99-NEW.GSPro 2.Encoder.1.0");
	}

	#[test]
	fn leaves_portable_profiles_alone() {
		let original = serde_json::json!({
			"context": "Keypad.0.0",
			"settings": { "device": null, "profile": "GSPro" },
			"image": "data:image/png;base64,aaaa"
		});
		let mut value = original.clone();
		assert!(!rewrite_device_ids(&mut value, "99-NEW"));
		assert_eq!(value, original);
	}

	#[test]
	fn recognizes_golf_deck_names_only() {
		let deck = |name: &str| DeviceInfo {
			id: "99-355499441494-153E".to_owned(),
			plugin: String::new(),
			name: name.to_owned(),
			rows: 3,
			columns: 6,
			encoders: 0,
			touchpoints: 0,
			infobars: 0,
			r#type: 0,
		};
		assert!(is_golf_deck(&deck("Ajazz AKP153")));
		assert!(is_golf_deck(&deck("Ajazz AKP153E")));
		assert!(is_golf_deck(&deck("Ajazz AKP153E (rev. 2)")));
		assert!(!is_golf_deck(&deck("Ajazz AKP153R")));
		assert!(!is_golf_deck(&deck("Ajazz AKP153R (rev. 2)")));
		let mut other = deck("Ajazz AKP153E");
		other.id = "sd-123".to_owned();
		assert!(!is_golf_deck(&other));
	}
}
