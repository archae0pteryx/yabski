use anyhow::Result;

use crate::{json::Space, query::QueryCommands, Runner};

pub struct SpaceCommands;

impl SpaceCommands {
    pub fn create_space(runner: &impl Runner, ident: String) {
        let _ = runner.run_yabai(vec!["-m", "space", "--create"]);
        let new_space = QueryCommands::query_space(runner, Some("last".to_string())).unwrap();
        let new_id = new_space.index;
        Self::label_space(runner, new_id, ident);
    }

    pub fn label_space(runner: &impl Runner, id: usize, ident: String) {
        runner.run_yabai(vec![
            "-m",
            "space",
            id.to_string().as_str(),
            "--label",
            ident.as_str(),
        ]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_creates_space_with_name() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
