use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use std::path::PathBuf;

pub(crate) async fn aggregate_log(path: &PathBuf, _with_warnings: bool) -> anyhow::Result<()> {
    let f = File::open(path).await?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut count = 0;
    while let Some(Ok(line)) = lines.next().await {
        count += 1;
        println!("{}", line);
    }

    println!("those were {} lines", count);
    Ok(())
}
