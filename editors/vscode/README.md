# Rome VS Code Extension

[Rome](https://rome.tools/) unifies your development stack by combining the functionality of separate tools. It uses a single configuration file, has fantastic performance, and works with any stack. This extension brings Rome to your editor so that you can:

- Format files *on save* or when issuing the *Format Document* command
- See lints while you type and apply code fixes
- Perform refactors

## Installation

You can install the code extension by heading to the extension's [Visual Studio Code Market Place page](https://marketplace.visualstudio.com/items?itemName=rome.rome) or from within VS Code by either:

- Open the *extensions* tab (_View_ → _Extensions)_ and search for Rome.
- Open the _Quick Open Overlay_ (Ctrl/Cmd+P or _Go -> Go to File_), enter `ext install rome.rome`, and hit enter.

## Getting Started

### Default Formatter

Configure Rome as the default formatter for supported files to ensure that VS Code uses Rome over other formatters that you may have installed. You can do so by opening a JavaScript or TypeScript and then:

- Open the Command Palette (Ctrl/Cmd+Shift+P or View → Command Palette)
- Select _Format Document With…_
- Select _Configure Default Formatter…_
- Select Rome

You can also enable Rome for specific languages only:

- [Open the `settings.json`](https://code.visualstudio.com/docs/getstarted/settings#_settingsjson): open the _Command Palette_(Ctr+Shift+P) and select _Preferences: Open User Settings (JSON)_
- And set the `editor.defaultFormatter` to `rome.rome` for the desired language

```json
{
	"editor.defaultFormatter": "<other formatter>",
	"[javascript]": {
		"editor.defaultFormatter": "rome.rome"
	}
}
```

This configuration sets Rome as the default formatter for JavaScript files. All other files will be formatted using `<other formatter>`

## Configuration Resolution

The extension automatically loads the `rome.json` file from the workspace’s root directory.

## Rome Resolution

The extension tries to use Rome from your project's local dependencies (`node_modules/rome`). We recommend adding Rome as a project dependency to ensure that NPM scripts and the extension use the same Rome version.

You can also explicitly specify the `rome` binary the extension should use by configuring the `rome.lspBin` setting in your editor options.

If the project has no dependency on Rome and no explicit path is configured, the extension uses the Rome version included in its bundle.

## Usage

### Format document

To format an entire document, open the _Command Palette_ (Ctrl/Cmd+Shift+P) and select _Format Document_.

To format a text range, select the text you want to format, open the _Command Palette_ (Ctrl/Cmd+Shift+P), and select _Format Selection_.

### Format on save

Rome respects VS Code's _Format on Save_ setting. To enable format on save, open the settings (_File_ -> _Preferences_ -> _Settings_), search for `editor.formatOnSave`, and enable the option.

## Extension Settings

### `rome.lspBin`

The `rome.lspBin` option overrides the Rome binary used by the extension. The workspace folder is used as the base path if the path is relative.

### `rome.rename`

Enables Rome to handle renames in the workspace (experimental).

## Known limitations

### Configuration per sub-directory

Rome doesn’t yet support loading the `rome.json` file from a directory other than the workspace root ([issue 3576](https://github.com/rome/tools/issues/3576), [issue 3289](https://github.com/rome/tools/issues/3289)). That means it is currently impossible to enable Rome only for a specific sub-folder or to use different configurations for different folders.

### Configuration resolution for multi-root workspaces

Rome isn't yet able to pick up the `rome.json` configuration in [multi-root workspaces](https://code.visualstudio.com/docs/editor/multi-root-workspaces) if the configuration isn't in the first root folder ([issue 3538](https://github.com/rome/tools/issues/3538)). You can work around this limitation by making the folder with the configuration the first root folder of the workspace (drag it to the top).

### Disable Rome for workspaces without a `rome.json` configuration

Rome's VS Code extension is active for every workspace regardless if the workspace contains a `rome.json` configuration ([issue 3506](https://github.com/rome/tools/issues/3506)). That may be surprising to you if you use other extensions like ESLint where the extension is disabled if the configuration is missing. This behavior is intentional because it's Rome's philosophy that the configuration should be optional.

You can work around this limitation by [disabling Rome per workspace](https://code.visualstudio.com/docs/editor/extension-marketplace#_disable-an-extension):

- Open the _Extensions_ panel
- Search for Rome
- Right-click on _Rome_ and select _Disable (Workspace)_
