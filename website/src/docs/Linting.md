# Linting

Rome comes with a builtin linter that analyzes your JavaScript code to flag programming errors, bugs, stylistic errors and more.

```javascript
// Lint your entire project.
rome lint

// Lint one or more specific files.
rome lint index.js hello.js
```

## Interpreting the result

If `rome` did not detect any problems, you'll get a result like this:

```javascript
ℹ 1 file linted
✔ No known problems!
```

However, if there is something not ok, `rome` will provide you with diagnostics warnings in the following format:

```javascript
<the affected file>:<line>:<column> <the linter rule that was violated>
-----------------------------------

x description of what is not ok

> 1 | the line of code that is problematic
    |     ^^^ <the part of the line that is not ok>

 ℹ helpful message(s) of what you can do to fix the problem
 ```

Let's look at example of what the linter might throw at you:

```javascript
foobar.js:1 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ import and export can only appear in a module

  > 1 │ import { join } from "path";
      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

  ℹ Change the extension to .mjs to turn this file into a module

  ℹ Add "type": "module" to your package.json
```

Rome will point you to the mistake in your JavaScript source code, and will potentially list you with options on how you can fix these.  You'll be presented with 1 message for each problem.

## Example

The following code has 2 unused variables: `join` in line 1 and `unused` in line 3.

```javascript
const {join} = require('path');

const unused = 'I am not used :)';

console.log('hello world!');
```

You'll notice that Rome provides you with the following detailed warnings:

```javascript
index.js:1:8 lint/unusedVariables ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unused variable join

  > 1 │ const { join } = require("path");
      │         ^^^^
    2 │
    3 │ const unused = "I am not used :)";

 index.js:3:6 lint/unusedVariables ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unused variable unused

    1 │ const { join } = require("path");
    2 │
  > 3 │ const unused = "I am not used :)";
      │       ^^^^^^
    4 │
    5 │ console.log("hello world!");

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ℹ 1 file linted
✖ Found 2 problems
```