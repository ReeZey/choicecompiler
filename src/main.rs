use crate::{
    data::{Config, RawConfig},
    filter::filter_selections,
    input::{ask, take_input},
};
use std::fs::read_to_string;

mod data;
mod ffmpeg;
mod filter;
mod input;

const DUB_RECS: &str = "recordings/dub_recordings";

fn main() -> anyhow::Result<()> {
    let config = read_to_string(RawConfig::PATH)?;
    let raw: RawConfig = toml::from_str(&config)?;
    let Config { game_path } = raw.resolve()?;

    let selected = ask(format!("{game_path}/{DUB_RECS}"), "what clip (number)?")?;
    let date = ask(
        format!("{game_path}/{DUB_RECS}/{selected}"),
        "which date (number)?",
    )?;
    let output_file = format!(
        "{}.mp4",
        take_input("output file name (text without .mp4)")?
    );

    filter_selections(&game_path, &selected, &date, &output_file)
}
