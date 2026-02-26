use crate::config::Keybinding;
use crate::hook::ParsedBinding;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP};

pub fn parse_all(bindings: &[Keybinding]) -> Result<Vec<ParsedBinding>, String> {
    let mut parsed_bindings = Vec::new();
    for kb in bindings {
        let parsed = parse_binding(&kb.binding, kb.command.clone())?;
        parsed_bindings.push(parsed);
    }
    Ok(parsed_bindings)
}

fn parse_binding(binding: &str, command: String) -> Result<ParsedBinding, String> {
    let parts: Vec<&str> = binding.split('+').collect();
    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut win = false;
    let mut vk = VIRTUAL_KEY(0);

    for part in parts {
        let part_lower = part.trim().to_lowercase();
        match part_lower.as_str() {
            "lwin" | "rwin" | "win" | "super" => win = true,
            "ctrl" | "control" => ctrl = true,
            "shift" => shift = true,
            "alt" => alt = true,
            "left" => vk = VK_LEFT,
            "right" => vk = VK_RIGHT,
            "up" => vk = VK_UP,
            "down" => vk = VK_DOWN,
            "1" => vk = VIRTUAL_KEY(b'1' as u16),
            "2" => vk = VIRTUAL_KEY(b'2' as u16),
            "3" => vk = VIRTUAL_KEY(b'3' as u16),
            "4" => vk = VIRTUAL_KEY(b'4' as u16),
            "5" => vk = VIRTUAL_KEY(b'5' as u16),
            "6" => vk = VIRTUAL_KEY(b'6' as u16),
            "7" => vk = VIRTUAL_KEY(b'7' as u16),
            "8" => vk = VIRTUAL_KEY(b'8' as u16),
            "9" => vk = VIRTUAL_KEY(b'9' as u16),
            "0" => vk = VIRTUAL_KEY(b'0' as u16),
            _ => {
                if part_lower.len() == 1 {
                    let c = part_lower.chars().next().unwrap();
                    if c.is_ascii_alphabetic() {
                        vk = VIRTUAL_KEY(c.to_ascii_uppercase() as u16);
                    } else {
                        return Err(format!("Unknown key: {}", part));
                    }
                } else {
                    return Err(format!("Unknown key: {}", part));
                }
            }
        }
    }

    if vk == VIRTUAL_KEY(0) {
        return Err(format!("No primary key specified in binding: {}", binding));
    }

    Ok(ParsedBinding {
        command,
        ctrl,
        alt,
        shift,
        win,
        vk,
    })
}
