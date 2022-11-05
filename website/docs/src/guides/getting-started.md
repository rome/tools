---
title: Getting Started
---

# Getting Started

## System Requirements

* Windows (including WSL), macOS, or Linux
* x86_64 or ARM64
* Node.js v14.18 or newer (not applicable if you use the standalone executable)

## Installation

The fastest way to download Rome is to use `npm` or your preferred package manager. The CLI is also available as a [standalone executable](/standalone-executable) if you want to use Rome without installing Node.js.

Run the following commands in a directory with a `package.json` file to install Rome.

> **Note**: It is also possible to install Rome globally rather than locally. However, this is not recommended.


#### npm

```bash
npm install --save-dev rome
```

You can now use `npx rome` to run Rome.

#### pnpm

```bash
pnpm install --save-dev rome
```

You can now use `pnpm exec rome` to run Rome.


#### yarn

```bash
yarn add rome --save-dev
```

You can now use `yarn run rome` to run Rome.

## Configuration

We recommend creating a `rome.json` configuration file for each project. It eliminates  the need to repeat the CLI options every time you run a command and ensures that Rome applies the same configuration in your editor. If you're happy with Rome's defaults, you don't have to create the configuration.

To create the configuration, run the `init` command in the root folder of your project:

```bash
npx rome init

# or
pnpm exec rome init

# or
yarn run rome init
```

After running the `init` command, you'll have a `rome.json` configuration file in your directory.

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true
    }
  }
}
```


The `linter.enabled: true` enables the linter and `rules.recommended: true` enables the [recommended rules](/lint/rules/).

Formatting is enabled because the configuration doesn't explicitly [disables](/configuration/#formatterenabled) formatting with `formatter.enabled: false`.

## Usage

You can lint any file or directory running:

```bash
npx rome check <files>

# or
pnpm exec rome check <files>

# or
yarn run rome check <files>
```

or format your files and directories with:


```bash
npx rome format <files> --write

# or
pnpm exec rome format <files> --write

# or
yarn run rome format <files> --write
```

<!-- Make sure to update the redirect in `static/_redirects` when changing the editors title -->
## Editor Setup

We recommend installing our editor integration to get the most out of Rome. The Rome editor integration allows you to:

* Format files on save or when issuing the Format command.
* Lint files and apply code fixes

### VS Code

Install our official [Rome VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome) from the Visual Studio Marketplace.

To make Rome the default formatter open a supported file (JavaScript or TypeScript) and:

* open the *Command Palette* (View or Command/Ctrl + Shift + P)
* select  *Format Document With...*
* select *Configure Default Formatter*
* select *Rome*.

### Other Editors

We would love to support more editors, but we don't have the capacity to implement and maintain multiple editor integrations at the moment. You can help us prioritize by [voting](https://github.com/rome/tools/discussions/3544) for your favourite editor. If you're interested in building an integration for Rome, please [reach out](https://github.com/rome/tools/issues/2390), and we would be more than happy to support you.

If you are looking for editor support in a JetBrains IDE like WebStorm, then visit the relevant [issue](https://youtrack.jetbrains.com/issue/WEB-46895/Support-for-Romejs) to upvote the ticket.


## CI Setup

If you're using Node.js, the recommended way to run Rome in CI is to use [your favourite package manager](/guides/getting-started#installation). This ensures that your CI pipeline uses the same version of Rome as you do inside of the editor or when running local CLI commands.


If you are working on a project that isn't using Node.js, then the best way to integrate Rome into your CI is to use the [setup-rome](https://github.com/rome/setup-rome#usage) GitHub Action or install the [standalone executable](/standalone-executable).


## Next Steps

Success! You’re now ready to use Rome. 🥳

* Learn more about how to use and configure the [formatter](/formatter)
* Learn more about how to use and configure the [linter](/linter)
* Get familiar with the [CLI options](/cli)
* Get familiar with the [configuration options](/configuration)
* Join our [community on Discord](https://discord.gg/rome)
