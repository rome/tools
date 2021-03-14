import {test} from "rome";
import {tryStaticEvaluationPath} from "./tryStaticEvaluationPath";
import {template} from "./template";
import {CompilerContext, CompilerPath, Scope} from "@internal/compiler";

import {MOCK_JS_ROOT} from "@internal/ast";

test(
	"evaluates a node under a path",
	(t) => {
		const path = new CompilerPath(
			template.expression`2 + 2 * 10;`,
			new CompilerContext({ast: MOCK_JS_ROOT}),
			{
				scope: new Scope({
					kind: "program",
					node: undefined,
					parentScope: undefined,
					rootScope: undefined,
				}),
			},
		);
		t.false(tryStaticEvaluationPath(path).bailed);
		t.is(tryStaticEvaluationPath(path).value, 22);
	},
);
