/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createHook} from "@romejs/js-compiler";
import {
	AnyJSComment,
	AnyJSCommentOptionalId,
	AnyJSExpression,
	AnyNode,
	JSAssignmentIdentifier,
	JSReferenceIdentifier,
	jsAssignmentIdentifier,
	jsBindingIdentifier,
	jsReferenceIdentifier,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@romejs/ast";

type VariableInjectorState = {
	bindings: Array<[string, undefined | AnyJSExpression]>;
};

type VariableInjectorArgs = {
	name?: string;
	init?: AnyJSExpression;
};

export const bindingInjector = createHook<
	VariableInjectorState,
	VariableInjectorArgs,
	[JSReferenceIdentifier, JSAssignmentIdentifier]
>({
	name: "bindingInjectorHook",
	initialState: {
		bindings: [],
	},
	call(
		path: Path,
		state: VariableInjectorState,
		opts: VariableInjectorArgs = {},
	) {
		const name = opts.name === undefined ? path.scope.generateUid() : opts.name;

		const ref = jsReferenceIdentifier.quick(name);

		// lol
		const ass = jsAssignmentIdentifier.quick(name);

		return {
			value: [ref, ass],
			state: {
				bindings: [...state.bindings, [name, opts.init]],
			},
		};
	},
	exit(path: Path, state: VariableInjectorState): AnyNode {
		const {node} = path;

		if (node.type !== "JSBlockStatement" && node.type !== "JSRoot") {
			throw new Error("Never should have been used as a provider");
		}

		const {bindings} = state;
		if (bindings.length === 0) {
			return node;
		}

		return {
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
		};
	},
});

export const variableInjectorVisitor = {
	name: "variableInjector",
	enter(path: Path) {
		const {node} = path;

		if (node.type === "JSBlockStatement" || node.type === "JSRoot") {
			path.provideHook(bindingInjector);
		}

		return node;
	},
};

type CommentInjectorState = {
	comments: Array<AnyJSComment>;
};

type CommentInjectorArg = AnyJSCommentOptionalId;

export const commentInjector = createHook<
	CommentInjectorState,
	CommentInjectorArg,
	string
>({
	name: "commentInjectorHook",
	initialState: {
		comments: [],
	},
	call(path: Path, state: CommentInjectorState, comment: CommentInjectorArg) {
		let commentWithId: AnyJSComment;
		let comments = state.comments;

		const {id} = comment;
		if (id === undefined) {
			commentWithId = path.context.comments.addComment(comment);
		} else {
			// This comment already has an id so update it
			commentWithId = {
				...comment,
				id,
			};
			path.context.comments.updateComment(commentWithId);

			// Remove from existing comments
			comments = comments.filter((comment) => comment.id !== id);
		}

		return {
			value: commentWithId.id,
			state: {
				comments: [...comments, commentWithId],
			},
		};
	},
	exit(path: Path, state: CommentInjectorState): AnyNode {
		const {node} = path;

		if (node.type !== "JSRoot") {
			throw new Error("Never should have been used as a provider");
		}

		return {
			...node,
			comments: [...node.comments, ...state.comments],
		};
	},
});

export const commentInjectorVisitor = {
	name: "commentInjector",
	enter(path: Path) {
		const {node, context} = path;

		if (node.type === "JSCommentBlock" || node.type === "JSCommentLine") {
			context.comments.updateComment(node);
		}

		if (node.type === "JSRoot") {
			context.comments.setComments(node.comments);
			return path.provideHook(commentInjector);
		}

		return node;
	},
};
