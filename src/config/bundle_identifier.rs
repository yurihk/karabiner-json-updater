use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
    #[serde(rename = "com.jetbrains.CLion")]
    CLion,
    #[serde(rename = "io.dynalist")]
    Dynalist, // https://help.dynalist.io/article/91-keyboard-shortcut-reference
    #[serde(rename = "com.github.atom")]
    Atom,
    #[serde(rename = "com.tinyspeck.slackmacgap")]
    Slack,
    #[serde(rename = "com.google.Chrome")]
    GoogleChrome,
    #[serde(rename = "co.mural.macOS")]
    Mural,
    #[serde(rename = "notion.id")]
    Notion,
}
