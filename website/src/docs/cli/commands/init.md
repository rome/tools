---
title: rome init
layout: layouts/page.liquid
---

# `rome init`

This command assists in the creation of a new Rome project. Actions that are performed:

 - `rome.rjson` is created that serves as your [project configuration](/docs/project-config).
 - `.editorconfig` is created that correctly sets editor indentation for those that support [EditorConfig](https://editorconfig.org/).
 - `rome check --apply` is ran which will automatically format and autofix your files.
 - Global variables are extracted from previous errors and automatically added to your project config.

## Uncomitted changes

Since this command can be destructive and may have unintended consequences, we check if you have any uncomitted changes. It's important to make sure you have everything committed in case you aren't happy with the effects of running this command. ie. you run into a bug, you don't like Rome, or want to try it some other time.

You can bypass this restriction by adding the `--allow-dirty` flag.

## Output

<pre class="language-text"><code class="language-text"><span style="color: CornflowerBlue">$</span> rome init

<strong> Welcome to Rome! Let&apos;s get you started... </strong>

 <strong>Summary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;"><strong>1</strong></span><span style="color: DodgerBlue;"> </span><span style="color: DodgerBlue;">file</span><span style="color: DodgerBlue;"> saved</span>
  <strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No problems found!</span>

 <strong>Files created</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <span style="opacity: 0.8;">- </span><strong><span style="text-decoration-style: dotted;">rome.rjson</span></strong>: Your project configuration. Documentation:
    <a href="https://romefrontend.dev/docs/project-config/">https://romefrontend.dev/docs/project-config/</a>
  <span style="opacity: 0.8;">- </span><strong><span style="text-decoration-style: dotted;">.editorconfig</span></strong>: Sets editor formatting and indentation options.
    Documentation: <a href="https://editorconfig.org/">https://editorconfig.org/</a>

 <strong>What next?</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <span style="opacity: 0.8;">1. </span><strong>Setup an editor extension</strong>
     Get live errors as you type and format when you save. Learn more:
     <a href="https://romefrontend.dev/docs/editor-integration/">https://romefrontend.dev/docs/editor-integration/</a>

  <span style="opacity: 0.8;">2. </span><strong>Try a command</strong>
     <i>rome check</i> is used to validate your code, verify formatting, and
     check for lint errors. Run <i>rome --help</i> for a full list of commands
     and flags.

  <span style="opacity: 0.8;">3. </span><strong>Read documentation</strong>
     Our website serves as a comprehensive source of guides and
     documentation <a href="https://romefrontend.dev/">https://romefrontend.dev/</a>

  <span style="opacity: 0.8;">4. </span><strong>Get involved in the community</strong>
     Ask questions, get support, or contribute by participating on
     GitHub (<a href="https://github.com/romefrontend/rome">https://github.com/romefrontend/rome</a>) or our community
     Discord (<a href="https://discord.gg/rome">https://discord.gg/rome</a>)

</pre></code>
