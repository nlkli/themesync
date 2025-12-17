use crate::models::*;
use rand::seq::IndexedRandom;
use strsim::levenshtein;

pub const LIST: [&str; 49] = [
    "base16_default_dark",
    "ashes_dark",
    "ashes_light",
    "autumn",
    "github_dark",
    "github_light",
    "vscode_dark_plus",
    "vesper",
    "ubuntu",
    "tokyo_night",
    "rose_pine_dawn",
    "rose_pine",
    "google",
    "gotham",
    "gruber_darker",
    "gruvbox_dark",
    "gruvbox_light",
    "gruvbox_material_dark_medium",
    "gruvbox_material_hard_dark",
    "gruvbox_material_hard_light",
    "gruvbox_material_medium_dark",
    "gruvbox_material_medium_light",
    "hardhacker",
    "high_contrast",
    "horizon_dark",
    "hyper",
    "iceberg",
    "iris",
    "iterm2",
    "kanagawa_dragon",
    "kanagawa_wave",
    "kimbie_dark",
    "kimbie_light",
    "kitty",
    "konsole_port",
    "low_contrast",
    "marine_dark",
    "meliora",
    "miasma",
    "midnight_haze",
    "monokai",
    "monokai_charcoal",
    "monokai_pro",
    "moonfly",
    "night_owl",
    "paper",
    "duskfox",
    "dayfox",
    "terafox",
];

pub const DARK_LIST: [&str; 39] = [
    "base16_default_dark",
    "ashes_dark",
    "autumn",
    "github_dark",
    "vscode_dark_plus",
    "vesper",
    "tokyo_night",
    "rose_pine",
    "google",
    "gotham",
    "gruber_darker",
    "gruvbox_dark",
    "gruvbox_material_dark_medium",
    "gruvbox_material_hard_dark",
    "gruvbox_material_medium_dark",
    "hardhacker",
    "high_contrast",
    "horizon_dark",
    "hyper",
    "iceberg",
    "iris",
    "iterm2",
    "kanagawa_dragon",
    "kanagawa_wave",
    "kimbie_dark",
    "kitty",
    "konsole_port",
    "low_contrast",
    "marine_dark",
    "meliora",
    "miasma",
    "midnight_haze",
    "monokai",
    "monokai_charcoal",
    "monokai_pro",
    "moonfly",
    "night_owl",
    "duskfox",
    "terafox",
];

pub const LIGHT_LIST: [&str; 9] = [
    "ashes_light",
    "github_light",
    "rose_pine_dawn",
    "gruvbox_light",
    "gruvbox_material_hard_light",
    "gruvbox_material_medium_light",
    "kimbie_light",
    "paper",
    "dayfox",
];

pub fn rand() -> Theme {
    let name = LIST.choose(&mut rand::rng());
    by_name(*name.unwrap_or(&LIST[0]))
}

pub fn search(query: &str) -> Theme {
    let query = query.to_lowercase();

    let score = |s: &str| -> usize {
        let s_lower = s.to_lowercase();
        if s_lower == query {
            0
        } else if s_lower.contains(&query) {
            1
        } else {
            levenshtein(&s_lower, &query) + s_lower.len() / 10
        }
    };

    let dark = DARK_LIST.iter()
        .min_by_key(|&&s| score(s))
        .unwrap_or(&DARK_LIST[0]);

    let light = LIGHT_LIST.iter()
        .min_by_key(|&&s| score(s))
        .unwrap_or(&LIGHT_LIST[0]);

    if score(dark) <= score(light) {
        by_name(dark)
    } else {
        by_name(light)
    }
}

fn new_term_colors(
    black: &str,
    red: &str,
    green: &str,
    yellow: &str,
    blue: &str,
    magenta: &str,
    cyan: &str,
    white: &str,
    orange: &str,
    pink: &str,
) -> TermColors {
    let orange = if orange.is_empty() {
        None
    } else {
        Some(orange.into())
    };
    let pink = if pink.is_empty() {
        None
    } else {
        Some(pink.into())
    };
    TermColors {
        black: black.into(),
        red: red.into(),
        green: green.into(),
        yellow: yellow.into(),
        blue: blue.into(),
        magenta: magenta.into(),
        cyan: cyan.into(),
        white: white.into(),
        orange: orange,
        pink: pink,
    }
}

pub fn by_name(name: &str) -> Theme {
    match name {
        "terafox" => Theme {
            name: Some("terafox".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2f3239", // black
                    "#e85c51", // red
                    "#7aa4a1", // green
                    "#fda47f", // yellow
                    "#5a93aa", // blue
                    "#ad5c7c", // magenta
                    "#a1cdd8", // cyan
                    "#ebebeb", // white
                    "#ff8349", // orange
                    "#cb7985", // pink
                ),
                bright: Some(new_term_colors(
                    "#4e5157", // black
                    "#eb746b", // red
                    "#8eb2af", // green
                    "#fdb292", // yellow
                    "#73a3b7", // blue
                    "#b97490", // magenta
                    "#afd4de", // cyan
                    "#eeeeee", // white
                    "#ff9664", // orange
                    "#d38d97", // pink
                )),
                dim: Some(new_term_colors(
                    "#282a30", // black
                    "#c54e45", // red
                    "#688b89", // green
                    "#d78b6c", // yellow
                    "#4d7d90", // blue
                    "#934e69", // magenta
                    "#89aeb8", // cyan
                    "#c8c8c8", // white
                    "#d96f3e", // orange
                    "#ad6771", // pink
                )),
                comment: Some("#6d7f8b".into()),
                variable: Some("#ebebeb".into()),
                status_line: None,
                background: Background::Colors([
                    "#0f1c1e".into(),
                    "#152528".into(),
                    "#1d3337".into(),
                    "#254147".into(),
                    "#2d4f56".into(),
                ]),
                foreground: Foreground::Colors([
                    "#eaeeee".into(),
                    "#e6eaea".into(),
                    "#cbd9d8".into(),
                    "#587b7b".into(),
                ]),
                selection: Selection::Colors(["#293e40".into(), "#425e5e".into()]),
                diff: None,
            }),
            config: Some(ThemeConfig {
                diff_blend: Some(DiffBlendConfig {
                    add: 0.2,
                    delete: 0.25,
                    change: 0.2,
                    text: 0.35,
                }),
                ..Default::default()
            }),
        },
        "dayfox" => Theme {
            name: Some("dayfox".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: TermColors {
                    black: "#352c24".into(),
                    red: "#a5222f".into(),
                    green: "#396847".into(),
                    yellow: "#AC5402".into(),
                    blue: "#2848a9".into(),
                    magenta: "#6e33ce".into(),
                    cyan: "#287980".into(),
                    white: "#f2e9e1".into(),
                    orange: Some("#955f61".into()),
                    pink: Some("#a440b5".into()),
                },
                bright: None,
                dim: None,
                comment: Some("#837a72".into()),
                variable: Some("#352c24".into()),
                status_line: None,
                background: Background::Colors([
                    "#e4dcd4".into(),
                    "#f6f2ee".into(),
                    "#dbd1dd".into(),
                    "#d3c7bb".into(),
                    "#aab0ad".into(),
                ]),
                foreground: Foreground::Colors([
                    "#302b5d".into(),
                    "#3d2b5a".into(),
                    "#643f61".into(),
                    "#824d5b".into(),
                ]),
                selection: Selection::Colors(["#e7d2be".into(), "#a4c1c2".into()]),
                diff: None,
            }),
            config: Some(ThemeConfig {
                shade_factor: Some(0.15),
                diff_blend: Some(DiffBlendConfig {
                    add: 0.2,
                    delete: 0.2,
                    change: 0.2,
                    text: 0.4,
                }),
                ..Default::default()
            }),
        },
        "duskfox" => Theme {
            name: Some("duskfox".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#393552", // black
                    "#eb6f92", // red
                    "#a3be8c", // green
                    "#f6c177", // yellow
                    "#569fba", // blue
                    "#c4a7e7", // magenta
                    "#9ccfd8", // cyan
                    "#e0def4", // white
                    "#ea9a97", // orange
                    "#eb98c3", // pink
                ),
                bright: Some(new_term_colors(
                    "#47407d", // black
                    "#f083a2", // red
                    "#b1d196", // green
                    "#f9cb8c", // yellow
                    "#65b1cd", // blue
                    "#ccb1ed", // magenta
                    "#a6dae3", // cyan
                    "#e2e0f7", // white
                    "#f0a4a2", // orange
                    "#f0a6cc", // pink
                )),
                dim: Some(new_term_colors(
                    "#322e42", // black
                    "#d84f76", // red
                    "#8aa872", // green
                    "#e6a852", // yellow
                    "#4a869c", // blue
                    "#a580d2", // magenta
                    "#7bb8c1", // cyan
                    "#b1acde", // white
                    "#d6746f", // orange
                    "#d871a6", // pink
                )),
                comment: Some("#817c9c".into()),
                variable: Some("#e0def4".into()),
                status_line: None,
                background: Background::Colors([
                    "#191726".into(),
                    "#232136".into(),
                    "#2d2a45".into(),
                    "#373354".into(),
                    "#4b4673".into(),
                ]),
                foreground: Foreground::Colors([
                    "#eae8ff".into(),
                    "#e0def4".into(),
                    "#cdcbe0".into(),
                    "#6e6a86".into(),
                ]),
                selection: Selection::Colors(["#433c59".into(), "#63577d".into()]),
                diff: None,
            }),
            config: Some(ThemeConfig {
                diff_blend: Some(DiffBlendConfig {
                    add: 0.2,
                    delete: 0.2,
                    change: 0.2,
                    text: 0.4,
                }),
                ..Default::default()
            }),
        },
        "paper" => Theme {
            name: Some("paper".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#cc3e28", // red
                    "#216609", // green
                    "#b58900", // yellow
                    "#1e6fcc", // blue
                    "#5c21a5", // magenta
                    "#158c86", // cyan
                    "#aaaaaa", // white
                    "#d17c15", // orange
                    "#b84a8a", // pink
                ),
                bright: Some(new_term_colors(
                    "#3a3a3a", // black
                    "#e2553f", // red
                    "#3a7d22", // green
                    "#cfa52a", // yellow
                    "#3a86e0", // blue
                    "#7540bf", // magenta
                    "#2fa19b", // cyan
                    "#c4c4c4", // white
                    "#e08f2f", // orange
                    "#cf6aa1", // pink
                )),
                dim: None,
                comment: Some("#6f6f6f".into()),
                variable: None,
                status_line: Some("#e6e1cf".into()),
                background: Background::Colors([
                    "#e3decf".into(),
                    "#f2eede".into(),
                    "#f8f5ea".into(),
                    "#e4dfce".into(),
                    "#c6c1b0".into(),
                ]),
                foreground: Foreground::Colors([
                    "#3a3a3a".into(),
                    "#000000".into(),
                    "#5a5a5a".into(),
                    "#7a7a7a".into(),
                ]),
                selection: Selection::Colors(["#e1dbc6".into(), "#f0cf7a".into()]),
                diff: None,
            }),
            config: None,
        },
        "night_owl" => Theme {
            // OK
            name: Some("night_owl".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#011627", // black
                    "#EF5350", // red
                    "#22DA6E", // green
                    "#C5E478", // yellow
                    "#82AAFF", // blue
                    "#C792EA", // magenta
                    "#21C7A8", // cyan
                    "#FFFFFF", // white
                    "#F78C6C", // orange
                    "#FF6FB1", // pink
                ),
                bright: Some(new_term_colors(
                    "#575656", // black
                    "#EF5350", // red
                    "#22DA6E", // green
                    "#FFEB95", // yellow
                    "#82AAFF", // blue
                    "#C792EA", // magenta
                    "#7FDBCA", // cyan
                    "#AAAAAA", // white
                    "#FF9E80", // orange
                    "#FF85C2", // pink
                )),
                dim: Some(new_term_colors(
                    "#010E1A", // black
                    "#C94644", // red
                    "#1BB45A", // green
                    "#9FBF64", // yellow
                    "#6B8FD6", // blue
                    "#A377C9", // magenta
                    "#1AA38A", // cyan
                    "#AEB7C6", // white
                    "#D97757", // orange
                    "#D95E9A", // pink
                )),
                comment: Some("#637777".into()),
                variable: None,
                status_line: Some("#1D3B53".into()),
                background: Background::Colors([
                    "#1D3B53".into(),
                    "#011627".into(),
                    "#0B253A".into(),
                    "#0D486E".into(),
                    "#2A4F6E".into(),
                ]),
                foreground: Foreground::Colors([
                    "#E4EBF7".into(),
                    "#D6DEEB".into(),
                    "#AEB7C6".into(),
                    "#8FA2B7".into(),
                ]),
                selection: Selection::Colors(["#0D486E".into(), "#1AA38A".into()]),
                diff: None,
            }),
            config: None,
        },
        "moonfly" => Theme {
            // OK
            name: Some("moonfly".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#323437", // black
                    "#ff5454", // red
                    "#8cc85f", // green
                    "#e3c78a", // yellow
                    "#80a0ff", // blue
                    "#cf87e8", // magenta
                    "#79dac8", // cyan
                    "#c6c6c6", // white
                    "#f0a36f", // orange
                    "#ff79c6", // pink
                ),
                bright: Some(new_term_colors(
                    "#949494", // black
                    "#ff5189", // red
                    "#36c692", // green
                    "#c6c684", // yellow
                    "#74b2ff", // blue
                    "#ae81ff", // magenta
                    "#85dc85", // cyan
                    "#e4e4e4", // white
                    "#ffb070", // orange
                    "#ff92d0", // pink
                )),
                dim: Some(new_term_colors(
                    "#1f2022", // black
                    "#cc3f3f", // red
                    "#6fa34c", // green
                    "#b59d6f", // yellow
                    "#5f79cc", // blue
                    "#a06bb8", // magenta
                    "#5fb3a5", // cyan
                    "#b0b0b0", // white
                    "#c88958", // orange
                    "#cc609f", // pink
                )),
                comment: Some("#6b6b6b".into()),
                variable: None,
                status_line: Some("#121212".into()),
                background: Background::Colors([
                    "#000000".into(),
                    "#080808".into(),
                    "#121212".into(),
                    "#1a1a1a".into(),
                    "#2a2a2a".into(),
                ]),
                foreground: Foreground::Colors([
                    "#b0b0b0".into(),
                    "#bdbdbd".into(),
                    "#eeeeee".into(),
                    "#8a8a8a".into(),
                ]),
                selection: Selection::Colors(["#303A46".into(), "#5D71AE".into()]),
                diff: None,
            }),
            config: None,
        },
        "monokai_pro" => Theme {
            name: Some("monokai_pro".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2c2525", // black
                    "#fd6883", // red
                    "#adda78", // green
                    "#f9cc6c", // yellow
                    "#f38d70", // blue
                    "#a8a9eb", // magenta
                    "#85dacc", // cyan
                    "#fff1f3", // white
                    "#f9a66c", // orange
                    "#fd83a2", // pink
                ),
                bright: Some(new_term_colors(
                    "#72696a", // black
                    "#ff8fa3", // red
                    "#c0d899", // green
                    "#fce18d", // yellow
                    "#f7a58c", // blue
                    "#bdbcf3", // magenta
                    "#a3e6dd", // cyan
                    "#fff1f3", // white
                    "#ffb07c", // orange
                    "#ff91b4", // pink
                )),
                dim: Some(new_term_colors(
                    "#1f1a1b", // black
                    "#d14d6f", // red
                    "#8fb06a", // green
                    "#d8b55a", // yellow
                    "#c97560", // blue
                    "#8c8cd5", // magenta
                    "#66bfb3", // cyan
                    "#e6dede", // white
                    "#d18c5a", // orange
                    "#d65f8c", // pink
                )),
                comment: Some("#807a7c".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1b1e".into(),
                    "#2D2A2E".into(),
                    "#3f3a3f".into(),
                    "#343036".into(),
                    "#191517".into(),
                ]),
                foreground: Foreground::Colors([
                    "#e8e0e2".into(),
                    "#fff1f3".into(),
                    "#d6cfd2".into(),
                    "#c9a0ab".into(),
                ]),
                selection: Selection::Colors(["#5A4E50".into(), "#BD7763".into()]),
                diff: None,
            }),
            config: None,
        },
        "monokai_charcoal" => Theme {
            name: Some("monokai_charcoal".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#1a1a1a", // black
                    "#f4005f", // red
                    "#98e024", // green
                    "#fa8419", // yellow
                    "#9d65ff", // blue
                    "#f4005f", // magenta
                    "#58d1eb", // cyan
                    "#c4c5b5", // white
                    "#fa8c42", // orange
                    "#f45fa0", // pink
                ),
                bright: Some(new_term_colors(
                    "#625e4c", // black
                    "#ff6a80", // red
                    "#b0eb46", // green
                    "#fbc77f", // yellow
                    "#b089ff", // blue
                    "#ff6a80", // magenta
                    "#7ed8ff", // cyan
                    "#f6f6ef", // white
                    "#ffaf73", // orange
                    "#ff7db3", // pink
                )),
                dim: Some(new_term_colors(
                    "#0f0f0f", // black
                    "#c30050", // red
                    "#7cb616", // green
                    "#c26812", // yellow
                    "#8140d6", // blue
                    "#c30050", // magenta
                    "#38a4bf", // cyan
                    "#a9a995", // white
                    "#c27136", // orange
                    "#c24d7d", // pink
                )),
                comment: Some("#7f7f7f".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#141414".into(),
                    "#000000".into(),
                    "#222222".into(),
                    "#1a1a1a".into(),
                    "#0d0d0d".into(),
                ]),
                foreground: Foreground::Colors([
                    "#b8b8b8".into(),
                    "#FFFFFF".into(),
                    "#e0e0e0".into(),
                    "#9a9a9a".into(),
                ]),
                selection: Selection::Colors(["#2a2a2a".into(), "#fa8419".into()]),
                diff: None,
            }),
            config: None,
        },
        "monokai" => Theme {
            name: Some("monokai".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#272822", // black
                    "#f92672", // red
                    "#a6e22e", // green
                    "#f4bf75", // yellow
                    "#66d9ef", // blue
                    "#ae81ff", // magenta
                    "#a1efe4", // cyan
                    "#f8f8f2", // white
                    "#f9a65f", // orange
                    "#f95fbf", // pink
                ),
                bright: Some(new_term_colors(
                    "#75715e", // black
                    "#ff7fa0", // red
                    "#c0eb55", // green
                    "#f7d88f", // yellow
                    "#8ee6ff", // blue
                    "#c8aaff", // magenta
                    "#c1fff7", // cyan
                    "#f9f8f5", // white
                    "#ffb77c", // orange
                    "#ff7fd1", // pink
                )),
                dim: Some(new_term_colors(
                    "#1c1b16", // black
                    "#c7155c", // red
                    "#8cbf1f", // green
                    "#c9a25a", // yellow
                    "#52b0c0", // blue
                    "#8c69c8", // magenta
                    "#80cfcf", // cyan
                    "#e6e6d8", // white
                    "#c97a40", // orange
                    "#c44ca8", // pink
                )),
                comment: Some("#7f7a66".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1f1b".into(),
                    "#272822".into(),
                    "#35352d".into(),
                    "#303028".into(),
                    "#1a1a17".into(),
                ]),
                foreground: Foreground::Colors([
                    "#ecece5".into(),
                    "#f8f8f2".into(),
                    "#cfcfc2".into(),
                    "#9a9583".into(),
                ]),
                selection: Selection::Colors(["#49483e".into(), "#CCA36A".into()]),
                diff: None,
            }),
            config: None,
        },
        "midnight_haze" => Theme {
            name: Some("midnight_haze".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2c2c3d", // black
                    "#ff6e6e", // red
                    "#9ec875", // green
                    "#ffa759", // yellow
                    "#70a7d4", // blue
                    "#d291e0", // magenta
                    "#96e0e0", // cyan
                    "#d8dee9", // white
                    "#ffb380", // orange
                    "#ff92d0", // pink
                ),
                bright: Some(new_term_colors(
                    "#414166", // black
                    "#ff8d8d", // red
                    "#b3d987", // green
                    "#ffc57f", // yellow
                    "#9bb3d3", // blue
                    "#ffa1ff", // magenta
                    "#9cd8d8", // cyan
                    "#ffffff", // white
                    "#ffd1a0", // orange
                    "#ffb3ff", // pink
                )),
                dim: Some(new_term_colors(
                    "#1f1f2b", // black
                    "#cc5c5c", // red
                    "#7fae61", // green
                    "#cc8946", // yellow
                    "#537ca8", // blue
                    "#a070b3", // magenta
                    "#74b3b3", // cyan
                    "#aab0bb", // white
                    "#e69966", // orange
                    "#cc77b3", // pink
                )),
                comment: Some("#7a7c92".into()),
                variable: None,
                status_line: Some("#1f1f2b".into()),
                background: Background::Colors([
                    "#0c0c16".into(),
                    "#121224".into(),
                    "#1a1a30".into(),
                    "#202038".into(),
                    "#2c2c3d".into(),
                ]),
                foreground: Foreground::Colors([
                    "#b6bcc8".into(),
                    "#d8dee9".into(),
                    "#aab0bb".into(),
                    "#7f8596".into(),
                ]),
                selection: Selection::Colors(["#45456a".into(), "#70a7d4".into()]),
                diff: None,
            }),
            config: None,
        },

        "miasma" => Theme {
            name: Some("miasma".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#222222", // black
                    "#685742", // red
                    "#5f875f", // green
                    "#b36d43", // yellow
                    "#78824b", // blue
                    "#bb7744", // magenta
                    "#c9a554", // cyan
                    "#d7c483", // white
                    "#c9974a", // orange
                    "#bb6677", // pink
                ),
                bright: Some(new_term_colors(
                    "#666666", // black
                    "#8c6f5a", // red
                    "#7ea77e", // green
                    "#d29366", // yellow
                    "#95a262", // blue
                    "#d49c66", // magenta
                    "#d4b56a", // cyan
                    "#e0d7b0", // white
                    "#e0ad6e", // orange
                    "#d88888", // pink
                )),
                dim: Some(new_term_colors(
                    "#1a1a1a", // black
                    "#584732", // red
                    "#4b6b4b", // green
                    "#8d4f30", // yellow
                    "#5f6b3a", // blue
                    "#996633", // magenta
                    "#9c8540", // cyan
                    "#bfb68f", // white
                    "#b87f45", // orange
                    "#a65b5b", // pink
                )),
                comment: Some("#7a7368".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1b1b1b".into(),
                    "#222222".into(),
                    "#2a2a24".into(),
                    "#3a3a32".into(),
                    "#3c3a33".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d0ccb8".into(),
                    "#c2c2b0".into(),
                    "#a89f7a".into(),
                    "#8e876a".into(),
                ]),
                selection: Selection::Colors(["#4a4a3d".into(), "#6b6a55".into()]),
                diff: None,
            }),
            config: None,
        },
        "meliora" => Theme {
            name: Some("meliora".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2a2421", // black
                    "#d49191", // red
                    "#b6b696", // green
                    "#c4b392", // yellow
                    "#9e96b6", // blue
                    "#b696b1", // magenta
                    "#98acc8", // cyan
                    "#ddd9d6", // white
                    "#c8ab7e", // orange
                    "#d69bb1", // pink
                ),
                bright: Some(new_term_colors(
                    "#2e2622", // black
                    "#d89393", // red
                    "#b9b99b", // green
                    "#c8b692", // yellow
                    "#a299b9", // blue
                    "#b997b4", // magenta
                    "#9bb0ca", // cyan
                    "#e1dbd9", // white
                    "#d0ba8c", // orange
                    "#e0a5c2", // pink
                )),
                dim: Some(new_term_colors(
                    "#2a2421", // black
                    "#d18989", // red
                    "#727246", // green
                    "#c1b090", // yellow
                    "#9b92b3", // blue
                    "#b393ad", // magenta
                    "#95a9c5", // cyan
                    "#e3d5ce", // white
                    "#bfa77c", // orange
                    "#c792a8", // pink
                )),
                comment: Some("#8c857d".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1e1a18".into(),
                    "#1c1917".into(),
                    "#24201e".into(),
                    "#2f2a27".into(),
                    "#302b28".into(),
                ]),
                foreground: Foreground::Colors([
                    "#dcd6d3".into(),
                    "#d6d0cd".into(),
                    "#bfb7b2".into(),
                    "#a79f9a".into(),
                ]),
                selection: Selection::Colors(["#4a433f".into(), "#5a524d".into()]),
                diff: None,
            }),
            config: None,
        },
        "marine_dark" => Theme {
            name: Some("marine_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#002221", // black
                    "#ea3431", // red
                    "#00b6b6", // green
                    "#f8b017", // yellow
                    "#4894fd", // blue
                    "#e01dca", // magenta
                    "#1ab2ad", // cyan
                    "#99dddb", // white
                    "#f0a035", // orange
                    "#e370d0", // pink
                ),
                bright: Some(new_term_colors(
                    "#006562", // black
                    "#ff5c5b", // red
                    "#1ce6e6", // green
                    "#f9c036", // yellow
                    "#60a8ff", // blue
                    "#f03ddf", // magenta
                    "#3ad2cb", // cyan
                    "#e6f6f6", // white
                    "#ffb347", // orange
                    "#f09ce6", // pink
                )),
                dim: Some(new_term_colors(
                    "#001b1b", // black
                    "#c6312e", // red
                    "#009999", // green
                    "#d79f0f", // yellow
                    "#387acc", // blue
                    "#b017b3", // magenta
                    "#149090", // cyan
                    "#88cccc", // white
                    "#d48c2a", // orange
                    "#b359b3", // pink
                )),
                comment: Some("#5a7d7d".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#001a1a".into(),
                    "#002221".into(),
                    "#00303a".into(),
                    "#003b46".into(),
                    "#00414d".into(),
                ]),
                foreground: Foreground::Colors([
                    "#bdeeee".into(),
                    "#e6f8f8".into(),
                    "#99dddb".into(),
                    "#80cfcf".into(),
                ]),
                selection: Selection::Colors(["#003b46".into(), "#1ab2ad".into()]),
                diff: None,
            }),
            config: None,
        },
        "low_contrast" => Theme {
            name: Some("low_contrast".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#bb0000", // red
                    "#00bb00", // green
                    "#bbbb00", // yellow
                    "#0000bb", // blue
                    "#bb00bb", // magenta
                    "#00bbbb", // cyan
                    "#bbbbbb", // white
                    "#bb7700", // orange
                    "#bb00aa", // pink
                ),
                bright: Some(new_term_colors(
                    "#222222", // black
                    "#dd2222", // red
                    "#22dd22", // green
                    "#dddd22", // yellow
                    "#2222dd", // blue
                    "#dd22dd", // magenta
                    "#22dddd", // cyan
                    "#dddddd", // white
                    "#dd9900", // orange
                    "#dd22cc", // pink
                )),
                dim: Some(new_term_colors(
                    "#000000", // black
                    "#990000", // red
                    "#009900", // green
                    "#999900", // yellow
                    "#000099", // blue
                    "#990099", // magenta
                    "#009999", // cyan
                    "#999999", // white
                    "#996600", // orange
                    "#990077", // pink
                )),
                comment: Some("#888888".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1f1f".into(),
                    "#333333".into(),
                    "#444444".into(),
                    "#555555".into(),
                    "#666666".into(),
                ]),
                foreground: Foreground::Colors([
                    "#bbbbbb".into(),
                    "#dddddd".into(),
                    "#ffffff".into(),
                    "#eeeeee".into(),
                ]),
                selection: Selection::Colors(["#666666".into(), "#bbbbbb".into()]),
                diff: None,
            }),
            config: None,
        },
        "konsole_port" => Theme {
            name: Some("konsole_port".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#b21818", // red
                    "#18b218", // green
                    "#b26818", // yellow
                    "#1818b2", // blue
                    "#b218b2", // magenta
                    "#18b2b2", // cyan
                    "#b2b2b2", // white
                    "#d27b2a", // orange
                    "#d22ad2", // pink
                ),
                bright: Some(new_term_colors(
                    "#686868", // black
                    "#ff5454", // red
                    "#54ff54", // green
                    "#ffff54", // yellow
                    "#5454ff", // blue
                    "#ff54ff", // magenta
                    "#54ffff", // cyan
                    "#ffffff", // white
                    "#ffaa33", // orange
                    "#ff55ff", // pink
                )),
                dim: Some(new_term_colors(
                    "#000000", // black
                    "#9c1414", // red
                    "#14a414", // green
                    "#996614", // yellow
                    "#14149c", // blue
                    "#9c149c", // magenta
                    "#14a4a4", // cyan
                    "#9c9c9c", // white
                    "#b26b20", // orange
                    "#b214b2", // pink
                )),
                comment: Some("#7d7d7d".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1f1f".into(),
                    "#2c2c2c".into(),
                    "#383838".into(),
                    "#444444".into(),
                    "#505050".into(),
                ]),
                foreground: Foreground::Colors([
                    "#b2b2b2".into(),
                    "#e3e3e3".into(),
                    "#ffffff".into(),
                    "#dddddd".into(),
                ]),
                selection: Selection::Colors(["#b26818".into(), "#b2b2b2".into()]),
                diff: None,
            }),
            config: None,
        },
        "kitty" => Theme {
            name: Some("kitty".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#cc0403", // red
                    "#19cb00", // green
                    "#cecb00", // yellow
                    "#0d73cc", // blue
                    "#cb1ed1", // magenta
                    "#0dcdcd", // cyan
                    "#dddddd", // white
                    "#ff7700", // orange
                    "#ff33aa", // pink
                ),
                bright: Some(new_term_colors(
                    "#767676", // black
                    "#f2201f", // red
                    "#23fd00", // green
                    "#fffd00", // yellow
                    "#1a8fff", // blue
                    "#fd28ff", // magenta
                    "#14ffff", // cyan
                    "#aaaaaa", // white
                    "#ffaa33", // orange
                    "#ff55ff", // pink
                )),
                dim: Some(new_term_colors(
                    "#000000", // black
                    "#990303", // red
                    "#149900", // green
                    "#999900", // yellow
                    "#0a4f99", // blue
                    "#990099", // magenta
                    "#0a9999", // cyan
                    "#bbbbbb", // white
                    "#994c00", // orange
                    "#990077", // pink
                )),
                comment: Some("#888888".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#111111".into(),
                    "#000000".into(),
                    "#222222".into(),
                    "#333333".into(),
                    "#444444".into(),
                ]),
                foreground: Foreground::Colors([
                    "#dddddd".into(),
                    "#cccccc".into(),
                    "#ffffff".into(),
                    "#aaaaaa".into(),
                ]),
                selection: Selection::Colors(["#272727".into(), "#555555".into()]),
                diff: None,
            }),
            config: None,
        },
        "kimbie_light" => Theme {
            name: Some("kimbie_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#fbebd4", // black
                    "#d43552", // red
                    "#b8bb26", // green
                    "#f0c674", // yellow
                    "#7cafc2", // blue
                    "#d3869b", // magenta
                    "#8abeb7", // cyan
                    "#6e5346", // white
                    "#e78a4f", // orange
                    "#d96fa5", // pink
                ),
                bright: Some(new_term_colors(
                    "#f7e4c6", // black
                    "#e04b68", // red
                    "#c0c838", // green
                    "#f3d087", // yellow
                    "#8bbad0", // blue
                    "#e19cb0", // magenta
                    "#9cd6cd", // cyan
                    "#4a3631", // white
                    "#f0a573", // orange
                    "#e38cbf", // pink
                )),
                dim: Some(new_term_colors(
                    "#e6d6bc", // black
                    "#b82f49", // red
                    "#a0a726", // green
                    "#d9b660", // yellow
                    "#6895a8", // blue
                    "#b97887", // magenta
                    "#7aa79f", // cyan
                    "#5a453b", // white
                    "#c9784a", // orange
                    "#b65f87", // pink
                )),
                comment: Some("#8c7b6b".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f9ead6".into(),
                    "#fbebd4".into(),
                    "#f5e1c2".into(),
                    "#f0d8b0".into(),
                    "#e5cba0".into(),
                ]),
                foreground: Foreground::Colors([
                    "#6e5346".into(),
                    "#6e5346".into(),
                    "#8c6a59".into(),
                    "#a17b65".into(),
                ]),
                selection: Selection::Colors(["#f0d8b0".into(), "#f3e0c1".into()]),
                diff: None,
            }),
            config: None,
        },
        "kimbie_dark" => Theme {
            name: Some("kimbie_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#221a0f", // black
                    "#c87e5a", // red
                    "#879a6b", // green
                    "#e4b581", // yellow
                    "#5d90cd", // blue
                    "#c792ea", // magenta
                    "#6bbab2", // cyan
                    "#d3af86", // white
                    "#d9915a", // orange
                    "#d982c8", // pink
                ),
                bright: Some(new_term_colors(
                    "#7d6f48", // black
                    "#dd8d6f", // red
                    "#9ab57d", // green
                    "#f0c497", // yellow
                    "#74a9e0", // blue
                    "#d9a7f0", // magenta
                    "#7dd0c9", // cyan
                    "#f2cca8", // white
                    "#e4a573", // orange
                    "#e3a0e0", // pink
                )),
                dim: Some(new_term_colors(
                    "#1c150c", // black
                    "#b0694f", // red
                    "#73845a", // green
                    "#cfa875", // yellow
                    "#4a74a0", // blue
                    "#aa6fca", // magenta
                    "#559896", // cyan
                    "#bfa87c", // white
                    "#c98253", // orange
                    "#b86fb2", // pink
                )),
                comment: Some("#7a6f5a".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1a150c".into(),
                    "#221a0f".into(),
                    "#2c2112".into(),
                    "#342813".into(),
                    "#42341e".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d3af86".into(),
                    "#d3af86".into(),
                    "#e0c399".into(),
                    "#f0d4aa".into(),
                ]),
                selection: Selection::Colors(["#3c2f1f".into(), "#5d482f".into()]),
                diff: None,
            }),
            config: None,
        },
        "kanagawa_wave" => Theme {
            name: Some("kanagawa_wave".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#090618", // black
                    "#c34043", // red
                    "#76946a", // green
                    "#c0a36e", // yellow
                    "#7e9cd8", // blue
                    "#957fb8", // magenta
                    "#6a9589", // cyan
                    "#c8c093", // white
                    "#ffa066", // orange
                    "#ff5d62", // pink
                ),
                bright: Some(new_term_colors(
                    "#727169", // black
                    "#e82424", // red
                    "#98bb6c", // green
                    "#e6c384", // yellow
                    "#7fb4ca", // blue
                    "#938aa9", // magenta
                    "#7aa89f", // cyan
                    "#dcd7ba", // white
                    "#ffb07f", // orange
                    "#ff7a80", // pink
                )),
                dim: Some(new_term_colors(
                    "#080512", // black
                    "#9b3033", // red
                    "#607856", // green
                    "#98855a", // yellow
                    "#6b7fae", // blue
                    "#7b6e94", // magenta
                    "#58766e", // cyan
                    "#b0a78c", // white
                    "#cc8553", // orange
                    "#cc4a52", // pink
                )),
                comment: Some("#7a726a".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1b1b23".into(),
                    "#1f1f28".into(),
                    "#292832".into(),
                    "#2d2c38".into(),
                    "#3a3845".into(),
                ]),
                foreground: Foreground::Colors([
                    "#c8c093".into(),
                    "#dcd7ba".into(),
                    "#e6d9a5".into(),
                    "#f0e8c0".into(),
                ]),
                selection: Selection::Colors(["#2d4f67".into(), "#c8c093".into()]),
                diff: None,
            }),
            config: None,
        },
        "kanagawa_dragon" => Theme {
            name: Some("kanagawa_dragon".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#0d0c0c", // black
                    "#c4746e", // red
                    "#8a9a7b", // green
                    "#c4b28a", // yellow
                    "#8ba4b0", // blue
                    "#a292a3", // magenta
                    "#8ea4a2", // cyan
                    "#C8C093", // white
                    "#ffa066", // orange
                    "#ff5d62", // pink
                ),
                bright: Some(new_term_colors(
                    "#a6a69c", // black
                    "#E46876", // red
                    "#87a987", // green
                    "#E6C384", // yellow
                    "#7FB4CA", // blue
                    "#938AA9", // magenta
                    "#7AA89F", // cyan
                    "#c5c9c5", // white
                    "#ffb07f", // orange
                    "#ff7a80", // pink
                )),
                dim: Some(new_term_colors(
                    "#0b0a0a", // black
                    "#9f5b57", // red
                    "#6e7b60", // green
                    "#9e8e6b", // yellow
                    "#6e8996", // blue
                    "#7f7180", // magenta
                    "#69837e", // cyan
                    "#b0aa83", // white
                    "#cc8553", // orange
                    "#cc4a52", // pink
                )),
                comment: Some("#7b7369".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#141313".into(),
                    "#181616".into(),
                    "#21201f".into(),
                    "#272524".into(),
                    "#353231".into(),
                ]),
                foreground: Foreground::Colors([
                    "#c8c093".into(),
                    "#c5c9c5".into(),
                    "#d0d3c5".into(),
                    "#dcdcdc".into(),
                ]),
                selection: Selection::Colors(["#2d4f67".into(), "#c8c093".into()]),
                diff: None,
            }),
            config: None,
        },
        "iterm2" => Theme {
            // OK
            name: Some("iterm2".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2e2e2e", // black
                    "#eb4129", // red
                    "#abe047", // green
                    "#f6c744", // yellow
                    "#47a0f3", // blue
                    "#7b5cb0", // magenta
                    "#64dbed", // cyan
                    "#e5e9f0", // white
                    "#f28c42", // orange
                    "#ec77d0", // pink
                ),
                bright: Some(new_term_colors(
                    "#565656", // black
                    "#ec5357", // red
                    "#c0e17d", // green
                    "#f9da6a", // yellow
                    "#49a4f8", // blue
                    "#a47de9", // magenta
                    "#99faf2", // cyan
                    "#ffffff", // white
                    "#f5a35b", // orange
                    "#b583ff", // pink
                )),
                dim: Some(new_term_colors(
                    "#1f1f1f", // black
                    "#c13a26", // red
                    "#93be3d", // green
                    "#d1ad3a", // yellow
                    "#3d86cc", // blue
                    "#6a4a96", // magenta
                    "#4fb3c3", // cyan
                    "#cfd4db", // white
                    "#d07a3a", // orange
                    "#b061a8", // pink
                )),
                comment: Some("#7a7a7a".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#0c101b".into(),
                    "#101421".into(),
                    "#181c2a".into(),
                    "#212535".into(),
                    "#34384a".into(),
                ]),
                foreground: Foreground::Colors([
                    "#f0ece6".into(),
                    "#fffbf6".into(),
                    "#b7b3ad".into(),
                    "#7a7a7a".into(),
                ]),
                selection: Selection::Colors(["#2a3f5f".into(), "#3b5d8a".into()]),
                diff: None,
            }),
            config: None,
        },
        "iris" => Theme {
            // OK
            name: Some("iris".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#111133", // black
                    "#d61d52", // red
                    "#48a842", // green
                    "#e1a51c", // yellow
                    "#5556d3", // blue
                    "#8650d3", // magenta
                    "#52afb7", // cyan
                    "#9f9aa7", // white
                    "#d67a1c", // orange
                    "#e15877", // pink
                ),
                bright: Some(new_term_colors(
                    "#484867", // black
                    "#e15877", // red
                    "#71ab3a", // green
                    "#c6a642", // yellow
                    "#6d6dc9", // blue
                    "#956ad3", // magenta
                    "#6ab6bd", // cyan
                    "#e8e6e9", // white
                    "#f0b25f", // orange
                    "#f58fc5", // pink
                )),
                dim: Some(new_term_colors(
                    "#18162a", // black
                    "#b31544", // red
                    "#3c8e32", // green
                    "#b89115", // yellow
                    "#4444a3", // blue
                    "#6b3fc1", // magenta
                    "#418989", // cyan
                    "#8b8890", // white
                    "#b36f14", // orange
                    "#b8499d", // pink
                )),
                comment: Some("#7a798e".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1d2e".into(),
                    "#272537".into(),
                    "#2d2b42".into(),
                    "#3a3858".into(),
                    "#4a4970".into(),
                ]),
                foreground: Foreground::Colors([
                    "#f0eff1".into(),
                    "#d1cfd7".into(),
                    "#b9b5c0".into(),
                    "#888495".into(),
                ]),
                selection: Selection::Colors(["#434168".into(), "#5556d3".into()]),
                diff: None,
            }),
            config: None,
        },
        "iceberg" => Theme {
            // OK
            name: Some("iceberg".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#1e2132", // black
                    "#e27878", // red
                    "#b4be82", // green
                    "#e2a478", // yellow
                    "#84a0c6", // blue
                    "#a093c7", // magenta
                    "#89b8c2", // cyan
                    "#c6c8d1", // white
                    "#d4a17a", // orange
                    "#c78ecf", // pink
                ),
                bright: Some(new_term_colors(
                    "#6b7089", // black
                    "#e98989", // red
                    "#c0ca8e", // green
                    "#e9b189", // yellow
                    "#91acd1", // blue
                    "#ada0d3", // magenta
                    "#95c4ce", // cyan
                    "#d2d4de", // white
                    "#e0b78f", // orange
                    "#bfa0e0", // pink
                )),
                dim: Some(new_term_colors(
                    "#161821", // black
                    "#bf6b6b", // red
                    "#9aa168", // green
                    "#c89363", // yellow
                    "#6b86a8", // blue
                    "#827eb0", // magenta
                    "#6e99a6", // cyan
                    "#aeb0ba", // white
                    "#b28861", // orange
                    "#9d7ecf", // pink
                )),
                comment: Some("#788097".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#161821".into(),
                    "#1e2132".into(),
                    "#272c42".into(),
                    "#2f3350".into(),
                    "#3b3f5c".into(),
                ]),
                foreground: Foreground::Colors([
                    "#aeb0ba".into(),
                    "#c6c8d1".into(),
                    "#d2d4de".into(),
                    "#e0e2eb".into(),
                ]),
                selection: Selection::Colors(["#272c42".into(), "#84a0c6".into()]),
                diff: None,
            }),
            config: None,
        },
        "hyper" => Theme {
            // OK
            name: Some("hyper".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#fe0100", // red
                    "#33ff00", // green
                    "#feff00", // yellow
                    "#0066ff", // blue
                    "#cc00ff", // magenta
                    "#00ffff", // cyan
                    "#d0d0d0", // white
                    "#ff8000", // orange
                    "#ff33cc", // pink
                ),
                bright: Some(new_term_colors(
                    "#808080", // black
                    "#fe0100", // red
                    "#33ff00", // green
                    "#feff00", // yellow
                    "#0066ff", // blue
                    "#cc00ff", // magenta
                    "#00ffff", // cyan
                    "#ffffff", // white
                    "#ff9933", // orange
                    "#ff66cc", // pink
                )),
                dim: Some(new_term_colors(
                    "#000000", // black
                    "#b30000", // red
                    "#29cc00", // green
                    "#b3b300", // yellow
                    "#0044cc", // blue
                    "#9900cc", // magenta
                    "#00cccc", // cyan
                    "#a0a0a0", // white
                    "#cc6600", // orange
                    "#cc33aa", // pink
                )),
                comment: Some("#666666".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#0a0a0a".into(),
                    "#000000".into(),
                    "#111111".into(),
                    "#1a1a1a".into(),
                    "#222222".into(),
                ]),
                foreground: Foreground::Colors([
                    "#a0a0a0".into(),
                    "#ffffff".into(),
                    "#d0d0d0".into(),
                    "#e0e0e0".into(),
                ]),
                selection: Selection::Colors(["#333333".into(), "#ffccff".into()]),
                diff: None,
            }),
            config: None,
        },
        "horizon_dark" => Theme {
            name: Some("horizon_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#16161c", // black
                    "#e95678", // red
                    "#29d398", // green
                    "#fab795", // yellow
                    "#26bbd9", // blue
                    "#ee64ac", // magenta
                    "#59e1e3", // cyan
                    "#d5d8da", // white
                    "#f6b26b", // orange
                    "#f075b5", // pink
                ),
                bright: Some(new_term_colors(
                    "#5b5858", // black
                    "#ec6a88", // red
                    "#3fdaa4", // green
                    "#fbc3a7", // yellow
                    "#3fc4de", // blue
                    "#f075b5", // magenta
                    "#6be4e6", // cyan
                    "#d5d8da", // white
                    "#f7c88f", // orange
                    "#f396d0", // pink
                )),
                dim: Some(new_term_colors(
                    "#101018", // black
                    "#bf425f", // red
                    "#22b38f", // green
                    "#d89e7a", // yellow
                    "#1f96b8", // blue
                    "#b84d92", // magenta
                    "#4fb7be", // cyan
                    "#bfc1c3", // white
                    "#c78b5f", // orange
                    "#9d3f90", // pink
                )),
                comment: Some("#6c7086".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1c1e26".into(),
                    "#16161c".into(),
                    "#222331".into(),
                    "#2a2b3a".into(),
                    "#333447".into(),
                ]),
                foreground: Foreground::Colors([
                    "#bfc1c3".into(),
                    "#e0e0e0".into(),
                    "#f0f0f0".into(),
                    "#d5d8da".into(),
                ]),
                selection: Selection::Colors(["#2a2b3a".into(), "#C4957E".into()]),
                diff: None,
            }),
            config: None,
        },
        "high_contrast" => Theme {
            name: Some("high_contrast".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#ff0000", // red
                    "#00ff00", // green
                    "#ffff00", // yellow
                    "#0000ff", // blue
                    "#ff00ff", // magenta
                    "#00ffff", // cyan
                    "#ffffff", // white
                    "#ff8000", // orange
                    "#ff5d62", // pink
                ),
                bright: Some(new_term_colors(
                    "#444444", // black
                    "#ff3333", // red
                    "#33ff33", // green
                    "#ffff33", // yellow
                    "#3333ff", // blue
                    "#ff33ff", // magenta
                    "#33ffff", // cyan
                    "#ffffff", // white
                    "#ff9933", // orange
                    "#ff7a70", // pink
                )),
                dim: Some(new_term_colors(
                    "#222222", // black
                    "#cc0000", // red
                    "#00cc00", // green
                    "#cccc00", // yellow
                    "#0000cc", // blue
                    "#cc00cc", // magenta
                    "#00cccc", // cyan
                    "#dddddd", // white
                    "#cc6600", // orange
                    "#cc4d52", // pink
                )),
                comment: Some("#888888".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#444444".into(),
                    "#333333".into(),
                    "#222222".into(),
                    "#111111".into(),
                    "#000000".into(),
                ]),
                foreground: Foreground::Colors([
                    "#dddddd".into(),
                    "#ffffff".into(),
                    "#eeeeee".into(),
                    "#cccccc".into(),
                ]),
                selection: Selection::Colors(["#666666".into(), "#ff5d62".into()]),
                diff: None,
            }),
            config: None,
        },
        "hardhacker" => Theme {
            name: Some("hardhacker".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#282433", // black
                    "#e965a5", // red
                    "#b1f2a7", // green
                    "#ebde76", // yellow
                    "#b1baf4", // blue
                    "#e192ef", // magenta
                    "#b3f4f3", // cyan
                    "#eee9fc", // white
                    "#ff5d62", // orange
                    "#f28ce3", // pink
                ),
                bright: Some(new_term_colors(
                    "#3f3951", // black
                    "#f08ac2", // red
                    "#c0fbbf", // green
                    "#f0e896", // yellow
                    "#c3c6f7", // blue
                    "#eaacef", // magenta
                    "#c4f8f8", // cyan
                    "#fff9ff", // white
                    "#ff7b70", // orange
                    "#f4a0f0", // pink
                )),
                dim: Some(new_term_colors(
                    "#1e1b2b", // black
                    "#c4558c", // red
                    "#8cd88d", // green
                    "#c9c25f", // yellow
                    "#9398d0", // blue
                    "#b172c7", // magenta
                    "#92c9c8", // cyan
                    "#d9d3e6", // white
                    "#d94849", // orange
                    "#c472c9", // pink
                )),
                comment: Some("#8b8699".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1d1a26".into(),
                    "#282433".into(),
                    "#322f3d".into(),
                    "#3c3848".into(),
                    "#464153".into(),
                ]),
                foreground: Foreground::Colors([
                    "#e0dafc".into(),
                    "#eee9fc".into(),
                    "#f5f0fd".into(),
                    "#fbf7ff".into(),
                ]),
                selection: Selection::Colors(["#423E51".into(), "#514C61".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_material_medium_light" => Theme {
            name: Some("gruvbox_material_medium_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#654735", // black
                    "#c14a4a", // red
                    "#6c782e", // green
                    "#b47109", // yellow
                    "#45707a", // blue
                    "#945e80", // magenta
                    "#4c7a5d", // cyan
                    "#eee0b7", // white
                    "#d68f4a", // orange
                    "#b46480", // pink
                ),
                bright: Some(new_term_colors(
                    "#7b5f48", // black
                    "#d55b5b", // red
                    "#7f8d34", // green
                    "#c5881a", // yellow
                    "#598994", // blue
                    "#a16c8e", // magenta
                    "#5e9270", // cyan
                    "#f3e8c2", // white
                    "#e0a060", // orange
                    "#c27598", // pink
                )),
                dim: Some(new_term_colors(
                    "#55412f", // black
                    "#a33838", // red
                    "#5c6225", // green
                    "#946b07", // yellow
                    "#3b5a63", // blue
                    "#804968", // magenta
                    "#3e614b", // cyan
                    "#dcd1a1", // white
                    "#b5763a", // orange
                    "#924a69", // pink
                )),
                comment: Some("#7c6f5a".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f6efd5".into(),
                    "#fbf1c7".into(),
                    "#f1e5b8".into(),
                    "#e8dba8".into(),
                    "#dcd0a0".into(),
                ]),
                foreground: Foreground::Colors([
                    "#9c7d65".into(),
                    "#654735".into(),
                    "#8f6350".into(),
                    "#a3735f".into(),
                ]),
                selection: Selection::Colors(["#f0e1b0".into(), "#d68f4a".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_material_medium_dark" => Theme {
            name: Some("gruvbox_material_medium_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#3c3836", // black
                    "#ea6962", // red
                    "#a9b665", // green
                    "#d8a657", // yellow
                    "#7daea3", // blue
                    "#d3869b", // magenta
                    "#89b482", // cyan
                    "#d4be98", // white
                    "#e5a663", // orange
                    "#d48eb2", // pink
                ),
                bright: Some(new_term_colors(
                    "#504b48", // black
                    "#f07a74", // red
                    "#bcc87b", // green
                    "#e2b36a", // yellow
                    "#90c1b8", // blue
                    "#e0aeb1", // magenta
                    "#9cc89a", // cyan
                    "#e7d9b2", // white
                    "#f0bc80", // orange
                    "#e8aac9", // pink
                )),
                dim: Some(new_term_colors(
                    "#2f2a28", // black
                    "#c85b54", // red
                    "#87924d", // green
                    "#b58e45", // yellow
                    "#658a87", // blue
                    "#aa7b82", // magenta
                    "#6d9971", // cyan
                    "#bcae80", // white
                    "#c78d4e", // orange
                    "#b87c92", // pink
                )),
                comment: Some("#7c756c".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1f1c1b".into(),
                    "#282828".into(),
                    "#35312f".into(),
                    "#3c3836".into(),
                    "#504a48".into(),
                ]),
                foreground: Foreground::Colors([
                    "#cfc1a4".into(),
                    "#d4be98".into(),
                    "#e0cfae".into(),
                    "#d8c598".into(),
                ]),
                selection: Selection::Colors(["#504b48".into(), "#e5a663".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_material_hard_light" => Theme {
            name: Some("gruvbox_material_hard_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#654735", // black
                    "#c14a4a", // red
                    "#6c782e", // green
                    "#b47109", // yellow
                    "#45707a", // blue
                    "#945e80", // magenta
                    "#4c7a5d", // cyan
                    "#f2e5bc", // white
                    "#d48b34", // orange
                    "#c46992", // pink
                ),
                bright: Some(new_term_colors(
                    "#7a5e49", // black
                    "#d16060", // red
                    "#7b8538", // green
                    "#c68b1f", // yellow
                    "#598b96", // blue
                    "#aa6b95", // magenta
                    "#60936a", // cyan
                    "#f5e8c0", // white
                    "#e39d51", // orange
                    "#d984ab", // pink
                )),
                dim: Some(new_term_colors(
                    "#57412f", // black
                    "#a23c3c", // red
                    "#5a6522", // green
                    "#996202", // yellow
                    "#365e66", // blue
                    "#7b4b6a", // magenta
                    "#3f6650", // cyan
                    "#dbd0a5", // white
                    "#b26e28", // orange
                    "#a8557c", // pink
                )),
                comment: Some("#8c816e".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f3ebce".into(),
                    "#f9f5d7".into(),
                    "#f0e7c3".into(),
                    "#e8dfb1".into(),
                    "#d6cca1".into(),
                ]),
                foreground: Foreground::Colors([
                    "#aa945f".into(),
                    "#654735".into(),
                    "#8c5a3d".into(),
                    "#a06c4e".into(),
                ]),
                selection: Selection::Colors(["#f2e5bc".into(), "#d48b34".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_material_hard_dark" => Theme {
            name: Some("gruvbox_material_hard_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#32302f", // black
                    "#ea6962", // red
                    "#a9b665", // green
                    "#d8a657", // yellow
                    "#7daea3", // blue
                    "#d3869b", // magenta
                    "#89b482", // cyan
                    "#d4be98", // white
                    "#e0af68", // orange
                    "#d4879f", // pink
                ),
                bright: Some(new_term_colors(
                    "#45423f", // black
                    "#f28c86", // red
                    "#c0d07f", // green
                    "#e0b878", // yellow
                    "#92c2bb", // blue
                    "#e0a9b8", // magenta
                    "#a1c7a0", // cyan
                    "#e6d8b2", // white
                    "#f2c27f", // orange
                    "#e3a8c0", // pink
                )),
                dim: Some(new_term_colors(
                    "#262422", // black
                    "#c75a53", // red
                    "#8ea04c", // green
                    "#b78f41", // yellow
                    "#60908d", // blue
                    "#b17886", // magenta
                    "#6f9b6e", // cyan
                    "#bfa87e", // white
                    "#c99654", // orange
                    "#b87592", // pink
                )),
                comment: Some("#7c7058".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1a1d1c".into(),
                    "#1d2021".into(),
                    "#262827".into(),
                    "#2f3230".into(),
                    "#3a3d3b".into(),
                ]),
                foreground: Foreground::Colors([
                    "#cfc29d".into(),
                    "#d4be98".into(),
                    "#e0cfa7".into(),
                    "#e8d9b1".into(),
                ]),
                selection: Selection::Colors(["#3a3d3b".into(), "#e0af68".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_material_dark_medium" => Theme {
            name: Some("gruvbox_material_dark_medium".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#665c54", // black
                    "#ea6962", // red
                    "#a9b665", // green
                    "#e78a4e", // yellow
                    "#7daea3", // blue
                    "#d3869b", // magenta
                    "#89b482", // cyan
                    "#dfbf8e", // white
                    "#e78a4e", // orange
                    "#d3869b", // pink
                ),
                bright: Some(new_term_colors(
                    "#928374", // black
                    "#f07a73", // red
                    "#b7c97a", // green
                    "#e3a84e", // yellow
                    "#8fc0b6", // blue
                    "#e09fb0", // magenta
                    "#9acfa0", // cyan
                    "#ead5b7", // white
                    "#f0a35a", // orange
                    "#e6a6bd", // pink
                )),
                dim: Some(new_term_colors(
                    "#504945", // black
                    "#c85f59", // red
                    "#8f9f5f", // green
                    "#c7783f", // yellow
                    "#6b9c92", // blue
                    "#b97386", // magenta
                    "#739e78", // cyan
                    "#c9ad7f", // white
                    "#c7763f", // orange
                    "#b86f86", // pink
                )),
                comment: Some("#928374".into()),
                variable: None,
                status_line: Some("#32302f".into()),
                background: Background::Colors([
                    "#1d2021".into(),
                    "#282828".into(),
                    "#32302f".into(),
                    "#3c3836".into(),
                    "#504945".into(),
                ]),
                foreground: Foreground::Colors([
                    "#bdae93".into(),
                    "#dfbf8e".into(),
                    "#ead5b7".into(),
                    "#a89984".into(),
                ]),
                selection: Selection::Colors(["#3c3836".into(), "#504945".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_light" => Theme {
            // Ok
            name: Some("gruvbox_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#fbf1c7", // black
                    "#cc241d", // red
                    "#98971a", // green
                    "#d79921", // yellow
                    "#458588", // blue
                    "#b16286", // magenta
                    "#689d6a", // cyan
                    "#7c6f64", // white
                    "#d65d0e", // orange
                    "#b16286", // pink
                ),
                bright: Some(new_term_colors(
                    "#928374", // black
                    "#9d0006", // red
                    "#79740e", // green
                    "#b57614", // yellow
                    "#076678", // blue
                    "#8f3f71", // magenta
                    "#427b58", // cyan
                    "#3c3836", // white
                    "#af3a03", // orange
                    "#9d3c74", // pink
                )),
                dim: Some(new_term_colors(
                    "#f2e5bc", // black
                    "#b01c1a", // red
                    "#878a16", // green
                    "#c28a1c", // yellow
                    "#3a7477", // blue
                    "#9d5676", // magenta
                    "#5a8c5f", // cyan
                    "#665c54", // white
                    "#bf4a0a", // orange
                    "#a6557a", // pink
                )),
                comment: Some("#928374".into()),
                variable: None,
                status_line: Some("#ebdbb2".into()),
                background: Background::Colors([
                    "#f2e5bc".into(),
                    "#fbf1c7".into(),
                    "#f7e9b5".into(),
                    "#ebdbb2".into(),
                    "#d5c4a1".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d5c4a1".into(),
                    "#3c3836".into(),
                    "#504945".into(),
                    "#7c6f64".into(),
                ]),
                selection: Selection::Colors(["#ebdbb2".into(), "#d5c4a1".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruvbox_dark" => Theme {
            // Ok
            name: Some("gruvbox_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#282828", // black
                    "#cc241d", // red
                    "#98971a", // green
                    "#d79921", // yellow
                    "#458588", // blue
                    "#b16286", // magenta
                    "#689d6a", // cyan
                    "#a89984", // white
                    "#fe8019", // orange
                    "#d65d8f", // pink
                ),
                bright: Some(new_term_colors(
                    "#928374", // black
                    "#fb4934", // red
                    "#b8bb26", // green
                    "#fabd2f", // yellow
                    "#83a598", // blue
                    "#d3869b", // magenta
                    "#8ec07c", // cyan
                    "#ebdbb2", // white
                    "#ff9f43", // orange
                    "#e28fb0", // pink
                )),
                dim: Some(new_term_colors(
                    "#1d2021", // black
                    "#9d1f1a", // red
                    "#7c7a14", // green
                    "#b37b1a", // yellow
                    "#3c6f72", // blue
                    "#8f4a6b", // magenta
                    "#5a7f58", // cyan
                    "#928374", // white
                    "#c56a12", // orange
                    "#b04b78", // pink
                )),
                comment: Some("#7c6f64".into()),
                variable: None,
                status_line: Some("#3c3836".into()),
                background: Background::Colors([
                    "#1d2021".into(),
                    "#282828".into(),
                    "#32302f".into(),
                    "#3c3836".into(),
                    "#504945".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d5c4a1".into(),
                    "#ebdbb2".into(),
                    "#bdae93".into(),
                    "#bdae93".into(),
                ]),
                selection: Selection::Colors(["#3c3836".into(), "#504945".into()]),
                diff: None,
            }),
            config: None,
        },
        "gruber_darker" => Theme {
            // Ok
            name: Some("gruber_darker".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#181818", // black
                    "#F43841", // red
                    "#73D936", // green
                    "#FFDD33", // yellow
                    "#96A6C8", // blue
                    "#9E95C7", // magenta
                    "#95A99F", // cyan
                    "#E4E4E4", // white
                    "#FF9C3A", // orange
                    "#E89AC7", // pink
                ),
                bright: Some(new_term_colors(
                    "#52494E", // black
                    "#FF4F58", // red
                    "#8EEB5A", // green
                    "#FFE066", // yellow
                    "#A9B7D9", // blue
                    "#AFAFD7", // magenta
                    "#A8BDB3", // cyan
                    "#F5F5F5", // white
                    "#FFB86C", // orange
                    "#F2B0D8", // pink
                )),
                dim: Some(new_term_colors(
                    "#121212", // black
                    "#C12F36", // red
                    "#73D936", // green
                    "#FFDD33", // yellow
                    "#96A6C8", // blue
                    "#847DAA", // magenta
                    "#95A99F", // cyan
                    "#CFCFCF", // white
                    "#D98730", // orange
                    "#C987B0", // pink
                )),
                comment: Some("#7A7A7A".into()),
                variable: None,
                status_line: Some("#222222".into()),
                background: Background::Colors([
                    "#101010".into(),
                    "#181818".into(),
                    "#202020".into(),
                    "#262626".into(),
                    "#303030".into(),
                ]),
                foreground: Foreground::Colors([
                    "#CFCFCF".into(),
                    "#E4E4E4".into(),
                    "#F0F0F0".into(),
                    "#B8B8B8".into(),
                ]),
                selection: Selection::Colors(["#2A2A2A".into(), "#3A3A3A".into()]),
                diff: None,
            }),
            config: None,
        },
        "gotham" => Theme {
            // Ok
            name: Some("gotham".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#0a0f14", // black
                    "#c33027", // red
                    "#26a98b", // green
                    "#edb54b", // yellow
                    "#227196", // blue
                    "#4e5165", // magenta
                    "#33859d", // cyan
                    "#98d1ce", // white
                    "#d26939", // orange
                    "#c07bdc", // pink
                ),
                bright: Some(new_term_colors(
                    "#10151b", // black
                    "#d26939", // red
                    "#081f2d", // green
                    "#245361", // yellow
                    "#0c4d6b", // blue
                    "#888ba5", // magenta
                    "#599caa", // cyan
                    "#d3ebe9", // white
                    "#e08a5b", // orange
                    "#d28ae8", // pink
                )),
                dim: Some(new_term_colors(
                    "#070b0f", // black
                    "#9e241d", // red
                    "#1e7f6a", // green
                    "#b58a38", // yellow
                    "#164a61", // blue
                    "#3e4154", // magenta
                    "#286a7c", // cyan
                    "#7fb3b0", // white
                    "#b8582f", // orange
                    "#9f5fb6", // pink
                )),
                comment: Some("#4a5a63".into()),
                variable: None,
                status_line: Some("#10151b".into()),
                background: Background::Colors([
                    "#070b0f".into(),
                    "#0a0f14".into(),
                    "#10151b".into(),
                    "#141b22".into(),
                    "#1b242c".into(),
                ]),
                foreground: Foreground::Colors([
                    "#7fb3b0".into(),
                    "#98d1ce".into(),
                    "#d3ebe9".into(),
                    "#b6e3e0".into(),
                ]),
                selection: Selection::Colors(["#1b3a42".into(), "#245361".into()]),
                diff: None,
            }),
            config: None,
        },
        "google" => Theme {
            // OK
            name: Some("google".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#1d1f21", // black
                    "#cc342b", // red
                    "#198844", // green
                    "#fba922", // yellow
                    "#3971ed", // blue
                    "#a36ac7", // magenta
                    "#3971ed", // cyan
                    "#c5c8c6", // white
                    "#f96f1c", // orange
                    "#d07acb", // pink
                ),
                bright: Some(new_term_colors(
                    "#969896", // black
                    "#cc342b", // red
                    "#198844", // green
                    "#fba922", // yellow
                    "#3971ed", // blue
                    "#a36ac7", // magenta
                    "#3971ed", // cyan
                    "#ffffff", // white
                    "#ff8a3d", // orange
                    "#d07acb", // pink
                )),
                dim: Some(new_term_colors(
                    "#151718", // black
                    "#9e2821", // red
                    "#146c36", // green
                    "#c88a1c", // yellow
                    "#2c5bbf", // blue
                    "#80529e", // magenta
                    "#2c5bbf", // cyan
                    "#9ea1a0", // white
                    "#c75a15", // orange
                    "#a85a9f", // pink
                )),
                comment: Some("#7c7f7d".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#151718".into(),
                    "#1d1f21".into(),
                    "#242628".into(),
                    "#2c2f31".into(),
                    "#3a3d3f".into(),
                ]),
                foreground: Foreground::Colors([
                    "#9ea1a0".into(),
                    "#c5c8c6".into(),
                    "#e0e2e0".into(),
                    "#ffffff".into(),
                ]),
                selection: Selection::Colors(["#373b41".into(), "#4b5056".into()]),
                diff: None,
            }),
            config: None,
        },
        "rose_pine" => Theme {
            // OK
            name: Some("rose_pine".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#26233a", // black
                    "#eb6f92", // red
                    "#31748f", // green
                    "#f6c177", // yellow
                    "#9ccfd8", // blue
                    "#c4a7e7", // magenta
                    "#ebbcba", // cyan
                    "#e0def4", // white
                    "#f6c177", // orange
                    "#eb6f92", // pink
                ),
                bright: Some(new_term_colors(
                    "#6e6a86", // black
                    "#eb6f92", // red
                    "#3e8fb0", // green
                    "#f6c177", // yellow
                    "#9ccfd8", // blue
                    "#c4a7e7", // magenta
                    "#ebbcba", // cyan
                    "#f2e9e1", // white
                    "#ffcb8b", // orange
                    "#f29ac1", // pink
                )),
                dim: Some(new_term_colors(
                    "#1f1d2e", // black
                    "#b4637a", // red
                    "#286983", // green
                    "#d1a36a", // yellow
                    "#7fb4c2", // blue
                    "#9d87c9", // magenta
                    "#d8a39d", // cyan
                    "#b8b5cf", // white
                    "#c89b6a", // orange
                    "#c77b9c", // pink
                )),
                comment: Some("#908caa".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#191724".into(),
                    "#191724".into(),
                    "#1f1d2e".into(),
                    "#26233a".into(),
                    "#403d52".into(),
                ]),
                foreground: Foreground::Colors([
                    "#e0def4".into(),
                    "#e0def4".into(),
                    "#c8c5dd".into(),
                    "#908caa".into(),
                ]),
                selection: Selection::Colors(["#403d52".into(), "#524f67".into()]),
                diff: None,
            }),
            config: None,
        },
        "rose_pine_dawn" => Theme {
            // OK
            name: Some("rose_pine_dawn".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#f2e9e1", // black
                    "#b4637a", // red
                    "#286983", // green
                    "#ea9d34", // yellow
                    "#56949f", // blue
                    "#907aa9", // magenta
                    "#d7827e", // cyan
                    "#575279", // white
                    "#ea9d34", // orange
                    "#d7827e", // pink
                ),
                bright: Some(new_term_colors(
                    "#9893a5", // black
                    "#b4637a", // red
                    "#286983", // green
                    "#ea9d34", // yellow
                    "#56949f", // blue
                    "#907aa9", // magenta
                    "#d7827e", // cyan
                    "#575279", // white
                    "#ea9d34", // orange
                    "#d7827e", // pink
                )),
                dim: Some(new_term_colors(
                    "#e6dfd9", // black
                    "#c17d8f", // red
                    "#4a7f95", // green
                    "#f1b562", // yellow
                    "#7aaab3", // blue
                    "#a995bd", // magenta
                    "#e2a09c", // cyan
                    "#6e6a86", // white
                    "#f1b562", // orange
                    "#e2a09c", // pink
                )),
                comment: Some("#797593".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f2e9e1".into(),
                    "#faf4ed".into(),
                    "#f4ede8".into(),
                    "#efe9e6".into(),
                    "#cecacd".into(),
                ]),
                foreground: Foreground::Colors([
                    "#6e6a86".into(),
                    "#575279".into(),
                    "#4a4661".into(),
                    "#9893a5".into(),
                ]),
                selection: Selection::Colors(["#dfdad9".into(), "#cecacd".into()]),
                diff: None,
            }),
            config: None,
        },
        "tokyo_night" => Theme {
            // OK
            name: Some("Tokyo Night".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#32344a", // black
                    "#f7768e", // red
                    "#9ece6a", // green
                    "#e0af68", // yellow
                    "#7aa2f7", // blue
                    "#ad8ee6", // magenta
                    "#449dab", // cyan
                    "#787c99", // white
                    "#ff9e64", // orange
                    "#ff7a93", // pink
                ),
                bright: Some(new_term_colors(
                    "#444b6a", // black
                    "#ff7a93", // red
                    "#b9f27c", // green
                    "#ff9e64", // yellow
                    "#7da6ff", // blue
                    "#bb9af7", // magenta
                    "#0db9d7", // cyan
                    "#acb0d0", // white
                    "#ffb378", // orange
                    "#ff9eb8", // pink
                )),
                dim: Some(new_term_colors(
                    "#2a2e42", // black
                    "#c35a6a", // red
                    "#7aa25c", // green
                    "#b18a55", // yellow
                    "#5c7fd9", // blue
                    "#8c6cc3", // magenta
                    "#357f8a", // cyan
                    "#5c607a", // white
                    "#cc7f4a", // orange
                    "#c8647a", // pink
                )),
                comment: Some("#565f89".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#16161e".into(),
                    "#1a1b26".into(),
                    "#1f2335".into(),
                    "#24283b".into(),
                    "#414868".into(),
                ]),
                foreground: Foreground::Colors([
                    "#c0caf5".into(),
                    "#a9b1d6".into(),
                    "#9aa5ce".into(),
                    "#737aa2".into(),
                ]),
                selection: Selection::Colors(["#2e3c64".into(), "#3d59a1".into()]),
                diff: None,
            }),
            config: None,
        },
        "ubuntu" => Theme {
            // OK
            name: Some("ubuntu".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#2e3436", // black
                    "#cc0000", // red
                    "#4e9a06", // green
                    "#c4a000", // yellow
                    "#3465a4", // blue
                    "#75507b", // magenta
                    "#06989a", // cyan
                    "#d3d7cf", // white
                    "#ce5c00", // orange
                    "#ef2929", // pink
                ),
                bright: Some(new_term_colors(
                    "#555753", // black
                    "#ef2929", // red
                    "#8ae234", // green
                    "#fce94f", // yellow
                    "#729fcf", // blue
                    "#ad7fa8", // magenta
                    "#34e2e2", // cyan
                    "#eeeeec", // white
                    "#f57900", // orange
                    "#fcaf3e", // pink
                )),
                dim: Some(new_term_colors(
                    "#1c1f21", // black
                    "#8f0000", // red
                    "#3a6e03", // green
                    "#8a7300", // yellow
                    "#274b7a", // blue
                    "#5c3f61", // magenta
                    "#046b6c", // cyan
                    "#a7aba3", // white
                    "#8f4a00", // orange
                    "#c17d11", // pink
                )),
                comment: Some("#888a85".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#2a0a1f".into(),
                    "#300a24".into(),
                    "#3a0f2b".into(),
                    "#431235".into(),
                    "#75507b".into(),
                ]),
                foreground: Foreground::Colors([
                    "#f6f6f4".into(),
                    "#eeeeec".into(),
                    "#c0c2bd".into(),
                    "#888a85".into(),
                ]),
                selection: Selection::Colors(["#4a1239".into(), "#84A6D3".into()]),
                diff: None,
            }),
            config: None,
        },
        "vesper" => Theme {
            // OK
            name: Some("vesper".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#101010", // black
                    "#f5a191", // red
                    "#90b99f", // green
                    "#e6b99d", // yellow
                    "#aca1cf", // blue
                    "#e29eca", // magenta
                    "#ea83a5", // cyan
                    "#a0a0a0", // white
                    "#e6b99d", // orange
                    "#e29eca", // pink
                ),
                bright: Some(new_term_colors(
                    "#7e7e7e", // black
                    "#ff8080", // red
                    "#99ffe4", // green
                    "#ffc799", // yellow
                    "#b9aeda", // blue
                    "#ecaad6", // magenta
                    "#f591b2", // cyan
                    "#ffffff", // white
                    "#ffc799", // orange
                    "#ecaad6", // pink
                )),
                dim: Some(new_term_colors(
                    "#0c0c0c", // black
                    "#c18478", // red
                    "#6f8f7b", // green
                    "#b89476", // yellow
                    "#857da3", // blue
                    "#b97fa7", // magenta
                    "#b5667f", // cyan
                    "#6f6f6f", // white
                    "#b89476", // orange
                    "#b97fa7", // pink
                )),
                comment: Some("#6f6f6f".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#0c0c0c".into(),
                    "#101010".into(),
                    "#141414".into(),
                    "#181818".into(),
                    "#2a2a2a".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d0d0d0".into(),
                    "#ffffff".into(),
                    "#b0b0b0".into(),
                    "#6f6f6f".into(),
                ]),
                selection: Selection::Colors(["#1f1f1f".into(), "#2a2a2a".into()]),
                diff: None,
            }),
            config: None,
        },
        "vscode_dark_plus" => Theme {
            // OK
            name: Some("vscode_dark_plus".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#000000", // black
                    "#cd3131", // red
                    "#0dbc79", // green
                    "#e5e510", // yellow
                    "#2472c8", // blue
                    "#bc3fbc", // magenta
                    "#11a8cd", // cyan
                    "#e5e5e5", // white
                    "#ce9178", // orange
                    "#c586c0", // pink
                ),
                bright: Some(new_term_colors(
                    "#666666", // black
                    "#f14c4c", // red
                    "#23d18b", // green
                    "#f5f543", // yellow
                    "#3b8eea", // blue
                    "#d670d6", // magenta
                    "#29b8db", // cyan
                    "#ffffff", // white
                    "#d19a66", // orange
                    "#d16d9e", // pink
                )),
                dim: Some(new_term_colors(
                    "#1e1e1e", // black
                    "#8b2b2b", // red
                    "#0b8f63", // green
                    "#b5b510", // yellow
                    "#1f5fa5", // blue
                    "#8f2f8f", // magenta
                    "#0e7f9a", // cyan
                    "#bdbdbd", // white
                    "#b07d62", // orange
                    "#a56a9a", // pink
                )),
                comment: Some("#6a9955".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#252526".into(),
                    "#1e1e1e".into(),
                    "#2a2a2a".into(),
                    "#2f2f2f".into(),
                    "#3c3c3c".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d4d4d4".into(),
                    "#cccccc".into(),
                    "#b3b3b3".into(),
                    "#858585".into(),
                ]),
                selection: Selection::Colors(["#264f78".into(), "#04395e".into()]),
                diff: None,
            }),
            config: None,
        },
        "github_light" => Theme {
            // OK
            name: Some("github_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#24292e", // black
                    "#d73a49", // red
                    "#28a745", // green
                    "#dbab09", // yellow
                    "#0366d6", // blue
                    "#5a32a3", // magenta
                    "#0598bc", // cyan
                    "#6a737d", // white
                    "#d18616", // orange
                    "#cb2431", // pink
                ),
                bright: Some(new_term_colors(
                    "#959da5", // black
                    "#cb2431", // red
                    "#22863a", // green
                    "#b08800", // yellow
                    "#005cc5", // blue
                    "#5a32a3", // magenta
                    "#3192aa", // cyan
                    "#d1d5da", // white
                    "#d18616", // orange
                    "#cb2431", // pink
                )),
                dim: Some(new_term_colors(
                    "#6a737d", // black
                    "#e5534b", // red
                    "#2ea043", // green
                    "#c9a40c", // yellow
                    "#3182f7", // blue
                    "#7b4bd3", // magenta
                    "#37a0b5", // cyan
                    "#8b949e", // white
                    "#d18616", // orange
                    "#cb2431", // pink
                )),
                comment: Some("#6a737d".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f6f8fa".into(),
                    "#ffffff".into(),
                    "#f0f3f6".into(),
                    "#f6f8fa".into(),
                    "#e1e4e8".into(),
                ]),
                foreground: Foreground::Colors([
                    "#c9d1d9".into(),
                    "#24292f".into(),
                    "#6a737d".into(),
                    "#959da5".into(),
                ]),
                selection: Selection::Colors(["#d1d5da".into(), "#ffd33d".into()]),
                diff: None,
            }),
            config: None,
        },
        "github_dark" => Theme {
            // OK
            name: Some("github_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#586069", // black
                    "#ea4a5a", // red
                    "#34d058", // green
                    "#ffea7f", // yellow
                    "#2188ff", // blue
                    "#b392f0", // magenta
                    "#39c5cf", // cyan
                    "#d1d5da", // white
                    "#d18616", // orange
                    "#f97583", // pink
                ),
                bright: Some(new_term_colors(
                    "#959da5", // black
                    "#f97583", // red
                    "#85e89d", // green
                    "#ffea7f", // yellow
                    "#79b8ff", // blue
                    "#b392f0", // magenta
                    "#56d4dd", // cyan
                    "#fafbfc", // white
                    "#d18616", // orange
                    "#f97583", // pink
                )),
                dim: Some(new_term_colors(
                    "#6e7781", // black
                    "#f28b95", // red
                    "#5fd68b", // green
                    "#fff3a0", // yellow
                    "#5aa0ff", // blue
                    "#c8a0f8", // magenta
                    "#50cddf", // cyan
                    "#c9d1d9", // white
                    "#d18616", // orange
                    "#f97583", // pink
                )),
                comment: Some("#8b949e".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1b1f23".into(),
                    "#24292e".into(),
                    "#2c313a".into(),
                    "#2a2f36".into(),
                    "#444c56".into(),
                ]),
                foreground: Foreground::Colors([
                    "#c9d1d9".into(),
                    "#d1d5da".into(),
                    "#959da5".into(),
                    "#6e7781".into(),
                ]),
                selection: Selection::Colors(["#444c56".into(), "#f2e5bc".into()]),
                diff: None,
            }),
            config: None,
        },
        "autumn" => Theme {
            // OK
            name: Some("autumn".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#212121", // black
                    "#F05E48", // red
                    "#99be70", // green
                    "#FAD566", // yellow
                    "#86c1b9", // blue
                    "#cfba8b", // magenta
                    "#72a59e", // cyan
                    "#c8c8c8", // white
                    "#d97706", // orange
                    "#f4a7b9", // pink
                ),
                bright: Some(new_term_colors(
                    "#404040", // black
                    "#F05E48", // red
                    "#99be70", // green
                    "#ffff9f", // yellow
                    "#86c1b9", // blue
                    "#cfba8b", // magenta
                    "#72a59e", // cyan
                    "#e8e8e8", // white
                    "#f59e0b", // orange
                    "#f472b6", // pink
                )),
                dim: Some(new_term_colors(
                    "#2a2a2a", // black
                    "#d94b3a", // red
                    "#89aa5f", // green
                    "#e0c555", // yellow
                    "#73a3a0", // blue
                    "#bfa873", // magenta
                    "#60908e", // cyan
                    "#a8a8a8", // white
                    "#b85d00", // orange
                    "#e68ca5", // pink
                )),
                comment: Some("#999470".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#232323".into(),
                    "#232323".into(),
                    "#2e2e2e".into(),
                    "#2b2b2b".into(),
                    "#3a3a3a".into(),
                ]),
                foreground: Foreground::Colors([
                    "#f3f2cc".into(),
                    "#f3f2cc".into(),
                    "#c8c8c8".into(),
                    "#a8a8a8".into(),
                ]),
                selection: Selection::Colors(["#44442a".into(), "#5e5e38".into()]),
                diff: None,
            }),
            config: None,
        },
        "ashes_light" => Theme {
            // OK
            name: Some("ashes_light".into()),
            light: Some(true),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#f3f4f5", // black
                    "#b57e6b", // red
                    "#6bb57e", // green
                    "#b5b76b", // yellow
                    "#b56bb5", // blue
                    "#b56b8c", // magenta
                    "#6bb5b5", // cyan
                    "#565e65", // white
                    "#d7943e", // orange
                    "#d86b91", // pink
                ),
                bright: Some(new_term_colors(
                    "#9aa0a6", // black
                    "#c7ae95", // red
                    "#95c7ae", // green
                    "#aec795", // yellow
                    "#ae95c7", // blue
                    "#c795ae", // magenta
                    "#95aec7", // cyan
                    "#f3f4f5", // white
                    "#e0af68", // orange
                    "#d3869b", // pink
                )),
                dim: Some(new_term_colors(
                    "#d0d3d6", // black
                    "#c0a38c", // red
                    "#8cc0a3", // green
                    "#bcc08c", // yellow
                    "#c08cc0", // blue
                    "#c08cac", // magenta
                    "#8cc0c0", // cyan
                    "#a0a8b0", // white
                    "#d7aa6f", // orange
                    "#d78ca7", // pink
                )),
                comment: Some("#7a828b".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#f3f4f5".into(),
                    "#f3f4f5".into(),
                    "#e1e3e5".into(),
                    "#d6d8da".into(),
                    "#c0c3c6".into(),
                ]),
                foreground: Foreground::Colors([
                    "#565e65".into(),
                    "#565e65".into(),
                    "#3e454c".into(),
                    "#747c84".into(),
                ]),
                selection: Selection::Colors(["#d6d8da".into(), "#aec795".into()]),
                diff: None,
            }),
            config: None,
        },
        "ashes_dark" => Theme {
            // OK
            name: Some("ashes_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#1c2023", // black
                    "#7f5f4f", // red
                    "#5f7f5f", // green
                    "#7f7f5f", // yellow
                    "#7f5f7f", // blue
                    "#7f5f7f", // magenta
                    "#5f7f7f", // cyan
                    "#c7ccd1", // white
                    "#d79921", // orange
                    "#d75f87", // pink
                ),
                bright: Some(new_term_colors(
                    "#747c84", // black
                    "#c7ae95", // red
                    "#95c7ae", // green
                    "#aec795", // yellow
                    "#ae95c7", // blue
                    "#c795ae", // magenta
                    "#95aec7", // cyan
                    "#f3f4f5", // white
                    "#e0af68", // orange
                    "#d3869b", // pink
                )),
                dim: Some(new_term_colors(
                    "#2a2f33", // black
                    "#a78c7a", // red
                    "#8ca78c", // green
                    "#a7b78c", // yellow
                    "#a78ca7", // blue
                    "#b78ca7", // magenta
                    "#8ca7b7", // cyan
                    "#bfc4c9", // white
                    "#d7aa6f", // orange
                    "#d78ca7", // pink
                )),
                comment: Some("#7f8c99".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#1c2023".into(),
                    "#1c2023".into(),
                    "#25292d".into(),
                    "#2a2f33".into(),
                    "#3a3f43".into(),
                ]),
                foreground: Foreground::Colors([
                    "#f3f4f5".into(),
                    "#c7ccd1".into(),
                    "#9aa0a6".into(),
                    "#747c84".into(),
                ]),
                selection: Selection::Colors(["#3a3f43".into(), "#5f7f5f".into()]),
                diff: None,
            }),
            config: None,
        },
        "base16_default_dark" => Theme {
            // OK
            name: Some("base16_default_dark".into()),
            light: Some(false),
            colors: Some(ThemeColors {
                base: new_term_colors(
                    "#181818", // black
                    "#ab4642", // red
                    "#a1b56c", // green
                    "#f7ca88", // yellow
                    "#7cafc2", // blue
                    "#ba8baf", // magenta
                    "#86c1b9", // cyan
                    "#d8d8d8", // white
                    "#e78c45", // orange
                    "#d8a3af", // pink
                ),
                bright: Some(new_term_colors(
                    "#585858", // black
                    "#ab4642", // red
                    "#a1b56c", // green
                    "#f7ca88", // yellow
                    "#7cafc2", // blue
                    "#ba8baf", // magenta
                    "#86c1b9", // cyan
                    "#f8f8f8", // white
                    "#e78c45", // orange
                    "#d8a3af", // pink
                )),
                dim: Some(new_term_colors(
                    "#282828", // black
                    "#8b3a36", // red
                    "#8ca456", // green
                    "#d9b475", // yellow
                    "#6597b0", // blue
                    "#9b7291", // magenta
                    "#6fa0a8", // cyan
                    "#b8b8b8", // white
                    "#c9733a", // orange
                    "#c37d8e", // pink
                )),
                comment: Some("#7c7c7c".into()),
                variable: None,
                status_line: None,
                background: Background::Colors([
                    "#181818".into(),
                    "#181818".into(),
                    "#202020".into(),
                    "#282828".into(),
                    "#383838".into(),
                ]),
                foreground: Foreground::Colors([
                    "#d8d8d8".into(),
                    "#d8d8d8".into(),
                    "#b8b8b8".into(),
                    "#a8a8a8".into(),
                ]),
                selection: Selection::Colors(["#3a3a3a".into(), "#5a5a5a".into()]),
                diff: None,
            }),
            config: None,
        },
        _ => Theme::default(),
    }
}
