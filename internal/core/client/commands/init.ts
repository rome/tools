import {createLocalCommand} from "@internal/core/client/commands";
import {markup} from "@internal/markup";
import ClientRequest from "@internal/core/client/ClientRequest";
import {SelectOption, SelectOptions} from "@internal/cli-reporter";
import {commandCategories} from "@internal/core/common/commands";

interface ConfigRadioOptions extends SelectOptions {
	json?: SelectOption;
	toml?: SelectOption;
}

interface IndentStyleOptions extends SelectOptions {
	space: SelectOption;
	tab: SelectOption;
}

interface Flags {
	y: boolean;
}

export default createLocalCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`Initialise the project by emitting a configuration file and a .editorconfig file`,
	usage: markup`<cmd>npx rome init</cmd>`,
	examples: [],
	defineFlags(c) {
		return {
			y: c.get(
				"y",
				{
					description: markup`Creates a configuration file without interacting with the user, using Rome's defaults.`,
				},
			).asBoolean(false),
		};
	},
	async callback(req: ClientRequest, flags: Flags) {
		const {client} = req;

		const hasProject = await client.query(
			{
				commandName: "init",
				commandFlags: {
					checkProject: true,
				},
			},
			"server",
		);

		if (hasProject.type !== "SUCCESS") {
			return true;
		}
		if (hasProject.data === true) {
			client.reporter.warn(
				markup`Rome has been already configured inside this project.`,
			);
			return true;
		}
		let configType = "json";
		let indentStyle = "tab";
		let identSizeAsNumber = 2;

		if (flags.y === false) {
			const configTypeOptions: ConfigRadioOptions = {
				json: {
					label: markup`JSON (package.json)`,
				},
				toml: {
					label: markup`TOML`,
				},
			};

			const indentationTypeOptions: IndentStyleOptions = {
				space: {
					label: markup`Spaces`,
				},
				tab: {
					label: markup`Tabs`,
				},
			};

			configType = await client.reporter.radio(
				markup`Please choose the extension of your Rome configuration file`,
				{
					options: configTypeOptions,
				},
			);

			indentStyle = await client.reporter.radio(
				markup`Please choose the type of indentation (default Tabs)`,
				{
					options: indentationTypeOptions,
				},
			);

			const indentSize = await client.reporter.question(
				markup`Please choose the size of the indentation (default 2)`,
			);

			identSizeAsNumber = Number(indentSize);
			if (
				isNaN(identSizeAsNumber) ||
				identSizeAsNumber === 0 ||
				identSizeAsNumber > 10
			) {
				client.reporter.warn(
					markup`You inserted a value that is not a number or is greater than 10. Rome will fallback to the default value (2).`,
				);
				client.reporter.info(
					markup`You can change this value later when this command is finished.`,
				);
				identSizeAsNumber = 2;
			}
		}

		await client.query(
			{
				commandName: "init",
				commandFlags: {
					configType,
					indentStyle,
					indentSize: identSizeAsNumber,
				},
			},
			"server",
		);

		return true;
	},
});
