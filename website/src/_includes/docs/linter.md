## Linter

You can use the linter via our [VS Code extension] or by downloading our CLI directly from our [release page].

> WARNING: The CLI and VS Code extension are packaged with separate binaries, which means that if you don't
> use our default options, you will have to **pass them to both the extension AND the CLI**.
>
> This is a temporary choice to allow people to play with our formatter. This will change in the near future.


> WARNING: this command is experimental for the time being won't do anything

### Use the formatter via CLI

You can start by running the CLI with the `--help` flag:

```shell
rome check --help
```

Which will show you the options available at the moment:

```shell
USAGE:
    rome check <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

```


[VS Code extension]: https://marketplace.visualstudio.com/items?itemName=rome.rome
[release page]: https://github.com/rome/tools/releases
