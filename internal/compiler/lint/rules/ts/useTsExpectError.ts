import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

const RE_LINE_IGNORE_SUPPRESSION = /^(\s*)@ts-ignore/;
const RE_BLOCK_IGNORE_SUPPRESSION = /^(\s*)@ts-ignore/;

export default createVisitor({
	name: "ts/useTsExpectError",
	enter(path) {
		const {node} = path;

		if (node.type !== "JSRoot") {
			return signals.retain;
		}

		if (
			!node.comments.some((x) =>
				(x.type === "CommentLine" && x.value.match(RE_LINE_IGNORE_SUPPRESSION)) ||
				x.value.split("\n").some((l) => l.match(RE_BLOCK_IGNORE_SUPPRESSION))
			)
		) {
			return signals.retain;
		}

		return path.addFixableDiagnostic(
			{
				fixed: signals.replace({
					...node,
					comments: node.comments.map((x) => {
						if (x.type === "CommentLine") {
							return {
								...x,
								value: x.value.replace(
									RE_LINE_IGNORE_SUPPRESSION,
									"$1@ts-expect-error",
								),
							};
						}

						return {
							...x,
							value: x.value.split("\n").map((l) =>
								l.replace(RE_BLOCK_IGNORE_SUPPRESSION, "$1$2$3@ts-expect-error")
							).join("\n"),
						};
					}),
				}),
			},
			descriptions.LINT.TS_USE_TS_EXPECT_ERROR,
		);
	},
});
