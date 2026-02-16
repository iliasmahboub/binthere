use std::collections::HashSet;

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
pub fn installed_program_names() -> HashSet<String> {
    let mut names = HashSet::new();

    let roots = [
        (HKEY_LOCAL_MACHINE, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
        (
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        ),
        (HKEY_CURRENT_USER, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
    ];

    for (root_hive, subkey_path) in roots {
        let root = RegKey::predef(root_hive);
        if let Ok(uninstall) = root.open_subkey(subkey_path) {
            for key_name in uninstall.enum_keys().flatten() {
                if let Ok(entry) = uninstall.open_subkey(key_name) {
                    if let Ok(display_name) = entry.get_value::<String, _>("DisplayName") {
                        let normalized = display_name.trim().to_ascii_lowercase();
                        if normalized.len() >= 4 {
                            names.insert(normalized);
                        }
                    }
                }
            }
        }
    }

    names
}

#[cfg(not(windows))]
pub fn installed_program_names() -> HashSet<String> {
    HashSet::new()
}

pub fn find_program_match(file_stem: &str, installed_names: &HashSet<String>) -> Option<String> {
    let candidate = file_stem.to_ascii_lowercase();

    installed_names.iter().find_map(|name| {
        let short = name.len() < 5;
        if !short && (candidate.contains(name) || name.contains(&candidate)) {
            Some(name.clone())
        } else {
            None
        }
    })
}
