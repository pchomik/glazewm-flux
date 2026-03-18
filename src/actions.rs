use crate::client::WsClient;
use crate::config::{Config, Workspace};
use crate::window;
use crate::workspace;
use std::error::Error;

pub fn execute(command: &str, config: &Config) {
    println!("Executing action: {}", command);
    let result = match command {
        "switch-to-next-workspace" => switch_to_next_workspace(config),
        "switch-to-prev-workspace" => switch_to_prev_workspace(config),
        "move-to-next-workspace" => move_to_next_workspace(config),
        "move-to-prev-workspace" => move_to_prev_workspace(config),
        cmd if cmd.starts_with("move-to-workspace-") => {
            if let Some(num_str) = cmd.strip_prefix("move-to-workspace-") {
                move_to_workspace(config, num_str)
            } else {
                Ok(())
            }
        }
        cmd if cmd.starts_with("switch-to-workspace-") => {
            if let Some(num_str) = cmd.strip_prefix("switch-to-workspace-") {
                switch_to_workspace(config, num_str)
            } else {
                Ok(())
            }
        }
        _ => {
            eprintln!("-> Unknown action: {}", command);
            Ok(())
        }
    };
    if let Err(e) = result {
        eprintln!("Error executing action {}: {}", command, e);
    }
}

fn switch_to_next_workspace(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("-> Switching to next workspace");
    let workspace_name =
        workspace::get_active_workspace(config).ok_or("Failed to get active workspace")?;
    let next_ws_list = get_adjacent_workspaces(config, &workspace_name, 1)
        .ok_or("Failed to get adjacent workspaces")?;
    println!("-> Next workspaces: {:?}", next_ws_list);
    let mut client = WsClient::connect(&config.ws)?;
    for next_ws in next_ws_list {
        client.focus_workspace(&next_ws)?;
    }
    let windows = window::get_windows(config)?;
    if let Some(first_window) = windows.first() {
        client.focus_container(&first_window.id)?;
    }
    Ok(())
}

fn switch_to_prev_workspace(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("-> Switching to previous workspace");
    let workspace_name =
        workspace::get_active_workspace(config).ok_or("Failed to get active workspace")?;
    let prev_ws_list = get_adjacent_workspaces(config, &workspace_name, -1)
        .ok_or("Failed to get previous workspaces")?;
    println!("-> Previous workspaces: {:?}", prev_ws_list);
    let mut client = WsClient::connect(&config.ws)?;
    for prev_ws in prev_ws_list {
        client.focus_workspace(&prev_ws)?;
    }
    let windows = window::get_windows(config)?;
    if let Some(first_window) = windows.first() {
        client.focus_container(&first_window.id)?;
    }
    Ok(())
}

fn move_to_next_workspace(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("-> Move to next workspace");
    let workspace_name =
        workspace::get_active_workspace(config).ok_or("Failed to get active workspace")?;
    let next_ws_list = get_adjacent_workspaces(config, &workspace_name, 1)
        .ok_or("Failed to get adjacent workspaces")?;
    println!("-> Next workspaces: {:?}", next_ws_list);
    let next_ws = next_ws_list.first().ok_or("Next workspace not found")?;
    let mut client = WsClient::connect(&config.ws)?;
    client.move_window(next_ws)?;
    Ok(())
}

fn move_to_prev_workspace(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("-> Move to prev workspace");
    let workspace_name =
        workspace::get_active_workspace(config).ok_or("Failed to get active workspace")?;
    let prev_ws_list = get_adjacent_workspaces(config, &workspace_name, -1)
        .ok_or("Failed to get previous workspaces")?;
    println!("-> Previous workspaces: {:?}", prev_ws_list);
    let prev_ws = prev_ws_list.first().ok_or("Previous workspace not found")?;
    let mut client = WsClient::connect(&config.ws)?;
    client.move_window(prev_ws)?;
    Ok(())
}

fn move_to_workspace(config: &Config, workspace_name: &str) -> Result<(), Box<dyn Error>> {
    println!("-> Moving to workspace {}", workspace_name);
    let workspace = get_workspace_by_name(config, workspace_name)
        .ok_or(format!("Workspace {} not found", workspace_name))?;
    let mut pos = 0;
    let active_ws_name =
        workspace::get_active_workspace(config).ok_or("Failed to get active workspace")?;
    for w in &config.workspaces {
        if let Some(idx) = w.names.iter().position(|n| n == &active_ws_name) {
            pos = idx;
            break;
        }
    }

    let target_name = workspace
        .names
        .get(pos)
        .or_else(|| workspace.names.first())
        .ok_or("Target name not found")?;

    let mut client = WsClient::connect(&config.ws)?;
    client.move_window(target_name)?;
    Ok(())
}

fn switch_to_workspace(config: &Config, workspace_name: &str) -> Result<(), Box<dyn Error>> {
    println!("-> Switching to workspace {}", workspace_name);
    let workspace = get_workspace_by_name(config, workspace_name)
        .ok_or(format!("Workspace {} not found", workspace_name))?;
    let mut client = WsClient::connect(&config.ws)?;
    for name in workspace.names {
        client.focus_workspace(&name)?;
    }
    let windows = window::get_windows(config)?;
    if let Some(first_window) = windows.first() {
        client.focus_container(&first_window.id)?;
    }
    Ok(())
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
