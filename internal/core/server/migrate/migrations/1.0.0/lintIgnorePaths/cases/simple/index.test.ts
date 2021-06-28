import {test, testOptions} from "rome";
import {testSingleMigration} from "@internal/core/server/migrate/migratorTestHelper";
import migration from "../../index";
import {createAbsoluteFilePath} from "@internal/path";

const dirname = testOptions.dirname ?? "";

test("Ignore paths migration", async (t) => {

	const testPath = createAbsoluteFilePath(dirname);
	await testSingleMigration({t, migration, name: "simple case", testPath});

})
