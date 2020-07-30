import {AnyNode} from "@romefrontend/ast";
import {
	Path,
	Scope,
	createHook,
	createVisitor,
	signals,
} from "@romefrontend/compiler";
import {getBindingIdentifiers} from "@romefrontend/js-ast-utils";
import {Dict} from "@romefrontend/typescript-helpers";
import {
	ArgumentsBinding,
	TypeBinding,
} from "@romefrontend/compiler/scope/bindings";
import {descriptions} from "@romefrontend/diagnostics";
import {HookCallReturn} from "@romefrontend/compiler/api/createHook";

type State = {
	usedBindings: Dict<boolean>;
	scope: undefined | Scope;
};

const initialState: State = {
	usedBindings: {},
	scope: undefined,
};

// Common variables that are sometimes impossible to avoid
const ignoreVariables = ["React"];

const provider = createHook<State, undefined, AnyNode>({
	name: "js/noUnusedVariablesProvider",
	initialState,
	call(path: Path, state: State): HookCallReturn<AnyNode, State> {
		const {node} = path;
		if (
			node.type !== "JSReferenceIdentifier" &&
			node.type !== "JSXReferenceIdentifier"
		) {
			throw new Error("Expected only JSIdentifier to be dispatched");
		}

		const binding = path.scope.getBindingFromPath(path);

		// Check if this binding belongs to the scope we're tracking
		if (binding === undefined || binding.scope !== state.scope) {
			return {
				bubble: true,
				value: node,
				state,
			};
		}

		// Mark this binding as used
		return {
			value: node,
			state: {
				...state,
				// @ts-ignore https://github.com/microsoft/TypeScript/issues/39278
				usedBindings: {
					...state.usedBindings,
					[node.name]: true,
				},
			},
		};
	},

	exit(path, state) {
		for (const name in state.usedBindings) {
			const used = state.usedBindings[name];
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

		return signals.retain;
	},
});

export default createVisitor({
	name: "js/unusedVariables",
	enter(path) {
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

			return path.provideHook(
				provider,
				{
					usedBindings,
					scope,
				},
			);
		}

		if (
			node.type === "JSXReferenceIdentifier" ||
			node.type === "JSReferenceIdentifier"
		) {
			return signals.replace(path.callHook(provider, undefined, node));
		}

		return signals.retain;
	},
});
