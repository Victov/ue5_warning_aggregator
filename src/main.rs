use anyhow::Result;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod command_aggregate;

#[async_std::main]
async fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("aggregate")
                .about("parse the log and aggregate contents")
                .arg(arg!(-w --warnings "collect warnings").action(ArgAction::SetTrue))
                .arg(
                    arg!(-l --logfile <FILE> "Path to logfile to operate on")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("aggregate") {
        let aggregate_warnings = *matches.get_one::<bool>("warnings").unwrap_or(&true);
        if let Some(path) = matches.get_one::<PathBuf>("logfile").as_ref() {
            println!("going to aggregate: with warnigns: {}", aggregate_warnings);
            command_aggregate::aggregate_log(path, aggregate_warnings).await?;
        } else {
            unreachable!("clap should enforce that there is always some path provided");
        }
    }

    Ok(())
}
