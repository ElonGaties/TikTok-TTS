use std::process::Stdio;
use axum::response::Stream;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, self};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use tokio_util::io::StreamReader;

pub async fn convert_dfwpm(b64_str: &str) -> Result<Stream<impl tokio::io::AsyncRead + Send + Sync>> {
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
        .arg("pipe:1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let cmd_stdin = cmd.stdin.take().expect("Failed to open FFmpeg stdin");
    let cmd_stdout = cmd.stdout.take().expect("Failed to open FFmpeg stdout");

    tokio::spawn(async move {
        let mut ffmpeg_stdin_writer = tokio::io::BufWriter::new(cmd_stdin);

        io::copy(&mut audio_data.as_slice(), &mut ffmpeg_stdin_writer)
            .await
            .expect("Failed to copy from stdin to FFmpeg stdin");
    });

    let reader = io::BufReader::new(cmd_stdout);

    /*if let Some(mut stdin) = cmd.stdin.take() {
        stdin.write_all(audio_data.as_slice()).await?;
    }*/

    //let status = cmd.wait().await?;

    Ok(reader)
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