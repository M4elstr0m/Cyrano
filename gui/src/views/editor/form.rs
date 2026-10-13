use dioxus::prelude::*;
use education::EducationSection;
use experience::ExperienceSection;
use hard_skills::HardSkillsSection;
use identity::IdentitySection;
use languages::LanguagesSection;
use links::LinksSection;
use lists::{HobbiesSection, SoftSkillsSection};
use projects::ProjectsSection;

mod date_range;
mod education;
mod experience;
mod hard_skills;
mod identity;
mod languages;
mod links;
mod lists;
mod projects;
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
        ProjectsSection {}
        HardSkillsSection {}
        HobbiesSection {}
        SoftSkillsSection {}
        LanguagesSection {}
    }
}
