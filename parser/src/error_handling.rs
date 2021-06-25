use colored::*;
use std::cmp;
use std::sync::{Arc, Mutex};

pub fn handle_parse_error(location: Arc<Mutex<Vec<usize>>>, composition: &str) -> (usize, usize) {
    let start_offset = 125;
    let end_offset = 50;
    let cmp_len = &composition.len();
    let end = cmp_len;

    let arg_len = location.lock().unwrap().len();
    if arg_len == 2 {
        let _end = location.lock().unwrap()[1];
    }

    let start = location.lock().unwrap()[0];

    let feed_start = cmp::max(0, start as isize - start_offset) as usize;
    let mut feed_end = cmp::min(end + end_offset, *cmp_len);
    if feed_end - feed_start > 300 {
        feed_end = feed_start + 300
    }
    let mut lines = 0;
    let mut columns = 0;
    for (n_c, c) in composition.chars().enumerate() {
        if n_c > start {
            break;
        }
        if c == '\n' {
            lines += 1;
            columns = 0;
        }

        columns += 1;
    }
    println!(
        "{}{}",
        &composition[feed_start..start].yellow(),
        &composition[start..feed_end].red(),
    );

    println!(
        "
            {}
            errors at line {}
            {}
            ",
        "working".yellow().underline(),
        lines.to_string().red().bold(),
        "broken".red().underline(),
    );

    (lines, columns - 2)
}
