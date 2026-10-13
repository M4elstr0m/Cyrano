use dioxus::prelude::*;
use education::EducationSection;
use experience::ExperienceSection;
use identity::IdentitySection;
use links::LinksSection;

mod date_range;
mod education;
mod experience;
mod identity;
mod links;
mod row;
mod string_list;
mod support;

#[component]
pub fn CvForm() -> Element {
    rsx! {
        IdentitySection {}
        LinksSection {}
        ExperienceSection {}
        EducationSection {}
    }
}
