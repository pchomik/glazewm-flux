use serde_json::Value;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
}

pub fn get_windows() -> Option<Vec<Window>> {
    let output = Command::new("glazewm.exe")
        .args(&["query", "windows"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    // Parse the JSON output
    let json: Value = serde_json::from_slice(&output.stdout).ok()?;

    // Traverse the workspace array
    let windows = json.get("data")?.get("windows")?.as_array()?;
    let mut windows_list = Vec::new();

    for window in windows {
        let window_type = window
            .get("type")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        if window_type != "window" {
            continue;
        }

        let display_state = window
            .get("displayState")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        if display_state != "shown" {
            continue;
        }

        let window_id = window
            .get("id")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();

        let window = Window { id: window_id };

        windows_list.push(window);
    }

    println!("Windows: {:?}", windows_list);

    if windows_list.is_empty() {
        None
    } else {
        Some(windows_list)
    }
}
