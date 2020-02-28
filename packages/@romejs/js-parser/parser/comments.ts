/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * Based on the comment attachment algorithm used in espree and estraverse.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

import {JSParser} from '../parser';
import {AnyComment, AnyNode, CallExpression} from '@romejs/js-ast';

function last<T>(stack: Array<T>): T {
  return stack[stack.length - 1];
}

export function addComment(parser: JSParser, comment: AnyComment): void {
  parser.state.trailingComments.push(comment);
  parser.state.leadingComments.push(comment);
}

export function attachComments(parser: JSParser, node: AnyNode): void {
  if (node.type === 'Program' && node.body.length > 0) {
    return undefined;
  }

  const stack = parser.state.commentStack;

  let firstChild, lastChild, trailingComments, i, j;

  if (parser.state.trailingComments.length > 0) {
    // If the first comment in trailingComments comes after the
    // current node, then we're good - all comments in the array will
    // come after the node and so it's safe to add them as official
    // trailingComments.
    if (
      parser.getLoc(parser.state.trailingComments[0]).start.index >=
      parser.getLoc(node).end.index
    ) {
      trailingComments = parser.state.trailingComments;
      parser.state.trailingComments = [];
    } else {
      // Otherwise, if the first comment doesn't come after the
      // current node, that means we have a mix of leading and trailing
      // comments in the array and that leadingComments contains the
      // same items as trailingComments. Reset trailingComments to
      // zero items and we'll handle this by evaluating leadingComments
      // later.
      parser.state.trailingComments.length = 0;
    }
  } else {
    if (stack.length > 0) {
      const lastInStack = last(stack);
      if (
        lastInStack.trailingComments &&
        lastInStack.trailingComments.length > 0 &&
        parser.getLoc(lastInStack.trailingComments[0]).start.index >=
          parser.getLoc(node).end.index
      ) {
        trailingComments = lastInStack.trailingComments;
        lastInStack.trailingComments = undefined;
      }
    }
  }

  // Eating the stack.
  if (
    stack.length > 0 &&
    parser.getLoc(last(stack)).start.index >= parser.getLoc(node).start.index
  ) {
    firstChild = stack.pop();
  }

  while (
    stack.length > 0 &&
    parser.getLoc(last(stack)).start.index >= parser.getLoc(node).start.index
  ) {
    lastChild = stack.pop();
  }

  if (!lastChild && firstChild) {
    lastChild = firstChild;
  }

  // Attach comments that follow a trailing comma on the last
  // property in an object literal or a trailing comma in function arguments
  // as trailing comments
  if (firstChild && parser.state.leadingComments.length > 0) {
    const lastComment = last(parser.state.leadingComments);

    if (firstChild.type === 'ObjectProperty') {
      if (
        parser.getLoc(lastComment).start.index >=
        parser.getLoc(node).start.index
      ) {
        if (parser.state.commentPreviousNode) {
          for (j = 0; j < parser.state.leadingComments.length; j++) {
            if (
              parser.getLoc(parser.state.leadingComments[j]).end.index <
              parser.getLoc(parser.state.commentPreviousNode).end.index
            ) {
              parser.state.leadingComments.splice(j, 1);
              j--;
            }
          }

          if (parser.state.leadingComments.length > 0) {
            firstChild.trailingComments = parser.state.leadingComments;
            parser.state.leadingComments = [];
          }
        }
      }
    } else if (
      node.type === 'CallExpression' &&
      node.arguments &&
      node.arguments.length
    ) {
      const lastArg = last((node as CallExpression).arguments);

      if (
        lastArg !== undefined &&
        parser.getLoc(lastComment).start.index >=
          parser.getLoc(lastArg).start.index &&
        parser.getLoc(lastComment).end.index <= parser.getLoc(node).end.index
      ) {
        if (parser.state.commentPreviousNode) {
          if (parser.state.leadingComments.length > 0) {
            lastArg.trailingComments = parser.state.leadingComments;
            parser.state.leadingComments = [];
          }
        }
      }
    }
  }

  if (lastChild) {
    const lastLeadingComments = lastChild.leadingComments;
    if (lastLeadingComments !== undefined) {
      if (
        lastChild !== node &&
        lastLeadingComments.length > 0 &&
        parser.getLoc(last(lastLeadingComments)).end.index <=
          parser.getLoc(node).start.index
      ) {
        node.leadingComments = lastLeadingComments;
        lastChild.leadingComments = undefined;
      } else {
        // A leading comment for an anonymous class had been stolen by its first ClassMethod,
        // so this takes back the leading comment.
        // See also: https://github.com/eslint/espree/issues/158
        for (i = lastLeadingComments.length - 2; i >= 0; --i) {
          if (
            parser.getLoc(lastLeadingComments[i]).end.index <=
            parser.getLoc(node).start.index
          ) {
            node.leadingComments = lastLeadingComments.splice(0, i + 1);
            break;
          }
        }
      }
    }
  } else if (parser.state.leadingComments.length > 0) {
    if (
      parser.getLoc(last(parser.state.leadingComments)).end.index <=
      parser.getLoc(node).start.index
    ) {
      if (parser.state.commentPreviousNode) {
        for (j = 0; j < parser.state.leadingComments.length; j++) {
          if (
            parser.getLoc(parser.state.leadingComments[j]).end.index <
            parser.getLoc(parser.state.commentPreviousNode).end.index
          ) {
            parser.state.leadingComments.splice(j, 1);
            j--;
          }
        }
      }
      if (parser.state.leadingComments.length > 0) {
        node.leadingComments = parser.state.leadingComments;
        parser.state.leadingComments = [];
      }
    } else {
      // https://github.com/eslint/espree/issues/2
      //
      // In special cases, such as return (without a value) and
      // debugger, all comments will end up as leadingComments and
      // will otherwise be eliminated. This step runs when the
      // commentStack is empty and there are comments left
      // in leadingComments.
      //
      // This loop figures out the stopping point between the actual
      // leading and trailing comments by finding the location of the
      // first comment that comes after the given node.
      for (i = 0; i < parser.state.leadingComments.length; i++) {
        if (
          parser.getLoc(parser.state.leadingComments[i]).end.index >
          parser.getLoc(node).start.index
        ) {
          break;
        }
      }

      // Split the array based on the location of the first comment
      // that comes after the node. Keep in mind that this could
      // result in an empty array, and if so, the array must be
      // deleted.
      const leadingComments = parser.state.leadingComments.slice(0, i);
      node.leadingComments =
        leadingComments.length === 0 ? undefined : leadingComments;

      // Similarly, trailing comments are attached later. The variable
      // must be reset to null if there are no trailing comments.
      trailingComments = parser.state.leadingComments.slice(i);
      if (trailingComments.length === 0) {
        trailingComments = undefined;
      }
    }
  }

  parser.state.commentPreviousNode = node;

  if (trailingComments !== undefined && trailingComments.length > 0) {
    const nodeLoc = parser.getLoc(node);

    const innerComments = trailingComments.filter(comment => {
      const commentLoc = parser.getLoc(comment);
      return (
        commentLoc.start.index >= nodeLoc.start.index &&
        commentLoc.end.index <= nodeLoc.end.index
      );
    });

    if (innerComments.length > 0) {
      node.innerComments = innerComments;
      trailingComments = trailingComments.filter(
        comment => !innerComments.includes(comment),
      );
    }

    if (trailingComments.length > 0) {
      node.trailingComments = trailingComments;
    }
  }

  stack.push(node);
}
