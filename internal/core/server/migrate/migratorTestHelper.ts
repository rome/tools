import "@internal/core";
import {AbsoluteFilePath} from "@internal/path";
import {Migration} from "@internal/core/server/migrate/Migration";
import {toml} from "@internal/codec-config";
import {TestHelper} from "rome";

interface TestSingleMigration {
	t: TestHelper;
	migration: Migration;
	testPath: AbsoluteFilePath;
}

export async function testSingleMigration(
	{migration, testPath, t}: TestSingleMigration,
): Promise<void> {
	const input = testPath.append("input.toml");

	if (!(await input.exists())) {
		throw new Error(
			`The directory ${testPath.getBasename()} must contain a input.toml file`,
		);
	}

	const consumer = toml.consumeValue({
		input: await input.readFileText(),
		path: input,
	});

	const snapshotFile = input.getParent().append(
		input.getExtensionlessBasename(),
	).join();

	const snapshot = t.customSnapshot(snapshotFile, {language: "toml"});

	snapshot.named(`${migration.name}: Input`, await input.readFileText());

	await migration.runMigration(consumer);

	snapshot.named(
		`${migration.name}: Output`,
		toml.stringifyFromConsumer({consumer, comments: new Map()}),
	);
}
