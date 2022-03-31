use std::io;

use similar::{udiff::UnifiedHunkHeader, ChangeTag, TextDiff};

use crate::{
    self as rome_console,
    fmt::{Display, Formatter},
    markup,
};

/// Utility struct to print a diff between two texts in the console
pub struct Diff<'a> {
    /// The previous version of the text
    pub left: &'a str,
    /// The next version of the text
    pub right: &'a str,
}

impl<'a> Display for Diff<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let diff = TextDiff::from_lines(self.left, self.right);

        let mut diff = diff.unified_diff();
        diff.context_radius(3);

        // Determine the maximum line length and line number to be displayed
        let mut max_line_length = 0;
        let mut max_line_number = 0;

        for hunk in diff.iter_hunks() {
            if let Some(first_op) = hunk.ops().first() {
                let left_end = first_op.old_range().end + 1;
                let right_end = first_op.new_range().end + 1;
                max_line_number = max_line_number.max(left_end).max(right_end);
            }

            for change in hunk.iter_changes() {
                max_line_length = max_line_length.max(change.value().trim_end().len());
            }
        }

        // Calculate the number of digits in the maximum line number (used to
        // right pad the line numbers when printing)
        let max_digits = count_digits(max_line_number);

        for hunk in diff.iter_hunks() {
            // Find the starting line number for the left and right sides
            let first_lines = hunk.ops().first().map(|first_op| {
                let left_start = first_op.old_range().start + 1;
                let right_start = first_op.new_range().start + 1;
                (left_start, right_start)
            });

            let (mut left_line, mut right_line) = match first_lines {
                Some(lines) => lines,
                // Hunk is empty, do not print anything and move to the next one
                None => continue,
            };

            // Print out the hunk header
            fmt.write_markup(markup! {
                <Info>{hunk.header()}</Info>"\n"
            })?;

            // Buffer the left and right sides to keep them aligned
            let mut left = Vec::new();
            let mut right = Vec::new();

            for change in hunk.iter_changes() {
                match change.tag() {
                    ChangeTag::Delete => {
                        left.push(Some(change.value().trim_end()));
                    }
                    ChangeTag::Insert => {
                        right.push(Some(change.value().trim_end()));
                    }
                    ChangeTag::Equal => {
                        let position = left.len().max(right.len());

                        // Ensure the two buffers are at the same position
                        // by padding them with empty lines
                        if left.len() < position {
                            left.resize(position, None);
                        }
                        if right.len() < position {
                            right.resize(position, None);
                        }

                        left.push(Some(change.value().trim_end()));
                        right.push(Some(change.value().trim_end()));
                    }
                }
            }

            // Print the two buffers in lockstep, specifically does *not* use
            // Iterator::zip since that would short-circuit once any of the two
            // iterators returned None while we want to fully consume both
            let mut left = left.into_iter();
            let mut right = right.into_iter();

            loop {
                let (left, right) = match (left.next(), right.next()) {
                    // Stop once both iterators return None
                    (None, None) => break,
                    (left, right) => (left.flatten(), right.flatten()),
                };

                // In the future this could be a per-character diff to help
                // make small changes stand out
                let is_same = left == right;

                if let Some(left) = left {
                    // Print the left line number and increment it
                    fmt.write_markup(markup! {
                        <Warn>
                            {format_args!("{: >1$} ", left_line, max_digits)}
                        </Warn>
                    })?;

                    left_line += 1;

                    // Don't print any padding space if the right side is empty anyway
                    let padding = if right.is_some() { max_line_length } else { 0 };

                    // Print the left side in red if the sides are different,
                    // use the standard text color otherwise
                    if !is_same {
                        fmt.write_markup(markup! {
                            <Error>{format_args!("- {left: <0$}", padding)}</Error>
                        })?;
                    } else {
                        fmt.write_fmt(format_args!("  {left: <0$}", padding))?;
                    }
                } else if right.is_some() {
                    // If the left side is empty but the right side isn't,
                    // print some padding spaces to align the columns
                    fmt.write_str(&" ".repeat(max_digits + 3 + max_line_length))?;
                }

                if let Some(right) = right {
                    // Print the right line number and increment it
                    fmt.write_markup(markup! {
                        <Warn>
                            {format_args!("{: >1$} ", right_line, max_digits)}
                        </Warn>
                    })?;

                    right_line += 1;

                    // Print the right side in green if the sides are different,
                    // use the standard text color otherwise
                    if !is_same {
                        fmt.write_markup(markup! {
                            <Success>"+ "{right}</Success>
                        })?;
                    } else if !right.is_empty() {
                        fmt.write_str("  ")?;
                        fmt.write_str(right)?;
                    }
                }

                // Print a line break
                writeln!(fmt)?;
            }
        }

        Ok(())
    }
}

impl Display for UnifiedHunkHeader {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        write!(fmt, "{self}")
    }
}

fn count_digits(mut value: usize) -> usize {
    let mut digits = 1;
    while value >= 10 {
        value /= 10;
        digits += 1;
    }

    digits
}

#[cfg(test)]
mod tests {
    use crate::{self as rome_console, diff::Diff, markup, BufferConsole, Console, Message};

    #[test]
    fn test_diff() {
        const SOURCE_LEFT: &str = "Lorem
ipsum
dolor
sit
amet,
function
name(
    args
) {}
consectetur
adipiscing
elit,
sed
do
eiusmod

incididunt
function
name(
    args
) {}";

        const SOURCE_RIGHT: &str = "Lorem
ipsum
dolor
sit
amet,
function name(args) {
}
consectetur
adipiscing
elit,
sed
do
eiusmod

incididunt
function name(args) {
}";

        const DIFF_RESULT: &str = "@@ -3,10 +3,8 @@
 3   dolor                 3   dolor
 4   sit                   4   sit
 5   amet,                 5   amet,
 6 - function              6 + function name(args) {
 7 - name(                 7 + }
 8 -     args
 9 - ) {}
10   consectetur           8   consectetur
11   adipiscing            9   adipiscing
12   elit,                10   elit,
@@ -15,7 +13,5 @@
15   eiusmod              13   eiusmod
16                        14 
17   incididunt           15   incididunt
18 - function             16 + function name(args) {
19 - name(                17 + }
20 -     args
21 - ) {}
";

        let diff = Diff {
            left: SOURCE_LEFT,
            right: SOURCE_RIGHT,
        };

        let mut console = BufferConsole::default();
        console.message(markup! {
            {diff}
        });

        let mut messages = console.buffer.into_iter();
        let message = match messages.next() {
            Some(Message::Message(msg)) => msg,
            other => panic!("unexpected message {other:?}"),
        };

        assert_eq!(message, DIFF_RESULT);

        assert!(messages.next().is_none());
    }
}
