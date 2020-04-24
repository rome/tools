/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject, isIterable} from '@romejs/typescript-helpers';
import {escapeString} from '@romejs/string-escape';
import {humanizeNumber, naturalCompare} from '@romejs/string-utils';
import {escapeMarkup, markupTag} from '@romejs/string-markup';

type RecursiveStack = Array<unknown>;

type FormatOptions = {
  markup: boolean;
  indent: string;
  stack: RecursiveStack;
  depth: number;
  maxDepth: number;
  compact: boolean;
};

type FormatPartialOptions = {
  maxDepth?: number;
  markup?: boolean;
  indent?: string;
  stack?: RecursiveStack;
  compact?: boolean;
};

const DEFAULT_OPTIONS: FormatOptions = {
  maxDepth: Infinity,
  markup: false,
  indent: '',
  depth: 0,
  stack: [],
  compact: false,
};

const INDENT = '  ';

function maybeEscapeMarkup(str: string, opts: FormatOptions): string {
  if (opts.markup) {
    return escapeMarkup(str);
  } else {
    return str;
  }
}

export const CUSTOM_PRETTY_FORMAT = Symbol();

export default function prettyFormat(
  obj: unknown,
  rawOpts: FormatPartialOptions = {},
): string {
  const opts: FormatOptions = {
    ...DEFAULT_OPTIONS,
    ...rawOpts,
  };

  if (opts.maxDepth === opts.depth) {
    return '[depth exceeded]';
  }

  switch (typeof obj) {
    case 'symbol': {
      const val = maybeEscapeMarkup(formatSymbol(obj), opts);
      return opts.markup ? markupTag('color', val, {fg: 'green'}) : val;
    }

    case 'string': {
      const val = maybeEscapeMarkup(formatString(obj), opts);
      return opts.markup ? markupTag('color', val, {fg: 'green'}) : val;
    }

    case 'bigint':
    case 'number': {
      const val = formatNumber(obj);
      return opts.markup ? markupTag('color', val, {fg: 'yellow'}) : val;
    }

    case 'boolean': {
      const val = formatBoolean(obj);
      return opts.markup ? markupTag('color', val, {fg: 'yellow'}) : val;
    }

    case 'undefined': {
      const val = formatUndefined();
      return opts.markup ? markupTag('color', val, {fg: 'brightBlack'}) : val;
    }

    case 'function':
      return formatFunction(obj, opts);

    case 'object':
      return formatObjectish((obj as Objectish), opts);

    default:
      throw new Error('Unknown type');
  }
}

function joinList(items: Array<string>, opts: FormatOptions): string {
  if (items.length === 0) {
    return '';
  }

  const lines = [];

  for (const item of items) {
    lines.push(`${opts.indent}${item}`);
  }

  return lines.join('\n');
}

function isNativeFunction(val: Function): boolean {
  return val.toString().endsWith('{ [native code] }');
}

function formatSymbol(val: Symbol): string {
  return String(val);
}

function formatString(val: string): string {
  return escapeString(val, {
    quote: "'",
  });
}

// This function is used by rome-json so make sure it can parse whatever you return here
export function formatNumber(val: bigint | number): string {
  if (typeof val === 'bigint') {
    return humanizeNumber(val, '_');
  } else if (isNaN(val)) {
    return 'NaN';
  } else if (Object.is(val, -0)) {
    return '-0';
  } else if (isFinite(val)) {
    return humanizeNumber(val, '_');
  } else if (Object.is(val, -Infinity)) {
    return '-Infinity';
  } else if (Object.is(val, +Infinity)) {
    return 'Infinity';
  } else {
    throw new Error("Don't know how to format this number");
  }
}

function formatUndefined(): string {
  return 'undefined';
}

function formatNull(): string {
  return 'null';
}

function formatBoolean(val: boolean): string {
  return val === true ? 'true' : 'false';
}

function formatFunction(val: Function, opts: FormatOptions): string {
  const name = val.name === '' ? 'anonymous' : maybeEscapeMarkup(val.name, opts);
  let label = `Function ${name}`;

  if (isNativeFunction(val)) {
    label = `Native` + label;
  }

  if (Object.keys(val).length === 0) {
    return label;
  }

  // rome-suppress-next-line lint/noExplicitAny
  return formatObject(label, (val as any), opts, []);
}

function getExtraObjectProps(obj: Objectish, opts: FormatOptions): {
  props: Array<string>;
  ignoreKeys: UnknownObject;
} {
  const props: Array<string> = [];
  const ignoreKeys: UnknownObject = {};

  if (obj instanceof Map) {
    for (const [key, val] of obj) {
      const formattedKey = typeof key === 'string'
        ? formatKey(key, opts)
        : prettyFormat(key, opts);
      props.push(`${formattedKey} => ${prettyFormat(val, opts)}`);
    }
  } else if (isIterable(obj)) {
    let i = 0;
    for (const val of obj) {
      ignoreKeys[String(i++)] = val;
      props.push(`${prettyFormat(val, opts)}`);
    }
  }

  return {ignoreKeys, props};
}

function formatKey(rawKey: string, opts: FormatOptions): string {
  const key = maybeEscapeMarkup(rawKey, opts);

  // Format as a string if it contains any special characters
  if (/[^A-Za-z0-9_$]/g.test(key)) {
    return formatString(key);
  } else {
    return key;
  }
}

// These are object keys that should always go at the top and ignore any alphabetization
// This is fairly arbitrary but should include generic identifier keys
export const PRIORITIZE_KEYS = ['id', 'type', 'kind', 'key', 'name', 'value'];

type KeyInfo = {
  key: string;
  object: boolean;
};

function sortKeys(obj: Objectish): Array<KeyInfo> {
  const sortedKeys: Set<string> = new Set(Object.keys(obj).sort(naturalCompare));

  const priorityKeys: Array<KeyInfo> = [];
  const otherKeys: Array<KeyInfo> = [];
  const objectKeys: Array<KeyInfo> = [];

  for (const key of PRIORITIZE_KEYS) {
    if (sortedKeys.has(key)) {
      priorityKeys.push({key, object: false});
      sortedKeys.delete(key);
    }
  }

  for (const key of sortedKeys) {
    const val = obj[key];

    // Objects with properties should be at the bottom
    let isObject = false;
    if (typeof val === 'object' && val != null && Object.keys(val).length > 0) {
      isObject = true;
    }
    if (Array.isArray(val) && val.length > 0) {
      isObject = true;
    }
    if (isObject) {
      objectKeys.push({key, object: true});
    } else {
      otherKeys.push({key, object: false});
    }
  }

  return [...priorityKeys, ...otherKeys, ...objectKeys];
}

function lineCount(str: string): number {
  return str.split('\n').length;
  formatKey;
}

function lineCountCompare(a: string, b: string): number {
  return lineCount(a) - lineCount(b);
}

function formatObject(
  label: string,
  obj: Objectish,
  opts: FormatOptions,
  labelKeys: Array<string>,
): string {
  // Detect circular references, and create a pointer to the specific value
  const {stack} = opts;
  if (stack.length > 0 && stack.includes(obj)) {
    label = `Circular ${label} ${stack.indexOf(obj)}`;
    return opts.markup ? markupTag('color', label, {fg: 'cyan'}) : label;
  }

  //
  const nextOpts: FormatOptions = {
    ...opts,
    stack: [...stack, obj],
    depth: opts.depth + 1,
    indent: opts.indent + INDENT,
  };
  const {ignoreKeys, props} = getExtraObjectProps(obj, nextOpts);

  // For props that have object values, we always put them at the end, sorted by line count
  const objProps = [];

  // Get string props
  for (const {key, object} of sortKeys(obj)) {
    const val = obj[key];
    if (key in ignoreKeys && ignoreKeys[key] === val) {
      continue;
    }

    if (opts.compact && val === undefined) {
      continue;
    }

    // Ignore any properties already displayed in the label
    if (labelKeys.includes(key)) {
      continue;
    }

    const prop = `${formatKey(key, opts)}: ${prettyFormat(val, nextOpts)}`;
    if (object) {
      objProps.push(prop);
    } else {
      props.push(prop);
    }
  }

  // Sort object props by line count and push them on
  for (const prop of objProps.sort(lineCountCompare)) {
    props.push(prop);
  }

  // Get symbol props
  for (const sym of Object.getOwnPropertySymbols(obj)) {
    const val: unknown = Reflect.get(obj, sym);
    props.push(`${prettyFormat(sym, opts)}: ${prettyFormat(val, nextOpts)}`);
  }

  //
  let open = '{';
  let close = '}';
  if (isIterable(obj)) {
    open = '[';
    close = ']';
  }

  //
  let inner = joinList(props, nextOpts);
  if (inner !== '') {
    if (props.length === 1 && !inner.includes('\n')) {
      // Single prop with no newlines shouldn't be indented
      inner = inner.trim();
    } else {
      inner = `\n${inner}\n${opts.indent}`;
    }
  }

  label = opts.markup ? markupTag('color', label, {fg: 'cyan'}) : label;
  return `${label} ${open}${inner}${close}`;
}

function formatRegExp(val: RegExp): string {
  return String(val);
}

function formatDate(val: Date): string {
  return val.toISOString();
}

type Objectish = {
  type?: unknown;
  [key: string]: unknown;
};

function formatObjectish(val: null | Objectish, opts: FormatOptions): string {
  if (val === null) {
    const val = formatNull();
    return opts.markup ? markupTag('emphasis', val) : val;
  }

  if (val instanceof RegExp) {
    const str = formatRegExp(val);
    return opts.markup ? markupTag('color', str, {fg: 'red'}) : str;
  }

  if (val instanceof Date) {
    const str = formatDate(val);
    return opts.markup ? markupTag('color', str, {fg: 'magenta'}) : str;
  }

  let label = 'null';

  if (val.constructor !== undefined) {
    label = maybeEscapeMarkup(val.constructor.name, opts);
  }

  let labelKeys: Array<string> = [];

  // If there's a string type or kind property then use it as the label
  if (typeof val.type === 'string') {
    label = maybeEscapeMarkup(val.type, opts);
    labelKeys.push('type');
  } else if (typeof val.kind === 'string') {
    label = maybeEscapeMarkup(val.kind, opts);
    labelKeys.push('kind');
  }

  return formatObject(label, val, opts, labelKeys);
}
