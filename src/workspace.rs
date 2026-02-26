use serde_json::Value;
use std::process::Command;

/// Queries glazewm for the workspaces to find the currently active workspace.
/// This acts as a fallback when `get_focused_window` returns `None` (e.g. on an empty desktop).
pub fn get_active_workspace() -> Option<String> {
    // Execute glazewm query
    let output = Command::new("glazewm.exe")
        .args(&["query", "workspaces"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    // Parse the JSON output
    let json: Value = serde_json::from_slice(&output.stdout).ok()?;

    // Traverse the workspace array
    let workspaces = json.get("data")?.get("workspaces")?.as_array()?;

    for ws in workspaces {
        if ws.get("hasFocus").and_then(|b| b.as_bool()) == Some(true) {
            return ws
                .get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_string());
        }
    }

    None
}
