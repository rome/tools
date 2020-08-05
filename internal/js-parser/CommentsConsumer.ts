/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyCommentOptionalId} from "@internal/ast";

export default class CommentsConsumer {
	constructor(seedComments: Array<AnyComment> = []) {
		this.idToComment = new Map();
		this.counter = seedComments.length;
		this.setComments(seedComments);
	}

	private idToComment: Map<string, AnyComment>;
	private counter: number;

	public setComments(comments: Array<AnyComment>) {
		this.idToComment.clear();

		for (const comment of comments) {
			this.idToComment.set(comment.id, comment);
		}
	}

	public getCommentsFromIds(ids: undefined | Array<string>): Array<AnyComment> {
		if (ids === undefined) {
			return [];
		}

		const comments: Array<AnyComment> = [];

		for (const id of ids) {
			const comment = this.getCommentFromId(id);
			if (comment !== undefined) {
				comments.push(comment);
			}
		}

		return comments;
	}

	private getCommentFromId(id: string): undefined | AnyComment {
		return this.idToComment.get(id);
	}

	public assertGetCommentFromId(id: string): AnyComment {
		const comment = this.getCommentFromId(id);
		if (comment === undefined) {
			throw new Error(`No comment found for id ${id}`);
		}
		return comment;
	}

	private getNextId(): string {
		return String(this.counter++);
	}

	public updateComment(comment: AnyComment) {
		this.idToComment.set(comment.id, comment);
	}

	public createComment(withoutId: AnyCommentOptionalId): AnyComment {
		const withId: AnyComment = {
			...withoutId,
			id: this.getNextId(),
		};
		this.idToComment.set(withId.id, withId);
		return withId;
	}
}
