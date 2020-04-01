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
  DiagnosticAdviceLog,
  DiagnosticAdviceList,
  DiagnosticAdviceCode,
  DiagnosticAdviceFrame,
  DiagnosticAdviceInspect,
  DiagnosticAdviceDiff,
  DiagnosticAdviceStacktrace,
  DiagnosticAdviceCommand,
  getDiagnosticHeader,
} from '@romejs/diagnostics';
import {Position} from '@romejs/parser-core';
import {toLines} from './utils';
import buildPatchCodeFrame from './buildPatchCodeFrame';
import buildMessageCodeFrame from './buildMessageCodeFrame';
import {escapeMarkup, markupTag} from '@romejs/string-markup';
import {DiagnosticsPrinterFlags} from './types';
import {number0Neg1} from '@romejs/ob1';
import DiagnosticsPrinter, {
  DiagnosticsPrinterFileSources,
} from './DiagnosticsPrinter';
import {AbsoluteFilePathSet} from '@romejs/path';
import {RAW_CODE_MAX_LENGTH} from './constants';
import {Diffs, diffConstants} from '@romejs/string-diff';

type AdvicePrintOptions = {
  printer: DiagnosticsPrinter;
  flags: DiagnosticsPrinterFlags;
  missingFileSources: AbsoluteFilePathSet;
  fileSources: DiagnosticsPrinterFileSources;
  reporter: Reporter;
  diagnostic: Diagnostic;
};

type PrintAdviceResult = {
  printed: boolean;
  truncated: boolean;
};

const DID_PRINT: PrintAdviceResult = {
  printed: true,
  truncated: false,
};

const DID_NOT_PRINT: PrintAdviceResult = {
  printed: false,
  truncated: false,
};

export default function printAdvice(
  item: DiagnosticAdviceItem,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  switch (item.type) {
    case 'log':
      return printLog(item, opts);

    case 'list':
      return printList(item, opts);

    case 'diff':
      return printDiff(item, opts);

    case 'code':
      return printCode(item, opts);

    case 'command':
      return printCommand(item, opts);

    case 'frame':
      return printFrame(item, opts);

    case 'stacktrace':
      return printStacktrace(item, opts);

    case 'inspect':
      return printInspect(item, opts);
  }
}

function printCommand(
  item: DiagnosticAdviceCommand,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  opts.reporter.command(item.command);
  return DID_PRINT;
}

function printInspect(
  item: DiagnosticAdviceInspect,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  const {reporter} = opts;
  reporter.indent(() => {
    reporter.inspect(item.data);
  });
  return DID_PRINT;
}

function removeCRLF(str: string): string {
  return str.replace(/\r/g, '');
}

function generateDiffHint(diffs: Diffs): undefined | DiagnosticAdviceItem {
  let expected = '';
  let received = '';

  for (const [type, text] of diffs) {
    switch (type) {
      case diffConstants.ADD:
        received += text;
        break;

      case diffConstants.DELETE:
        expected += text;
        break;

      case diffConstants.EQUAL:
        expected += text;
        received += text;
        break;
    }
  }

  if (expected.trim() === received.trim()) {
    return {
      type: 'log',
      category: 'info',
      message: 'Only difference is leading and trailing whitespace',
    };
  }

  const receivedNoCRLF = removeCRLF(received);
  if (expected === receivedNoCRLF) {
    return {
        type: 'log',
        category: 'info',
        message: 'Identical except the received uses CRLF newlines, while the expected does not',
      };
  }

  const expectedNoCRLF = removeCRLF(expected);
  if (received === expectedNoCRLF) {
    return {
        type: 'log',
        category: 'info',
        message: 'Identical except the expected uses CRLF newlines, while the received does not',
      };
  }
}

function printDiff(
  item: DiagnosticAdviceDiff,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  const {frame, truncated} = buildPatchCodeFrame(
    item.diff,
    opts.flags.verboseDiagnostics,
  );
  if (frame === '') {
    return DID_NOT_PRINT;
  }

  opts.reporter.logAll(frame);

  const {legend} = item;
  if (legend !== undefined) {
    opts.reporter.spacer();
    opts.reporter.logAll(`<green>+ ${legend.add}</green>`);
    opts.reporter.logAll(`<red>- ${legend.delete}</red>`);
    opts.reporter.spacer();
  }

  const hint = generateDiffHint(item.diff);
  if (hint !== undefined) {
    opts.reporter.spacer();
    printAdvice(hint, opts);
    opts.reporter.spacer();
  }

  return {
    printed: true,
    truncated,
  };
}

function printList(
  item: DiagnosticAdviceList,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  if (item.list.length === 0) {
    return DID_NOT_PRINT;
  } else {
    opts.reporter.list(item.list, {
      truncate: opts.flags.verboseDiagnostics ? undefined : 20,
      reverse: item.reverse,
      ordered: item.ordered,
    });
    return DID_PRINT;
  }
}

function printCode(
  item: DiagnosticAdviceCode,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  const {reporter} = opts;

  let truncated = item.code.length > RAW_CODE_MAX_LENGTH;
  let code = item.code.slice(0, RAW_CODE_MAX_LENGTH);

  if (truncated) {
      code +=
      `\n<dim><number>${item.code.length - RAW_CODE_MAX_LENGTH}</number> more characters truncated</dim>`;
  }

  reporter.indent(() => {
    reporter.logAll(escapeMarkup(code));
  });

  return {
    printed: true,
    truncated,
  };
}

function printFrame(
  item: DiagnosticAdviceFrame,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  const {reporter} = opts;
  const {marker} = item;
  const {start, end, filename} = item.location;
  let {sourceText} = item.location;
  const path = opts.printer.createFilePath(filename);

  let cleanMarker: string = '';
  if (marker !== undefined) {
    cleanMarker = markupTag('emphasis', cleanMessage(marker));
  }

  let lines: Array<string> = [];
  if (sourceText !== undefined) {
    lines = toLines({
      path,
      input: sourceText,
      sourceType: item.location.sourceType,
      language: item.location.language,
    });
  } else if (filename !== undefined) {
    const source = opts.fileSources.get(path);
    if (source !== undefined) {
      lines = source.lines;
      sourceText = source.sourceText;
    }
  } else if (path.isAbsolute() && opts.missingFileSources.has(
      path.assertAbsolute(),
    )) {
    lines = ['<dim>File does not exist</dim>'];
  }

  if (sourceText === undefined) {
    sourceText = '';
  }

  const frame = buildMessageCodeFrame(sourceText, lines, start, end, cleanMarker);
  if (frame.trim() === '') {
    return DID_NOT_PRINT;
  }

  reporter.logAll(frame);
  return DID_PRINT;
}

function printStacktrace(
  item: DiagnosticAdviceStacktrace,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
  // Here we duplicate some of the list logic that is in Reporter

  // This is different as we also want to push frames after some of the items

  const {diagnostic} = opts;
  const {frames} = item;

  let shownCodeFrames = 0;

  const isFirstPart = diagnostic.description.advice !== undefined &&
      diagnostic.description.advice[0] ===
      item;
  if (!isFirstPart) {
    opts.reporter.info(item.title === undefined ? 'Stack trace' : item.title);
    opts.reporter.forceSpacer();
  }

  opts.reporter.processedList(frames, (frame, display) => {
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
      logParts.push(markupTag('dim', escapeMarkup(prefix)));
    }

    // Build path
    const objParts = [];
    if (object !== undefined) {
      objParts.push(markupTag('magenta', escapeMarkup(object)));
    }
    if (property !== undefined) {
      objParts.push(markupTag('cyan', escapeMarkup(property)));
    }
    if (objParts.length > 0) {
      logParts.push(objParts.join('.'));
    }

    // Add suffix
    if (suffix !== undefined) {
      logParts.push(markupTag('green', escapeMarkup(suffix)));
    }

    // Add source
    if (filename !== undefined && line !== undefined && column !== undefined) {
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
        logParts.push(`(<dim>${header}</dim>)`);
      }
    }

    display(logParts.join(' '));

    // Push on frame
    if (shownCodeFrames < 2 && filename !== undefined && line !== undefined &&
          column !==
          undefined) {
      const pos: Position = {
        index: number0Neg1,
        line,
        column,
      };

      const skipped = printFrame({
        type: 'frame',
        marker: undefined,
        location: {
          language,
          filename,
          sourceType: 'module',
          mtime: undefined,
          start: pos,
          end: pos,
          sourceText: code,
        },
      }, opts);
      if (!skipped) {
        opts.reporter.forceSpacer();
        shownCodeFrames++;
      }
    }
  }, {
    ordered: true,
    truncate: opts.flags.verboseDiagnostics ? undefined : 20,
  });

  return DID_PRINT;
}

function printLog(
  item: DiagnosticAdviceLog,
  opts: AdvicePrintOptions,
): PrintAdviceResult {
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

  return item.compact ? DID_NOT_PRINT : DID_PRINT;
}

function cleanMessage(msg: string): string {
  msg = msg.trim();
  if (msg.endsWith('.')) {
    msg = msg.slice(0, -1);
  }
  return msg;
}
