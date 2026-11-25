//! Seed Golf Deck profiles the first time an Ajazz AKP153 / AKP153E is seen.

use crate::shared::{DeviceInfo, config_dir};

use std::fs;
use std::path::Path;

use anyhow::Context;
use tauri::Manager;

const GOLF_HOME: &str = "Golf Home";
const GOLF_HOME_FILE: &str = "Golf Home.json";
/// Bump when the bundled profile pack changes so devices re-run the sync.
/// 1 was Sim Deck 2.14.3 (7 pages). 2 was 2.14.4 (byte hash, which never matched a resaved file).
/// 3 compares Golf Home semantically and fills the new sim keys.
const PROFILE_SET: u32 = 3;
/// Signature of the unmodified Golf Home shipped in Sim Deck 2.14.3 (action
/// UUIDs and settings only; no key artwork). Used to recognize a stock 2.14.3
/// page after OpenDeck has re-saved it.
const GOLF_HOME_V1: &str = include_str!("../assets/golf-home-2.14.3.json");
const SWITCH_PROFILE_UUID: &str = "com.amansprojects.starterpack.switchprofile";
/// Profile names that were already on Golf Home in 2.14.3. Newer switch keys are the ones we merge in.
const V1_HOME_PROFILES: &[&str] = &["GSPro", "PurePlay", "Muni Golf", "Sim Tuner"];

/// Ajazz AKP153, AKP153E (0300:1010), and AKP153E rev. 2 (0300:3010).
/// The bundled plugin reports these as "Sim Deck". The older Ajazz names still match.
/// The profile folder is the device id (`99-…`), which the rename does not change.
pub fn is_golf_deck(device: &DeviceInfo) -> bool {
	device.id.starts_with("99-") && matches!(device.name.as_str(), "Sim Deck" | "Ajazz AKP153" | "Ajazz AKP153E" | "Ajazz AKP153E (rev. 2)")
}

pub async fn seed_if_needed(device: &DeviceInfo) {
	if !is_golf_deck(device) {
		return;
	}
	if let Err(error) = seed(device).await {
		log::warn!("Golf Deck profile seed skipped for {}: {error:#}", device.id);
	}
}

async fn seed(device: &DeviceInfo) -> Result<(), anyhow::Error> {
	if device.id.contains('/') || device.id.contains('\\') || device.id.contains("..") {
		anyhow::bail!("refusing to seed profiles for unsafe device id");
	}

	let Some(bundled) = bundled_profile_dir() else {
		log::info!("No Golf Deck profile pack in this build; skipping seed for {}", device.id);
		return Ok(());
	};

	let profiles_root = config_dir().join("profiles");
	let dest = profiles_root.join(&device.id);
	let applied = read_applied_set(&device.id);
	if directory_has_profiles(&dest) {
		if applied >= PROFILE_SET {
			log::info!("Golf Deck profile set {PROFILE_SET} already applied for {}", device.id);
			return Ok(());
		}
		let report = sync_missing_profiles(&dest, &bundled, &device.id)?;
		forget_loaded_profile(&device.id, GOLF_HOME).await;
		write_applied_set(&device.id, PROFILE_SET)?;
		log::info!(
			"Golf Deck profile set {PROFILE_SET} for {}: added [{}], Golf Home {}",
			device.id,
			report.added.join(", "),
			report.golf_home
		);
		return Ok(());
	}

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
		let report = sync_missing_profiles(&dest, &bundled, &device.id)?;
		forget_loaded_profile(&device.id, GOLF_HOME).await;
		write_applied_set(&device.id, PROFILE_SET)?;
		log::info!(
			"Golf Deck profiles appeared for {} during seed; added [{}], Golf Home {}",
			device.id,
			report.added.join(", "),
			report.golf_home
		);
		return Ok(());
	}
	if dest.exists() {
		fs::remove_dir_all(&dest)?;
	}
	fs::rename(&staging, &dest)?;
	select_golf_home(&profiles_root, &device.id, &dest)?;
	forget_loaded_profile(&device.id, GOLF_HOME).await;
	write_applied_set(&device.id, PROFILE_SET)?;
	log::info!("Seeded {copied} Golf Deck profiles for {} and selected {GOLF_HOME}", device.id);
	Ok(())
}

struct SyncReport {
	added: Vec<String>,
	golf_home: &'static str,
}

fn sync_missing_profiles(dest: &Path, bundled: &Path, device_id: &str) -> Result<SyncReport, anyhow::Error> {
	fs::create_dir_all(dest)?;
	let mut added = Vec::new();
	let mut golf_home = "unchanged";
	for entry in fs::read_dir(bundled).with_context(|| format!("reading {}", bundled.display()))?.flatten() {
		let name = entry.file_name();
		let name_str = name.to_string_lossy();
		if !name_str.ends_with(".json") || !entry.file_type().is_ok_and(|kind| kind.is_file()) {
			continue;
		}
		let prepared = prepare_profile_bytes(&fs::read(entry.path())?, device_id, &name_str);
		let target = dest.join(&name);
		if name_str == GOLF_HOME_FILE {
			golf_home = reconcile_golf_home(&target, &prepared, device_id, &mut added, &name_str)?;
			continue;
		}
		if target.exists() {
			continue;
		}
		fs::write(&target, prepared)?;
		added.push(name_str.into_owned());
	}
	added.sort();
	Ok(SyncReport { added, golf_home })
}

fn reconcile_golf_home(target: &Path, bundled_bytes: &[u8], device_id: &str, added: &mut Vec<String>, name: &str) -> Result<&'static str, anyhow::Error> {
	if !target.exists() {
		fs::write(target, bundled_bytes)?;
		added.push(name.to_owned());
		return Ok("added");
	}

	let existing = fs::read(target)?;
	let Ok(on_disk) = serde_json::from_slice::<serde_json::Value>(&existing) else {
		log::warn!("Golf Home for {device_id} is not JSON; leaving it in place");
		return Ok("kept");
	};
	let Ok(bundled) = serde_json::from_slice::<serde_json::Value>(bundled_bytes) else {
		log::warn!("Bundled Golf Home is not JSON; leaving the on-disk copy in place");
		return Ok("kept");
	};
	let v1 = serde_json::from_str::<serde_json::Value>(GOLF_HOME_V1).context("embedded 2.14.3 Golf Home is not JSON")?;

	if golf_home_matches(&on_disk, &bundled) {
		return Ok("unchanged");
	}
	if golf_home_matches(&on_disk, &v1) {
		fs::write(target, bundled_bytes)?;
		log::info!("Replaced Golf Home for {device_id} with the bundled layout; it still matched Sim Deck 2.14.3");
		return Ok("replaced");
	}

	let mut merged = on_disk;
	let positions = merge_new_sim_keys(&mut merged, &bundled);
	if positions.is_empty() {
		log::info!("Kept user-edited Golf Home for {device_id}; no empty slots for the new sim keys");
		return Ok("kept");
	}
	rewrite_device_ids(&mut merged, device_id);
	fs::write(target, serde_json::to_vec_pretty(&merged)?)?;
	log::info!("Kept user-edited Golf Home for {device_id} and merged new sim keys at positions {positions:?}");
	Ok("merged")
}

/// Drop a cached profile so the next load reads the file we just wrote.
async fn forget_loaded_profile(device_id: &str, profile: &str) {
	crate::store::profiles::PROFILE_STORES.write().await.remove_profile(device_id, profile);
}

#[derive(Debug, PartialEq, Eq)]
struct SlotSig {
	uuid: String,
	profile: String,
	down: String,
	up: String,
	children: usize,
}

fn golf_home_matches(left: &serde_json::Value, right: &serde_json::Value) -> bool {
	slot_signatures(left, "keys") == slot_signatures(right, "keys")
		&& slot_signatures(left, "sliders") == slot_signatures(right, "sliders")
		&& slot_signatures(left, "infobars") == slot_signatures(right, "infobars")
}

fn slot_signatures(profile: &serde_json::Value, field: &str) -> Vec<Option<SlotSig>> {
	let Some(slots) = profile.get(field).and_then(|value| value.as_array()) else {
		return Vec::new();
	};
	let mut signatures: Vec<Option<SlotSig>> = slots.iter().map(slot_signature).collect();
	while signatures.last().is_some_and(|slot| slot.is_none()) {
		signatures.pop();
	}
	signatures
}

fn slot_signature(slot: &serde_json::Value) -> Option<SlotSig> {
	let uuid = slot.get("action").and_then(|action| action.get("uuid")).and_then(|uuid| uuid.as_str()).unwrap_or("").trim();
	if uuid.is_empty() {
		return None;
	}
	let settings = slot.get("settings").cloned().unwrap_or(serde_json::Value::Null);
	Some(SlotSig {
		uuid: uuid.to_owned(),
		profile: setting_text(&settings, "profile"),
		down: setting_text(&settings, "down"),
		up: setting_text(&settings, "up"),
		children: slot.get("children").and_then(|children| children.as_array()).map(|children| children.len()).unwrap_or(0),
	})
}

fn setting_text(settings: &serde_json::Value, key: &str) -> String {
	match settings.get(key) {
		Some(serde_json::Value::String(text)) => text.trim().to_owned(),
		Some(serde_json::Value::Null) | None => String::new(),
		Some(other) => other.to_string(),
	}
}

fn is_empty_slot(slot: &serde_json::Value) -> bool {
	slot_signature(slot).is_none()
}

fn is_new_sim_switch(slot: &serde_json::Value) -> bool {
	let Some(sig) = slot_signature(slot) else {
		return false;
	};
	sig.uuid == SWITCH_PROFILE_UUID && !sig.profile.is_empty() && !V1_HOME_PROFILES.contains(&sig.profile.as_str())
}

/// Copy bundled Switch Profile keys for sims that were not on the 2.14.3 Golf Home into slots the user left empty.
fn merge_new_sim_keys(user: &mut serde_json::Value, bundled: &serde_json::Value) -> Vec<usize> {
	let Some(bundled_keys) = bundled.get("keys").and_then(|keys| keys.as_array()) else {
		return Vec::new();
	};
	if !user.get("keys").is_some_and(|keys| keys.is_array()) {
		user["keys"] = serde_json::json!([]);
	}
	let user_keys = user.get_mut("keys").and_then(|keys| keys.as_array_mut()).unwrap();
	while user_keys.len() < bundled_keys.len() {
		user_keys.push(serde_json::Value::Null);
	}
	let mut positions = Vec::new();
	for (index, bundled_slot) in bundled_keys.iter().enumerate() {
		if !is_empty_slot(&user_keys[index]) || !is_new_sim_switch(bundled_slot) {
			continue;
		}
		user_keys[index] = bundled_slot.clone();
		positions.push(index);
	}
	positions
}

fn prepare_profile_bytes(bytes: &[u8], device_id: &str, name: &str) -> Vec<u8> {
	match serde_json::from_slice::<serde_json::Value>(bytes) {
		Ok(mut value) => {
			if rewrite_device_ids(&mut value, device_id) {
				serde_json::to_vec_pretty(&value).unwrap_or_else(|_| bytes.to_vec())
			} else {
				bytes.to_vec()
			}
		}
		Err(error) => {
			log::warn!("Bundled profile {name} is not JSON ({error}); copying bytes unchanged");
			bytes.to_vec()
		}
	}
}

fn seed_state_path() -> std::path::PathBuf {
	config_dir().join("golfdeck-seed.json")
}

fn read_applied_set(device_id: &str) -> u32 {
	let Ok(bytes) = fs::read(seed_state_path()) else {
		return 0;
	};
	let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
		return 0;
	};
	value.get("devices").and_then(|devices| devices.get(device_id)).and_then(|version| version.as_u64()).unwrap_or(0) as u32
}

fn write_applied_set(device_id: &str, set: u32) -> Result<(), anyhow::Error> {
	let path = seed_state_path();
	let mut root = fs::read(&path)
		.ok()
		.and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
		.unwrap_or_else(|| serde_json::json!({ "devices": {} }));
	if !root.get("devices").is_some_and(|devices| devices.is_object()) {
		root["devices"] = serde_json::json!({});
	}
	root["devices"][device_id] = serde_json::json!(set);
	root["profile_set"] = serde_json::json!(PROFILE_SET);
	fs::write(&path, serde_json::to_string_pretty(&root)? + "\n")?;
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

fn bundled_profile_dir() -> Option<std::path::PathBuf> {
	let app = crate::APP_HANDLE.get()?;
	let path = app
		.path()
		.resolve("golfdeck-profiles", tauri::path::BaseDirectory::Resource)
		.ok()?;
	path.is_dir().then_some(path)
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
		assert!(is_golf_deck(&deck("Sim Deck")));
		assert!(is_golf_deck(&deck("Ajazz AKP153")));
		assert!(is_golf_deck(&deck("Ajazz AKP153E")));
		assert!(is_golf_deck(&deck("Ajazz AKP153E (rev. 2)")));
		assert!(!is_golf_deck(&deck("Ajazz AKP153R")));
		assert!(!is_golf_deck(&deck("Ajazz AKP153R (rev. 2)")));
		let mut other = deck("Ajazz AKP153E");
		other.id = "sd-123".to_owned();
		assert!(!is_golf_deck(&other));
	}

	fn resaved_v1_golf_home() -> serde_json::Value {
		let mut home: serde_json::Value = serde_json::from_str(GOLF_HOME_V1).unwrap();
		rewrite_device_ids(&mut home, "99-355499441494-153E");
		let state = &mut home["keys"][0]["states"][0];
		state["opendeck_extra"] = serde_json::json!(true);
		state["background_colour"] = serde_json::json!("#000000");
		home["keys"][0]["settings"]["device"] = serde_json::json!("99-355499441494-153E");
		home
	}

	/// Signature-only 2.14.4+ Golf Home: Switch Profile keys at slots 4, 6, and 10
	/// are the new sims merged into a user-edited 2.14.3 page.
	fn bundled_v2_golf_home() -> serde_json::Value {
		let switch = |profile: &str| {
			serde_json::json!({
				"action": { "uuid": SWITCH_PROFILE_UUID },
				"settings": { "profile": profile }
			})
		};
		let mut keys = vec![serde_json::Value::Null; 11];
		keys[0] = switch("GSPro");
		keys[1] = switch("PurePlay");
		keys[2] = switch("Muni Golf");
		keys[3] = switch("Sim Tuner");
		keys[4] = switch("Awesome Golf");
		keys[6] = switch("TGC 2019");
		keys[10] = switch("Infinite Tees");
		serde_json::json!({ "keys": keys })
	}

	#[test]
	fn v1_golf_home_fixture_has_no_embedded_artwork() {
		assert!(!GOLF_HOME_V1.contains("data:image"), "2.14.3 Golf Home fixture must not include key artwork");
		let home: serde_json::Value = serde_json::from_str(GOLF_HOME_V1).unwrap();
		assert_eq!(home["keys"][0]["settings"]["profile"], "GSPro");
		assert_eq!(home["keys"][1]["settings"]["profile"], "PurePlay");
		assert_eq!(home["keys"][2]["settings"]["profile"], "Muni Golf");
		assert_eq!(home["keys"][3]["settings"]["profile"], "Sim Tuner");
	}

	#[test]
	fn resaved_v1_golf_home_is_replaced() {
		let root = std::env::temp_dir().join(format!("simdeck-seed-replace-{}", std::process::id()));
		let _ = fs::remove_dir_all(&root);
		let bundled = root.join("bundled");
		let dest = root.join("device");
		fs::create_dir_all(&bundled).unwrap();
		fs::create_dir_all(&dest).unwrap();
		let v2 = br#"{"keys":[{"action":{"uuid":"com.amansprojects.starterpack.switchprofile"},"settings":{"profile":"Awesome Golf"}}]}"#;
		fs::write(bundled.join("Golf Home.json"), v2).unwrap();
		fs::write(bundled.join("Awesome Golf.json"), br#"{"page":"awesome"}"#).unwrap();
		fs::write(dest.join("Golf Home.json"), serde_json::to_vec_pretty(&resaved_v1_golf_home()).unwrap()).unwrap();
		fs::write(dest.join("GSPro.json"), br#"{"page":"kept"}"#).unwrap();

		let report = sync_missing_profiles(&dest, &bundled, "99-355499441494-153E").unwrap();
		assert_eq!(report.golf_home, "replaced");
		assert_eq!(fs::read(dest.join("Golf Home.json")).unwrap(), v2);
		assert_eq!(fs::read(dest.join("Awesome Golf.json")).unwrap(), br#"{"page":"awesome"}"#);
		assert_eq!(fs::read(dest.join("GSPro.json")).unwrap(), br#"{"page":"kept"}"#);
		assert!(report.added.iter().any(|name| name == "Awesome Golf.json"));
		let _ = fs::remove_dir_all(&root);
	}

	#[test]
	fn edited_golf_home_is_kept_and_gains_new_sim_keys() {
		let root = std::env::temp_dir().join(format!("simdeck-seed-merge-{}", std::process::id()));
		let _ = fs::remove_dir_all(&root);
		let bundled = root.join("bundled");
		let dest = root.join("device");
		fs::create_dir_all(&bundled).unwrap();
		fs::create_dir_all(&dest).unwrap();
		let mut edited = resaved_v1_golf_home();
		edited["keys"][0]["settings"]["profile"] = serde_json::json!("Custom Page");
		fs::write(dest.join("Golf Home.json"), serde_json::to_vec_pretty(&edited).unwrap()).unwrap();

		let v2 = bundled_v2_golf_home();
		fs::write(bundled.join("Golf Home.json"), serde_json::to_vec(&v2).unwrap()).unwrap();
		fs::write(bundled.join("Infinite Tees.json"), br#"{"page":"tees"}"#).unwrap();

		let report = sync_missing_profiles(&dest, &bundled, "99-355499441494-153E").unwrap();
		assert_eq!(report.golf_home, "merged");
		let saved: serde_json::Value = serde_json::from_slice(&fs::read(dest.join("Golf Home.json")).unwrap()).unwrap();
		assert_eq!(saved["keys"][0]["settings"]["profile"], "Custom Page");
		assert_eq!(saved["keys"][4]["settings"]["profile"], "Awesome Golf");
		assert_eq!(saved["keys"][6]["settings"]["profile"], "TGC 2019");
		assert_eq!(saved["keys"][10]["settings"]["profile"], "Infinite Tees");
		assert!(saved["keys"][5].is_null());
		assert!(dest.join("Infinite Tees.json").is_file());
		let _ = fs::remove_dir_all(&root);
	}

	#[test]
	fn upgrade_adds_missing_profiles_and_keeps_modified_golf_home() {
		let root = std::env::temp_dir().join(format!("simdeck-seed-{}", std::process::id()));
		let _ = fs::remove_dir_all(&root);
		let bundled = root.join("bundled");
		let dest = root.join("device");
		fs::create_dir_all(&bundled).unwrap();
		fs::create_dir_all(&dest).unwrap();
		fs::write(bundled.join("Golf Home.json"), br#"{"keys":["new-home"]}"#).unwrap();
		fs::write(bundled.join("Awesome Golf.json"), br#"{"keys":["awesome"]}"#).unwrap();
		fs::write(bundled.join("GSPro.json"), br#"{"keys":["gspro-new"]}"#).unwrap();
		fs::write(dest.join("Golf Home.json"), br#"{"keys":[{"action":{"uuid":"custom.action"},"settings":{"profile":"Nope"}}]}"#).unwrap();
		fs::write(dest.join("GSPro.json"), br#"{"keys":["user-gspro"]}"#).unwrap();

		let report = sync_missing_profiles(&dest, &bundled, "99-TEST").unwrap();
		assert_eq!(report.added, vec!["Awesome Golf.json".to_owned()]);
		assert_eq!(report.golf_home, "kept");
		assert_eq!(
			fs::read(dest.join("Golf Home.json")).unwrap(),
			br#"{"keys":[{"action":{"uuid":"custom.action"},"settings":{"profile":"Nope"}}]}"#
		);
		assert_eq!(fs::read(dest.join("GSPro.json")).unwrap(), br#"{"keys":["user-gspro"]}"#);
		assert_eq!(fs::read(dest.join("Awesome Golf.json")).unwrap(), br#"{"keys":["awesome"]}"#);
		let _ = fs::remove_dir_all(&root);
	}

	#[test]
	fn upgrade_leaves_golf_home_when_it_already_matches_the_bundle() {
		let root = std::env::temp_dir().join(format!("simdeck-seed-same-{}", std::process::id()));
		let _ = fs::remove_dir_all(&root);
		let bundled = root.join("bundled");
		let dest = root.join("device");
		fs::create_dir_all(&bundled).unwrap();
		fs::create_dir_all(&dest).unwrap();
		let home = br#"{"page":"golf-home-v2"}"#;
		fs::write(bundled.join("Golf Home.json"), home).unwrap();
		fs::write(dest.join("Golf Home.json"), home).unwrap();
		fs::write(bundled.join("Infinite Tees.json"), br#"{"page":"tees"}"#).unwrap();

		let report = sync_missing_profiles(&dest, &bundled, "99-TEST").unwrap();
		assert_eq!(report.golf_home, "unchanged");
		assert_eq!(report.added, vec!["Infinite Tees.json".to_owned()]);
		assert_eq!(fs::read(dest.join("Golf Home.json")).unwrap(), home);
		let _ = fs::remove_dir_all(&root);
	}
}
