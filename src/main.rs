pub mod schema;
use clap::{
    command,
    Arg,
    Command,
    ArgAction,
    value_parser,
};
use std::path::PathBuf;
use axbind::{
    parse_glob,
};
#[tokio::main]
async fn main() {
    let clap_matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(Arg::new("GLOB").action(ArgAction::Append).value_parser(parse_glob))
        .arg(Arg::new("config_dir").long("config").short('c').value_parser(value_parser!(PathBuf)))
        .get_matches();
}

