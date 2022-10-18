#![allow(unused)]
use crate::{
    cli::{CliArgs, What},
    json::{Display, Space, Window},
    space::SpaceCommands,
};
use anyhow::Result;
use clap::Parser;
use cli::With;
use space::LabelOpts;
use core::fmt::Formatter;
use query::QueryCommands;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    process::{Command, Output},
};
mod cli;
mod json;
mod query;
mod space;
mod window;

static FIREFOX_APP_LABEL: &str = "Firefox Developer Edition";
static VS_CODE_APP_LABEL: &str = "Code";
static ITERM_APP_LABEL: &str = "iTerm";

fn main() -> Result<()> {
    let args = CliArgs::parse();
    match args.what {
        What::Query => {
            QueryCommands::exec_query_option(args);
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
        What::Label => {
            exec_label_option(args);
        }
    }
    Ok(())
}

pub enum DomainInfo {
    Spaces(Vec<Space>),
    Space(Option<Space>),
    Windows(Vec<Window>),
    Window(Option<Window>),
    Displays(Vec<Display>),
    Display(Option<Display>),
}

fn exec_label_option(args: CliArgs) {
    let runner = YabaiRunner::new();
    match args.with {
        With::Space => {
            SpaceCommands::create_labeled_space(&runner, args);
        },
        _ => {}
    }
}

fn exec_create_option(args: CliArgs) {
    let runner = YabaiRunner::new();
    let where_ = args.where_.clone();
    if let Some(label) = where_ {
        SpaceCommands::create_labeled_space(&runner, args);
        return ();
    }

    SpaceCommands::create_space(&runner);
}

fn exec_focus_option(args: CliArgs) {}

fn exec_destroy_option(args: CliArgs) {}

pub trait Runner {
    fn run_yabai(&self, args: Vec<&str>) -> String;
}

#[derive(Clone)]
pub struct YabaiRunner;

impl Runner for YabaiRunner {
    fn run_yabai(&self, args: Vec<&str>) -> String {
        let output = Command::new("yabai")
            .args(args)
            .output()
            .expect("failed to execute process");
        let o = output.stdout.iter().map(|c| *c as char).collect();
        o
    }
}

impl YabaiRunner {
    pub fn new() -> Self {
        YabaiRunner {}
    }
}
