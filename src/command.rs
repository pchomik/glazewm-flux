use std::process::Command;

pub fn spawn_glazewm(args: &[&str]) -> std::process::Output {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    Command::new("glazewm.exe")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute glazewm.exe")
}
