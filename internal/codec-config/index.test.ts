import {createFixtureTests} from "@internal/test-helpers";
import {consumeConfig, stringifyConfig} from "@internal/codec-config/index";

const promise = createFixtureTests(
	async (fixture, t) => {
		const snapshotFile = fixture.dir.append("input").join();

		for (const file of fixture.files.values()) {
			const path = file.absolute;
			const filename = file.relative;
			if (filename.join().includes("input.toml")) {
				const inputContent = file.contentAsText();

				const val = consumeConfig({
					input: inputContent,
					path: filename,
				});

				const ext = path.getExtensions();const dotlessExtension = path.getDotlessExtensions();

				const snapshot = t.customSnapshot(
					snapshotFile,
					{language: dotlessExtension},
				);

				snapshot.named(`parse ${ext}`, val.consumer.asUnknown());

				snapshot.named(`stringify ${ext}`, stringifyConfig(val));
			}
		}
	},
	undefined,
);

// @ts-expect-error Doesn't support top-level await
await promise;
