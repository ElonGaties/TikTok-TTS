use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};

pub async fn convert_dfwpm(b64_str: &str) -> Result<()> {
    let audio_data = general_purpose::STANDARD.decode(b64_str)?;

    let mut cmd = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg("-")
        .arg("-c:a")
        .arg("dfpwm")
        .arg("-ac")
        .arg("1")
        .arg("-ar")
        .arg("48000")
        .arg(format!("output.dfpwm"))
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = cmd.stdin.take() {
        stdin.write_all(audio_data.as_slice()).await?;
    }

    let status = cmd.wait().await?;

    print!("{:?}", status);

    Ok(())
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