import {createDiagnosticsCategory} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const tomlParser = createDiagnosticsCategory({
	INVALID_KEY_CHAR: (char: string) => ({
		message: markup`The character <emphasis>${char}</emphasis> isn't a valid character in a key`,
	}),
	NO_VALUE_FOR_KEY: (keyName: string) => ({
		message: markup`The key <emphasis>${keyName}</emphasis> doesn't have any value`,
	}),
	VALUE_NOT_RECOGNISED: (keyName: string) => ({
		message: markup`Unable to parse the value associated to key <emphasis>${keyName}</emphasis>`,
	}),
	UNCLOSED_VALUE: (value: string) => ({
		message: markup`Text string <emphasis>${value}<emphasis> doesn't have a closing quote.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Add a quote (") at the end`,
			},
		],
	}),
});
