mod nvim;

use std::{collections::HashMap, fmt};

use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use serde::Deserialize;

fn main() -> Result<()> {
    let icons = {
        let json = include_str!(concat!(env!("OUT_DIR"), "/nerd-fonts/glyphnames.json"));
        let mut icons = HashMap::new();
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json)?;
        for (name, value) in map {
            if let Some(serde_json::Value::String(ref icon)) = value.get("char") {
                icons.insert(name, icon.to_string());
            }
        }
        icons
    };

    let input: Input = toml::from_str(
        &std::fs::read_to_string("./input.toml").context("failed to read `input.toml`")?,
    )?;
    let mut output = Output::default();

    for (idx, (name, icon)) in input.icon.into_iter().enumerate() {
        output.icons.push(
            icons
                .get(&icon.name)
                .with_context(|| format!("icon name '{}' not found", icon.name))?
                .to_string(),
        );
        output
            .dark_colors
            .push(Color::new(&icon.color).lightness(input.lightness.dark));
        output
            .light_colors
            .push(Color::new(&icon.color).lightness(input.lightness.light));
        output.names.push(name.clone());

        for filename in icon.filename {
            if output
                .idx_by_filename
                .insert(filename.clone(), idx)
                .is_some()
            {
                bail!("'{filename}' already exists");
            }
        }
        for filetype in icon.filetype {
            if output
                .idx_by_filetype
                .insert(filetype.clone(), idx)
                .is_some()
            {
                bail!("'{filetype}' already exists");
            }
        }
        for extension in icon.extension {
            if output
                .idx_by_extension
                .insert(extension.clone(), idx)
                .is_some()
            {
                bail!("'{extension}' already exists");
            }
        }
    }

    nvim::generate(output)?;

    Ok(())
}

#[derive(Deserialize)]
struct Input {
    lightness: Lightness,
    icon: IndexMap<String, Icon>,
}

#[derive(Deserialize)]
struct Lightness {
    dark: f32,
    light: f32,
}

#[derive(Deserialize)]
struct Icon {
    name: String,
    color: String,
    #[serde(default)]
    filename: Vec<String>,
    #[serde(default)]
    filetype: Vec<String>,
    #[serde(default)]
    extension: Vec<String>,
}

#[derive(Debug, Default)]
struct Output {
    icons: Vec<String>,
    names: Vec<String>,
    dark_colors: Vec<Color>,
    light_colors: Vec<Color>,
    idx_by_filename: IndexMap<String, usize>,
    idx_by_filetype: IndexMap<String, usize>,
    idx_by_extension: IndexMap<String, usize>,
}

struct Color {
    rgb: [u8; 3],
}

impl Color {
    fn new(hex: &str) -> Self {
        Self {
            rgb: [
                u8::from_str_radix(hex.get(1..3).unwrap(), 16).unwrap(),
                u8::from_str_radix(hex.get(3..5).unwrap(), 16).unwrap(),
                u8::from_str_radix(hex.get(5..7).unwrap(), 16).unwrap(),
            ],
        }
    }

    fn lightness(self, l: f32) -> Self {
        let mut lch = lab::LCh::from_rgb(&self.rgb);
        lch.l = l;
        Self { rgb: lch.to_rgb() }
    }

    fn hex(&self) -> String {
        self.rgb
            .iter()
            .fold(String::from("#"), |s, b| format!("{s}{b:02x}"))
    }

    fn ansi(&self) -> u8 {
        rgb2ansi256::rgb_to_ansi256(self.rgb[0], self.rgb[1], self.rgb[2])
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.hex())
    }
}
