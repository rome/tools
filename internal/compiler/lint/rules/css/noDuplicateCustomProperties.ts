import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {CSSCustomProperty} from "@internal/ast";

export default createLintVisitor({
	name: "css/noDuplicateCustomProperties",
	enter(path) {
		const {node} = path;

		if (
			node.type === "CSSBlock" &&
			node.startingTokenValue === "{" &&
			node.value
		) {
			const cssCustomProperties = node.value.reduce(
				(properties, child) => {
					if (child.type === "CSSDeclaration") {
						if (typeof child.name !== "string") {
							properties.push(child.name);
						}
					}
					return properties;
				},
				[] as CSSCustomProperty[],
			);

			const tempArray = [...cssCustomProperties].sort();
			let duplicate: CSSCustomProperty | undefined = undefined;
			for (let i = 0; i < tempArray.length; i++) {
				if (
					i + 1 < tempArray.length &&
					tempArray[i + 1].value === tempArray[i].value
				) {
					duplicate = tempArray[i];
					break;
				}
			}

			if (duplicate) {
				path.context.addNodeDiagnostic(
					duplicate,
					descriptions.LINT.CSS_NO_DUPLICATE_CUSTOM_PROPERTIES,
				);
			}
		}

		return signals.retain;
	},
});
