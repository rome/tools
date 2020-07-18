import {ConstBinding, Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "js/shoutyConstants",
	enter(path: Path): TransformExitResult {
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
				return path.context.addFixableDiagnostic(
					{
						old: node,
						fixed: binding.value,
					},
					descriptions.LINT.JS_SHOUTY_CONSTANTS(binding.node.loc),
				);
			}
		}

		return node;
	},
};
