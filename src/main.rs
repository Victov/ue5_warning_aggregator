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
                .arg(
                    arg!(-s --min_similarity "Required minimal warning similarity [0-100] percent")
                    .action(ArgAction::Set)
                    .default_value("80")
                    .value_parser(clap::builder::RangedI64ValueParser::<i32>::new().range(0..100)),
                )
                .arg(
                    arg!(-f --min_frequency "Required minimal frequency of warning before being reported")
                    .action(ArgAction::Set)
                    .default_value("1")
                    .value_parser(clap::builder::RangedI64ValueParser::<i32>::new().range(0..100))
                    )

                .arg(
                    arg!(-l --logfile <FILE> "Path to logfile to operate on")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("aggregate") {
        let minimal_similarity = *matches
            .get_one::<i32>("min_similarity")
            .expect("should have default!");
        let minimal_similarity: f32 = minimal_similarity as f32 / 100.0f32;
        let minimal_frequency = *matches
            .get_one::<i32>("min_frequency")
            .expect("shoudl have default!");

        if let Some(path) = matches.get_one::<PathBuf>("logfile").as_ref() {
            command_aggregate::aggregate_log(path, minimal_similarity, minimal_frequency).await?;
        } else {
            unreachable!("clap should enforce that there is always some path provided");
        }
    }

    Ok(())
}
