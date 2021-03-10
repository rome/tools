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

export default createLocalCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`Initialise the project by emitting a configuration file and a .editorconfig file`,
	usage: markup`<cmd>npx rome init</cmd>`,
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
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
		if (hasProject.data === false) {
			client.reporter.warn(
				markup`Rome has been already configured inside this project.`,
			);
			return true;
		}

		const configTypeOptions: ConfigRadioOptions = {
			json: {
				label: markup`JSON (package.json)`,
			},
			// TODO: not supported yet
			// toml: {
			// 	label: markup`TOML`,
			// },
			// TODO: not supported yet
			// yaml: {
			// 	label: markup`YAML`,
			// },
		};

		const indentationTypeOptions: IndentStyleOptions = {
			space: {
				label: markup`Spaces`,
			},
			tab: {
				label: markup`Tabs`,
			},
		};

		const configType = await client.reporter.radio(
			markup`Please choose the extension of your Rome configuration file`,
			{
				options: configTypeOptions,
			},
		);

		const indentStyle = await client.reporter.radio(
			markup`Please choose the type of indentation (default Tabs)`,
			{
				options: indentationTypeOptions,
			},
		);

		const indentSize = await client.reporter.question(
			markup`Please choose the size of the indentation (default 2)`,
		);

		const identSizeAsNumber = Number(indentSize);
		if (isNaN(identSizeAsNumber) || identSizeAsNumber > 10) {
			client.reporter.warn(
				markup`You inserted a value that is not a number o is greater than 10. Rome will fallback to the default value (1).`,
			);
			client.reporter.info(
				markup`You can change this value later when this command is finished.`,
			);
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
