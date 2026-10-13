use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl Variant {
    fn class(self) -> &'static str {
        match self {
            Variant::Primary => "bg-primary text-on-primary shadow-card hover:opacity-90",
            Variant::Secondary => "border border-line bg-inset text-ink hover:bg-line",
            Variant::Ghost => "text-soft hover:bg-inset",
            Variant::Danger => "bg-danger text-white hover:opacity-90",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    #[default]
    Md,
    Sm,
}

impl Size {
    fn class(self) -> &'static str {
        match self {
            Size::Md => "px-5 py-2.5 text-sm",
            Size::Sm => "px-3 py-1.5 text-xs",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: Variant,
    #[props(default)] size: Size,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    let size_class = size.class();
    let state_class = if disabled {
        "opacity-50"
    } else {
        "cursor-pointer"
    };

    rsx! {
        button {
            class: "inline-flex select-none items-center justify-center gap-2 rounded-full font-medium press {size_class} {variant_class} {state_class} {class}",
            r#type: "button",
            disabled,
            onclick: move |event| {
                if !disabled {
                    onclick.call(event);
                }
            },
            {children}
        }
    }
}
