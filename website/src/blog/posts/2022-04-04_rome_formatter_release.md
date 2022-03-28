---
title: Announcing Rome Formatter
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

Why use our new formatter? Let’s start with performance. Our formatter is extremely fast. It uses our brand new Rust codebase to parse and format faster than the competition. This is extremely important for a few reasons. For one, it’s very common for developers to have a git hook or continuous integration workflow that verifies that their repository is correctly formatted. For existing JavaScript formatters such as Prettier, this can take over 30 seconds on a top computer!

As a quick example of our performance, we decided to compare our formatter to Prettier, a popular JavaScript formatter by formatting some large open source projects on an M1 Macbook Air with 8GB of RAM. A quick cautionary note, we are not running a proper statistically rigorous setup like [Criterion.rs](https://github.com/bheisler/criterion.rs). However, we believe that these numbers are an accurate representation of the performance you will experience by using Rome. We plan on providing more rigorous benchmarks in the future.

We started by running our formatter on the TypeScript compiler's codebase. While Prettier took over 30 seconds on average to format the codebase, Rome took less than 5 seconds.

<div style="display: flex; flex-direction: column; align-items: center; padding: 20px">
  <img
    style="max-width: 600px"
    alt="Bar graph comparing Prettier and Rome performance in formatting the TypeScript compiler's codebase. Prettier is at 30 seconds while Rome is at 2.8 seconds"
    src="/img/blog/formatter_benchmark_typescript.png"
  />
  Formatting the TypeScript src/ directory in place, lower is better
</div>

We got similar results when we ran the formatter on ESLint and Webpack's codebases:

<div style="display: flex; flex-direction: column; align-items: center; padding: 20px">
  <img
    style="max-width: 600px"
    alt="Bar graph comparing Prettier and Rome performance in formatting the Webpack codebase. Prettier is at 6.8 seconds seconds while Rome is at 0.74 seconds"
    src="/img/blog/formatter_benchmark_webpack.png"
  />
  Formatting the Webpack lib/ directory in place, lower is better
</div>

<div style="display: flex; flex-direction: column; align-items: center; padding: 20px">
  <img
    style="max-width: 600px"
    alt="Bar graph comparing Prettier and Rome performance in formatting the ESLint codebase. Prettier is at 5.1 seconds seconds while Rome is at 0.40 seconds"
    src="/img/blog/formatter_benchmark_eslint.png"
  />
  Formatting the ESLint lib/ and packages/ directory in place, lower is better
</div>

With our new formatter, we made sure to focus on ease of adoption. We know most users already use Prettier to format their code. Therefore, we tried to defer to Prettier as much as possible for styling decisions. While we can’t guarantee perfect Prettier compatibility, we have tried to minimize the differences as much as possible. We plan on doing additional work in the future to reach Prettier compatibility, and to document any purposeful deviations from Prettier.

In addition, our formatter has first-class IDE support. We wanted our formatter to be like any other tool in your IDE. One particular characteristic of IDEs is that they’re very good at error recovery. Even if you have a syntax error on line 5, the IDE still provides highlighting and code fixes on line 15. Therefore, it was really important to us that the formatter work with incorrect code. We put a lot of work into building an error tolerant JavaScript/TypeScript parser that could quickly recover from errors. That way, no matter how many syntax errors in your code, our formatter could still make your code look better.

<img style="max-width: 800px" alt="Formatting TypeScript code with a syntax error" src="/img/blog/formatter_broken_code_demo.gif" />

We're still refining our formatter's error recovery, so we made it an opt-in feature for the first release. Feel free to try it out and give us some feedback. In the future we plan on having it on by default.

And of course, this is just the beginning. We’re going to expand the formatter to capture all the different languages we use in web development, such as CSS, HTML, JSON, etc. We also plan on announcing some new products in the near future. Not to mention, we’re still dedicated to our mission of having Rome be the single, all-in-one toolchain for your web development needs. If you’re interested in joining this mission, we’re hiring. Stay tuned!
