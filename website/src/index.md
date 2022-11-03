---
layout: layouts/base.liquid
description: End-to-end developer tools
social-image: social-logo.png
no-sidebar: true
---

<div class="homepage">
  <aside hidden class="latest-post" aria-labelledby="latest-post">
    <h2>Latest blog post</h2>
    <div class="info">
      <h3>
      <a href="{{ post.url }}">{{ post.data.title }}</a>
      </h3>
      <div class="author">
        {{ post.date | dateFormat }}
      </div>
    </div>
  </aside>

  <section>
    <h1>Rome is a unified <span>formatter</span></h1>
    <a href="/docs" class="docs">Documentation</a>
    <p>Rome unifies your development stack by combining the functionality of separate tools.</p>
    <p>Single configuration file, amazing performance, and works with any stack.</p>
    <ul class="supported-languages">
      <li>
        <div class="icon">{% include svg/homepage/javascript.svg %}</div>
        <div class="language">JavaScript</div>
      </li>
      <li>
        <div class="icon">{% include svg/homepage/typescript.svg %}</div>
        <div class="language">TypeScript</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/json.svg %}</div>
        <div class="language">JSON</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/html.svg %}</div>
        <div class="language">HTML</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/markdown.svg %}</div>
        <div class="language">Markdown</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/css.svg %}</div>
        <div class="language">CSS</div>
        <div class="soon-indicator">Soon</div>
      </li>
    </ul>
  </section>

  <hr>

  <section>
    <h2>Supercharge your workflow.</h2>
    <ul class="component-list">
      <li class="active">Formatter</li>
      <li>Linter</li>
      <li class="soon">
        <div class="text">Compiler</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="text">Bundler</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="text">Testing</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li class="soon">
        <div class="text">Documentation</div>
        <div class="soon-indicator">Soon</div>
      </li>
    </ul>
    <p class="founder-clout">Created by the founder of <a class="babel" href="https://babeljs.io/">{% include svg/homepage/babel.svg %}</a> and <a href="https://yarnpkg.com/" class="yarn">{% include svg/homepage/yarn.svg %}</a></p>
  </section>

  <hr class="full">

  <section>
    <h2 class="sr-only">Features</h2>
    <ul class="features">
      <li>
        <div class="icon">{% include svg/homepage/chevron.svg %}</div>
        <h3>Fast</h3>
        <p>Built with Rust and an architecture that scales to meet the performance demands of any project.</p>
      </li>
      <li>
        <div class="icon">{% include svg/homepage/layers.svg %}</div>
        <h3>Simple</h3>
        <p>No configuration file required to get started. Drop it into a project of any size.</p>
      </li>
      <li>
        <div class="icon">{% include svg/homepage/maximize.svg %}</div>
        <h3>Scalable</h3>
        <p>Lorem ipsum to asdfasdfasdfasdf</p>
      </li>
      <li>
        <div class="icon">{% include svg/homepage/action.svg %}</div>
        <h3>Actionable &amp; Informative</h3>
        <p>When you run into a problem we tell you exactly where it is and how to fix it.</p>
      </li>
      <li>
        <div class="icon">{% include svg/homepage/box.svg %}</div>
        <h3>Batteries Included</h3>
        <p>Out of the box support for all the language features you use today, with first class support for TypeScript and JSX.</p>
      </li>
    </ul>
  </section>

  <section class="try-rome">
    <h2>Try Rome today</h2>
    <p>Install Rome using your preferred package manager or get it as a VS Code extension.</p>
    <div><a href="#" class="button vscode-button">{% include svg/homepage/vscode.svg %} Get VS Code Extension</a></div>
    <div><a href="#" class="button install-button">Installation guide</a></div>
    <div class="window console-window">
    </div>
    <div class="window vscode-window">
    </div>
  </section>
</div>
