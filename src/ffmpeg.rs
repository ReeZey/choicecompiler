use std::process::Command;

use crate::data::{Paths, VoiceFile};

pub fn run(voices: &[(String, VoiceFile)], paths: Paths<'_>) -> anyhow::Result<()> {
    let Paths {
        base_path,
        backing_path,
        dubbed_path,
        output_file,
    } = paths;

    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", &format!("{base_path}/dub_video.ogv")]);
    cmd.args(["-i", &format!("{base_path}/{backing_path}")]);

    let mut filters = Vec::new();
    let mut mix_inputs = String::new();

    filters.push("[1:a]aresample=48000,volume=1.0[bg]".to_string());
    mix_inputs.push_str("[bg]");

    for (idx, (fil, v)) in voices.iter().enumerate() {
        let dubbed_file = format!("{dubbed_path}/_dubrecord_{fil}.wav");

        cmd.args(["-i", &dubbed_file]);

        let input_index = idx + 2;
        let delay_ms = (v.data.dub_timestamps[0] * 1000.0).round() as u64;
        let label = format!("a{idx}");

        filters.push(format!(
            "[{input_index}:a]adelay=delays={delay_ms}:all=1[{label}]"
        ));

        mix_inputs.push_str(&format!("[{label}]"));
    }

    filters.push(format!(
        "{mix_inputs}amix=inputs={}:duration=longest:normalize=0[mixed]",
        voices.len() + 1
    ));

    let filter_complex = filters.join(";");

    cmd.arg("-filter_complex")
        .arg(filter_complex)
        .arg("-map")
        .arg("0:v:0")
        .arg("-map")
        .arg("[mixed]")
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("medium")
        .arg("-crf")
        .arg("23")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg("192k")
        .arg("-ac")
        .arg("1")
        .arg("-shortest")
        .arg("-movflags")
        .arg("+faststart")
        .arg(output_file);

    println!("{}", cmd.status()?);

    Ok(())
}
