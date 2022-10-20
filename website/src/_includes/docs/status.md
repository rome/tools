
## Development Status

**Rome is being written in Rust, so it doesn't support all the intended features**.

Rome's vision is to cover the following features:

- Bundling
- Compiling
- Documentation Generation
- Formatting
- Linting
- Minification
- Testing
- Type Checking
- ... and more

### Supported Features

- [Formatter](/docs#formatter)
- [Linter](/docs#linter)

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


#### JavaScript Support

Rome supports ES2022.

Rome supports only the official syntax. The team starts developments of the new syntax when a proposal reaches
[Stage 3](https://github.com/tc39/proposals#stage-3).

#### TypeScript Support

Rome doesn't support decorators (the old proposal). Rome's parser will ignore them as if they were comments.
This means that programs with decorators are still valid, but they won't
benefit all the underlying features such as formatter, analyzers, etc.
