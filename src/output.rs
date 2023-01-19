use anyhow::Result;
use async_std::{
    fs::File,
    io::{BufWriter, WriteExt},
};
use std::path::PathBuf;

pub(crate) fn output_stdout(warnings: &Vec<(String, i32)>) {
    for (warning, count) in warnings {
        println!("Count: {} \n\n\t{}\n", count, warning);
    }
}

pub(crate) async fn output_file(warnings: &Vec<(String, i32)>, path: &PathBuf) -> Result<()> {
    let f = File::create(path).await?;
    {
        let mut writer = BufWriter::new(f);
        for (warning, count) in warnings {
            writer
                .write_fmt(format_args!("Count: {} \n\n\t{}\n", count, warning))
                .await?;
        }
    }

    Ok(())
}
