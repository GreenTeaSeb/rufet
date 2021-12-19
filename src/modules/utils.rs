use crate::color::AnsiExt;

pub trait Module {
    fn get_val(&self) -> String {
        String::default()
    }
    fn format(&self) -> String;
}

pub trait BorderExt {
    fn add_margin(&self, margin: &usize) -> Self;
    fn align(&self, alignment: &str) -> Self;
    fn add_height(&self, height: &usize) -> Self;
}

impl BorderExt for String {
    fn add_margin(&self, margin: &usize) -> Self {
        self.lines()
            .map(|x| {
                format!(
                    "{padl}{:padr$}\n",
                    x,
                    padl = " ".repeat(*margin),
                    padr = if x.to_string().remove_ansi().ne(x) {
                        x.chars().count() + margin
                    } else {
                        *margin
                    },
                )
            })
            .collect::<String>()
    }
    fn add_height(&self, height: &usize) -> Self {
        format!(
            "{:\n^h$}",
            self,
            h = if height < &1 {
                0
            } else {
                self.chars().count() + height - self.lines().count()
            }
        )
    }
    fn align(&self, alignment: &str) -> Self {
        let longest_string = self
            .lines()
            .map(|x| x.to_string().remove_ansi().chars().count())
            .max()
            .unwrap_or(0);
        self.lines()
            .map(|x| match alignment {
                "center" => format!(
                    "{:^width$}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                ),
                "right" => format!(
                    "{:>width$}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                ),
                _ => format!(
                    "{:<width$}\n",
                    x,
                    width = if x.to_string().remove_ansi().ne(x) {
                        longest_string
                            + (x.chars().count() - x.to_string().remove_ansi().chars().count())
                    } else {
                        longest_string
                    },
                ),
            })
            .collect::<String>()
    }
}
