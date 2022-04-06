---
title: Rome Formatter and Rust Update
description: An update from the Rome team on the Rust rewrite progress
author_name: Rome Team
author_avatar: /img/circle-indent-logo.png
date: 2022-02-08
tags:
	- update
	- post
permalink: /blog/2022/02/08/rome-formatter-and-rust-update.html
layout: layouts/blog.liquid
---

With our [last update](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust), we announced a complete rewrite in Rust. As many can attest, rewriting can be a rather long, difficult process. Our team has been working extremely hard on this undertaking and has made great progress. We’ll write a more thorough post about our experience with Rust and rewriting in the future, but here’s a short summary of our efforts.

We started our Rust rewrite by forking the excellent RSLint parser. A huge thanks the entire RSLint team for their work, especially to [Riccardo D'Ambrosio](https://github.com/RDambrosio016) for helping us with this process. We chose the RSLint parser as we wanted a more modern, editor-focused architecture inspired by projects such as Roslyn, rust-analyzer, and TypeScript. The parser produces a concrete syntax tree (CST) that represents the original code completely, whitespace, comments and all. While a compiler can throw away semantically irrelevant info such as comments, an editor must preserve these bits of *trivia*.

However, this CST is not the easiest to navigate, which required an abstract syntax tree (AST) facade on top of this CST that allowed for a cleaner interface. This parsing infrastructure will be the foundation for Rome’s focus on first class editor support. By representing the code in full fidelity, we can offer a code manipulation API for features like refactoring, code fixes, and many others.

We’re also very thrilled to announce the first showcase of this parser in our Rome Formatter: A blazing fast JavaScript and TypeScript formatter. Formatters are a perfect showcase of our parsing architecture, as they too need to preserve trivia. But why create a new formatter? When we began our Rust rewrite, we fell in love with rustfmt, Rust’s own formatter. It formatted code instantly, unlike JavaScript formatters which could often take quite a few seconds on larger files. We wanted that same experience in JavaScript. Rome Formatter, like all our tools, will have the editor experience as the core. We plan on shipping a VSCode extension in the next few months that will allow for one-click adoption and usage.

We’ve also expanded our team, hiring four new members this past six months, bringing us to a total of eight team members. We’re really excited to welcome [Daniel](https://github.com/xunilrj), [Léo-Paul](https://github.com/leops), and [Nicholas](https://github.com/NicholasLYang) onto the team.

Lastly, we shipped out swag to everybody who donated to our [fundraising campaign](https://rome.tools/funding/). We’ve received an [amazing](https://twitter.com/KrComet/status/1486556451011444737) [response](https://twitter.com/vcarl_/status/1486723806269874177)!

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">Whoooa, just received goodies from <a href="https://twitter.com/rometools?ref_src=twsrc%5Etfw">@rometools</a> 🤩 <a href="https://t.co/GRP96udzde">pic.twitter.com/GRP96udzde</a></p>&mdash; Hyeseong Kim (@KrComet) <a href="https://twitter.com/KrComet/status/1486556451011444737?ref_src=twsrc%5Etfw">January 27, 2022</a></blockquote>

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">My <a href="https://twitter.com/rometools?ref_src=twsrc%5Etfw">@rometools</a> swag came! The huge fluffy blanket is a good touch going into February <a href="https://t.co/VRbd38akCx">pic.twitter.com/VRbd38akCx</a></p>&mdash; Carl Vitullo (@vcarl_) <a href="https://twitter.com/vcarl_/status/1486723806269874177?ref_src=twsrc%5Etfw">January 27, 2022</a></blockquote>



---

We have a lot more in the pipeline. Stay tuned for future blog posts, other product announcements and some exciting updates.

<script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
