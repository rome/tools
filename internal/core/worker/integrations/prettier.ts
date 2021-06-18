import IntegrationLoader from "@internal/core/common/IntegrationLoader";
import Worker from "../Worker";
import {FileReference, WorkerProject} from "@internal/core";
import {Duration, DurationMeasurer} from "@internal/numbers";
import {Diagnostic} from "@internal/diagnostics";

const prettierLoader = new IntegrationLoader({
	name: "prettier",
	range: "^2.0.0",
	normalize: (consumer) => {
		return consumer
	},
});

export interface MaybeRunPrettier {
	formatted: string;
	timing: Duration;
	diagnostics: Diagnostic[]
}

export async function maybeRunPrettier(
	{ref, project, worker}: {
		worker: Worker;
		ref: FileReference;
		project: WorkerProject;
	},
): Promise<
	| undefined
	|  MaybeRunPrettier
	> {
	const options = project.config.integrations.prettier;
	if (!options.enabled) {
		return undefined;
	}

	const loader = await prettierLoader.load(project.configPath, project.directory);

	const timer = new DurationMeasurer();

	const content = await worker.readFileText(ref);

	// NOTE: check if we need diagnostics
	const diagnostics: Diagnostic[] = [];

	const formatted = await prettierLoader.wrap(async () => {
		const format = loader.module.get("format").asWrappedFunction();
		return await format(content, {
			...options,
			filepath: ref.real.join()
		}).asPromise();

	});

	return {
		formatted: formatted.asString(),
		timing: timer.since(),
		diagnostics
	}
}
