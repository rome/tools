/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  GUTTER,
  CODE_FRAME_CONTEXT_LINES,
  CODE_FRAME_SELECTED_INDENT,
  CODE_FRAME_INDENT,
  MAX_CODE_FRAME_LINES,
  HALF_MAX_CODE_FRAME_LINES,
} from './constants';
import {Position} from '@romejs/parser-core';
import {escapeMarkup} from '@romejs/string-markup';
import {cleanEquivalentString} from './utils';
import {leftPad, formatAnsi, stripAnsi} from '@romejs/string-ansi';
import {
  sub,
  get0,
  number0,
  coerce1to0,
  coerce0,
  inc,
  Number0,
  coerce0to1,
  number1Neg1,
} from '@romejs/ob1';

export default function buildMessageCodeFrame(
  allLines: Array<string>,
  start: Position,
  end: Position,
  maybeMarkerMessage?: string,
): string {
  let markerMessage: string = maybeMarkerMessage === undefined
    ? '' : maybeMarkerMessage;

  const startLineIndex = coerce1to0(start.line);

  let endLineIndex = coerce1to0(end.line);
  let endLineCol = end.column;

  let markerOffset = end.column;
  let markerSize: Number0 = number0;

  // Increase the amount of lines we should show for "context"
  let contextStartIndex = coerce0(Math.max(0, get0(startLineIndex) -
  CODE_FRAME_CONTEXT_LINES));
  let contextEndIndex = coerce0(Math.min(
    allLines.length - 1,
    get0(endLineIndex) + CODE_FRAME_CONTEXT_LINES,
  ));

  let formattedLines: Array<
      | {
        gutter: string;
        line: string;
        lineIndex: Number0;
      }
      | undefined> = [];
  for (let i = contextStartIndex;
  i <= contextEndIndex;
  i = inc(i)) {
    let line: undefined | string = allLines[get0(i)];
    if (line === undefined) {
      continue;
    }

    // Ensure that the frame doesn't start with whitespace
    if (line.trim() === '' && formattedLines.length === 0 && i !== startLineIndex) {
      continue;
    }

    // If this is within the highlighted line range
    const shouldHighlight: boolean = i >= startLineIndex && i <= endLineIndex;

    if (shouldHighlight) {
      // Range to highlight
      let startCol = number0;
      let endCol = coerce0(line.length);

      // First line in selection
      if (i === startLineIndex) {
        startCol = start.column;
      }

      // Last line in selection
      if (i === endLineIndex) {
        endCol = endLineCol;

        // Sometimes the end will be the same as the start if we just want to make a single place

        // and not perform highlighting
        if (endCol > startCol) {
          markerSize = sub(endCol, startCol);
          markerOffset = sub(markerOffset, markerSize);
        }
      }
    }

    const lineNo = coerce0to1(i);
    let gutter = `${String(lineNo)}${GUTTER}`;

    if (shouldHighlight) {
      gutter = `${CODE_FRAME_SELECTED_INDENT}${gutter}`;
    } else {
      gutter = `${CODE_FRAME_INDENT}${gutter}`;
    }

    formattedLines.push({
      gutter,
      line: escapeMarkup(line),
      lineIndex: i,
    });
  }

  // If we have too many lines in our selection, then collapse them to an ellipsis
  const pruned = formattedLines.length > MAX_CODE_FRAME_LINES + 2;
  if (pruned) {
    const start = formattedLines.slice(0, HALF_MAX_CODE_FRAME_LINES);
    const end = formattedLines.slice(-HALF_MAX_CODE_FRAME_LINES);
    formattedLines = start.concat([undefined], end);
  }

  // Remove trailing blank lines
  for (let i = formattedLines.length - 1;
  i >= 0;
  i--) {
    const info = formattedLines[i];
    if (info !== undefined && info.line === '') {
      formattedLines.pop();
    } else {
      break;
    }
  }

  // If there's no lines to target then return the normal marker
  if (formattedLines.length === 0 || end.line === number1Neg1 || start.line ===
  number1Neg1) {
    return CODE_FRAME_INDENT + markerMessage;
  }

  // Don't output a gutter if there's only a single line
  const noGutter = allLines.length === 1;

  // Calculate max size of gutter, this is the maximum visible line plus the futter length plus the frame indent
  const lastLine = formattedLines[formattedLines.length - 1];
  if (lastLine === undefined) {
    throw new Error('Expected there to be a last line');
  }

  // Calculate the max width of the gutter based on the line count
  const maxVisibleLineNo = get0(lastLine.lineIndex) + 1;
  const maxGutterLength = String(maxVisibleLineNo).length + GUTTER.length +
  CODE_FRAME_INDENT.length;

  // If what the marker is highlighting equals the marker message then it's redundant so don't show the message
  if (markerMessage !== '' && start.line === end.line) {
    const markerLine = stripAnsi(allLines[get0(coerce1to0(start.line))]);
    const text = markerLine.slice(get0(start.column), get0(end.column));
    if (cleanEquivalentString(text) === cleanEquivalentString(markerMessage)) {
      markerMessage = '';
    }
  }

  const pointerLength: number = Math.max(get0(markerSize), 1);
  const pointer: string = formatAnsi.red(formatAnsi.bold('^'.repeat(
    pointerLength,
  )));
  const pointerIndent: string = ' '.repeat(get0(markerOffset));

  // If the marker is just pointing to the first character and we have no message, no point showing it
  const noMarkerLine = get0(markerOffset) === 0 && pointerLength === 1 &&
    markerMessage === '';

  // Output no gutter with a soft indent if this is true
  if (noGutter) {
    const result = [...allLines].map((line) => escapeMarkup(line));
    if (!noMarkerLine) {
      result.push(`${pointerIndent}${pointer} ${markerMessage}`);
    }
    return CODE_FRAME_INDENT + result.join(`\n${CODE_FRAME_INDENT}`);
  }

  // Build marker
  const markerGutterIndent: string = ' '.repeat(maxGutterLength - GUTTER.length);
  const markerLine: string =
  `${markerGutterIndent}${formatAnsi.bold(GUTTER)}${pointerIndent}${pointer} ${markerMessage}`;

  // Build up the line we display when source lines are omitted
  const omittedDots = '...';
  const omittedLine = leftPad(
    formatAnsi.bold(omittedDots) + GUTTER,
    maxGutterLength,
  );

  // Build the frame
  const result = [];
  for (const selection of formattedLines) {
    if (!selection) {
      result.push(omittedLine);
      continue;
    }

    const {gutter, line, lineIndex} = selection;

    if (noGutter) {
      result.push(line);
    } else {
      result.push(formatAnsi.bold(leftPad(gutter, maxGutterLength)) + line);
    }
    if (lineIndex === endLineIndex && !noMarkerLine) {
      result.push(markerLine);
    }
  }

  const frame: string = result.join('\n');
  return frame;
}
