pub mod analyzer;
pub mod formatter;
pub mod parser;

#[cfg(feature = "dhat-heap")]
fn print_diff(before: dhat::HeapStats, current: dhat::HeapStats) -> dhat::HeapStats {
    use humansize::{file_size_opts as options, FileSize};

    println!("\tMemory");
    println!("\t\tCurrent Blocks: {}", current.curr_blocks);
    println!(
        "\t\tCurrent Bytes: {}",
        current.curr_bytes.file_size(options::CONVENTIONAL).unwrap()
    );
    println!("\t\tMax Blocks: {}", current.max_blocks);
    println!(
        "\t\tMax Bytes: {}",
        current.max_bytes.file_size(options::CONVENTIONAL).unwrap()
    );

    println!(
        "\t\tTotal Blocks: {}",
        current.total_blocks - before.total_blocks
    );
    println!(
        "\t\tTotal Bytes: {}",
        (current.total_bytes - before.total_bytes)
            .file_size(options::CONVENTIONAL)
            .unwrap()
    );

    current
}
