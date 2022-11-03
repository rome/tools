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
        <div class="soon-indicator">Dec 2022</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/html.svg %}</div>
        <div class="language">HTML</div>
        <div class="soon-indicator">June 2023</div>
      </li>
      <li class="soon">
        <div class="icon">{% include svg/homepage/css.svg %}</div>
        <div class="language">CSS</div>
        <div class="soon-indicator">Nov 2023</div>
      </li>
    </ul>
  </section>

  <hr>

  <section class="supercharge">
    <h2>Supercharge your workflow.</h2>
    <p class="heading-tagline">Packed full of useful features like theming, smart tokens, CSS prop, as prop, utils, and a fully-typed API.</p>
    <ul class="component-list">
      <li class="active" data-class="component-window-formatter">Formatter</li>
      <li data-class="component-window-linter">Linter</li>
      <li data-class="component-window-compiler" class="soon">
        <div class="text">Compiler</div>
        <div class="soon-indicator">March 2023</div>
      </li>
      <li data-class="component-window-bundler" class="soon">
        <div class="text">Bundler</div>
        <div class="soon-indicator">March 2023</div>
      </li>
      <li data-class="component-window-testing" class="soon">
        <div class="text">Testing</div>
        <div class="soon-indicator">June 2023</div>
      </li>
    </ul>
    <div class="component-window component-window-formatter">
      <div class="code">
        <h4>Code</h4>
      </div>
      <div class="output">
        <h4>Output</h4>
      </div>
      <div class="performance">
        <h4>Performance</h4>
        <p class="progress-header"><span class="tool-name">Rome</span> finished in <span class="time-good">0.1s</span></p>
        <div class="progress"><div class="progress-bar progress-bar-good" style="width: 20px;"></div></div>
        <p class="progress-header"><span class="tool-name">Prettier</span> finished in <span class="time-bad">5.2s</span></p>
        <div class="progress"><div class="progress-bar progress-bar-bad" style="width: 270px;"></div></div>
        <p class="multiplier">52×</p>
        <p>Rome is 52× faster than Prettier when formatting 1,000 lines of code.</p>
      </div>
    </div>
    <p class="founder-clout">Created by the founder of <a target="_blank" class="babel" href="https://babeljs.io/">{% include svg/homepage/babel.svg %}</a> and <a target="_blank" href="https://yarnpkg.com/" class="yarn">{% include svg/homepage/yarn.svg %}</a></p>
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
