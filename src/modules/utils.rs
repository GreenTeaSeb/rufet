use crate::borders::Border;
use crate::color::AnsiExt;

pub trait Module {
    fn get_val(&self) -> String {
        String::default()
    }
    fn format(&self) -> String;
}

pub trait BorderExt {
    fn add_padding(&self, padding: &usize) -> Self;
    fn add_border(&self, height: &usize, alignment: &str, visible: bool) -> Self;
}

impl BorderExt for String {
    fn add_padding(&self, padding: &usize) -> Self {
        self.lines()
            .map(|x| {
                format!(
                    "{padl}{:padr$}\n",
                    x,
                    padl = " ".repeat(*padding),
                    padr = if x.to_string().remove_ansi().ne(x) {
                        x.chars().count() + padding
                    } else {
                        *padding
                    },
                )
            })
            .collect::<String>()
    }
    fn add_border(&self, height: &usize, alignment: &str, visible: bool) -> Self {
        let data_height = format!(
            "{:\n^h$}",
            self,
            h = if height < &1 {
                0
            } else {
                self.chars().count() + height - self.lines().count()
            }
        );

        if self.is_empty() || !visible {
            return data_height;
        }
        let longest_string = self
            .lines()
            .map(|x| x.to_string().remove_ansi().chars().count())
            .max()
            .unwrap_or(0);
        let data_formated = data_height
            .lines()
            .map(|x| match alignment {
                "left" => format!(
                    "{left}{:<width$}{right}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                    left = Border::get(&Border::Left),
                    right = Border::get(&Border::Right),
                ),
                "right" => format!(
                    "{left}{:>width$}{right}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                    left = Border::get(&Border::Left),
                    right = Border::get(&Border::Right),
                ),
                _ => format!(
                    "{left}{:^width$}{right}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                    left = Border::get(&Border::Left),
                    right = Border::get(&Border::Right),
                ),
            })
            .collect::<String>();
        format!(
            "{cor0}{top}{cor1}\n{data}{cor2}{bot}{cor3}\n",
            top = Border::get(&Border::Top).repeat(longest_string),
            data = data_formated,
            bot = Border::get(&Border::Bottom).repeat(longest_string),
            cor0 = Border::get(&Border::TopCornerLeft),
            cor1 = Border::get(&Border::TopCornerRight),
            cor2 = Border::get(&Border::BotCornerLeft),
            cor3 = Border::get(&Border::BotCornerRight)
        )
    }
}
