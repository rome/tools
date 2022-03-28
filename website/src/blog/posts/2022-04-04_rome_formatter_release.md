---
title: Rome Formatter
description: Release of Rome Formatter, a super fast formatter for JavaScript and TypeScript
author_name: Rome Team
author_avatar: /img/favicon.svg
date: 2022-02-08
tags:
- update
- post
permalink: /blog/2022/04/04/rome_formatter_release.html
layout: layouts/blog.liquid
---
We’re very excited to announce our first release of Rome Formatter, a new formatter for JavaScript and TypeScript. It perfectly expresses our goals of excellent performance, and first-class IDE support.

You can try out our formatter by installing our [Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome). We also have a CLI in an early alpha state that you can download [from our releases page](https://github.com/rome/tools/releases).

Why use our new formatter? Let’s start with performance. Our formatter is extremely fast. It uses our brand new Rust codebase to parse and format faster than the competition. This is extremely important for a few reasons. For one, it’s very common for developers to have a git hook or continuous integration workflow ****that verifies that code is correctly formatted. For existing JavaScript formatters, this can take over 30 seconds on a top computer! Now imagine how long it takes on a much slower virtual machine for continuous integration.

As a quick example of our performance, we decided to compare our formatter to Prettier, a popular JavaScript formatter. A quick cautionary note, we’ll concede that we are not running a proper statistically rigorous setup like [Criterion.rs](https://github.com/bheisler/criterion.rs). However, we believe that these numbers are an accurate representation of the performance you will experience by using Rome. We plan on providing more rigorous benchmarks in the future.

We started by running our formatter on the TypeScript compiler's codebase. While Prettier took over 30 seconds on average to format the codebase, Rome and dprint took less than 5 seconds each.

<div style="display: flex; flex-direction: column; align-items: center; padding: 20px">
  <img
    style="max-width: 600px"
    alt="Bar graph comparing Prettier and Rome for formatting speed. Prettier is at 30 seconds while Rome is at 5 seconds"
    src="/img/blog/formatter_benchmark_typescript_codebase.png"
  />
  Formatting the TypeScript src/ directory in place, lower is better
</div>


One consideration we put into the formatter was ease of adoption. We know most users already use Prettier or other formatters. Therefore, we made it our goal to imitate Prettier as much as possible. While we can’t guarantee perfect Prettier compatibility, we have tried to minimize the differences as much as possible. We plan on doing additional work in the future to reach Prettier compatibility, and to document any purposeful deviations from Prettier.

In addition, our formatter has first-class IDE support. We wanted our formatter to be like any other tool in your IDE. One particular characteristic of IDEs is that they’re very good at error recovery. Even if you have a syntax error on line 5, the IDE still provides highlighting and code fixes on line 15. Therefore, it was really important to us that the formatter work with incorrect code. We put a lot of work into building an error tolerant JavaScript/TypeScript parser. That way, no matter how many syntax errors in your code, our formatter could still make your code look better.

<img style="max-width: 800px" alt="Formatting TypeScript code with a syntax error" src="/img/blog/formatter_broken_code_demo.gif" />


And of course, this is just the beginning. We’re going to expand the formatter to capture all the different languages we use in web development, such as CSS, HTML, JSON, etc. We also plan on announcing some new products in the near future. Not to mention, we’re still dedicated to our mission of having Rome be the single, all-in-one toolchain for your web development needs. If you’re interested in joining this mission, we’re hiring. Stay tuned!
