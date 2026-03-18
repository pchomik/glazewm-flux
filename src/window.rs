use crate::client::WsClient;
use crate::config::Config;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
}

pub fn get_windows(config: &Config) -> Result<Vec<Window>, Box<dyn std::error::Error>> {
    let mut client = WsClient::connect(&config.ws)?;
    let output = client.query_windows()?;

    let json: Value = serde_json::from_str(&output)?;

    let windows = json
        .get("data")
        .and_then(|d| d.get("windows"))
        .and_then(|w| w.as_array())
        .ok_or("Could not parse windows array")?;
    let mut windows_list = Vec::new();

    for window in windows {
        if window.get("type").and_then(|n| n.as_str()) != Some("window") {
            continue;
        }

        if window.get("displayState").and_then(|n| n.as_str()) != Some("shown") {
            continue;
        }

        if let Some(id) = window.get("id").and_then(|n| n.as_str()) {
            windows_list.push(Window { id: id.to_string() });
        }
    }

    println!("Windows: {:?}", windows_list);

    if windows_list.is_empty() {
        Err("No windows found".into())
    } else {
        Ok(windows_list)
    }
}
