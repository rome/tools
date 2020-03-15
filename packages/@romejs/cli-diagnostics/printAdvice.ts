/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {
  Diagnostic,
  DiagnosticAdviceItem,
  DiagnosticAdviceItemLog,
  DiagnosticAdviceItemList,
  DiagnosticAdviceItemCode,
  DiagnosticAdviceItemFrame,
  DiagnosticAdviceItemInspect,
  DiagnosticAdviceItemDiff,
  DiagnosticAdviceItemStacktrace,
} from '@romejs/diagnostics';
import {Position} from '@romejs/parser-core';
import {toLines} from './utils';
import {getDiagnosticHeader} from '@romejs/diagnostics';
import buildPatchCodeFrame from './buildPatchCodeFrame';
import buildMessageCodeFrame from './buildMessageCodeFrame';
import {escapeMarkup} from '@romejs/string-markup';
import {formatAnsi} from '@romejs/string-ansi';
import {DiagnosticsPrinterFlags} from './types';
import {number0Neg1} from '@romejs/ob1';
import {DiagnosticsPrinterFileSources} from './DiagnosticsPrinter';
import {createUnknownFilePath, AbsoluteFilePathSet} from '@romejs/path';

type AdvicePrintOptions = {
  flags: DiagnosticsPrinterFlags;
  missingFileSources: AbsoluteFilePathSet;
  fileSources: DiagnosticsPrinterFileSources;
  reporter: Reporter;
  diagnostic: Diagnostic;
};

export default function printAdvice(
  item: DiagnosticAdviceItem,
  opts: AdvicePrintOptions,
): boolean {
  switch (item.type) {
    case 'log':
      return printLog(item, opts);

    case 'list':
      return printList(item, opts);

    case 'diff':
      return printDiff(item, opts);

    case 'code':
      return printCode(item, opts);

    case 'frame':
      return printFrame(item, opts);

    case 'stacktrace':
      return printStacktrace(item, opts);

    case 'inspect':
      return printInspect(item, opts);
  }
}

function printInspect(
  item: DiagnosticAdviceItemInspect,
  opts: AdvicePrintOptions,
): boolean {
  const {reporter} = opts;
  reporter.indent(() => {
    reporter.inspect(item.data);
  });
  return false;
}

function printDiff(
  item: DiagnosticAdviceItemDiff,
  opts: AdvicePrintOptions,
): boolean {
  const frame = buildPatchCodeFrame(item.diff);
  if (frame === '') {
    return true;
  }

  opts.reporter.logAll(escapeMarkup(frame));
  return false;
}

function printList(
  item: DiagnosticAdviceItemList,
  opts: AdvicePrintOptions,
): boolean {
  if (item.list.length === 0) {
    return true;
  } else {
    opts.reporter.list(item.list, {
      truncate: opts.flags.verboseDiagnostics ? undefined : 20,
      reverse: item.reverse,
      ordered: item.ordered,
    });
    return false;
  }
}

function printCode(
  item: DiagnosticAdviceItemCode,
  opts: AdvicePrintOptions,
): boolean {
  const {reporter} = opts;
  const {code} = item;
  reporter.indent(() => {
    reporter.logAll(escapeMarkup(code));
  });
  return false;
}

function printFrame(
  item: DiagnosticAdviceItemFrame,
  opts: AdvicePrintOptions,
): boolean {
  const {reporter} = opts;
  const {start, end, filename, sourceText, marker} = item;
  const path = createUnknownFilePath(filename);

  let cleanMarker: string = '';
  if (marker !== undefined) {
    cleanMarker = formatAnsi.bold(cleanMessage(marker));
  }

  let lines: undefined | Array<string>;
  if (sourceText !== undefined) {
    lines = toLines({
      path,
      input: sourceText,
      sourceType: item.sourceType,
      language: item.language,
    });
  } else if (filename !== undefined) {
    lines = opts.fileSources.get(path);
  } else if (
    path.isAbsolute() &&
    opts.missingFileSources.has(path.assertAbsolute())
  ) {
    lines = [formatAnsi.dim('file does not exist')];
  }
  if (lines === undefined) {
    lines = [];
  }

  const frame = buildMessageCodeFrame(lines, start, end, cleanMarker);
  if (frame.trim() === '') {
    return true;
  }

  reporter.logAll(escapeMarkup(frame));
  return false;
}

function printStacktrace(
  item: DiagnosticAdviceItemStacktrace,
  opts: AdvicePrintOptions,
): boolean {
  // Here we duplicate some of the list logic that is in Reporter
  // This is different as we also want to push frames after some of the items

  const {diagnostic} = opts;
  const {frames} = item;

  let shownCodeFrames = 0;

  const isFirstPart = diagnostic.advice[0] === item;
  if (!isFirstPart) {
    opts.reporter.info(item.title === undefined ? 'Stack trace' : item.title);
    opts.reporter.forceSpacer();
  }

  opts.reporter.processedList(
    frames,
    (frame, display) => {
      const {
        filename,
        object,
        suffix,
        property,
        prefix,
        line,
        column,
        language,
        sourceText: code,
      } = frame;

      const logParts = [];

      // Add prefix
      if (prefix !== undefined) {
        logParts.push(formatAnsi.dim(escapeMarkup(prefix)));
      }

      // Build path
      const objParts = [];
      if (object !== undefined) {
        objParts.push(formatAnsi.magenta(escapeMarkup(object)));
      }
      if (property !== undefined) {
        objParts.push(formatAnsi.cyan(escapeMarkup(property)));
      }
      if (objParts.length > 0) {
        logParts.push(objParts.join('.'));
      }

      // Add suffix
      if (suffix !== undefined) {
        logParts.push(formatAnsi.green(escapeMarkup(suffix)));
      }

      // Add source
      if (
        filename !== undefined &&
        line !== undefined &&
        column !== undefined
      ) {
        const header = getDiagnosticHeader({
          filename,
          start: {
            index: number0Neg1,
            line,
            column,
          },
        });

        if (logParts.length === 0) {
          logParts.push(header);
        } else {
          logParts.push(`(${formatAnsi.dim(header)})`);
        }
      }

      display(logParts.join(' '));

      // Push on frame
      if (
        shownCodeFrames < 2 &&
        filename !== undefined &&
        line !== undefined &&
        column !== undefined
      ) {
        const pos: Position = {
          index: number0Neg1,
          line,
          column,
        };

        const skipped = printFrame(
          {
            type: 'frame',
            language,
            filename,
            sourceType: 'module',
            marker: undefined,
            mtime: undefined,
            start: pos,
            end: pos,
            sourceText: code,
          },
          opts,
        );
        if (!skipped) {
          opts.reporter.forceSpacer();
          shownCodeFrames++;
        }
      }
    },
    {
      ordered: true,
      truncate: opts.flags.verboseDiagnostics ? undefined : 20,
    },
  );

  return false;
}

function printLog(
  item: DiagnosticAdviceItemLog,
  opts: AdvicePrintOptions,
): boolean {
  const {reporter} = opts;
  const {message, category} = item;

  if (message !== undefined) {
    switch (category) {
      case 'none':
        reporter.logAll(message);
        break;

      case 'warn':
        reporter.warn(message);
        break;

      case 'info':
        reporter.info(message);
        break;

      case 'error':
        reporter.error(message);
        break;

      default:
        throw new Error(`Unknown message item log category ${category}`);
    }
  }

  return item.compact;
}

function cleanMessage(msg: string): string {
  msg = msg.trim();
  if (msg.endsWith('.')) {
    msg = msg.slice(0, -1);
  }
  return msg;
}
