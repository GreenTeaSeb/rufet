use crate::borders::Border;
use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Logo {
    format: String,
    padding: usize,
    margin: usize,
    height: usize,
    border: Border,
    alignment: String,

    rule: Vec<Rule>,
}
impl Module for Logo {
    fn format(&self) -> String {
        let formatted = if !self.rule.is_empty() {
            self.rule
                .iter()
                .map(|rule| self.format.replace(&rule.id, &rule.get_colored()))
                .collect::<String>()
        } else {
            self.format.clone()
        }
        .add_margin(&self.padding)
        .align(&self.alignment);
        self.border.add_border(&formatted)
    }
}

impl Default for Logo {
    fn default() -> Self {
        Self {
            format: tux2().to_string(),
            margin: 0,
            padding: 0,
            height: 0,
            border: Border {
                visible: true,
                ..Border::default()
            },
            alignment: "center".to_string(),
            rule: vec![],
        }
    }
}

fn tux2() -> &'static str {
    "\x1b[48;5;0m                                           \x1b[0m
\x1b[48;5;0m                 \x1b[38;5;102m▄\x1b[48;5;243m\x1b[38;5;249m▄\x1b[48;5;8m\x1b[38;5;102m▄\x1b[48;5;242m\x1b[38;5;239m▄\x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;237m\x1b[38;5;235m▄\x1b[48;5;237m\x1b[38;5;235m▄\x1b[48;5;239m\x1b[38;5;236m▄\x1b[48;5;0m\x1b[38;5;238m▄\x1b[48;5;0m\x1b[38;5;241m▄\x1b[48;5;0m                \x1b[0m
\x1b[48;5;0m                \x1b[48;5;242m\x1b[38;5;8m▄\x1b[48;5;250m\x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;242m▄\x1b[48;5;59m\x1b[38;5;237m▄\x1b[48;5;237m\x1b[38;5;235m▄\x1b[48;5;235m     \x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;0m\x1b[38;5;239m▄\x1b[48;5;0m               \x1b[0m
\x1b[48;5;0m               \x1b[38;5;243m▄\x1b[48;5;102m\x1b[38;5;8m▄\x1b[48;5;243m\x1b[38;5;59m▄\x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;235m   \x1b[38;5;238m▄\x1b[48;5;235m\x1b[38;5;237m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m  \x1b[48;5;237m\x1b[38;5;236m▄\x1b[48;5;0m               \x1b[0m
\x1b[48;5;0m               \x1b[48;5;242m\x1b[38;5;241m▄\x1b[48;5;240m\x1b[38;5;255m▄\x1b[48;5;236m\x1b[38;5;188m▄\x1b[48;5;235m\x1b[38;5;242m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;254m▄\x1b[48;5;250m\x1b[38;5;252m▄\x1b[48;5;237m\x1b[38;5;145m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m \x1b[48;5;236m \x1b[48;5;241m \x1b[48;5;0m              \x1b[0m
\x1b[48;5;0m               \x1b[48;5;241m\x1b[38;5;242m▄\x1b[48;5;255m \x1b[48;5;235m \x1b[48;5;250m\x1b[38;5;246m▄\x1b[48;5;234m  \x1b[48;5;255m\x1b[38;5;254m▄\x1b[48;5;247m\x1b[38;5;234m▄\x1b[48;5;234m \x1b[48;5;7m\x1b[38;5;250m▄\x1b[48;5;242m\x1b[38;5;234m▄\x1b[48;5;234m \x1b[48;5;236m \x1b[48;5;241m \x1b[48;5;0m              \x1b[0m
\x1b[48;5;0m               \x1b[48;5;242m\x1b[38;5;59m▄\x1b[48;5;236m \x1b[48;5;237m\x1b[38;5;221m▄\x1b[48;5;221m \x1b[48;5;214m \x1b[48;5;130m\x1b[38;5;214m▄\x1b[48;5;214m \x1b[48;5;136m\x1b[38;5;214m▄\x1b[48;5;234m\x1b[38;5;214m▄\x1b[48;5;245m\x1b[38;5;166m▄\x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;234m \x1b[48;5;235m \x1b[48;5;240m\x1b[38;5;239m▄\x1b[48;5;0m              \x1b[0m
\x1b[48;5;0m               \x1b[48;5;59m\x1b[38;5;242m▄\x1b[48;5;221m\x1b[38;5;172m▄\x1b[48;5;221m  \x1b[48;5;214m  \x1b[38;5;208m▄\x1b[48;5;214m\x1b[38;5;172m▄\x1b[48;5;208m\x1b[38;5;130m▄\x1b[48;5;130m \x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;235m\x1b[38;5;239m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;0m              \x1b[0m
\x1b[48;5;0m               \x1b[48;5;59m\x1b[38;5;238m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;172m\x1b[38;5;188m▄\x1b[48;5;130m\x1b[38;5;173m▄\x1b[48;5;166m\x1b[38;5;130m▄\x1b[48;5;130m  \x1b[38;5;239m▄\x1b[48;5;130m\x1b[38;5;240m▄\x1b[48;5;238m\x1b[38;5;241m▄\x1b[48;5;234m \x1b[48;5;8m\x1b[38;5;240m▄\x1b[48;5;238m\x1b[38;5;242m▄\x1b[48;5;236m \x1b[48;5;240m\x1b[38;5;237m▄\x1b[48;5;0m             \x1b[0m
\x1b[48;5;0m              \x1b[48;5;242m\x1b[38;5;238m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;255m \x1b[48;5;254m\x1b[38;5;255m▄\x1b[48;5;252m\x1b[38;5;254m▄\x1b[48;5;249m\x1b[38;5;188m▄\x1b[48;5;102m\x1b[38;5;7m▄\x1b[48;5;242m\x1b[38;5;248m▄\x1b[48;5;242m\x1b[38;5;246m▄\x1b[48;5;243m\x1b[38;5;246m▄\x1b[48;5;102m\x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;249m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;239m\x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;240m\x1b[38;5;237m▄\x1b[48;5;0m\x1b[38;5;241m▄\x1b[48;5;0m           \x1b[0m
\x1b[48;5;0m             \x1b[48;5;241m\x1b[38;5;237m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;233m\x1b[38;5;255m▄\x1b[48;5;255m   \x1b[48;5;254m\x1b[38;5;255m▄\x1b[48;5;253m\x1b[38;5;255m▄\x1b[48;5;252m\x1b[38;5;254m▄\x1b[48;5;250m\x1b[38;5;253m▄\x1b[48;5;250m\x1b[38;5;253m▄\x1b[48;5;7m\x1b[38;5;253m▄\x1b[48;5;252m\x1b[38;5;254m▄\x1b[48;5;254m \x1b[48;5;233m   \x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;239m\x1b[38;5;236m▄\x1b[48;5;0m\x1b[38;5;240m▄\x1b[48;5;0m          \x1b[0m
\x1b[48;5;0m           \x1b[38;5;242m▄\x1b[48;5;240m\x1b[38;5;236m▄\x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;233m\x1b[38;5;255m▄\x1b[48;5;255m \x1b[38;5;15m▄\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;15m   \x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;255m   \x1b[48;5;254m  \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;237m\x1b[38;5;235m▄\x1b[48;5;240m\x1b[38;5;238m▄\x1b[48;5;0m         \x1b[0m
\x1b[48;5;0m          \x1b[38;5;241m▄\x1b[48;5;238m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;255m \x1b[38;5;15m▄\x1b[48;5;15m         \x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;255m \x1b[48;5;254m \x1b[48;5;253m \x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;238m \x1b[48;5;237m\x1b[38;5;240m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;233m \x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;240m\x1b[38;5;238m▄\x1b[48;5;0m\x1b[38;5;240m▄\x1b[48;5;0m       \x1b[0m
\x1b[48;5;0m         \x1b[38;5;242m▄\x1b[48;5;238m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;188m \x1b[48;5;255m \x1b[48;5;15m           \x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;254m\x1b[38;5;255m▄\x1b[48;5;253m \x1b[48;5;252m \x1b[48;5;188m\x1b[38;5;252m▄\x1b[48;5;243m\x1b[38;5;240m▄\x1b[48;5;239m\x1b[38;5;246m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;233m \x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;240m\x1b[38;5;238m▄\x1b[48;5;0m\x1b[38;5;242m▄\x1b[48;5;0m      \x1b[0m
\x1b[48;5;0m        \x1b[38;5;8m▄\x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;238m\x1b[38;5;242m▄\x1b[48;5;250m \x1b[48;5;253m \x1b[48;5;255m \x1b[48;5;15m            \x1b[48;5;255m \x1b[48;5;253m \x1b[48;5;251m \x1b[48;5;7m\x1b[38;5;250m▄\x1b[48;5;238m\x1b[38;5;251m▄\x1b[48;5;247m\x1b[38;5;59m▄\x1b[48;5;236m \x1b[48;5;233m \x1b[38;5;232m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;242m\x1b[38;5;241m▄\x1b[48;5;0m      \x1b[0m
\x1b[48;5;0m        \x1b[48;5;242m\x1b[38;5;240m▄\x1b[48;5;236m \x1b[48;5;8m\x1b[38;5;246m▄\x1b[48;5;249m\x1b[38;5;145m▄\x1b[48;5;188m\x1b[38;5;252m▄\x1b[48;5;255m \x1b[48;5;15m           \x1b[38;5;255m▄\x1b[48;5;255m\x1b[38;5;254m▄\x1b[48;5;188m\x1b[38;5;252m▄\x1b[48;5;7m\x1b[38;5;250m▄\x1b[48;5;249m\x1b[38;5;145m▄\x1b[48;5;250m\x1b[38;5;249m▄\x1b[48;5;238m\x1b[38;5;236m▄\x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;232m  \x1b[48;5;234m \x1b[48;5;59m \x1b[48;5;0m      \x1b[0m
\x1b[48;5;0m        \x1b[48;5;240m\x1b[38;5;214m▄\x1b[48;5;234m\x1b[38;5;172m▄\x1b[48;5;245m\x1b[38;5;238m▄\x1b[48;5;248m\x1b[38;5;241m▄\x1b[48;5;251m\x1b[38;5;249m▄\x1b[48;5;254m\x1b[38;5;188m▄\x1b[48;5;255m \x1b[48;5;15m   \x1b[38;5;255m▄\x1b[48;5;255m \x1b[48;5;15m    \x1b[38;5;255m▄\x1b[48;5;255m \x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;252m\x1b[38;5;251m▄\x1b[48;5;249m\x1b[38;5;145m▄\x1b[48;5;248m\x1b[38;5;247m▄\x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;234m\x1b[38;5;242m▄\x1b[48;5;233m\x1b[38;5;237m▄\x1b[48;5;232m\x1b[38;5;235m▄\x1b[48;5;233m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;238m▄\x1b[48;5;242m\x1b[38;5;243m▄\x1b[48;5;0m      \x1b[0m
\x1b[48;5;0m      \x1b[38;5;222m▄\x1b[48;5;214m\x1b[38;5;220m▄\x1b[48;5;214m\x1b[38;5;220m▄\x1b[48;5;214m \x1b[48;5;172m\x1b[38;5;214m▄\x1b[48;5;238m\x1b[38;5;166m▄\x1b[48;5;243m\x1b[38;5;236m▄\x1b[48;5;250m\x1b[38;5;240m▄\x1b[48;5;253m\x1b[38;5;250m▄\x1b[48;5;255m\x1b[38;5;253m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;255m \x1b[38;5;254m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;255m \x1b[38;5;254m▄\x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;188m\x1b[38;5;251m▄\x1b[48;5;250m\x1b[38;5;145m▄\x1b[48;5;247m\x1b[38;5;208m▄\x1b[48;5;208m \x1b[48;5;8m\x1b[38;5;59m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;232m\x1b[38;5;0m▄\x1b[48;5;0m  \x1b[48;5;233m\x1b[38;5;232m▄\x1b[48;5;237m\x1b[38;5;130m▄\x1b[48;5;0m\x1b[38;5;173m▄\x1b[48;5;0m     \x1b[0m
\x1b[48;5;0m  \x1b[38;5;215m▄\x1b[48;5;0m\x1b[38;5;214m▄\x1b[48;5;0m\x1b[38;5;214m▄\x1b[48;5;222m\x1b[38;5;220m▄\x1b[48;5;220m    \x1b[48;5;214m \x1b[48;5;208m\x1b[38;5;214m▄\x1b[48;5;0m\x1b[38;5;172m▄\x1b[48;5;234m\x1b[38;5;0m▄\x1b[48;5;236m\x1b[38;5;233m▄\x1b[48;5;249m\x1b[38;5;232m▄\x1b[48;5;252m\x1b[38;5;248m▄\x1b[48;5;254m\x1b[38;5;251m▄\x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;255m\x1b[38;5;254m▄\x1b[48;5;254m\x1b[38;5;253m▄\x1b[48;5;254m\x1b[38;5;188m▄\x1b[48;5;253m\x1b[38;5;251m▄\x1b[48;5;252m\x1b[38;5;250m▄\x1b[48;5;249m\x1b[38;5;248m▄\x1b[48;5;247m\x1b[38;5;246m▄\x1b[48;5;208m\x1b[38;5;214m▄\x1b[48;5;214m \x1b[48;5;178m\x1b[38;5;214m▄\x1b[48;5;233m\x1b[38;5;172m▄\x1b[48;5;0m\x1b[38;5;130m▄\x1b[48;5;0m\x1b[38;5;130m▄\x1b[48;5;0m\x1b[38;5;130m▄\x1b[48;5;94m\x1b[38;5;130m▄\x1b[48;5;130m\x1b[38;5;172m▄\x1b[48;5;166m \x1b[48;5;0m     \x1b[0m
\x1b[48;5;0m  \x1b[48;5;214m          \x1b[48;5;208m\x1b[38;5;214m▄\x1b[48;5;130m\x1b[38;5;172m▄\x1b[48;5;0m  \x1b[48;5;239m \x1b[48;5;247m\x1b[38;5;243m▄\x1b[48;5;250m\x1b[38;5;246m▄\x1b[48;5;251m\x1b[38;5;145m▄\x1b[48;5;252m\x1b[38;5;249m▄\x1b[48;5;252m\x1b[38;5;249m▄\x1b[48;5;251m\x1b[38;5;145m▄\x1b[48;5;7m\x1b[38;5;248m▄\x1b[48;5;249m\x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;245m▄\x1b[48;5;246m\x1b[38;5;102m▄\x1b[48;5;245m\x1b[38;5;232m▄\x1b[48;5;215m\x1b[38;5;214m▄\x1b[48;5;214m   \x1b[48;5;178m\x1b[38;5;214m▄\x1b[48;5;172m\x1b[38;5;214m▄\x1b[48;5;172m\x1b[38;5;208m▄\x1b[48;5;172m \x1b[38;5;208m▄\x1b[48;5;208m \x1b[48;5;166m\x1b[38;5;208m▄\x1b[48;5;0m\x1b[38;5;166m▄\x1b[48;5;0m   \x1b[0m
\x1b[48;5;0m  \x1b[48;5;214m           \x1b[48;5;208m \x1b[48;5;166m\x1b[38;5;172m▄\x1b[48;5;234m\x1b[38;5;130m▄\x1b[48;5;237m\x1b[38;5;236m▄\x1b[48;5;239m\x1b[38;5;238m▄\x1b[48;5;243m\x1b[38;5;59m▄\x1b[48;5;245m\x1b[38;5;243m▄\x1b[48;5;246m\x1b[38;5;8m▄\x1b[48;5;246m\x1b[38;5;8m▄\x1b[48;5;246m\x1b[38;5;8m▄\x1b[48;5;245m\x1b[38;5;8m▄\x1b[48;5;102m\x1b[38;5;8m▄\x1b[48;5;8m\x1b[38;5;232m▄\x1b[48;5;232m  \x1b[48;5;214m\x1b[38;5;208m▄\x1b[48;5;214m   \x1b[48;5;208m  \x1b[38;5;172m▄\x1b[48;5;208m\x1b[38;5;172m▄\x1b[48;5;208m   \x1b[48;5;202m\x1b[38;5;166m▄\x1b[48;5;166m\x1b[38;5;130m▄\x1b[48;5;0m  \x1b[0m
\x1b[48;5;0m  \x1b[48;5;214m\x1b[38;5;172m▄\x1b[48;5;214m   \x1b[38;5;208m▄\x1b[48;5;208m   \x1b[48;5;214m\x1b[38;5;208m▄\x1b[48;5;214m\x1b[38;5;208m▄\x1b[48;5;214m  \x1b[48;5;208m \x1b[48;5;166m \x1b[48;5;232m\x1b[38;5;130m▄\x1b[48;5;232m\x1b[38;5;234m▄\x1b[48;5;232m\x1b[38;5;233m▄\x1b[48;5;242m\x1b[38;5;232m▄\x1b[48;5;243m\x1b[38;5;232m▄\x1b[48;5;243m\x1b[38;5;232m▄\x1b[48;5;236m\x1b[38;5;232m▄\x1b[48;5;232m\x1b[38;5;233m▄\x1b[48;5;232m\x1b[38;5;233m▄\x1b[48;5;232m\x1b[38;5;234m▄\x1b[48;5;233m\x1b[38;5;235m▄\x1b[48;5;233m\x1b[38;5;179m▄\x1b[48;5;208m \x1b[48;5;214m  \x1b[48;5;208m  \x1b[48;5;172m \x1b[38;5;208m▄\x1b[48;5;172m\x1b[38;5;208m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;166m \x1b[38;5;0m▄\x1b[48;5;130m\x1b[38;5;0m▄\x1b[48;5;0m   \x1b[0m
\x1b[48;5;0m  \x1b[48;5;172m\x1b[38;5;0m▄\x1b[48;5;208m\x1b[38;5;0m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;208m\x1b[38;5;172m▄\x1b[48;5;208m     \x1b[48;5;214m\x1b[38;5;208m▄\x1b[48;5;214m\x1b[38;5;208m▄\x1b[48;5;208m \x1b[38;5;202m▄\x1b[48;5;166m \x1b[48;5;237m\x1b[38;5;0m▄\x1b[48;5;235m\x1b[38;5;0m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;235m\x1b[38;5;0m▄\x1b[48;5;235m\x1b[38;5;0m▄\x1b[48;5;236m\x1b[38;5;0m▄\x1b[48;5;237m\x1b[38;5;0m▄\x1b[48;5;172m \x1b[48;5;208m \x1b[48;5;214m  \x1b[48;5;208m  \x1b[38;5;166m▄\x1b[48;5;172m\x1b[38;5;166m▄\x1b[48;5;166m \x1b[48;5;130m\x1b[38;5;0m▄\x1b[48;5;0m      \x1b[0m
\x1b[48;5;0m        \x1b[48;5;166m\x1b[38;5;0m▄\x1b[48;5;166m\x1b[38;5;0m▄\x1b[48;5;166m\x1b[38;5;0m▄\x1b[48;5;166m\x1b[38;5;130m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;202m\x1b[38;5;166m▄\x1b[48;5;166m\x1b[38;5;202m▄\x1b[48;5;130m\x1b[38;5;0m▄\x1b[48;5;0m          \x1b[48;5;172m\x1b[38;5;0m▄\x1b[48;5;208m\x1b[38;5;172m▄\x1b[48;5;208m \x1b[38;5;166m▄\x1b[48;5;208m\x1b[38;5;166m▄\x1b[48;5;166m\x1b[38;5;130m▄\x1b[48;5;166m\x1b[38;5;0m▄\x1b[48;5;0m         \x1b[0m
\x1b[48;5;0m                                           \x1b[0m
"
}
