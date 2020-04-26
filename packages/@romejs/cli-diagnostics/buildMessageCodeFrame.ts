/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  CODE_FRAME_CONTEXT_LINES,
  CODE_FRAME_INDENT,
  CODE_FRAME_SELECTED_INDENT,
  GUTTER,
  HALF_MAX_CODE_FRAME_LINES,
  MAX_CODE_FRAME_LINES,
} from './constants';
import {Position} from '@romejs/parser-core';
import {cleanEquivalentString} from './utils';
import {
  Number0,
  ob1Coerce0,
  ob1Coerce0To1,
  ob1Coerce1To0,
  ob1Get0,
  ob1Inc,
  ob1Number0,
  ob1Number1Neg1,
  ob1Sub,
} from '@romejs/ob1';
import {markupToPlainText} from '@romejs/string-markup';

export default function buildMessageCodeFrame(
  sourceText: string,
  allLines: Array<string>,
  start: undefined | Position,
  end: undefined | Position,
  markerMessage: string,
): string {
  if (start === undefined || end === undefined) {
    return CODE_FRAME_INDENT + markerMessage;
  }

  const startLineIndex = ob1Coerce1To0(start.line);

  let endLineIndex = ob1Coerce1To0(end.line);
  let endLineCol = end.column;

  let markerOffset = end.column;
  let markerSize: Number0 = ob1Number0;

  // Increase the amount of lines we should show for "context"
  let contextStartIndex = ob1Coerce0(
    Math.max(0, ob1Get0(startLineIndex) - CODE_FRAME_CONTEXT_LINES),
  );
  let contextEndIndex = ob1Coerce0(
    Math.min(
      allLines.length - 1,
      ob1Get0(endLineIndex) + CODE_FRAME_CONTEXT_LINES,
    ),
  );

  let formattedLines: Array<
    | {
        gutter: string;
        line: string;
        lineIndex: Number0;
      }
    | undefined
  > = [];
  for (let i = contextStartIndex; i <= contextEndIndex; i = ob1Inc(i)) {
    let line: undefined | string = allLines[ob1Get0(i)];
    if (line === undefined) {
      continue;
    }

    // Ensure that the frame doesn't start with whitespace
    if (
      line.trim() === '' &&
      formattedLines.length === 0 &&
      i !== startLineIndex
    ) {
      continue;
    }

    // If this is within the highlighted line range
    const shouldHighlight: boolean = i >= startLineIndex && i <= endLineIndex;

    if (shouldHighlight) {
      // Range to highlight
      let startCol = ob1Number0;
      let endCol = ob1Coerce0(line.length);

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
          markerSize = ob1Sub(endCol, startCol);
          markerOffset = ob1Sub(markerOffset, markerSize);
        }
      }
    }

    const lineNo = ob1Coerce0To1(i);
    let gutter = `${String(lineNo)}${GUTTER}`;

    if (shouldHighlight) {
      gutter = `${CODE_FRAME_SELECTED_INDENT}${gutter}`;
    } else {
      gutter = `${CODE_FRAME_INDENT}${gutter}`;
    }

    formattedLines.push({
      gutter,
      line,
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
  for (let i = formattedLines.length - 1; i >= 0; i--) {
    const info = formattedLines[i];
    if (info !== undefined && info.line === '') {
      formattedLines.pop();
    } else {
      break;
    }
  }

  // If there's no lines to target then return the normal marker
  if (
    formattedLines.length === 0 ||
    end.line === ob1Number1Neg1 ||
    start.line === ob1Number1Neg1
  ) {
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
  const maxVisibleLineNo = ob1Get0(lastLine.lineIndex) + 1;
  const maxGutterLength =
    String(maxVisibleLineNo).length + GUTTER.length + CODE_FRAME_INDENT.length;

  // If what the marker is highlighting equals the marker message then it's redundant so don't show the message
  if (markerMessage !== '') {
    const text = sourceText.slice(ob1Get0(start.index), ob1Get0(end.index));
    if (
      cleanEquivalentString(text) ===
      cleanEquivalentString(markupToPlainText(markerMessage))
    ) {
      markerMessage = '';
    }
  }

  const pointerLength: number = Math.max(ob1Get0(markerSize), 1);
  const pointer: string = `<error><emphasis>${'^'.repeat(pointerLength)}</emphasis></error>`;
  const pointerIndent: string = ' '.repeat(ob1Get0(markerOffset));

  // If the marker is just pointing to the first character and we have no message, no point showing it
  const noMarkerLine =
    ob1Get0(markerOffset) === 0 && pointerLength === 1 && markerMessage === '';

  // Output no gutter with a soft indent if this is true
  if (noGutter) {
    const result = [...allLines];
    if (!noMarkerLine) {
      result.push(`${pointerIndent}${pointer} ${markerMessage}`);
    }
    return CODE_FRAME_INDENT + result.join(`\n${CODE_FRAME_INDENT}`);
  }

  // Build marker
  const markerGutterIndent: string = ' '.repeat(maxGutterLength - GUTTER.length);
  const markerLine: string = `${markerGutterIndent}<emphasis>${GUTTER}</emphasis>${pointerIndent}${pointer} ${markerMessage}`;

  // Build up the line we display when source lines are omitted
  const omittedLine =
    `<pad count="${String(maxGutterLength)}"><emphasis>...</emphasis></pad>` +
    GUTTER;

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
      result.push(
        `<emphasis><pad count="${String(maxGutterLength)}">${gutter}</pad></emphasis>` +
        line,
      );
    }
    if (lineIndex === endLineIndex && !noMarkerLine) {
      result.push(markerLine);
    }
  }

  const frame: string = result.join('\n');
  return frame;
}
