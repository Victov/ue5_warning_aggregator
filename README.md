# Unreal Engine 5 Warning Aggregator
Tool to scan logs from your Unreal Engine project and aggregate frequently-occurring warnings. The tool then outputs a sorted list of warnings, most frequent warnings first. The tool works based on a similarity score, so warnings have to be roughly similar to get grouped, rather than be identical. 

### usage

Run from cargo with a minimal similarity score of 80% and warnings have to occur at least twice before being outputted.
`cargo run -- aggregate --logfile <path_to_log> --min_similarity 80 --min_frequency 2`

### planned features
- Scan and aggregate multiple files (e.g. scan output of build, cook and runtime)
- Improved output clarity
- Output to file instead of console

