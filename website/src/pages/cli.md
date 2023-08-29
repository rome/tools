---
title: CLI
emoji: ⌨️
category: reference
description: Available commands and arguments in the Rome CLI.
---



# Command summary

  * [`rome`↴](#rome)
  * [`rome version`↴](#rome-version)
  * [`rome rage`↴](#rome-rage)
  * [`rome start`↴](#rome-start)
  * [`rome stop`↴](#rome-stop)
  * [`rome check`↴](#rome-check)
  * [`rome lint`↴](#rome-lint)
  * [`rome format`↴](#rome-format)
  * [`rome ci`↴](#rome-ci)
  * [`rome init`↴](#rome-init)
  * [`rome lsp-proxy`↴](#rome-lsp-proxy)
  * [`rome migrate`↴](#rome-migrate)

# rome

Rome official CLI. Use it to check the health of your project or run it to check single files.

**Usage**: **`rome`** _`COMMAND ...`_

**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information
- **`-V`**, **`--version`** &mdash; 
  Prints version information



**Available commands:**
- **`version`** &mdash; 
  Shows the Rome version information and quit
- **`rage`** &mdash; 
  Prints information for debugging
- **`start`** &mdash; 
  Start the Rome daemon server process
- **`stop`** &mdash; 
  Stop the Rome daemon server process
- **`check`** &mdash; 
  Run various checks on a set of files.
- **`lint`** &mdash; 
  Run various checks on a set of files.
- **`format`** &mdash; 
  Run the formatter on a set of files.
- **`ci`** &mdash; 
  Command to use in CI environments. Run various checks of a set of files.
- **`init`** &mdash; 
  Bootstraps a new rome project. Creates a configuration file with some defaults.
- **`lsp-proxy`** &mdash; 
  Acts as a server for the Language Server Protocol over stdin/stdout
- **`migrate`** &mdash; 
  It updates the configuration when there are breaking changes


# rome version

Shows the Rome version information and quit

**Usage**: **`rome`** **`version`** 

**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome rage

Prints information for debugging

**Usage**: **`rome`** **`rage`** 

**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome start

Start the Rome daemon server process

**Usage**: **`rome`** **`start`** 

**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome stop

Stop the Rome daemon server process

**Usage**: **`rome`** **`stop`** 

**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome check

Run various checks on a set of files.

**Usage**: **`rome`** **`check`** \[**`--apply`**\] \[**`--apply-unsafe`**\] \[_`PATH`_\]...

**The configuration that is contained inside the file `rome.json`**
- **`    --vcs-client-kind`**=_`<git>`_ &mdash; 
  The kind of client.
- **`    --vcs-enabled`**=_`<true|false>`_ &mdash; 
  Whether Rome should integrate itself with the VCS client
- **`    --vcs-use-ignore-file`**=_`<true|false>`_ &mdash; 
  Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files specified in the ignore file.
- **`    --vcs-root`**=_`PATH`_ &mdash; 
  The folder where Rome should check for VCS files. By default, Rome will use the same folder where `rome.json` was found.

  If Rome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Rome won't use the VCS integration, and a diagnostic will be emitted
- **`    --files-max-size`**=_`NUMBER`_ &mdash; 
  The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
- **`    --files-ignore-unknown`**=_`<true|false>`_ &mdash; 
  Tells Rome to not emit diagnostics when handling files that doesn't know
- **`    --indent-style`**=_`<tab|space>`_ &mdash; 
  The indent style.

  ```shell rome format --indent-style=tab ```
- **`    --indent-size`**=_`NUMBER`_ &mdash; 
  The size of the indentation, 2 by default
- **`    --line-width`**=_`NUMBER`_ &mdash; 
  What's the max width of a line. Defaults to 80.
- **`    --quote-style`**=_`<double|single>`_ &mdash; 
  The style for quotes. Defaults to double.
- **`    --jsx-quote-style`**=_`<double|single>`_ &mdash; 
  The style for JSX quotes. Defaults to double.
- **`    --quote-properties`**=_`<preserve|as-needed>`_ &mdash; 
  When properties in objects are quoted. Defaults to asNeeded.
- **`    --trailing-comma`**=_`<all|es5|none>`_ &mdash; 
  Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
- **`    --semicolons`**=_`<always|as-needed>`_ &mdash; 
  Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.



**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available positional items:**
- _`PATH`_ &mdash; 
  Single file, single path or list of paths



**Available options:**
- **`    --apply`** &mdash; 
  Apply safe fixes, formatting
- **`    --apply-unsafe`** &mdash; 
  Apply safe fixes and unsafe fixes, formatting and import sorting
- **`    --formatter-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the formatter check.
- **`    --linter-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the linter check.
- **`    --organize-imports-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the organize imports.
- **`    --stdin-file-path`**=_`PATH`_ &mdash; 
  A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome check --stdin-file-path=file.js"
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome lint

Run various checks on a set of files.

**Usage**: **`rome`** **`lint`** \[**`--apply`**\] \[**`--apply-unsafe`**\] \[_`PATH`_\]...

**The configuration that is contained inside the file `rome.json`**
- **`    --vcs-client-kind`**=_`<git>`_ &mdash; 
  The kind of client.
- **`    --vcs-enabled`**=_`<true|false>`_ &mdash; 
  Whether Rome should integrate itself with the VCS client
- **`    --vcs-use-ignore-file`**=_`<true|false>`_ &mdash; 
  Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files specified in the ignore file.
- **`    --vcs-root`**=_`PATH`_ &mdash; 
  The folder where Rome should check for VCS files. By default, Rome will use the same folder where `rome.json` was found.

  If Rome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Rome won't use the VCS integration, and a diagnostic will be emitted
- **`    --files-max-size`**=_`NUMBER`_ &mdash; 
  The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
- **`    --files-ignore-unknown`**=_`<true|false>`_ &mdash; 
  Tells Rome to not emit diagnostics when handling files that doesn't know
- **`    --indent-style`**=_`<tab|space>`_ &mdash; 
  The indent style.

  ```shell rome format --indent-style=tab ```
- **`    --indent-size`**=_`NUMBER`_ &mdash; 
  The size of the indentation, 2 by default
- **`    --line-width`**=_`NUMBER`_ &mdash; 
  What's the max width of a line. Defaults to 80.
- **`    --quote-style`**=_`<double|single>`_ &mdash; 
  The style for quotes. Defaults to double.
- **`    --jsx-quote-style`**=_`<double|single>`_ &mdash; 
  The style for JSX quotes. Defaults to double.
- **`    --quote-properties`**=_`<preserve|as-needed>`_ &mdash; 
  When properties in objects are quoted. Defaults to asNeeded.
- **`    --trailing-comma`**=_`<all|es5|none>`_ &mdash; 
  Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
- **`    --semicolons`**=_`<always|as-needed>`_ &mdash; 
  Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.



**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available positional items:**
- _`PATH`_ &mdash; 
  Single file, single path or list of paths



**Available options:**
- **`    --apply`** &mdash; 
  Apply safe fixes, formatting
- **`    --apply-unsafe`** &mdash; 
  Apply safe fixes and unsafe fixes, formatting and import sorting
- **`    --stdin-file-path`**=_`PATH`_ &mdash; 
  A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome lint --stdin-file-path=file.js"
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome format

Run the formatter on a set of files.

**Usage**: **`rome`** **`format`** \[**`--write`**\] \[_`PATH`_\]...

**Options applied to the formatter**
- **`    --indent-style`**=_`<tab|space>`_ &mdash; 
  The indent style.

  ```shell rome format --indent-style=tab ```
- **`    --indent-size`**=_`NUMBER`_ &mdash; 
  The size of the indentation, 2 by default
- **`    --line-width`**=_`NUMBER`_ &mdash; 
  What's the max width of a line. Defaults to 80.



**Set of properties to integrate Rome with a VCS software.**
- **`    --vcs-client-kind`**=_`<git>`_ &mdash; 
  The kind of client.
- **`    --vcs-enabled`**=_`<true|false>`_ &mdash; 
  Whether Rome should integrate itself with the VCS client
- **`    --vcs-use-ignore-file`**=_`<true|false>`_ &mdash; 
  Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files specified in the ignore file.
- **`    --vcs-root`**=_`PATH`_ &mdash; 
  The folder where Rome should check for VCS files. By default, Rome will use the same folder where `rome.json` was found.

  If Rome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Rome won't use the VCS integration, and a diagnostic will be emitted



**The configuration of the filesystem**
- **`    --files-max-size`**=_`NUMBER`_ &mdash; 
  The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
- **`    --files-ignore-unknown`**=_`<true|false>`_ &mdash; 
  Tells Rome to not emit diagnostics when handling files that doesn't know



**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available positional items:**
- _`PATH`_ &mdash; 
  Single file, single path or list of paths.



**Available options:**
- **`    --quote-style`**=_`<double|single>`_ &mdash; 
  The style for quotes. Defaults to double.
- **`    --jsx-quote-style`**=_`<double|single>`_ &mdash; 
  The style for JSX quotes. Defaults to double.
- **`    --quote-properties`**=_`<preserve|as-needed>`_ &mdash; 
  When properties in objects are quoted. Defaults to asNeeded.
- **`    --trailing-comma`**=_`<all|es5|none>`_ &mdash; 
  Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
- **`    --semicolons`**=_`<always|as-needed>`_ &mdash; 
  Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
- **`    --stdin-file-path`**=_`PATH`_ &mdash; 
  A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path=file.js".
- **`    --write`** &mdash; 
  Writes formatted files to file system.
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome ci

Command to use in CI environments. Run various checks of a set of files.

**Usage**: **`rome`** **`ci`** \[**`--formatter-enabled`**=_`<true|false>`_\] \[**`--linter-enabled`**=_`<true|false>`_\] \[**`--organize-imports-enabled`**=_`<true|false>`_\] \[_`PATH`_\]...

**The configuration that is contained inside the file `rome.json`**
- **`    --vcs-client-kind`**=_`<git>`_ &mdash; 
  The kind of client.
- **`    --vcs-enabled`**=_`<true|false>`_ &mdash; 
  Whether Rome should integrate itself with the VCS client
- **`    --vcs-use-ignore-file`**=_`<true|false>`_ &mdash; 
  Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files specified in the ignore file.
- **`    --vcs-root`**=_`PATH`_ &mdash; 
  The folder where Rome should check for VCS files. By default, Rome will use the same folder where `rome.json` was found.

  If Rome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Rome won't use the VCS integration, and a diagnostic will be emitted
- **`    --files-max-size`**=_`NUMBER`_ &mdash; 
  The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
- **`    --files-ignore-unknown`**=_`<true|false>`_ &mdash; 
  Tells Rome to not emit diagnostics when handling files that doesn't know
- **`    --indent-style`**=_`<tab|space>`_ &mdash; 
  The indent style.

  ```shell rome format --indent-style=tab ```
- **`    --indent-size`**=_`NUMBER`_ &mdash; 
  The size of the indentation, 2 by default
- **`    --line-width`**=_`NUMBER`_ &mdash; 
  What's the max width of a line. Defaults to 80.
- **`    --quote-style`**=_`<double|single>`_ &mdash; 
  The style for quotes. Defaults to double.
- **`    --jsx-quote-style`**=_`<double|single>`_ &mdash; 
  The style for JSX quotes. Defaults to double.
- **`    --quote-properties`**=_`<preserve|as-needed>`_ &mdash; 
  When properties in objects are quoted. Defaults to asNeeded.
- **`    --trailing-comma`**=_`<all|es5|none>`_ &mdash; 
  Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
- **`    --semicolons`**=_`<always|as-needed>`_ &mdash; 
  Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.



**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available positional items:**
- _`PATH`_ &mdash; 
  Single file, single path or list of paths



**Available options:**
- **`    --formatter-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the formatter check.
- **`    --linter-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the linter check.
- **`    --organize-imports-enabled`**=_`<true|false>`_ &mdash; 
  Allow to enable or disable the organize imports.
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome init

Bootstraps a new rome project. Creates a configuration file with some defaults.

**Usage**: **`rome`** **`init`** 

**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome lsp-proxy

Acts as a server for the Language Server Protocol over stdin/stdout

**Usage**: **`rome`** **`lsp-proxy`** \[**`--colors`**=_`<off|force>`_\] \[**`--use-server`**\] \[**`--verbose`**\] \[**`--config-path`**=_`PATH`_\] \[**`--max-diagnostics`**=_`NUMBER`_\] \[**`--skip-errors`**\] \[**`--no-errors-on-unmatched`**\]

**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available options:**
- **`-h`**, **`--help`** &mdash; 
  Prints help information


# rome migrate

It updates the configuration when there are breaking changes

**Usage**: **`rome`** **`migrate`** \[**`--write`**\]

**Global options applied to all commands**
- **`    --colors`**=_`<off|force>`_ &mdash; 
  Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
- **`    --use-server`** &mdash; 
  Connect to a running instance of the Rome daemon server.
- **`    --verbose`** &mdash; 
  Print additional verbose advices on diagnostics
- **`    --config-path`**=_`PATH`_ &mdash; 
  Set the filesystem path to the directory of the rome.json configuration file
- **`    --max-diagnostics`**=_`NUMBER`_ &mdash; 
  Cap the amount of diagnostics displayed (default: 20)
- **`    --skip-errors`** &mdash; 
  Skip over files containing syntax errors instead of emitting an error diagnostic.
- **`    --no-errors-on-unmatched`** &mdash; 
  Silence errors that would be emitted in case no files were processed during the execution of the command.
- **`    --json`** &mdash; 
  Reports information using the JSON format



**Available options:**
- **`    --write`** &mdash; 
  Writes the new configuration file to disk
- **`-h`**, **`--help`** &mdash; 
  Prints help information


