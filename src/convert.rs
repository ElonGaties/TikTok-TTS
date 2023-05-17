use tokio::process::Command;

pub async fn convert_dfwpm() {

}

pub async fn check_ffmpeg() -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await;

    if output.is_ok() {
        Ok(())
    } else {
        Err("ffmpeg is not installed or not in PATH".to_string())
    }
}