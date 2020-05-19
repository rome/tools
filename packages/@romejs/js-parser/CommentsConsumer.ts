/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSComment, AnyJSCommentOptionalId} from "@romejs/ast";

export default class CommentsConsumer {
	constructor(seedComments: Array<AnyJSComment> = []) {
		this.idToComment = new Map();
		this.counter = seedComments.length;
		this.setComments(seedComments);
	}

	idToComment: Map<string, AnyJSComment>;
	counter: number;

	setComments(comments: Array<AnyJSComment>) {
		this.idToComment.clear();

		for (const comment of comments) {
			this.idToComment.set(comment.id, comment);
		}
	}

	getCommentsFromIds(ids: undefined | Array<string>): Array<AnyJSComment> {
		if (ids === undefined) {
			return [];
		}

		const comments: Array<AnyJSComment> = [];

		for (const id of ids) {
			const comment = this.getCommentFromId(id);
			if (comment !== undefined) {
				comments.push(comment);
			}
		}

		return comments;
	}

	getIdsFromComments(comments: Array<AnyJSComment>): Array<string> {
		return comments.map((comment) => comment.id);
	}

	getAllComments(): Array<AnyJSComment> {
		return Array.from(this.idToComment.values());
	}

	getCommentFromId(id: string): undefined | AnyJSComment {
		return this.idToComment.get(id);
	}

	assertGetCommentFromId(id: string): AnyJSComment {
		const comment = this.getCommentFromId(id);
		if (comment === undefined) {
			throw new Error(`No comment found for id ${id}`);
		}
		return comment;
	}

	getNextId(): string {
		return String(this.counter++);
	}

	updateComment(comment: AnyJSComment) {
		this.idToComment.set(comment.id, comment);
	}

	removeComment(id: string) {
		this.idToComment.delete(id);
	}

	addComment(withoutId: AnyJSCommentOptionalId): AnyJSComment {
		const withId: AnyJSComment = {
			...withoutId,
			id: this.getNextId(),
		};
		this.idToComment.set(withId.id, withId);
		return withId;
	}
}
