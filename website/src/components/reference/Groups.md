{/** this file is auto generated, use `cargo lintdoc` to update it */}
- `Accessibility`: Rules focused on preventing accessibility problems.
- `Complexity`: Rules that focus on inspecting complex code that could be simplified.
- `Correctness`: Rules that detect code that is guaranteed to be incorrect or useless.
- `Performance`: Rules catching ways your code could be written to run faster, or generally be more efficient.
- `Security`: Rules that detect potential security flaws.
- `Style`: Rules enforcing a consistent and idiomatic way of writing your code.
- `Suspicious`: Rules that detect code that is likely to be incorrect or useless.
- `Nursery`: New rules that are still under development.  Nursery rules require explicit opt-in via configuration on stable versions because they may still have bugs or performance problems. They are enabled by default on nightly builds, but as they are unstable their diagnostic severity may be set to either error or warning, depending on whether we intend for the rule to be recommended or not when it eventually gets stabilized. Nursery rules get promoted to other groups once they become stable or may be removed.  Rules that belong to this group are not subject to semantic version.
