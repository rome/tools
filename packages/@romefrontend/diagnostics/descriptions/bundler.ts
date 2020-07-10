import {createDiagnosticsCategory} from "./index";

export const bundler = createDiagnosticsCategory({
	TOP_LEVEL_AWAIT_IN_LEGACY: {
		category: "bundler/topLevelAwait",
		message: "This module contains a top level await which isn't supported in wrapper mode",
	},
	DETECTED_CYCLE: (
		localName: string,
		target: string,
		culprit: string,
		path: Array<string>,
	) => {
		function formatPart(part: string, index?: number): string {
			const tagged = `<filelink target="${part}" />`;
			if (part === culprit) {
				return `<highlight i="0" legend>${tagged}</highlight>`;
			} else if (part === target) {
				return `<highlight i="1" legend>${tagged}</highlight>`;
			} else if (index === 0) {
				return `${tagged} <inverse> ENTRY </inverse>`;
			} else {
				return tagged;
			}
		}

		return {
			category: "bundler/moduleCycle",
			message: `The variable <emphasis>${localName}</emphasis> won't be initialized yet`,
			advice: [
				{
					type: "log",
					category: "info",
					text: "This is because the module it belongs to wont be executed yet. This is due to a circular dependency creating a module cycle.",
				},
				{
					type: "log",
					category: "info",
					text: `The likely cause is the file ${formatPart(culprit)} that was required by ${formatPart(
						target,
					)} which created a circular dependency:`,
				},
				{
					type: "list",
					reverse: true,
					ordered: true,
					list: path.map(formatPart),
				},
			],
		};
	},
});
