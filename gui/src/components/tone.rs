#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Success,
    Danger,
}

impl Tone {
    pub fn badge_class(self) -> &'static str {
        match self {
            Tone::Neutral => "bg-inset text-soft",
            Tone::Accent => "bg-accent-soft text-accent",
            Tone::Success => "bg-inset text-success",
            Tone::Danger => "bg-inset text-danger",
        }
    }

    pub fn text_class(self) -> &'static str {
        match self {
            Tone::Neutral => "text-ink",
            Tone::Accent => "text-accent",
            Tone::Success => "text-success",
            Tone::Danger => "text-danger",
        }
    }
}
