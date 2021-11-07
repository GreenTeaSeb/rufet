use crate::borders::Border;

pub struct Output {
    data: String,
    logo: String,
}

impl Output {
    pub fn new(data_in: String, logo_in: String) -> Self {
        Self {
            data: Self::add_border(data_in, 5, 15),
            logo: Self::add_border(logo_in, 10, 0),
        }
    }

    fn add_border(data: String, padding: usize, height: usize) -> String {
        if data.is_empty() {
            return data;
        }
        let longest_string = data.lines().map(|x| x.chars().count()).max().unwrap_or(0) + padding;
        let data_formated = format!(
            "{:\n^h$}",
            data,
            h = if height == 0 {
                0
            } else {
                data.chars().count() + height
            }
        )
        .lines()
        .map(|x| {
            format!(
                "{left}{:^width$}{right}\n",
                x,
                width = longest_string,
                left = Border::get(&Border::Left),
                right = Border::get(&Border::Right),
            )
        })
        .collect::<String>();
        format!(
            "{cor0}{top}{cor1}\n{data}{cor2}{bot}{cor3}",
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
impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = std::cmp::max(self.logo.lines().count(), self.data.lines().count());
        let mut data_lines = self.data.lines();
        let mut logo_lines = self.logo.lines();
        let mut output = String::default();
        (0..l).for_each(|_| {
            let data_line = data_lines.nth(0).unwrap_or("");
            let logo_line = logo_lines.nth(0).unwrap_or("");
            output = format!("{}{}{}\n", output, logo_line, data_line);
        });
        write!(f, "{}", output)
    }
}
