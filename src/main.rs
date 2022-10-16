#![allow(unused)]
use crate::{
    cli::{CliArgs, What},
    json::{Display, Space, Window},
    space::SpaceCommands,
};
use anyhow::Result;
use clap::Parser;
use cli::With;
use label::LabelCommands;
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
mod label;

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
            LabelCommands::label_space(&runner, args);
        },
        _ => {}
    }

}

fn exec_create_option(args: CliArgs) {
    let runner = YabaiRunner::new();
    let ident = args.where_;
    if let Some(ident) = ident {
        SpaceCommands::create_space(&runner, ident);
    }
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
