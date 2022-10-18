use anyhow::Result;

use crate::{cli::CliArgs, json::Space, query::QueryCommands, Runner};

pub struct SpaceCommands;

pub struct LabelOpts {
    pub space_ident: Option<String>,
    pub label: String,
}

impl SpaceCommands {
    // open, create, label focus
    pub fn open_or_create(runner: &impl Runner, args: CliArgs) {
        
    }
    pub fn create_labeled_space(runner: &impl Runner, args: CliArgs) {
        let new_space = Self::create_space(runner);
        let label_opts = LabelOpts {
            space_ident: Some(new_space.id.to_string()),
            label: args.where_.unwrap(),
        };
        Self::label_space(runner, label_opts);
    }

    pub fn create_space(runner: &impl Runner) -> Space {
        let _ = runner.run_yabai(vec!["-m", "space", "--create"]);
        let new_space = QueryCommands::query_space(runner, Some("last".to_string()));
        new_space.unwrap()
    }

    pub fn label_space(runner: &impl Runner, label_opts: LabelOpts) {
        if let Some(to) = label_opts.space_ident {
            runner.run_yabai(vec![
                "-m",
                "space",
                to.as_str(),
                "--label",
                label_opts.label.as_str(),
            ]);
            return ();
        }

        runner.run_yabai(vec!["-m", "space", "--label", label_opts.label.as_str()]);
        return ();
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
