pub mod analyzer;
pub mod formatter;
pub mod parser;

#[cfg(feature = "dhat-on")]
fn print_diff(before: dhat::Stats, current: dhat::Stats) -> dhat::Stats {
    use humansize::{file_size_opts as options, FileSize};

    println!("\tMemory");
    if let Some(heap) = &current.heap {
        println!("\t\tCurrent Blocks: {}", heap.curr_blocks);
        println!(
            "\t\tCurrent Bytes: {}",
            heap.curr_bytes.file_size(options::CONVENTIONAL).unwrap()
        );
        println!("\t\tMax Blocks: {}", heap.max_blocks);
        println!(
            "\t\tMax Bytes: {}",
            heap.max_bytes.file_size(options::CONVENTIONAL).unwrap()
        );
    }

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
