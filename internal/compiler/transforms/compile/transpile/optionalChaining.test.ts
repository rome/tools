import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {createDefaultProjectConfig} from "@internal/project";
import {dedent} from "@internal/string-utils";
import {formatAST} from "@internal/formatter";
import transform from "@internal/compiler/methods/transform";
import {TransformProjectDefinition} from "@internal/compiler";

async function transformCode(sourceText: string): Promise<string> {
	const ast = parseJS({
		sourceType: "script",
		path: "unknown",
		input: sourceText,
	});

	const project: TransformProjectDefinition = {
		config: createDefaultProjectConfig(),
		configHashes: [],
		directory: undefined,
	};

	const result = await transform({
		project,
		options: {},
		ast,
		sourceText: dedent(sourceText),
	});

	return formatAST(result.ast).code;
}

test(
	"optional chaining gets compiled correct",
	async (t) => {
		const response = await transformCode(
			`
			a1?.b();

			a2?.b?.c();

			a3.b?.();

			a4?.b?.();

			a5.b?.<T>();

			(a6 ? b6 : c6)?.zoo();`,
		);

		t.snapshot(response);
	},
);
