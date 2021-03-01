import IntegrationLoader from "@internal/core/common/IntegrationLoader";
import {FileReference} from "@internal/core/common/types/files";
import {DIAGNOSTIC_CATEGORIES, Diagnostics} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {Duration, DurationMeasurer} from "@internal/numbers";
import {Position} from "@internal/parser-core";
import {WorkerProject} from "../types";
import Worker from "../Worker";

const eslintLoader = new IntegrationLoader({
	name: "eslint",
	range: "^7.0.0",
	normalize: (consumer, cwd) => {
		const Factory = consumer.get("ESLint").asFunction();
		const eslint = Reflect.construct(
			Factory,
			[
				{
					cwd: cwd.join(),
					globInputPaths: false,
					fix: true,
					rulePaths: [
						"/Users/sebmck/Scratch/TypeScript/scripts/eslint/built/rules",
					],
				},
			],
		);
		return consumer.setValue(eslint);
	},
});

// Run and convert ESLint diagnostics if project config integrations.eslint is enabled
export async function maybeRunESLint(
	{ref, project, worker}: {
		worker: Worker;
		ref: FileReference;
		project: WorkerProject;
	},
): Promise<
	| undefined
	| {
			timing: Duration;
			diagnostics: Diagnostics;
		}
> {
	if (!project.config.integrations.eslint.enabled) {
		return undefined;
	}

	const timer = new DurationMeasurer();

	const diagnostics: Diagnostics = [];

	const loader = await eslintLoader.load(project.configPath, project.directory);

	const ignored = await eslintLoader.wrap(async () => {
		const isPathIgnored = loader.module.get("isPathIgnored").asWrappedFunction();
		return (await isPathIgnored(ref.real.join()).asPromise()).asBoolean();
	});

	if (!ignored) {
		const content = await worker.readFileText(ref);

		const results = await eslintLoader.wrap(async () => {
			const lintText = loader.module.get("lintText").asWrappedFunction();
			return await lintText(content, {filePath: ref.real.join()}).asPromise();
		});

		const result = results.getIndex(0);

		for (const message of result.get("messages").asIterable()) {
			const start: Position = {
				line: message.get("line").asOneIndexedNumber(),
				column: message.get("column").asOneIndexedNumber().toZeroIndexed(),
			};
			let end: Position = start;

			if (message.has("endLine") && message.has("endColumn")) {
				end = {
					line: message.get("endLine").asOneIndexedNumber(),
					column: message.get("endColumn").asOneIndexedNumber().toZeroIndexed(),
				};
			}

			diagnostics.push({
				description: {
					message: markup`${message.get("message").asString()}`,
					advice: [],
					category: DIAGNOSTIC_CATEGORIES.eslint,
					categoryValue: message.get("ruleId").asStringOrVoid(),
				},
				location: {
					path: ref.uid,
					start,
					end,
				},
			});
		}
	}

	return {
		diagnostics,
		timing: timer.since(),
	};
}
