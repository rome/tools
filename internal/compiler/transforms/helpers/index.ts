/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createVisitor, signals} from "@internal/compiler";
import {
	AnyComment,
	AnyCommentOptionalId,
	AnyJSExpression,
	JSAssignmentIdentifier,
	JSReferenceIdentifier,
	jsAssignmentIdentifier,
	jsBindingIdentifier,
	jsReferenceIdentifier,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@internal/ast";
import {isRoot} from "@internal/ast-utils";

type VariableInjectorState = {
	bindings: Array<[string, undefined | AnyJSExpression]>;
};

export function injectBinding(
	path: Path,
	name: string = path.scope.generateUid(),
	init?: AnyJSExpression,
): [JSReferenceIdentifier, JSAssignmentIdentifier] {
	const ref = jsReferenceIdentifier.quick(name);

	// lol
	const ass = jsAssignmentIdentifier.quick(name);

	path.context.getVisitorState(variableInjectorVisitor).set(
		(state) => {
			return {
				bindings: [...state.bindings, [name, init]],
			};
		},
		{
			required: true,
		},
	);

	return [ref, ass];
}

export const variableInjectorVisitor = createVisitor<VariableInjectorState>({
	name: "variableInjector",
	enter(path, state) {
		const {node} = path;

		if (node.type === "JSBlockStatement" || node.type === "JSRoot") {
			state.reset({bindings: []});
		}

		return signals.retain;
	},

	exit(path, state) {
		const {node} = path;

		if (node.type === "JSBlockStatement" || node.type === "JSRoot") {
			const {bindings} = state.get();
			if (bindings.length === 0) {
				return signals.retain;
			}

			return signals.replace({
				...node,
				body: [
					jsVariableDeclarationStatement.quick(
						jsVariableDeclaration.create({
							kind: "var",
							declarations: bindings.map(([name, init]) => {
								return jsVariableDeclarator.create({
									id: jsBindingIdentifier.quick(name),
									init,
								});
							}),
						}),
					),
					...node.body,
				],
			});
		}

		return signals.retain;
	},
});

type CommentState = {
	comments: Array<AnyComment>;
};

export function injectComment(path: Path, comment: AnyCommentOptionalId): string {
	let commentWithId: AnyComment;
	const {id} = comment;

	if (id === undefined) {
		commentWithId = path.context.comments.createComment(comment);
	} else {
		// This comment already has an id so update it
		commentWithId = {
			...comment,
			id,
		};
		path.context.comments.updateComment(commentWithId);
	}

	path.context.getVisitorState(commentInjectorVisitor).set(
		(state) => {
			let comments = state.comments;

			if (id !== undefined) {
				// Remove from existing comments
				comments = comments.filter((comment) => comment.id !== id);
			}

			return {
				comments: [...comments, commentWithId],
			};
		},
		{
			required: true,
		},
	);

	return commentWithId.id;
}

export const commentInjectorVisitor = createVisitor<CommentState>({
	name: "commentInjector",

	enter(path, state) {
		const {node, context} = path;

		if (node.type === "CommentBlock" || node.type === "CommentLine") {
			context.comments.updateComment(node);
		}

		if (isRoot(node)) {
			context.comments.setComments(node.comments);
			state.reset({comments: []});
		}

		return signals.retain;
	},

	exit(path, state) {
		const {node} = path;

		if (isRoot(node) && state.owns()) {
			const {comments} = state.get();
			if (comments.length > 0) {
				return signals.replace({
					...node,
					comments: [...node.comments, ...comments],
				});
			}
		}

		return signals.retain;
	},
});
