use crate::borders::Border;
use crate::Value;
pub trait Module {
    fn print(config: Option<&Value>, default: &'static str) -> String {
        match &Self::get_val() {
            Some(value) => {
                let output = Self::get_from(config, "format", default)
                    .replace("$value", format!("{}", value).as_str());
                Self::with_border(config, output, false)
            }
            None => String::default(),
        }
    }
    fn get_from(config: Option<&Value>, key: &str, default: &str) -> String {
        match config {
            Some(con) => match con.get(key) {
                Some(con) => con.as_str().get_or_insert(default).to_string(),
                None => String::from(default),
            },
            None => default.to_string(),
        }
    }
    fn get_border(config: Option<&Value>, default: bool) -> bool {
        match config {
            Some(con) => match con.get("border") {
                Some(con) => *con.as_bool().get_or_insert(default),
                None => default,
            },
            None => default,
        }
    }
    fn with_border(config: Option<&Value>, input: String, default: bool) -> String {
        if Self::get_border(config, default) {
            Self::add_border(
                input,
                Self::get_int(config, "padding") as usize,
                Self::get_int(config, "height") as usize,
                &Self::get_from(config, "align", "center"),
            ) + &"\n".to_string()
        } else {
            input
        }
    }
    fn get_int(config: Option<&Value>, key: &str) -> i64 {
        match config {
            Some(con) => match con.get(key) {
                Some(con) => *con.as_integer().get_or_insert(0),

                None => 0,
            },
            None => 0,
        }
    }
    fn get_val() -> Option<String> {
        None
    }

    fn add_border(data: String, padding: usize, height: usize, alignment: &str) -> String {
        if data.is_empty() {
            return data;
        }
        let longest_string = data.lines().map(|x| x.chars().count()).max().unwrap_or(0) + padding;
        let data_formated = format!(
            "{:\n^h$}",
            data,
            h = if height < 1 {
                0
            } else {
                data.chars().count() + height - data.lines().count()
            }
        )
        .lines()
        .map(|x| match alignment {
            "left" => format!(
                "{left}{:<width$}{right}\n",
                x,
                width = longest_string,
                left = Border::get(&Border::Left),
                right = Border::get(&Border::Right),
            ),
            "right" => format!(
                "{left}{:>width$}{right}\n",
                x,
                width = longest_string,
                left = Border::get(&Border::Left),
                right = Border::get(&Border::Right),
            ),
            _ => format!(
                "{left}{:^width$}{right}\n",
                x,
                width = longest_string,
                left = Border::get(&Border::Left),
                right = Border::get(&Border::Right),
            ),
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
