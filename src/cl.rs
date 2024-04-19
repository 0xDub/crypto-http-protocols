#[derive(Debug, Clone)]
pub enum CL {
    Pink,
    Purple,
    Green,
    DullGreen,
    Blue,
    DullRed,
    Red,
    Orange,
    Teal,
    DullTeal,
    Dull,
    End,
}

impl CL {
    pub fn get(&self) -> &str {
        match self {
            CL::Pink => "\x1b[38;5;201m",
            CL::Purple => "\x1b[38;5;135m",
            CL::Green => "\x1b[38;5;46m",
            CL::DullGreen => "\x1b[38;5;29m",
            CL::Blue => "\x1b[38;5;27m",
            CL::DullRed => "\x1b[38;5;124m",
            CL::Red => "\x1b[38;5;196m",
            CL::Orange => "\x1b[38;5;208m",
            CL::Teal => "\x1b[38;5;14m",
            CL::DullTeal => "\x1b[38;5;153m",
            CL::Dull => "\x1b[38;5;8m",
            CL::End => "\x1b[37m",
        }
    }
}
