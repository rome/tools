---
title: Announcing Rome Formatter
description: Release of Rome Formatter, a super fast formatter for JavaScript and TypeScript
author_name: Rome Team
author_avatar: /img/favicon.svg
date: 2022-04-04
tags:
- formatter
- typescript
- release
permalink: /blog/2022/04/04/rome_formatter_release.html
layout: layouts/blog.liquid
cover-image: formatter-cover.webp
social-image: social-logo-formatter.png
---
We’re very excited to announce our first release of Rome Formatter, a new formatter for JavaScript and TypeScript. It perfectly expresses our goals of excellent performance, and first-class IDE integration.

You can try out our formatter by installing our [Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome). We also have a CLI in an early alpha state that you can download [from our releases page](https://github.com/rome/tools/releases).

# Performance

Why use Rome Formatter? Let’s start with performance. Our formatter is extremely fast. It uses our brand-new Rust codebase to parse and format faster than the competition. This is extremely important for a few reasons. For one, it’s very common for developers to have a git hook or continuous integration workflow that verifies that their repository is correctly formatted. For existing JavaScript formatters such as Prettier, this can take over 30 seconds on a top computer!

With a Docker container on a continuous integration workflow, this can be even longer. Nobody likes waiting for CI to finish. It disrupts your flow, wasting time and energy, not to mention running up your AWS or GCP bill.

As a quick example of Rome Formatter's performance, we decided to compare it to Prettier, a popular  formatter, by formatting some large open source projects such as ESLint, Webpack, and TypeScript on an M1 Macbook Air with 8GB of RAM [^1]. We found that formatting with Rome is 9-12x faster than formatting with Prettier.

[^1]: A quick cautionary note, we are not running a proper statistically rigorous setup like [Criterion.rs](https://github.com/bheisler/criterion.rs). However, we believe that these numbers are an accurate representation of the performance you will experience by using Rome. We plan on providing more rigorous benchmarks in the future.

<div style="display: flex; flex-direction: column; align-items: center; padding: 20px">
  <img
    style="max-width: 800px"
    alt="Bar graph comparing Prettier and Rome performance in formatting the ESLint, Webpack and TypeScript compiler codebases. For ESLint, Prettier is at 5.1 seconds while Rome is at 0.4 seconds. For Webpack, Prettier is at 6.9 seconds and Rome is at 0.74 seconds. For TypeScript, Prettier is at 30 seconds while Rome is at 2.8 seconds"
    src="/img/blog/formatter_benchmark.png"
  />
  Benchmarks of formatting popular open source repositories, lower is better
</div>


# IDE Integration

In addition, our formatter has first-class IDE support. We wanted our formatter to be like any other tool in your IDE. One particular characteristic of IDEs is that they’re very good at error recovery. Even if you have a syntax error on line 5, the IDE still provides highlighting and code fixes on line 15. With existing formatters such as Prettier, this is not the case. One single syntax error can hold up formatting for the rest of the code.

We wanted to change that. Therefore, it was really important to us that the formatter work with incorrect code. We put a lot of work into building an error tolerant JavaScript/TypeScript parser that could quickly recover from errors, and then extending our formatter to work on this broken code. That way, no matter how many syntax errors in your code, our formatter could still make it look better.

<img style="max-width: 800px" alt="Formatting TypeScript code with a syntax error" src="/img/blog/formatter_broken_code_demo.gif" />

We're still refining our formatter's error recovery, so we made it an opt-in feature for the first release. Feel free to try it out and give us some feedback. In the future we plan on having it on by default.

# Prettier Compatibility

With our new formatter, we made sure to focus on ease of adoption. We know most users already use Prettier to format their code. Therefore, we decided to embrace Prettier's styling decisions as much as possible. While we can’t guarantee perfect Prettier compatibility, we have tried to minimize the differences as much as possible. We plan on doing additional work in the future to reach Prettier compatibility, and to document any purposeful deviations from Prettier.

# More Coming

And of course, this is just the beginning. We’re going to expand the formatter to capture all the different languages we use in web development, such as CSS, HTML, JSON, etc. We also plan on announcing some new products in the near future. Not to mention, we’re still dedicated to our mission of having Rome be the single, all-in-one toolchain for your web development needs. If you’re interested in joining this mission, [we’re hiring](https://careers.rome.tools). Stay tuned!
