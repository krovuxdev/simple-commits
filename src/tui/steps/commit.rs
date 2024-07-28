use std::io::Stderr;

use promptuity::{prompts::SelectOption, Promptuity};

use crate::tui::widgets::{Autocomplete, AutocompletePriority};
use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{structs::COMMIT_TYPES, Step, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut Promptuity<Stderr>,
        state: &mut crate::tui::State,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let commit = p.prompt(&mut Autocomplete::new(
            "Select a type",
            true,
            AutocompletePriority::Label,
            COMMIT_TYPES
                .map(|c| SelectOption::new(c, c.label.to_owned()).with_hint(c.hint))
                .to_vec(),
        ));

        state.commit.set_type(Some(commit?));
        Ok(())
    }
}
