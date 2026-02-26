use crate::command;
use crate::config::{Config, Workspace};
use crate::window;
use crate::workspace;

pub fn execute(command: &str, config: &Config) {
    println!("Executing action: {}", command);
    match command {
        "switch-to-next-workspace" => switch_to_next_workspace(config),
        "switch-to-prev-workspace" => switch_to_prev_workspace(config),
        "move-to-next-workspace" => move_to_next_workspace(config),
        "move-to-prev-workspace" => move_to_prev_workspace(config),
        cmd if cmd.starts_with("move-to-workspace-") => {
            if let Some(num_str) = cmd.strip_prefix("move-to-workspace-") {
                move_to_workspace(config, num_str);
            }
        }
        cmd if cmd.starts_with("switch-to-workspace-") => {
            if let Some(num_str) = cmd.strip_prefix("switch-to-workspace-") {
                switch_to_workspace(config, num_str);
            }
        }
        _ => eprintln!("-> Unknown action: {}", command),
    }
}

fn switch_to_next_workspace(config: &Config) {
    println!("-> Switching to next workspace");
    if let Some(workspace_name) = workspace::get_active_workspace() {
        if let Some(next_ws_list) = get_adjacent_workspaces(config, &workspace_name, 1) {
            println!("-> Next workspace: {:?}", next_ws_list);
            for ws in next_ws_list {
                let _ = command::spawn_glazewm(&["command", "focus", "--workspace", &ws]);
            }
            if let Some(windows) = window::get_windows() {
                let _ = command::spawn_glazewm(&[
                    "command",
                    "focus",
                    "--container-id",
                    &windows.first().unwrap().id,
                ]);
            }
        }
    }
}

fn switch_to_prev_workspace(config: &Config) {
    println!("-> Switching to previous workspace");
    if let Some(workspace_name) = workspace::get_active_workspace() {
        if let Some(prev_ws_list) = get_adjacent_workspaces(config, &workspace_name, -1) {
            println!("-> Previous workspace: {:?}", prev_ws_list);
            for ws in prev_ws_list {
                let _ = command::spawn_glazewm(&["command", "focus", "--workspace", &ws]);
            }
            if let Some(windows) = window::get_windows() {
                let _ = command::spawn_glazewm(&[
                    "command",
                    "focus",
                    "--container-id",
                    &windows.first().unwrap().id,
                ]);
            }
        }
    }
}

fn move_to_next_workspace(config: &Config) {
    println!("-> Switching to next workspace");
    if let Some(workspace_name) = workspace::get_active_workspace() {
        if let Some(next_ws_list) = get_adjacent_workspaces(config, &workspace_name, 1) {
            if let Some(next_ws) = next_ws_list.first() {
                let _ = command::spawn_glazewm(&["command", "move", "--workspace", next_ws]);
            }
        }
    }
}

fn move_to_prev_workspace(config: &Config) {
    println!("-> Switching to previous workspace");
    if let Some(workspace_name) = workspace::get_active_workspace() {
        if let Some(prev_ws_list) = get_adjacent_workspaces(config, &workspace_name, -1) {
            if let Some(prev_ws) = prev_ws_list.first() {
                let _ = command::spawn_glazewm(&["command", "move", "--workspace", prev_ws]);
            }
        }
    }
}

fn move_to_workspace(config: &Config, workspace_name: &str) {
    println!("-> Moving to workspace {}", workspace_name);
    if let Some(workspace) = get_workspace_by_name(config, workspace_name) {
        let mut pos = 0;
        if let Some(active_ws_name) = workspace::get_active_workspace() {
            for w in &config.workspaces {
                if let Some(idx) = w.names.iter().position(|n| n == &active_ws_name) {
                    pos = idx;
                    break;
                }
            }
        }

        if let Some(target_name) = workspace.names.get(pos).or_else(|| workspace.names.first()) {
            let _ = command::spawn_glazewm(&["command", "move", "--workspace", target_name]);
        }
    }
}

fn switch_to_workspace(config: &Config, workspace_name: &str) {
    println!("-> Switching to workspace {}", workspace_name);
    if let Some(workspace) = get_workspace_by_name(config, workspace_name) {
        for name in workspace.names {
            let _ = command::spawn_glazewm(&["command", "focus", "--workspace", &name]);
        }
        if let Some(windows) = window::get_windows() {
            let _ = command::spawn_glazewm(&[
                "command",
                "focus",
                "--container-id",
                &windows.first().unwrap().id,
            ]);
        }
    }
}

fn get_workspace_by_name(config: &Config, name: &str) -> Option<Workspace> {
    config
        .workspaces
        .iter()
        .find(|workspace| workspace.name == name)
        .cloned()
}

fn get_adjacent_workspaces(
    config: &Config,
    current_name: &str,
    direction: i32,
) -> Option<Vec<String>> {
    let workspaces = &config.workspaces;
    if workspaces.is_empty() {
        return None;
    }

    let len = workspaces.len() as i32;
    let current_idx = workspaces
        .iter()
        .position(|w| w.names.iter().any(|n| n == current_name))?;

    let target_idx = (current_idx as i32 + direction).rem_euclid(len) as usize;
    Some(workspaces[target_idx].names.clone())
}
