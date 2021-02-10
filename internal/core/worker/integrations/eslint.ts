import IntegrationLoader from "@internal/core/common/IntegrationLoader";
import {FileReference} from "@internal/core/common/types/files";
import {Diagnostics} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {Position} from "@internal/parser-core";
import {ob1Coerce1To0} from "@internal/ob1";
import {WorkerProject} from "../types";
import Worker from "../Worker";

const eslintLoader = new IntegrationLoader({
	name: "eslint",
	range: "^7.0.0",
	normalize: (consumer) => {
		const Factory = consumer.get("ESLint").asFunction();
		const eslint = Reflect.construct(Factory, [{fix: true}]);
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
			timingNs: bigint;
			diagnostics: Diagnostics;
		}
> {
	if (!project.config.integrations.eslint.enabled) {
		return undefined;
	}

	const content = await worker.readFileText(ref);
	const start = process.hrtime.bigint();

	const diagnostics: Diagnostics = [];

	console.log("before", project.configPath.join());
	const loader = await eslintLoader.load(project.configPath);
	console.log("after");

	const results = await eslintLoader.wrap(async () => {
		const lintText = loader.module.get("lintText").asWrappedFunction();
		return await lintText(content, {filePath: ref.real.join()}).asPromise();
	});

	const result = results.getIndex(0);

	for (const message of result.get("messages").asIterable()) {
		const start: Position = {
			line: message.get("line").asOneIndexedNumber(),
			column: ob1Coerce1To0(message.get("column").asOneIndexedNumber()),
		};
		let end: Position = start;

		if (message.has("endLine") && message.has("endColumn")) {
			end = {
				line: message.get("endLine").asOneIndexedNumber(),
				column: ob1Coerce1To0(message.get("endColumn").asOneIndexedNumber()),
			};
		}

		diagnostics.push({
			description: {
				message: markup`${message.get("message").asString()}`,
				advice: [],
				category: "eslint",
				categoryValue: message.get("ruleId").asStringOrVoid(),
			},
			location: {
				path: ref.uid,
				start,
				end,
			},
		});
		console.log(message.asAny());
	}

	const end = process.hrtime.bigint();

	return {
		diagnostics,
		timingNs: end - start,
	};
}
