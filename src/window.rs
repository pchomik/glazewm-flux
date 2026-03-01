use crate::client::WsClient;
use crate::config::Config;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
}

pub fn get_windows(config: &Config) -> Option<Vec<Window>> {
    let mut client = WsClient::connect(&config.ws).ok()?;
    let output = client.query_windows().ok()?;

    let json: Value = serde_json::from_str(&output).ok()?;

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
