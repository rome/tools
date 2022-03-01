## Diagnostics

Diagnostics are what Rome calls errors. They are emitted absolutely everywhere Rome finds a problem. This includes CLI argument parsing, JSON normalization, module resolution, lint errors, and more.

We've built Rome to be fantastic at displaying diagnostics. When we show you an error we want to give you all the information you need to understand why it appeared, how you can fix it, and how to avoid it in the future.

Rome stands out in the following ways:

**Rich UI:** Our diagnostic format allows us to show you rich information with formatting. This includes line wrapping in the terminal, syntax highlighting, lists, hyperlinks, and more.

**Fixes:** We provide [fixes](#applying-fixes) for many lint errors, which can be applied automatically. If there are multiple ways to fix something then we [suggest](#suggested-fixes) multiple fixes that you can choose.

**Reviewing:** We offer an [interactive CLI command](#reviewing) to make this process even easier. It allows you to go through each diagnostic and perform actions on them such as inserting a suppression comment or applying a specific fix.

**Editor:** You can use an [editor integration](#editor-integration) to bring the power of Rome into your editor. This includes lint errors as you type, automatic formatting when saved, and code actions to select specific fixes.

**Safety:** We save a copy of all files before we modify them and cache them. This cache can be managed with the [`rome recover` command](#rome-recover). You will always be able to revert when Rome modifies your code even without a commit history.

#### Anatomy

Diagnostics consist of six main parts:

- The header contains the **filename**, **line**, and **column**. They refer to the position that we believe is the main cause of a problem.
- The **category** of the error. Each diagnostic belong to a different category that should help to identify
	the typology of the error.
- Followed is the **message** which contains a single-line summary of what we believe is wrong.
- The **code frame** contains a snippet of the file referred in the header. This allows you to see what it's referring to without having to jump into your editor and look it up.
- **Advice** is freeform and appears at the end of a diagnostic. It can include additional messages, lists, other code frames, and more. It gives you more details about why you're seeing the diagnostic, and how you might fix it.

{% include docs/diagnostic-anatomy.md %}
