use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoiceFile<'s> {
    #[serde(borrow)]
    pub data: VoiceData<'s>,
}

#[allow(dead_code, reason = "idk")]
#[derive(Debug, Deserialize)]
pub struct VoiceData<'s> {
    #[serde(borrow)]
    caption: &'s str,
    #[serde(borrow)]
    image: Option<&'s str>,
    pub dub_timestamps: Vec<f32>,
    #[serde(borrow)]
    dub_characters: Vec<&'s str>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawConfig<'s> {
    #[serde(borrow)]
    game_path: &'s str,
}

impl<'s> RawConfig<'s> {
    pub const PATH: &str = "config.toml";

    pub fn resolve(self) -> anyhow::Result<Config> {
        Ok(Config {
            game_path: expand_str::expand_string_with_env(&self.game_path)
                // expand_str err type has a lifetime 'a so we cant consume self otherwise:
                .map_err(|e| anyhow::format_err!("{e:?}"))?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub game_path: String,
}

#[derive(Debug)]
pub struct Paths<'s> {
    pub base_path: Box<str>,
    pub backing_path: String,
    pub dubbed_path: Box<str>,
    pub output_file: &'s str,
}

#[cfg(all(test, windows))]
mod tests {
    use crate::data::RawConfig;

    #[test]
    fn resolve_config() {
        let raw = RawConfig {
            game_path: "%appdata%",
        };
        let resolved = raw.clone().resolve().unwrap();
        assert_ne!(resolved.game_path, raw.game_path);
    }
}
