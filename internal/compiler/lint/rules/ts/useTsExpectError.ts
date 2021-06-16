import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

const RE_LINE_IGNORE_SUPPRESSION = /^(\s*)@ts-ignore/;
const RE_BLOCK_IGNORE_SUPPRESSION = /^(\s*)(\**)(\s*)@ts-ignore/;

export default createLintVisitor({
	name: "ts/useTsExpectError",
	enter(path) {
		const {node} = path;

		if (node.type !== "JSRoot") {
			return signals.retain;
		}

		/**
		 * Used to provide a meaningful diagnosic message instead of pointing at the fist line of a file
		 */
		const firstCommentToFixIndex = node.comments.findIndex((x) =>
			(x.type === "CommentLine" && x.value.match(RE_LINE_IGNORE_SUPPRESSION)) ||
			x.value.split("\n").some((l) => l.match(RE_BLOCK_IGNORE_SUPPRESSION))
		);

		if (firstCommentToFixIndex === -1) {
			return signals.retain;
		}

		return path.addFixableDiagnostic(
			{
				target: node.comments[firstCommentToFixIndex],
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
