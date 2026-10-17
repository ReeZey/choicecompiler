use std::{
    borrow::Cow,
    fs::{read_dir, read_to_string},
};

use crate::{
    DUB_RECS,
    data::{Paths, VoiceFile},
    ffmpeg,
};

const BACKING_TRACK: &str = "_backing_track";
const VALID_EXTENSIONS: [Cow<'static, str>; 2] = [Cow::Borrowed("txt"), Cow::Borrowed("ini")];

pub fn filter_selections(
    game_path: &str,
    selected: &str,
    date: &str,
    output_file: &str,
) -> anyhow::Result<()> {
    let base_path = format!("{game_path}/packs_voice/{selected}").into_boxed_str();
    let dubbed_path = format!("{game_path}/{DUB_RECS}/{selected}/{date}").into_boxed_str();

    let mut backing_path = format!("{BACKING_TRACK}.wav");

    let mut voices: Vec<(String, VoiceFile)> = vec![];

    read_dir(&*base_path)?
        .into_iter()
        .filter_map(|f| f.ok().map(|f| f.path()))
        .map(|p| {
            let stem = p
                .file_stem()
                .ok_or(anyhow::format_err!("no file stem"))?
                .to_string_lossy()
                .to_string();

            if stem.as_str() == BACKING_TRACK {
                backing_path = p
                    .file_name()
                    .ok_or(anyhow::format_err!("no file name"))?
                    .to_string_lossy()
                    .to_string();
            }

            Ok::<_, anyhow::Error>((p, stem))
        })
        .filter_map(|x| x.ok()) // ?? maybe
        .filter(|(p, _)| {
            p.extension()
                .map_or_default(|e| VALID_EXTENSIONS.contains(&e.to_string_lossy()))
        })
        .filter(|(p, _)| {
            p.file_name()
                .map_or_default(|x| !x.to_string_lossy().starts_with(char::is_numeric))
        })
        .map(|(path, stem)| {
            // leak this memory, so that we read_to_string and can use serde borrow in VoiceFile
            // this is a short-lived program and we it's dumb to clone the entire file
            // that and the os will just free it later on
            let leaked_contents: &'static str = read_to_string(&path)?.leak();
            let voice = toml::from_str(&leaked_contents)?;

            voices.push((stem, voice));

            Ok(())
        })
        .collect::<Result<(), anyhow::Error>>()?;

    ffmpeg::run(
        &voices,
        Paths {
            base_path,
            backing_path,
            dubbed_path,
            output_file,
        },
    )?;

    Ok(())
}
