import {
	CompilerPath,
	Scope,
	createLintVisitor,
	signals,
} from "@internal/compiler";
import {getBindingIdentifiers} from "@internal/js-ast-utils";
import {Dict} from "@internal/typescript-helpers";
import {
	ArgumentsBinding,
	Binding,
	TypeBinding,
} from "@internal/compiler/scope/bindings";
import {descriptions} from "@internal/diagnostics";

type State = {
	usedBindings: Dict<boolean>;
	scope: undefined | Scope;
};

// Common variables that are sometimes impossible to avoid
const ignoreVariables = ["React"];

function getEnclosingFunctionBodyScope(
	refPath: CompilerPath,
	binding: Binding,
): Scope | undefined {
	const block = refPath.findAncestry((path) => {
		const node = path.node;
		return (
			((node.type === "JSVariableDeclarator" &&
			(node.init?.type === "JSFunctionExpression" ||
			node.init?.type === "JSArrowFunctionExpression")) ||
			node.type === "JSFunctionDeclaration" ||
			node.type === "JSClassDeclaration") &&
			node.id === binding.node
		);
	});

	return block?.scope;
}

function isSelfReference(refPath: CompilerPath, binding: Binding) {
	let scope: Scope | undefined = refPath.scope;
	const blockScope = getEnclosingFunctionBodyScope(refPath, binding);
	if (blockScope) {
		while (scope) {
			if (scope === blockScope) {
				return true;
			}
			scope = scope.parentScope;
		}
	}
	return false;
}

function maybeFunctionNameBinding(binding: Binding) {
	const kind = binding.kind;
	return (
		kind === "let" ||
		kind === "const" ||
		kind === "var" ||
		kind === "function" ||
		kind === "class"
	);
}

export default createLintVisitor<State>({
	name: "js/noUnusedVariables",
	enter(path, state) {
		const {node, scope} = path;

		if (scope.node === node) {
			let hasBindings = false;
			const usedBindings: Dict<boolean> = {};

			// Get all the non-exported bindings in this file and mark them as unused
			for (const [name, binding] of scope.getOwnBindings()) {
				if (binding instanceof ArgumentsBinding) {
					continue;
				}

				if (binding.isExported) {
					continue;
				}

				// Type parameter of a mapped type is always used as it's a property key
				if (binding instanceof TypeBinding && binding.typeKind === "mapped type") {
					continue;
				}

				hasBindings = true;
				usedBindings[name] = false;
			}

			if (!hasBindings) {
				return signals.retain;
			}

			// For functions, special case parameters
			if (
				node.type === "JSFunctionDeclaration" ||
				node.type === "JSFunctionExpression" ||
				node.type === "JSObjectMethod" ||
				node.type === "JSClassMethod" ||
				node.type === "TSDeclareMethod" ||
				node.type === "TSDeclareFunction" ||
				node.type === "JSArrowFunctionExpression"
			) {
				let ignoreLast = true;
				let includeRest = false;

				// If there's no rest then only consider the last parameter to be unused
				const {rest} = node.head;
				if (rest !== undefined) {
					ignoreLast = false;
				}

				if (node.type === "TSDeclareFunction" || node.type === "TSDeclareMethod") {
					// This is an interface and has no body
					ignoreLast = false;
					includeRest = true;
				} else {
					// For functions that have a single throw statement in the body, consider all their arguments
					// to be used as this is typically an interface definition
					const {body} = node;
					if (
						body.type === "JSBlockStatement" &&
						body.body.length === 1 &&
						body.body[0].type === "JSThrowStatement"
					) {
						ignoreLast = false;
						includeRest = true;
					}
				}

				// Mark parameters as used
				let params = [...node.head.params];
				if (ignoreLast) {
					params.pop();
				}
				if (includeRest && rest !== undefined) {
					params.push(rest);
				}
				for (const {name} of getBindingIdentifiers(params)) {
					usedBindings[name] = true;
				}
			}

			if (
				node.type === "JSCatchClause" &&
				node.param &&
				node.param.type === "JSBindingIdentifier"
			) {
				// Mark error param as used as they are required
				usedBindings[node.param.name] = true;
			}

			// For a named function expression, don't consider the id to be unused
			if (node.type === "JSFunctionExpression" && node.id !== undefined) {
				usedBindings[node.id.name] = true;
			}

			state.reset({
				usedBindings,
				scope,
			});
		}

		if (
			node.type === "JSXReferenceIdentifier" ||
			node.type === "JSReferenceIdentifier"
		) {
			const binding = path.scope.getBindingFromPath(path);
			if (binding !== undefined) {
				let used = true;

				if (
					node.type === "JSReferenceIdentifier" &&
					maybeFunctionNameBinding(binding)
				) {
					used = !isSelfReference(path, binding);
				}

				state.set(
					(state): State => {
						return {
							...state,
							usedBindings: {
								...state.usedBindings,
								[node.name]: state.usedBindings[node.name] === false
									? used
									: state.usedBindings[node.name],
							},
						};
					},
					{
						find: (state) => {
							return binding.scope === state.scope;
						},
					},
				);
			}
		}

		return signals.retain;
	},

	exit(path, state) {
		if (state.owns()) {
			const {usedBindings} = state.get();
			for (const name in usedBindings) {
				const used = usedBindings[name];
				const binding = path.scope.getBinding(name);

				if (
					used === false &&
					binding !== undefined &&
					!ignoreVariables.includes(name)
				) {
					path.context.addNodeDiagnostic(
						binding.node,
						descriptions.LINT.JS_NO_UNUSED_VARIABLES(binding.kind, name),
					);
				}
			}
		}

		return signals.retain;
	},
});
