use qute::*;

#[derive(Debug, Default)]
struct Style<'a> {
    color: [u8; 3],
    background: Option<[u8; 3]>,
    effect: &'a str,
}

pub fn to_colored(input: &String) -> String {
    let text_reg = regex::RegexBuilder::new(r"\{(.*?)\}\((.*?)\)")
        .dot_matches_new_line(true)
        .build();
    let mut output = input.clone();
    for cap in text_reg.unwrap().captures_iter(input) {
        let line = &cap[0];
        let text = &cap[1];
        let mut style = Style {
            color: [255, 255, 255],
            background: None,
            effect: "normal",
        };
        let _ = &cap[2].split(';').for_each(|rule| {
            let rule = rule.trim();
            match rule.get(..3) {
                Some("fg:") => {
                    let rgb = to_rgb(rule.trim_start_matches("fg:"));
                    style.color = [rgb[0], rgb[1], rgb[2]];
                }
                Some("bg:") => {
                    let rgb = to_rgb(rule.trim_start_matches("bg:"));
                    style.background = Some([rgb[0], rgb[1], rgb[2]]);
                }
                _ => style.effect = rule,
            }
        });
        let mut colored =
            qute!(&text).set_rgb_color(style.color[0], style.color[1], style.color[2]);
        if let Some(bg) = style.background {
            colored = colored.set_rgb_background(bg[0], bg[1], bg[2]);
        }
        match style.effect {
            "bold" => colored = colored.bold(),
            "dim" => colored = colored.dim(),
            "underline" => colored = colored.underline(),
            "reverse" => colored = colored.reverse(),
            "blink" => colored = colored.blink(),
            "hidden" => colored = colored.hidden(),
            "strikethrough" => colored = colored.strikethrough(),
            _ => (),
        }
        output = output.replace(line, &colored.to_string())
    }
    output
}

fn to_rgb(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|p| p.trim().parse::<u8>().unwrap_or_default())
        .collect::<Vec<u8>>()
}
