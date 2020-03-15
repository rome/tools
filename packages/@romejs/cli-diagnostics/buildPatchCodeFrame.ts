/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CODE_FRAME_INDENT, CODE_FRAME_CONTEXT_LINES, GUTTER} from './constants';
import {leftPad, formatAnsi} from '@romejs/string-ansi';
import {showInvisibles} from './utils';
import {
  Diffs,
  constants as diffConstants,
  groupDiffByLines,
} from '@romejs/string-diff';

function formatDiffLine(diffs: Diffs) {
  return diffs
    .map(([type, text]) => {
      if (type === diffConstants.DELETE) {
        return formatAnsi.red(showInvisibles(text));
      } else if (type === diffConstants.ADD) {
        return formatAnsi.green(showInvisibles(text));
      } else {
        // type === diffConstants.EQUAL
        return text;
      }
    })
    .join('');
}

const DELETE_MARKER = formatAnsi.red('-');
const ADD_MARKER = formatAnsi.green('+');

export default function buildPatchCodeFrame(rawDiffs: Diffs): string {
  const diffsByLine = groupDiffByLines(rawDiffs);
  let lastVisibleLine = -1;

  // Calculate the parts of the diff we should show
  const shownLines: Set<number> = new Set();
  for (let i = 0; i < diffsByLine.length; i++) {
    const diffs = diffsByLine[i];

    let hasChange = false;
    for (const [type] of diffs) {
      if (type === diffConstants.DELETE || type === diffConstants.ADD) {
        hasChange = true;
        break;
      }
    }

    if (hasChange) {
      for (
        let start = i - CODE_FRAME_CONTEXT_LINES;
        start < i + CODE_FRAME_CONTEXT_LINES;
        start++
      ) {
        shownLines.add(start);

        if (start > lastVisibleLine) {
          lastVisibleLine = start;
        }
      }
    }
  }

  const lineLength = String(lastVisibleLine).length;

  // Don't output a gutter if there's only a single line
  const noGutter = diffsByLine.length === 1;

  // Build the actual frame
  const frame = [];
  let lastDisplayedLine = -1;
  for (let i = 0; i < diffsByLine.length; i++) {
    if (shownLines.has(i) === false) {
      continue;
    }

    const diffs = diffsByLine[i];
    const lineNo = i + 1;

    if (noGutter) {
      frame.push('  ' + formatDiffLine(diffs));
      lastDisplayedLine = lineNo;
      continue;
    }

    const deletions: Diffs = [];
    const addition: Diffs = [];

    let hasDeletions = false;
    let hasAddition = false;

    for (const tuple of diffs) {
      let [type] = tuple;

      if (type === diffConstants.DELETE) {
        hasDeletions = true;
        deletions.push(tuple);
      }

      if (type === diffConstants.ADD) {
        hasAddition = true;
        addition.push(tuple);
      }

      if (type === diffConstants.EQUAL) {
        addition.push(tuple);
        deletions.push(tuple);
      }
    }

    if (lastDisplayedLine !== lineNo - 1 && lastDisplayedLine !== -1) {
      frame.push(
        formatAnsi.bold(CODE_FRAME_INDENT + '.'.repeat(lineLength) + GUTTER),
      );
    }

    const gutter = formatAnsi.bold(
      CODE_FRAME_INDENT + leftPad(String(lineNo), lineLength) + GUTTER,
    );

    if (hasAddition) {
      frame.push(gutter + ADD_MARKER + ' ' + formatDiffLine(addition));
    }

    if (hasDeletions) {
      frame.push(gutter + DELETE_MARKER + ' ' + formatDiffLine(deletions));
    }

    if (!hasAddition && !hasDeletions) {
      // Output one of the lines, they're the same
      frame.push(gutter + '  ' + formatDiffLine(addition));
    }

    lastDisplayedLine = lineNo;
  }

  return frame.join('\n');
}
