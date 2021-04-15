import {createFixtureTests} from "@internal/test-helpers";
import {parseCommit} from "./index";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const inputContent = await inputFile.readAsText();

	const commit = parseCommit(inputContent);

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.customSnapshot(outputFile).snapshot(commit);
});

// @ts-ignore Doesn't support top-level await
await promise;
