use crate::{
    cli::{CliArgs, With},
    json::{Display, Space, Window},
    DomainInfo, Runner, YabaiRunner,
};
use anyhow::Result;

pub struct QueryCommands;

impl QueryCommands {
    pub fn exec_query_option(args: CliArgs) -> Result<DomainInfo> {
        let runner = YabaiRunner::new();
        match args.with {
            With::Spaces => {
                let spaces = Self::query_spaces(&runner)?;
                dbg!(&spaces);
                Ok(DomainInfo::Spaces(spaces))
            }
            With::Space => {
                let space = Self::query_space(&runner, args.where_);
                dbg!(&space);
                Ok(DomainInfo::Space(space))
            }
            With::Windows => {
                let windows = Self::query_windows(&runner)?;
                dbg!(&windows);
                Ok(DomainInfo::Windows(windows))
            }
            With::Window => {
                let window = Self::query_window(&runner, args.where_);
                dbg!(&window);
                Ok(DomainInfo::Window(window))
            }
            With::Displays => {
                let displays = Self::query_displays(&runner);
                dbg!(&displays);
                Ok(DomainInfo::Displays(displays))
            }
            With::Display => {
                let display = Self::query_display(&runner, args.where_);
                dbg!(&display);
                Ok(DomainInfo::Display(display))
            }
        }
    }

    pub fn query(runner: &impl Runner, query_params: Vec<&str>) -> String {
        let base_query = vec!["-m", "query"];
        let qp = [base_query, query_params].concat();
        runner.run_yabai(qp)
    }

    pub fn query_displays(runner: &impl Runner) -> Vec<Display> {
        let q = Self::query(runner, vec!["--displays"]);
        let s: Vec<Display> = serde_json::from_str(q.as_str()).unwrap();
        s
    }

    pub fn query_display(runner: &impl Runner, ident: Option<String>) -> Option<Display> {
        if let Some(ident) = ident {
            let res = Self::query(runner, vec!["--displays", "--display", ident.as_str()]);
            let d: Display = serde_json::from_str(res.as_str()).unwrap();
            return Some(d);
        }
        None
    }

    pub fn query_spaces(runner: &impl Runner) -> Result<Vec<Space>> {
        let q = Self::query(runner, vec!["--spaces"]);
        let s: Vec<Space> = serde_json::from_str(q.as_str())?;
        Ok(s)
    }

    pub fn query_space(runner: &impl Runner, ident: Option<String>) -> Option<Space> {
        let res = Self::query(runner, vec!["--spaces", "--space", ident.unwrap_or(String::new()).as_str()]);
        let s: Space = serde_json::from_str(res.as_str()).unwrap();
        return Some(s);
    }

    pub fn query_windows(runner: &impl Runner) -> Result<Vec<Window>> {
        let q = Self::query(runner, vec!["--windows"]);
        let w: Vec<Window> = serde_json::from_str(q.as_str())?;
        Ok(w)
    }

    pub fn query_window(runner: &impl Runner, ident: Option<String>) -> Option<Window> {
        if let Some(ident) = ident {
            let res = Self::query(runner, vec!["--windows", "--window", ident.as_str()]);
            let w: Window = serde_json::from_str(res.as_str()).unwrap();
            return Some(w);
        }
        None
    }
}
