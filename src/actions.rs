use crate::client::WsClient;
use crate::config::{Config, Workspace};
use crate::window;
use crate::workspace;

pub fn execute(command: &str, config: &Config) {
    println!("Executing action: {}", command);
    match command {
        "switch-to-next-workspace" => {
            switch_to_next_workspace(config);
        }
        "switch-to-prev-workspace" => {
            switch_to_prev_workspace(config);
        }
        "move-to-next-workspace" => {
            move_to_next_workspace(config);
        }
        "move-to-prev-workspace" => {
            move_to_prev_workspace(config);
        }
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

fn switch_to_next_workspace(config: &Config) -> Option<()> {
    println!("-> Switching to next workspace");
    let workspace_name = workspace::get_active_workspace(config)?;
    let next_ws_list = get_adjacent_workspaces(config, &workspace_name, 1)?;
    println!("-> Next workspaces: {:?}", next_ws_list);
    let mut client = WsClient::connect(&config.ws).ok()?;
    for next_ws in next_ws_list {
        client.focus_workspace(&next_ws).ok()?;
    }
    let windows = window::get_windows(config)?;
    client.focus_container(&windows.first().unwrap().id).ok()?;
    Some(())
}

fn switch_to_prev_workspace(config: &Config) -> Option<()> {
    println!("-> Switching to previous workspace");
    let workspace_name = workspace::get_active_workspace(config)?;
    let prev_ws_list = get_adjacent_workspaces(config, &workspace_name, -1)?;
    println!("-> Previous workspaces: {:?}", prev_ws_list);
    let mut client = WsClient::connect(&config.ws).ok()?;
    for prev_ws in prev_ws_list {
        client.focus_workspace(&prev_ws).ok()?;
    }
    let windows = window::get_windows(config)?;
    client.focus_container(&windows.first().unwrap().id).ok()?;
    Some(())
}

fn move_to_next_workspace(config: &Config) -> Option<()> {
    println!("-> Move to next workspace");
    let workspace_name = workspace::get_active_workspace(config)?;
    let next_ws_list = get_adjacent_workspaces(config, &workspace_name, 1)?;
    println!("-> Next workspaces: {:?}", next_ws_list);
    let next_ws = next_ws_list.first()?;
    let mut client = WsClient::connect(&config.ws).ok()?;
    client.move_window(&next_ws).ok()?;
    Some(())
}

fn move_to_prev_workspace(config: &Config) -> Option<()> {
    println!("-> Move to prev workspace");
    let workspace_name = workspace::get_active_workspace(config)?;
    let prev_ws_list = get_adjacent_workspaces(config, &workspace_name, -1)?;
    println!("-> Previous workspaces: {:?}", prev_ws_list);
    let prev_ws = prev_ws_list.first()?;
    let mut client = WsClient::connect(&config.ws).ok()?;
    client.move_window(&prev_ws).ok()?;
    Some(())
}

fn move_to_workspace(config: &Config, workspace_name: &str) -> Option<()> {
    println!("-> Moving to workspace {}", workspace_name);
    let workspace = get_workspace_by_name(config, workspace_name)?;
    let mut pos = 0;
    let active_ws_name = workspace::get_active_workspace(config)?;
    for w in &config.workspaces {
        if let Some(idx) = w.names.iter().position(|n| n == &active_ws_name) {
            pos = idx;
            break;
        }
    }

    let target_name = workspace
        .names
        .get(pos)
        .or_else(|| workspace.names.first())?;

    let mut client = WsClient::connect(&config.ws).ok()?;
    client.move_window(&target_name).ok()?;
    Some(())
}

fn switch_to_workspace(config: &Config, workspace_name: &str) -> Option<()> {
    println!("-> Switching to workspace {}", workspace_name);
    let workspace = get_workspace_by_name(config, workspace_name)?;
    let mut client = WsClient::connect(&config.ws).ok()?;
    for name in workspace.names {
        client.focus_workspace(&name).ok()?;
    }
    let windows = window::get_windows(config)?;
    client.focus_container(&windows.first().unwrap().id).ok()?;
    Some(())
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
