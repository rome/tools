## Internals

### Process Model

Rome consists of three process types:

 - **Client**. eg. CLI. Responsible for building a request and dispatching it to a server. If there is a running [daemon](#daemon) then it's used as the server, otherwise, the CLI creates a server inside of its process, which only the lifetime of the request.
 - **Server**. Where the magic happens. Watches the file system and maintains an in-memory copy for fast lookups. Spawns workers and distributes and coordinates work between them. Responds to requests from the CLI.
 - **Worker**. Where distributed work occurs. These are operations that are specific to a certain file. Produces artifacts that can be easily cached. Computed values contain enough information to aggregate with other file operations to provide cross-file analysis.

### Immutable AST

All parsed ASTs are treated as immutable. This allows reference equality to be used to quickly determine if a node has been modified and can be used as keys in a `WeakMap` for effective memory caching.

### Recoverable Parsers

All parsers are recoverable, always deriving an AST despite syntax errors. This allows operations that validate code to be chained together. This surfaces as many problems as possible at once and reduces the waterfall of fixing errors only to be faced with more.

### Portable Cache

Internally we use unique IDs to refer to files rather than absolute paths. This allows cache artifacts to be transferred between different machines. Included are hashes of the project config, mtime, and other file information to allow for easy validation.

This can be utilized in a CI environment, or even in a network cache for a group of developers. We will add the relevant hooks in the future to allow this to be used more effectively, including a dedicated network cache server.

### Terminal Rendering

We have our own HTML-ish markup format that is used to declare formatting in strings. We use this format everywhere rather than traditional embedded ANSI color codes. This allows us to remain output agnostic. We currently support rendering to ANSI, HTML, and plain text.

All the "terminal screenshots" you see in the docs were generated from regular Rome CLI commands with the `--output-format html --output-columns 80` flags set.

Tags are not color-specific. ie. rather than `<green>` we have `<success>`. This makes our markup even more semantic and versatile.

When rendering we perform layout calculation according to a provided column width, in most cases reported to us by the shell. This layout calculation includes line wrapping, padding, horizontal rules, and text alignment.

We avoid the common pitfalls of in-band ANSI formatting by doing the formatting as the final step when all the text has been split into non-overlapping ranges for ANSI code insertion.

### Type Safety

While we are in JavaScript land, we embrace TypeScript by using as many strong types as possible. We have sparing usages of wide types like `object` and `any` casts. With no dependencies we are able to extend this coverage and confidence everywhere. We never consume arbitrary data like JSON without first passing it through some validation and normalization process.

### Self Hosted

Rome is bundled, compiled, linted, and tested by itself. Once Rome was built and had the capabilities necessary to build itself, we removed the other tools and instead used a build of Rome.

Read more about self hosting at [Self-hosting (compilers) - Wikipedia](https://en.wikipedia.org/wiki/Self-hosting_(compilers))
