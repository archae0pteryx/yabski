use crate::{Runner, cli::CliArgs};

pub struct LabelCommands;

impl LabelCommands {
    pub fn label_space(runner: &impl Runner, args: CliArgs) {
      let from_or_label = args.where_;
      let to = args.to;

      if let Some(to) = to {
        runner.run_yabai(vec!["-m", "space", from_or_label.unwrap().as_str(), "--label", to.as_str()]);
        return ();
      }

      if let Some(label) = from_or_label {
        runner.run_yabai(vec!["-m", "space", "--label", label.as_str()]);
        return ();
      }

      panic!("No label provided");
    }
}
