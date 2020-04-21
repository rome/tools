/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyCommentOptionalId} from '@romejs/js-ast';

export default class CommentsConsumer {
  constructor(seedComments: Array<AnyComment> = []) {
    this.idToComment = new Map();
    this.counter = seedComments.length;
    this.setComments(seedComments);
  }

  idToComment: Map<string, AnyComment>;
  counter: number;

  setComments(comments: Array<AnyComment>) {
    this.idToComment.clear();

    for (const comment of comments) {
      this.idToComment.set(comment.id, comment);
    }
  }

  getCommentsFromIds(ids: undefined | Array<string>): Array<AnyComment> {
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

  getIdsFromComments(comments: Array<AnyComment>): Array<string> {
    return comments.map((comment) => comment.id);
  }

  getAllComments(): Array<AnyComment> {
    return Array.from(this.idToComment.values());
  }

  getCommentFromId(id: string): undefined | AnyComment {
    return this.idToComment.get(id);
  }

  assertGetCommentFromId(id: string): AnyComment {
    const comment = this.getCommentFromId(id);
    if (comment === undefined) {
      throw new Error(`No comment found for id ${id}`);
    }
    return comment;
  }

  getNextId(): string {
    return String(this.counter++);
  }

  updateComment(comment: AnyComment) {
    this.idToComment.set(comment.id, comment);
  }

  removeComment(id: string) {
    this.idToComment.delete(id);
  }

  addComment(withoutId: AnyCommentOptionalId): AnyComment {
    const withId: AnyComment = {
      ...withoutId,
      id: this.getNextId(),
    };
    this.idToComment.set(withId.id, withId);
    return withId;
  }
}
