#![allow(unused)]
use anyhow::Result;
use clap::Parser;
use cli::With;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::HashMap, process::Command};

use crate::{
    cli::{CliArgs, What},
    json::{Display, Space, Window},
};
mod cli;
mod json;

static FIREFOX_APP_LABEL: &str = "Firefox Developer Edition";
static VS_CODE_APP_LABEL: &str = "Code";
static ITERM_APP_LABEL: &str = "iTerm";

fn main() -> Result<()> {
    let args = CliArgs::parse();
    match args.what {
        What::Query => {
            exec_query_option(args);
        }
        What::Create => {
            exec_create_option(args);
        }
        What::Focus => {
            exec_focus_option(args);
        }
        What::Destroy => {
            exec_destroy_option(args);
        }
        What::RunningApps => {
            exec_current_apps_option(args);
        }
    }
    Ok(())
}

enum DomainInfo {
    Spaces(Vec<Space>),
    Space(Option<Space>),
    Windows(Vec<Window>),
    Window(Option<Window>),
    Displays(Vec<Display>),
    Display(Option<Display>),
}

fn exec_query_option(args: CliArgs) -> Result<DomainInfo> {
    match args.with {
        With::Spaces => {
            let spaces = query_spaces()?;
            dbg!(&spaces);
            Ok(DomainInfo::Spaces(spaces))
        }
        With::Space => {
            let space = query_space(args.where_);
            dbg!(&space);
            Ok(DomainInfo::Space(space))
        }
        With::Windows => {
            let windows = query_windows()?;
            dbg!(&windows);
            Ok(DomainInfo::Windows(windows))
        }
        With::Window => {
            let window = query_window(args.where_);
            dbg!(&window);
            Ok(DomainInfo::Window(window))
        }
        With::Displays => {
            let displays = query_displays()?;
            dbg!(&displays);
            Ok(DomainInfo::Displays(displays))
        }
        With::Display => {
            let display = query_display(args.where_);
            dbg!(&display);
            Ok(DomainInfo::Display(display))
        }
    }
}

fn exec_create_option(args: CliArgs) {
    let ident = args.where_;
    if let Some(ident) = ident {
        create_space(ident);
    }
}

fn create_space(ident: String) -> Result<()> {
    let _ = run_command("yabai", &["-m", "space", "--create"]).unwrap();
    let new_space = query_space(Some("last".to_string())).unwrap();
    let new_id = new_space.index;
    let _ = run_command(
        "yabai",
        &[
            "-m",
            "space",
            new_id.to_string().as_str(),
            "--label",
            ident.as_str(),
        ],
    )
    .unwrap();
    // dbg!(&new_space);
    Ok(())
}

fn exec_focus_option(args: CliArgs) {}

fn exec_destroy_option(args: CliArgs) {}

fn exec_current_apps_option(args: CliArgs) {
    match args.what {
        What::RunningApps => {
            let apps = query_running_apps();
            dbg!(apps);
        }
        _ => {
            anyhow::Error::msg("Invalid option");
        }
    }
}

fn get_window_by_app(name: &str) -> Result<Vec<(String, usize)>> {
    let apps = query_running_apps()?;
    let has = apps
        .iter()
        .filter(|(app, window_id)| app == name)
        .map(|f| f.to_owned())
        .collect::<Vec<_>>();
    Ok(has)
}

fn query_running_apps() -> Result<Vec<(String, usize)>> {
    let windows = query_windows()?;
    let apps = windows
        .iter()
        .map(|w| (w.app.to_owned(), w.id))
        .collect::<Vec<(String, usize)>>();
    Ok(apps)
}

fn query_displays() -> Result<Vec<Display>> {
    let q = query("--displays")?;
    let s: Vec<Display> = serde_json::from_str(q.as_str())?;
    Ok(s)
}

fn query_display(ident: Option<String>) -> Option<Display> {
    if let Some(ident) = ident {
        let res =
            run_command("yabai", &["-m", "query", "--displays", "--display", &ident]).unwrap();
        let d: Display = serde_json::from_str(res.as_str()).unwrap();
        return Some(d);
    }
    None
}

fn query_spaces() -> Result<Vec<Space>> {
    let q = query("--spaces")?;
    let s: Vec<Space> = serde_json::from_str(q.as_str())?;
    Ok(s)
}

fn query_space(ident: Option<String>) -> Option<Space> {
    if let Some(ident) = ident {
        let res = run_command("yabai", &["-m", "query", "--spaces", "--space", &ident]).unwrap();
        let s: Space = serde_json::from_str(res.as_str()).unwrap();
        return Some(s);
    }
    None
}

fn query_windows() -> Result<Vec<Window>> {
    let q = query("--windows")?;
    let w: Vec<Window> = serde_json::from_str(q.as_str())?;
    Ok(w)
}

fn query_window(ident: Option<String>) -> Option<Window> {
    if let Some(ident) = ident {
        let res = run_command("yabai", &["-m", "query", "--windows", "--window", &ident]).unwrap();
        let w: Window = serde_json::from_str(res.as_str()).unwrap();
        return Some(w);
    }
    None
}

fn query(domain: &str) -> Result<String> {
    let res = run_command("yabai", &["-m", "query", domain])?;
    Ok(res)
}

fn run_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");
    let o = output.stdout.iter().map(|c| *c as char).collect();
    Ok(o)
}
