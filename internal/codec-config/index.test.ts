import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {consumeConfig, stringifyConfig} from "@internal/codec-config/index";
import {decodeUTF8} from "@internal/binary";

const promise = createFixtureTests(
	async (fixture, t) => {
		const snapshotFile = fixture.dir.append("input").join();

		for (const file of fixture.files.values()) {
			const path = file.absolute;
			const filename = file.relative;
			const inputContent = removeCarriageReturn(decodeUTF8(file.content));

			const val = consumeConfig({
				input: inputContent,
				path: filename,
			});

			const ext = path.getExtensions();

			t.namedSnapshot(
				`parse ${ext}`,
				val.consumer.asUnknown(),
				undefined,
				{filename: snapshotFile},
			);
			t.namedSnapshot(
				`stringify ${ext}`,
				stringifyConfig(val),
				undefined,
				{filename: snapshotFile},
			);
		}
	},
	undefined,
	true,
);

// @ts-ignore Doesn't support top-level await
await promise;
