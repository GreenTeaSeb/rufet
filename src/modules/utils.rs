use crate::borders::Border;
use crate::color::to_colored;

pub trait Module {
    fn get_val(&self) -> String {
        String::default()
    }
    fn format(&self) -> String;
}

pub fn default_format(input: &String, value: &String) -> String {
    to_colored(&input.replace("$value", &format!("{}", value).to_string()))
}

pub fn add_padding(input: &String, padding: usize) -> String {
    let text_reg = regex::Regex::new(r"\u001b[^m]*?m").unwrap();
    let longest_string = input
        .lines()
        .map(|x| {
            if text_reg.is_match(x) == true {
                text_reg.replace_all(x, "").chars().count()
            } else {
                x.chars().count()
            }
        })
        .max()
        .unwrap_or(0);
    input
        .lines()
        .map(|x| {
            format!(
                "{padl}{data}{padr}\n",
                data = x,
                padl = " ".repeat(padding),
                padr = if text_reg.is_match(x) == true {
                    " ".repeat(
                        longest_string + padding - text_reg.replace_all(x, "").chars().count(),
                    )
                } else {
                    " ".repeat((longest_string + padding) - x.len())
                },
            )
        })
        .collect::<String>()
}
pub fn add_border(data: String, height: usize, alignment: &str) -> String {
    if data.is_empty() {
        return data;
    }
    let text_reg = regex::Regex::new(r"\u001b[^m]*?m").unwrap();
    let longest_string = data
        .lines()
        .map(|x| {
            if text_reg.is_match(x) == true {
                text_reg.replace_all(x, "").chars().count()
            } else {
                x.chars().count()
            }
        })
        .max()
        .unwrap_or(0);
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
            width = if text_reg.is_match(x) == true {
                x.chars().count() + longest_string - text_reg.replace_all(x, "").chars().count()
            } else {
                longest_string
            },
            left = Border::get(&Border::Left),
            right = Border::get(&Border::Right),
        ),
        "right" => format!(
            "{left}{:>width$}{right}\n",
            x,
            width = if text_reg.is_match(x) == true {
                x.chars().count() + longest_string - text_reg.replace_all(x, "").chars().count()
            } else {
                longest_string
            },
            left = Border::get(&Border::Left),
            right = Border::get(&Border::Right),
        ),
        _ => format!(
            "{left}{:^width$}{right}\n",
            x,
            width = if text_reg.is_match(x) == true {
                x.chars().count() + longest_string - text_reg.replace_all(x, "").chars().count()
            } else {
                longest_string
            },
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
