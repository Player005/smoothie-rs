#[macro_use] // to parse --json in video.rs
extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

extern crate colored;
extern crate ffprobe; // cli wrapper

// rustsynth output
extern crate anyhow;
extern crate num_rational;

extern crate regex;

mod gui;
mod cli;
mod cmd;
// mod ffpb;
// mod ffpb2;
mod parse;
mod recipe;
mod render;
mod utils;
//mod vapoursynth;
mod video;

use crate::{cli::Arguments, cmd::SmCommand, video::Payload};
use std::env;
use utils::verbosity_init;

fn main() {
    if enable_ansi_support::enable_ansi_support().is_err() {
        println!("Failed enabling ANSI color support, expect broken colors!")
    }

    // unused for now as it spams the API each time you launch it :/...
    // parse::parse_update();

    let mut args: Arguments = cli::setup_args();
    // args.input is the only one being mutated in video.rs

    // Recipe and WidgetMetadata
    let (mut recipe, _metadata) = recipe::get_recipe(&mut args);
    // mutable because args.verbose sets `[miscellaneous] always verbose:` to true
    // loads defaults.ini, then overrides recipe.ini over it

    verbosity_init(
        args.verbose,
        recipe.get_bool("miscellaneous", "always verbose"),
    );

    let is_conhost: bool = (env::var("WT_SESSION").is_err() && env::var("ALACRITY_LOG").is_err())
        || env::var("NO_SMOOTHIE_WIN32").is_ok();
    // user is neither running Windows Terminal and alacritty, OR has NO_SMOOTHIE_WIN32 defined

    if args.tui
        && is_conhost
        && cfg!(target_os = "windows")
        && !recipe.get_bool("miscellaneous", "always verbose")
        && !args.verbose
    {
        utils::set_window_position(&recipe);
    }

    let _payloads: Vec<Payload>;

    if args.input.is_empty() && !args.tui {
        let _ = gui::sm_gui(recipe.clone(), _metadata);
        _payloads = vec![];
    } else {
        _payloads = video::resolve_input(&mut args, &recipe);
    }
    

    let commands: Vec<SmCommand> = cmd::build_commands(args, _payloads, recipe);

    render::_vpipe_render2(commands);
}
