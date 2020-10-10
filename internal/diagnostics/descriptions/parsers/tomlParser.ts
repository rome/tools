import {createDiagnosticsCategory} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const tomlParser = createDiagnosticsCategory({
	NO_VALUE_FOR_KEY: (keyName: string) => ({
		message: markup`The key "${keyName}" doesn't have any value`,
	}),
	VALUE_NOT_RECOGNISED: (keyName: string) => ({
		message: markup`Unable to parse the value associated to key ${keyName}`,
	}),
	UNCLOSED_VALUE: (value: string) => ({
		message: markup`Text string "${value}" doesn't have a closing quote.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Add a quote (") at the end of ${value}`,
			},
		],
	}),
});
