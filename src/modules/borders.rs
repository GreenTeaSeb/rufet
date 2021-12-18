use crate::color::AnsiExt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Border {
    pub top: [char; 3],
    pub sides: [char; 2],
    pub bottom: [char; 3],
    pub visible: bool,
    pub color: String,
    pub background: String,
}

impl Border {
    pub fn add_border(&self, data: &str) -> String {
        if !self.visible {
            return data.to_string();
        }
        let longest_string = data
            .lines()
            .map(|x| x.to_string().remove_ansi().chars().count())
            .max()
            .unwrap_or(0);
        let data_formated = data
            .lines()
            .map(|x| {
                format!(
                    "{left}{:<width$}{right}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                    left = &self.sides[0].to_string().format_to_ansi(
                        &self.color.as_foreground(),
                        &self.background.as_background(),
                        ""
                    ),
                    right = &self.sides[1].to_string().format_to_ansi(
                        &self.color.as_foreground(),
                        &self.background.as_background(),
                        ""
                    ),
                )
            })
            .collect::<String>();
        format!(
            "{cor0}{top}{cor1}\n{data}{cor2}{bot}{cor3}\n",
            top = &self.top[1]
                .to_string()
                .repeat(longest_string)
                .format_to_ansi(
                    &self.color.as_foreground(),
                    &self.background.as_background(),
                    ""
                ),
            data = data_formated,
            bot = &self.bottom[1]
                .to_string()
                .repeat(longest_string)
                .format_to_ansi(
                    &self.color.as_foreground(),
                    &self.background.as_background(),
                    ""
                ),
            cor0 = &self.top[0].to_string().format_to_ansi(
                &self.color.as_foreground(),
                &self.background.as_background(),
                ""
            ),
            cor1 = &self.top[2].to_string().format_to_ansi(
                &self.color.as_foreground(),
                &self.background.as_background(),
                ""
            ),
            cor2 = &self.bottom[0].to_string().format_to_ansi(
                &self.color.as_foreground(),
                &self.background.as_background(),
                ""
            ),
            cor3 = &self.bottom[2].to_string().format_to_ansi(
                &self.color.as_foreground(),
                &self.background.as_background(),
                ""
            ),
        )
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: ['╭', '─', '╮'],
            bottom: ['╰', '─', '╯'],
            sides: ['│', '│'],
            visible: false,
            color: String::from("#eed"),
            background: String::from(""),
        }
    }
}
