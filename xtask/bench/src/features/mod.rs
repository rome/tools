pub mod analyzer;
pub mod formatter;
pub mod parser;

#[cfg(feature = "dhat-heap")]
fn print_stats(current: dhat::HeapStats, before: Option<dhat::HeapStats>) -> dhat::HeapStats {
    use humansize::{format_size_i, DECIMAL};

    println!("\tMemory");
    println!("\t\tCurrent Blocks: {}", current.curr_blocks);
    println!(
        "\t\tCurrent Bytes: {}",
        format_size_i(current.curr_bytes, DECIMAL)
    );
    println!("\t\tMax Blocks: {}", current.max_blocks);
    println!(
        "\t\tMax Bytes: {}",
        format_size_i(current.max_bytes, DECIMAL)
    );

    if let Some(before) = before {
        let new_blocks = current.total_blocks - before.total_blocks;
        let new_bytes = current.total_bytes - before.total_bytes;
        println!("\t\tNew Blocks: {new_blocks}",);
        println!("\t\tNew Bytes: {}", format_size_i(new_bytes, DECIMAL));
    }

    println!("\t\tTotal Blocks: {}", current.total_blocks);
    println!(
        "\t\tTotal Bytes: {}",
        format_size_i(current.total_bytes, DECIMAL)
    );

    current
}
