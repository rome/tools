---
layout: layouts/base-page.liquid
description: End-to-end developer tools
social-image: social-logo.png
body-class: homepage-body
---

<div class="homepage">
  <section>
    <h1 class="sr-only">Rome is a unified formatter and linter</h1>
    <div aria-hidden="true" class="h1">
      Rome is a unified
      <ul>
        <li class="formatter">formatter</li>
        <li class="linter" hidden>linter</li>
      </ul>
    </div>
    <a target="_blank" href="/docs" class="button">Documentation</a>
    <aside hidden class="latest-post" aria-labelledby="latest-post">
      <h3>
      <a href="{{ post.url }}">{{ post.data.title }}</a>
      </h3>
      <div class="author">
        {{ post.date | dateFormat }}
      </div>
    </aside>
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

  <hr class="half">

  <section class="supercharge">
    <h2>Supercharge your workflow</h2>
    <p class="heading-tagline">Experience the seamless integration from a vertical developer tool. Full of useful features like formatting, linting, and more coming soon.</p>
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
      <!--<li data-class="component-window-testing" class="soon">
        <div class="text">Documentation</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li data-class="component-window-testing" class="soon">
        <div class="text">Minification</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li data-class="component-window-testing" class="soon">
        <div class="text">Repo Management</div>
        <div class="soon-indicator">Soon</div>
      </li>
      <li data-class="component-window-testing" class="soon">
        <div class="text">Task Runner</div>
        <div class="soon-indicator">Soon</div>
      </li>-->
    </ul>
    <div class="component-window component-window-formatter">
      <div class="code">
        <h4>Code</h4>
        {% highlight js %}
function HelloWorld({greeting = "hello", greeted = '"World"', silent = false, onMouseOver,}) {

  if(!greeting){return null};

     // TODO: Don't use random in render
  let num = Math.floor (Math.random() * 1E+7).toString().replace(/\.\d+/ig, "")

  return <div className='HelloWorld' title={`You are visitor number ${ num }`} onMouseOver={onMouseOver}>

    <strong>{ greeting.slice( 0, 1 ).toUpperCase() + greeting.slice(1).toLowerCase() }</strong>
    {greeting.endsWith(",") ? " " : <span style={{color: '\grey'}}>", "</span> }
    <em>
	{ greeted }
    </em>
    { (silent)
      ? "."
      : "!"}

    </div>;

}
{% endhighlight %}
      </div>
      <div class="output">
        <h4>Output</h4>
        {% highlight js %}
function HelloWorld({
  greeting = "hello",
  greeted = '"World"',
  silent = false,
  onMouseOver,
}) {
  if (!greeting) {
    return null;
  }

  // TODO: Don't use random in render
  let num = Math.floor(Math.random() * 1e7)
    .toString()
    .replace(/\.\d+/gi, "");

  return (
    <div
      className="HelloWorld"
      title={`You are visitor number ${num}`}
      onMouseOver={onMouseOver}
    >
      <strong>
        {greeting.slice(0, 1).toUpperCase() + greeting.slice(1).toLowerCase()}
      </strong>
      {greeting.endsWith(",") ? (
        " "
      ) : (
        <span style={{ color: "grey" }}>", "</span>
      )}
      <em>{greeted}</em>
      {silent ? "." : "!"}
    </div>
  );
}
{% endhighlight %}
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
    <p class="founder-clout">Created by the founder of <a target="_blank" class="babel" href="https://babeljs.io/">{% include svg/homepage/babel.svg %}<span class="sr-only">Babel</span></a> and <a target="_blank" href="https://yarnpkg.com/" class="yarn">{% include svg/homepage/yarn.svg %}<span class="sr-only">Yarn</span></a></p>
  </section>

  <hr class="full">

  <section>
    <h2 class="sr-only">Features</h2>
    <ul class="features">
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/chevron.svg %}</div>
        <h3>Fast</h3>
        <p>Built with Rust and an innovative architecture inspired by <a href="https://rust-analyzer.github.io/">rust-analyzer</a>. We are able to do even more while being faster.</p>
      </li>
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/layers.svg %}</div>
        <h3>Simple</h3>
        <p>Zero configuration needed to get started. <a href="/docs/#configuration">Extensive options available</a> for when you need them.</p>
      </li>
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/maximize.svg %}</div>
        <h3>Scalable</h3>
        <p>Designed to handle codebases of any size. Focus on growing <strong>product</strong> instead of your tools.</p>
      </li>
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/lightning.svg %}</div>
        <h3>Optimized</h3>
        <p>With tight internal integration we are able to reuse previous work and any improvement to one tool improves them all.</p>
      </li>
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/action.svg %}</div>
        <h3>Actionable &amp; Informative</h3>
        <p>Avoid obscure error messages, when we tell you something is wrong, we tell you exactly where the problem is and how to fix it.</p>
      </li>
      <li>
        <div class="icon foreground-svg">{% include svg/homepage/box.svg %}</div>
        <h3>Batteries Included</h3>
        <p>Out of the box support for all the language features you use today. First class support for TypeScript and JSX.</p>
      </li>
    </ul>
  </section>

  <section class="try-rome">
    <h2>Try Rome today</h2>
    <p>Install Rome using your preferred package manager or get it as a VS Code extension.</p>
    <div><a target="_blank" href="https://marketplace.visualstudio.com/items?itemName=rome.rome" class="button vscode-button">{% include svg/homepage/vscode.svg %} Get VS Code Extension</a></div>
    <div><a target="_blank" href="/docs/#getting-started" class="button install-button">Installation guide</a></div>
    <div class="window console-window">
      <div class="command">
        <div class="line"><span class="shell-symbol">$</span> npm install <span class="rome">rome</span></div>
        <div class="line"><span>Added 1 package</span></div>
      </div>
      <div class="command">
        <div class="line"><span class="shell-symbol">$</span> <span class="rome">rome</span> lint</div>
        <div class="line"><span>Checked 780 files in 12ms.</span></div>
      </div>
      <div class="command">
        <div class="line"><span class="shell-symbol">$</span> <span class="rome">rome</span> format</div>
        <div class="line"><span>Checked 650 files in 42ms.</span></div>
      </div>
    </div>
    <div class="window vscode-window">
    </div>
  </section>
</div>
