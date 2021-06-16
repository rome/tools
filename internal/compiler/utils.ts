import {LintVisitor, TypedVisitor, Visitor} from "./types";
import {UnknownObject} from "@internal/typescript-helpers";
import {CompilerPath, signals} from ".";
import {Markup} from "@internal/markup";

export function createVisitor<State extends UnknownObject>(
	visitor: Visitor<State>,
) {
	return visitor;
}

export function transformVisitor<
	State extends UnknownObject,
	PathType extends CompilerPath
>(
	name: string,
	visitor: TypedVisitor<State, PathType>,
	isPathType: (path: CompilerPath) => path is PathType,
): Visitor<State> {
	return {
		name: `${name}/${visitor.name}`,
		enter: (path, state) => {
			if (visitor.enter !== undefined && isPathType(path)) {
				return visitor.enter(path, state);
			}
			return signals.retain;
		},
		exit: (path, state) => {
			if (visitor.exit !== undefined && isPathType(path)) {
				return visitor.exit(path, state);
			}
			return signals.retain;
		},
	};
}

export interface CreateLintVisitor<State extends UnknownObject> {
	recommended: boolean;
	visitor: LintVisitor<State>;
	meta?: LintVisitorMeta;
}

export interface LintVisitorMeta {
	/**
	 * Description of the rule
	 */
	description: Markup;
}

/**
 *
 * @param {LintVisitor} visitor A visitor that instructs the compiler how the code should change, if it changes
 * @param {boolean} [recommended=true] Marks a rule as recommended by Rome
 * @param {LintVisitorMeta} meta Metadata useful for documentation, examples, etc.
 */
export function createLintVisitor<State extends UnknownObject>(
	visitor: LintVisitor<State>,
	recommended: boolean = true,
	meta?: LintVisitorMeta,
): CreateLintVisitor<State> {
	return {
		visitor,
		recommended,
		meta,
	};
}
