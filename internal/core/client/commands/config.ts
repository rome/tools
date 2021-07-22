import {createLocalCommand} from "../commands";
import {commandCategories} from "@internal/core/common/commands";
import {markup} from "@internal/markup";
import {Reporter} from "@internal/cli-reporter";

export const remove = createLocalCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - remove "\\<key>"`,
	usage: "<key>",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req) {
		const {client, query} = req;
		const {reporter} = client;

		if (await confirmDeletion(reporter)) {
			await client.query(
				{
					commandName: "config remove",
					args: query.args,
				},
				"server",
			);
		}

		return true;
	},
});

export const pop = createLocalCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - remove "\\<values>" from "\\<key>"`,
	usage: "<key> <...values>",
	examples: [
		{
			command: "rome config pop aliases.paths path1 path2",
			description: markup`Remove multiple values from aliases.paths configuration.`,
		},
	],
	defineFlags() {
		return {};
	},
	async callback(req) {
		const {client, query} = req;
		const {reporter} = client;

		if (await confirmDeletion(reporter)) {
			await client.query(
				{
					commandName: "config pop",
					args: query.args,
				},
				"server",
			);
		}

		return true;
	},
});

function confirmDeletion(reporter: Reporter): Promise<boolean> {
	reporter.warn("Your configuration will be affected by this command.");
	return reporter.radioConfirm(
		"Are you sure you want to remove the specified configs?",
	);
}
