use crate::client::WsClient;
use crate::config::Config;
use serde_json::Value;

pub fn get_active_workspace(config: &Config) -> Option<String> {
    let mut client = WsClient::connect(&config.ws).ok()?;
    let output = client.query_workspaces().ok()?;

    let json: Value = serde_json::from_str(&output).ok()?;

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
