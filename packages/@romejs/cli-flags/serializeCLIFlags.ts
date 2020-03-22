/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticLocation} from '@romejs/diagnostics';
import {toKebabCase} from '@romejs/string-utils';
import {ConsumeSourceLocationRequestTarget} from '@romejs/consume';
import {Number0, coerce0, number1, number0Neg1} from '@romejs/ob1';
import {Dict} from '@romejs/typescript-helpers';

type SerializeCLIData = {
  prefix: undefined | string;
  args: Array<string>;
  defaultFlags: Dict<unknown>;
  flags: Dict<unknown>;
  shorthandFlags: Set<string>;
};

export type SerializeCLITarget =
  | {
    type: 'flag';
    key: string;
    target?: ConsumeSourceLocationRequestTarget;
  }
  | {
    type: 'arg';
    key: number;
  }
  | {
    type: 'arg-range';
    from: number;
    to?: number;
  }
  | {type: 'none'};

function normalizeFlagValue(val: unknown): unknown {
  if (val === 'true') {
    return true;
  } else if (typeof val === 'object' && val != null) {
    return String(val);
  } else {
    return val;
  }
}

export function serializeCLIFlags(
  data: SerializeCLIData,
  cliTarget: SerializeCLITarget,
): DiagnosticLocation {
  const {args, flags, defaultFlags} = data;

  let code = `$ `;
  if (data.prefix !== undefined) {
    code += `${data.prefix} `;
  }
  let startColumn: Number0 = number0Neg1;
  let endColumn: Number0 = number0Neg1;

  // Add args
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    let isTarget = false;
    if (cliTarget.type === 'arg' && i === cliTarget.key) {
      isTarget = true;
    }
    if (cliTarget.type === 'arg-range' && cliTarget.from === i) {
      isTarget = true;
    }

    if (isTarget) {
      startColumn = coerce0(code.length);
    }

    code += `${arg} `;

    let isEndTarget = isTarget;

    // We are the end target if we're within the from-to range or we're greater than from with no to
    if (cliTarget.type === 'arg-range' && i > cliTarget.from &&
      (cliTarget.to === undefined || cliTarget.to <= i)) {
      isEndTarget = true;
    }

    if (isEndTarget) {
      endColumn = coerce0(code.length - 1);
    }
  }

  // Add flags
  for (const key in flags) {
    const val = normalizeFlagValue(flags[key]);

    // Ignore pointless default values
    if (val === normalizeFlagValue(defaultFlags[key])) {
      continue;
    }

    const isTarget = cliTarget.type === 'flag' && key === cliTarget.key;

    if (isTarget) {
      startColumn = coerce0(code.length);
    }

    const flagPrefix = data.shorthandFlags.has(key) ? '-' : '--';
    const kebabKey = toKebabCase(key);
    if (val === false) {
      code += `${flagPrefix}no-${kebabKey} `;
    } else {
      code += `${flagPrefix}${kebabKey} `;
    }

    // Booleans are always indicated with just their flag
    if (typeof val !== 'boolean') {
      // Only point to the value for flags that specify it
      if (isTarget && cliTarget.type === 'flag' &&
        (cliTarget.target === 'value' || cliTarget.target === 'inner-value')) {
        startColumn = coerce0(code.length);
      }

      // Number or string
      code += `${String(val)} `;
    }

    if (isTarget) {
      endColumn = coerce0(code.length - 1);
    }
  }

  if (startColumn === number0Neg1 || endColumn === number0Neg1) {
    startColumn = coerce0(code.length - 1);
    endColumn = startColumn;
  }

  return {
    language: 'shell',
    mtime: undefined,
    sourceText: code,
    filename: 'argv',
    start: {
      line: number1,
      column: startColumn,
      index: startColumn,
    },
    end: {
      line: number1,
      column: endColumn,
      index: endColumn,
    },
  };
}
