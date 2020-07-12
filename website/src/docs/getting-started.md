---
title: Getting Started
layout: layouts/page.njk
---

# Getting Started

## 1. Installation

Rome is available for installation via [Yarn](https://yarnpkg.com/):

```bash
yarn add rome
```

or [npm](https://www.npmjs.com/):

```bash
npm install rome
```

## 2. Project setup

To get started with Rome navigate to your project folder and run:

```bash
rome init
```

This will create a `rome.rjson` files that contains your project configuration.

Refer to [Project Configuration](/docs/project-config) for more details.

> Note: [RJSON](/docs/rjson) is a superset of JSON that supports more-concise syntax and features such as comments.

## 3. Celebrate! 🎉

### Run some commands

[`rome check`](/docs/cli/commands/check)

```bash
rome check
```

### Install an editor extension

Get the most out of Rome by integrating it into your editor. You will get lint errors as you type, and saving will automatically format your files.

Right now Rome only has an official extension for VSCode which you can install [here](TODO) or by searching for "Rome" in the VSCode marketplace.

See [Editor Integration](/docs/editor-integration) for more details.

