---
layout: layouts/page.liquid
---

# Standalone CLI

## Use Rome without Node.js

Using Rome's standalone CLI binary can be a great choice if you aren't already using Node or npm (or any other package manager). Or in other words, Rome shouldn't be the only reason for you to have a `package.json`.

> Note: If you're already using npm or another package manager, then using the package manager is the [preferred way to install](/docs/getting-started#installation) Rome. You're already familiar with the tooling, and installing and updating are simpler.


### System Requirements

* Windows (including WSL), macOS, or Linux
* x86_64 or ARM64

### Picking the right binary

You have to pick the correct binary for your platform for Rome work. The following table should help you do so.

| CPU Architecture | Windows        | macOS                          | Linux         |
|------------------|----------------|--------------------------------|---------------|
| `arm64`          | `win32-arm64`  | `darwin-arm64` (M1 or newer)   | `linux-arm64` |
| `x64`            | `win32-x64`    | `darwin-x64`                   | `linux-x64`   |

> NOTE: Use the Linux variant for Windows Subsystem for Linux (WSL)

### Install Rome

To install Rome, grab the executable for your platform from the [latest CLI release](https://github.com/rome/tools/releases) on GitHub and give it execution permission.

```shell
# macOS arm (M1 or newer)
curl -L https://github.com/rome/tools/releases/download/cli%2Fv<version>/rome-darwin-arm64 -o rome
chmod +x rome

# Linux (x86_64)
curl -L https://github.com/rome/tools/releases/download/cli%2Fv<version>/rome-win32-x64.exe -o rome
chmod +x rome

# Windows (x86_64, Powershell)
Invoke-WebRequest -Uri "https://github.com/rome/tools/releases/download/cli%2Fv<version>/rome-win32-x64.exe" -OutFile "rome.exe"
```

> Note: Make sure to replace `<version>` with the Rome version you want to install.

Now you can use Rome by simply running `./rome`.

### Next Steps

Read more about how to use Rome in our getting [started section](/docs/configuration).
