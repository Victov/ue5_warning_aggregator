use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use similar_string::find_best_similarity;
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) async fn aggregate_log(
    path: &PathBuf,
    minimal_similarity: f32,
    minimal_frequency: i32,
) -> anyhow::Result<()> {
    let f = File::open(path).await?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut lines_with_warning = lines
        .filter_map(|a| a.ok())
        .filter(|l: &String| l.contains("Warning: "));

    let mut occurrences_table: HashMap<String, i32> = HashMap::new();
    while let Some(line) = lines_with_warning.next().await {
        let keys: Vec<String> = occurrences_table.keys().cloned().collect();
        if let Some((existing_key, score)) = find_best_similarity(line.clone(), &keys) {
            if score > minimal_similarity as f64 {
                let count: &mut i32 = occurrences_table.get_mut(&existing_key).unwrap();
                *count += 1;
            } else {
                occurrences_table.insert(line.clone(), 0);
            }
        } else {
            occurrences_table.insert(line.clone(), 0);
        }
    }

    let mut results: Vec<(String, i32)> = occurrences_table
        .into_iter()
        .filter(|(_, count)| *count > minimal_frequency)
        .collect();
    results.sort_by(|(_, count1), (_, count2)| count1.cmp(count2));

    for (warning, count) in results {
        println!("Count: {}, \n\n\t{}\n", count, warning);
    }

    Ok(())
}
