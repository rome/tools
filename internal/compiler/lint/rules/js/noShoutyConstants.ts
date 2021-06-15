import {ConstBinding, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noShoutyConstants",
	enter(path) {
		const {node, scope} = path;

		if (node.type === "JSReferenceIdentifier") {
			const binding = scope.getBinding(node.name);
			if (
				binding instanceof ConstBinding &&
				binding.value !== undefined &&
				binding.value.type === "JSStringLiteral" &&
				binding.value.value === node.name &&
				!binding.isExported &&
				(binding.scope.kind === "block" || binding.scope.kind === "program")
			) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(binding.value),
					},
					descriptions.LINT.JS_NO_SHOUTY_CONSTANTS(binding.node.loc),
				);
			}
		}

		return signals.retain;
	},
});
