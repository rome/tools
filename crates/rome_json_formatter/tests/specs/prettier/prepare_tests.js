const {extractPrettierTests} = require("../../../../rome_formatter_test/src/prettier/prepare_tests");

async function main() {
	await extractPrettierTests("json", {
		parser: "json",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
