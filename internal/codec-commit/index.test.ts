import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {parseCommit} from "./index";
import {decodeUTF8} from "@internal/binary";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const inputContent = removeCarriageReturn(decodeUTF8(inputFile.content));

	const commit = parseCommit(inputContent);

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.snapshot(commit, undefined, {filename: outputFile});
});

// @ts-ignore Doesn't support top-level await
await promise;
