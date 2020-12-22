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

			(a6 ? b6 : c6)?.zoo();

			a7?.b.c();

			a8?.b.c;
      
			a9.b?.c.d();

			a10.b?.c.d;

			a10.b?.c?.d;

			a11.b?.c.d?.e();

			a12.b?.c.d?.e;

			a13.b.c.d.e?.();

			a14?.['b'].c?.['d'];
			`,
		);

		t.snapshot(response);
	},
);
