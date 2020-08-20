import {test} from "rome";
import {tryStaticEvaluationPath} from "./tryStaticEvaluationPath";
import {template} from "./template";
import {CompilerContext, Path, Scope} from "@internal/compiler";

import {MOCK_PROGRAM} from "@internal/ast";

test(
	"evaluates a node under a path",
	(t) => {
		const path = new Path(
			template.expression`2 + 2 * 10;`,
			new CompilerContext({ast: MOCK_PROGRAM}),
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
