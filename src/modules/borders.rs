pub enum Border {
    Top,
    Bottom,
    Left,
    Right,
    TopCornerRight,
    TopCornerLeft,
    BotCornerRight,
    BotCornerLeft,
}

impl Border {
    pub fn get(&self) -> String {
        let border = match *self {
            Border::Top => "─",
            Border::Bottom => "─",
            Border::Left => "│",
            Border::Right => "│",
            Border::TopCornerLeft => "╭",
            Border::TopCornerRight => "╮",
            Border::BotCornerRight => "╯",
            Border::BotCornerLeft => "╰",
        };
        format!("{}", border)
    }
}
