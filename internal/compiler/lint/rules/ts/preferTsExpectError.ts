import {createVisitor, signals} from "@internal/compiler";

const RE = {
	line: /^(\s*)@ts-ignore/,
	block: /^(\s*)(\**)(\s*)@ts-ignore/,
};

export default createVisitor({
	name: "ts/preferTsExpectError",
	enter(path) {
		const {node} = path;

		if (node.type !== "JSRoot") {
			return signals.retain;
		}

		if (
			!node.comments.some((x) =>
				(x.type === "CommentLine" && x.value.match(RE.line)) ||
				x.value.split("\n").some((l) => l.match(RE.block))
			)
		) {
			return signals.retain;
		}

		return signals.replace({
			...node,
			comments: node.comments.map((x) => {
				if (x.type === "CommentLine") {
					return {
						...x,
						value: x.value.replace(RE.line, "$1@ts-expect-error"),
					};
				}

				return {
					...x,
					value: x.value.split("\n").map((l) =>
						l.replace(RE.block, "$1$2$3@ts-expect-error")
					).join("\n"),
				};
			}),
		});
	},
});
