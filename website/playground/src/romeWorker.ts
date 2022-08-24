import init, {
	DiagnosticPrinter,
	RomePath,
	Workspace,
} from "@rometools/wasm-web";
import {
	SourceType,
	LoadingState,
	PlaygroundState,
	IndentStyle,
	RomeOutput,
	QuoteStyle,
} from "./types";

let workspace: Workspace | null = null;

const PATH_SCRIPT = {
	path: "main.js",
	id: 0,
};

const PATHS_MODULE = [
	{ path: "main.mjs", id: 1 },
	{ path: "main.jsx", id: 2 },
	{ path: "main.ts", id: 3 },
	{ path: "main.tsx", id: 4 },
];

function getPathForType(
	sourceType: SourceType,
	isTypeScript: boolean,
	isJsx: boolean,
): RomePath {
	if (sourceType === SourceType.Script) {
		return PATH_SCRIPT;
	}

	return PATHS_MODULE[Number(isTypeScript) * 2 + Number(isJsx)];
}

type CurrentFile = {
	path: RomePath;
	version: number;
};

let currentFile: CurrentFile | null = null;

self.addEventListener("message", async (e) => {
	switch (e.data.type) {
		case "init": {
			try {
				await init();

				workspace = new Workspace();

				self.postMessage({ type: "init", loadingState: LoadingState.Success });
			} catch (err) {
				console.error(err);
				self.postMessage({ type: "init", loadingState: LoadingState.Error });
			}

			break;
		}

		case "format": {
			if (!workspace) {
				console.error("Workspace was not initialized");
				break;
			}

			const playgroundState: PlaygroundState = e.data.playgroundState;
			const {
				code,
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				isTypeScript,
				isJsx,
				sourceType,
				cursorPosition,
			} = playgroundState;

			workspace.update_settings({
				settings: {
					format: {
						enabled: true,
						format_with_errors: true,
						line_width: lineWidth,
						indent_style:
							indentStyle === IndentStyle.Tab ? "Tab" : { Space: indentWidth },
					},
					linter: {
						enabled: true,
					},
					languages: {
						javascript: {
							format: {
								quote_style: quoteStyle === QuoteStyle.Double ? "Double" : "Single",
							},
						},
					},
				},
			});

			const path = getPathForType(sourceType, isTypeScript, isJsx);
			if (currentFile?.path === path) {
				workspace.change_file({
					path,
					version: currentFile.version++,
					content: code,
				});
			} else {
				if (currentFile) {
					workspace.close_file({
						path: currentFile.path,
					});
				}

				currentFile = {
					path,
					version: 0,
				};

				workspace.open_file({
					path,
					version: currentFile.version++,
					content: code,
				});
			}

			const syntax_tree = workspace.get_syntax_tree({
				path,
			});
			const control_flow_graph = workspace.get_control_flow_graph({
				path,
				cursor: cursorPosition,
			});
			const formatter_ir = workspace.get_formatter_ir({
				path,
			});

			const diagnostics = workspace.pull_diagnostics({
				path,
				categories: ["Syntax", "Lint"],
			});

			const printer = new DiagnosticPrinter(path.path, code);
			for (const diag of diagnostics.diagnostics) {
				printer.print(diag);
			}

			const printed = workspace.format_file({
				path,
			});

			const romeOutput: RomeOutput = {
				ast: syntax_tree.ast,
				cst: syntax_tree.cst,
				errors: printer.finish(),
				formatted_code: printed.code,
				formatter_ir,
				control_flow_graph,
			};

			self.postMessage({
				type: "formatted",
				romeOutput,
			});
			break;
		}

		default:
			console.error(`Unknown message '${e.data.type}'.`);
	}
});
