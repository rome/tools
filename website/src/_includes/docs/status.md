
## Development Status

**Rome is being rewritten in Rust, so it doesn't support all the intended features**.

Rome is currently only supported as a [formatter](#formatter) for JavaScript and TypeScript.** 

We plan on covering the following areas:

- Bundling
- Compiling
- Documentation Generation
- Formatting
- Linting
- Minification
- Testing
- Type Checking
- ... and more

### Language Support

| Language                           | Parsing                                                 | Formatting                                              | Linting                                                 |
|------------------------------------|---------------------------------------------------------|---------------------------------------------------------|---------------------------------------------------------|
| [JavaScript](/#javascript-support) | <span aria-label="Supported" role="img">✅</span>        | <span aria-label="Supported" role="img">✅</span>        | <span aria-label="Supported" role="img">✅</span>        |
| [TypeScript](/#typescript-support) | <span aria-label="Supported" role="img">✅</span>        | <span aria-label="Supported" role="img">✅</span>        | <span aria-label="Supported" role="img">✅</span>        |
| JSX                                | <span aria-label="Supported" role="img">✅</span>        | <span aria-label="Supported" role="img">✅️</span>       | <span aria-label="Supported" role="img">✅</span>        |
| JSON                               | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> |
| HTML                               | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> |
| CSS                                | <span aria-label="Not in progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> |
| Markdown                           | <span aria-label="Not in progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> | <span aria-label="Not in Progress" role="img">🚫</span> |


#### JavaScript support

Rome supports only the official syntax. The team starts developments of the new syntax when a proposal reaches
[Stage 3](https://github.com/tc39/proposals#stage-3). 

Rome supports ES2022 version of the language.

#### TypeScript support

Rome doesn't support decorators (the old proposal). Rome's parser will ignore them as if they were comments.
This means that programs with decorators are still valid, but they won't 
benefit all the underlying features such as formatter, analyzers, etc.