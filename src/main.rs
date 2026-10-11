use std::{fs::{self, File, read_to_string}, io::{Read, Write, stdin, stdout}, process::Command};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VoiceFile {
    data: VoiceData
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct VoiceData {
    caption: String,
    image: String,
    dub_timestamps: Vec<f32>,
    dub_characters: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
    game_path: String
}

fn ask_pack(game_path: &str) -> String {
    loop {
        let mut dubbed_packs = vec![];

        for (idx, dp) in fs::read_dir(format!("{}/recordings/dub_recordings", game_path)).unwrap().enumerate() {
            if let Ok(pack) = dp {
                let name = pack.file_name().to_str().unwrap().to_string();
                println!("{}. {:?}", idx, name);
                dubbed_packs.push(name);
            }
        }
        let input= take_input(Some("what clip (number)?"));

        let number = match input.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("not a number: [{}]", input);
                continue
            }
        };

        if let Some(abc) = dubbed_packs.get(number) {
            return abc.to_owned();
        }

        println!("invalid dubbed_pack idx: [{}]", input);
    }
}

fn ask_date(game_path: &str, selected: &str) -> String {
    loop {
        let mut dates = vec![];

        for (idx, date) in fs::read_dir(format!("{}/recordings/dub_recordings/{}", game_path, selected)).unwrap().enumerate() {
            if let Ok(pack) = date {
                let name = pack.file_name().to_str().unwrap().to_string();
                println!("{}. {:?}", idx, name);
                dates.push(name);
            }
        }
        let input= take_input(Some("which date (number)?"));

        let number = match input.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("not a number: [{}]", input);
                continue
            }
        };

        if let Some(abc) = dates.get(number) {
            return abc.to_owned();
        }

        println!("invalid date idx: [{}]", input);
    }
}

fn take_input(prefix: Option<&str>) -> String {
    if let Some(pre) = prefix {
        print!("{} >", pre);
        stdout().flush().unwrap();
    }
    let mut input=String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    return input;
}

fn main() {
    let config_str = read_to_string("config.toml").unwrap();
    let mut config: Config = toml::from_str(&config_str).unwrap();

    config.game_path = expand_str::expand_string_with_env(&config.game_path).unwrap();

    let selected = ask_pack(&config.game_path);
    let date = ask_date(&config.game_path, &selected);
    let output_file = format!("{}.mp4", take_input(Some("output file name (text without .mp4)")));


    convert(&config.game_path, &selected, &date, &output_file);
}

fn convert(game_path: &str, selected: &str, date: &str, output_file: &str) {
    let original_path = format!("{}/packs_voice/{}", game_path, selected);
    let dubbed_path = format!("{}/recordings/dub_recordings/{}/{}", game_path, selected, date);


    let files = fs::read_dir(&original_path).unwrap();

    let mut voices: Vec<(String, VoiceFile)> = vec![];

    for f in files {
        if let Ok(fil) = f {
            let path = fil.path();
            match path.extension() {
                Some(ext) => {
                    match ext.to_str().unwrap() {
                        "txt" => {
                            let mut content = String::new();
                            File::open(&path).unwrap().read_to_string(&mut content).unwrap();
                            let voice: VoiceFile = toml::from_str(&content).unwrap();

                            println!("{:#?}", voice);
                            voices.push((path.file_stem().unwrap().to_string_lossy().to_string(), voice));
                        },
                        _ => { }
                    }
                }
                None => {}
            }
            println!("{:?}", fil.file_name());
        }
    }

    

    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", &format!("{}/dub_video.ogv", original_path)]);
    cmd.args(["-i", &format!("{}/_backing_track.mp3", original_path)]);

    let mut filters = Vec::new();
    let mut labels = Vec::new();

    filters.push(
        "[1:a]aresample=48000,volume=0.25[bg]".to_string()
    );
    labels.push("[bg]".to_string());

    for (idx, (fil, _v)) in voices.iter().enumerate()  {
        let dubbed_file = format!("{}/_dubrecord_{}.wav", dubbed_path, fil);

        cmd.args(["-i", &dubbed_file]);

        let input_index = idx + 2;
        let delay_ms = (_v.data.dub_timestamps[0] * 1000.0).round() as u64;
        let label = format!("a{idx}");

        filters.push(format!(
            "[{input_index}:a]adelay=delays={delay_ms}:all=1[{label}]"
        ));

        labels.push(format!("[{label}]"));
    }

    let mix_inputs = labels.join("");
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

    let status = cmd.status().unwrap();

    println!("{}", status);
}
