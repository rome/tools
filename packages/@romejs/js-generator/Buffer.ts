/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {GeneratorOptions} from './Generator';
import {Position, SourceLocation} from '@romejs/parser-core';
import {Mappings, SourceMapConsumer} from '@romejs/codec-source-map';
import {SourceMapGenerator} from '@romejs/codec-source-map';
import {
  number1,
  number0,
  number0Neg1,
  Number1,
  Number0,
  inc,
  add,
  coerce0,
} from '@romejs/ob1';

const SPACES_RE = /^[ \t]+$/;

/**
 * The Buffer class exists to manage the queue of tokens being pushed onto the output string
 * in such a way that the final string buffer is treated as write-only until the final .get()
 * call. This allows V8 to optimize the output efficiently by not requiring it to store the
 * string in contiguous memory.
 */

export default class Buffer {
  constructor(opts: GeneratorOptions, code: string) {
    this.originalCode = code;
    this.opts = opts;
    this.mappings = [];
    this.inputSourceMap =
      opts.inputSourceMap === undefined
        ? undefined
        : new SourceMapConsumer(opts.inputSourceMap);

    this.buf = [];
    this.last = '';
    this._queue = [];

    this.position = {
      index: number0Neg1,
      column: number0,
      line: number1,
    };

    this.sourcePosition = {
      column: undefined,
      filename: undefined,
      identifierName: undefined,
      line: undefined,
    };
  }

  inputSourceMap: undefined | SourceMapConsumer;
  originalCode: string;
  opts: GeneratorOptions;
  mappings: Mappings;
  lastGenLine: undefined | Number1;
  lastSourceLine: undefined | Number1;
  lastSourceColumn: undefined | Number0;

  buf: Array<string>;
  last: string;

  // [str, line, column, identifierName, filename]
  _queue: Array<
    [
      string,
      undefined | Number1,
      undefined | Number0,
      undefined | string,
      undefined | string,
    ]
  >;

  position: Position;
  sourcePosition: {
    identifierName: undefined | string;
    line: undefined | Number1;
    column: undefined | Number0;
    filename: undefined | string;
  };

  getSourceMap() {
    const {opts} = this;

    const map = new SourceMapGenerator({
      file: opts.sourceMapTarget,
      sourceRoot: opts.sourceRoot,
    });

    if (opts.sourceFileName !== undefined) {
      map.setSourceContent(opts.sourceFileName, this.originalCode);
    }

    for (const mapping of this.mappings) {
      map.addMapping(mapping);
    }

    return map.toJSON();
  }

  getMappings(): Mappings {
    return this.mappings.slice();
  }

  /**
   * Mark the current generated position with a source position. May also be passed null line/column
   * values to insert a mapping to nothing.
   */
  mark(
    generatedLine: Number1,
    generatedColumn: Number0,
    originalLine: undefined | Number1,
    originalColumn: undefined | Number0,
    identifierName: undefined | string,
    filename: undefined | string = this.opts.sourceFileName,
  ) {
    // TODO: emit a mapping with `original: undefined` in this case - after
    // deduplicating using lastSourceLine and lastSourceColumn.
    if (originalLine === undefined || originalColumn === undefined) {
      return undefined;
    }

    // If this mapping points to the same source location as the last one, we can ignore it since
    // the previous one covers it.
    if (
      this.lastGenLine === generatedLine &&
      this.lastSourceLine === originalLine &&
      this.lastSourceColumn === originalColumn
    ) {
      return undefined;
    }

    this.lastGenLine = generatedLine;
    this.lastSourceLine = originalLine;
    this.lastSourceColumn = originalColumn;

    // undefined to allow for more compact json serialization
    const name = identifierName === undefined ? undefined : identifierName;
    const source: undefined | string = filename;

    // Forward mappings if provided with an inputSourceMap
    const {inputSourceMap} = this;
    if (inputSourceMap !== undefined) {
      const actual = inputSourceMap.exactOriginalPositionFor(
        originalLine,
        originalColumn,
      );
      if (actual === undefined) {
        // If we were given an input source map and we didn't find the original location in it then omit it since it probably doesn't make sense
        return;
      } else {
        originalLine = actual.line;
        originalColumn = actual.column;
      }
    }

    this.mappings.push({
      generated: {line: generatedLine, column: generatedColumn},
      original: {line: originalLine, column: originalColumn},
      name,
      source,
    });
  }

  /**
   * Get the final string output from the buffer.
   */
  getCode(): string {
    this.flush();
    return this.buf.join('').trimRight();
  }

  /**
   * Add a string to the buffer that cannot be reverted.
   */
  append(str: string): void {
    this.flush();
    const {column, filename, identifierName, line} = this.sourcePosition;
    this._append(str, line, column, identifierName, filename);
  }

  /**
   * Add a string to the buffer than can be reverted.
   */
  queue(str: string): void {
    // Drop trailing spaces when a newline is inserted.
    if (str === '\n') {
      while (this._queue.length > 0 && SPACES_RE.test(this._queue[0][0])) {
        this._queue.shift();
      }
    }

    const {column, filename, identifierName, line} = this.sourcePosition;
    this._queue.unshift([str, line, column, identifierName, filename]);
  }

  flush(): void {
    let item;
    while ((item = this._queue.pop())) {
      this._append(...item);
    }
  }

  _append(
    str: string,
    line: undefined | Number1,
    column: undefined | Number0,
    identifierName: undefined | string,
    filename: undefined | string,
  ): void {
    // If there the line is ending, adding a new mapping marker is redundant
    if (str[0] !== '\n') {
      this.mark(
        this.position.line,
        this.position.column,
        line,
        column,
        identifierName,
        filename,
      );
    }

    this.buf.push(str);
    this.last = str[str.length - 1];

    for (let i = 0; i < str.length; i++) {
      this.position.index = inc(this.position.index);

      if (str[i] === '\n') {
        this.position.line = inc(this.position.line);
        this.position.column = number0;
      } else {
        this.position.column = inc(this.position.column);
      }
    }
  }

  removeTrailingNewline(): void {
    if (this._queue.length > 0 && this._queue[0][0] === '\n') {
      this._queue.shift();
    }
  }

  removeLastSemicolon(): void {
    if (this._queue.length > 0 && this._queue[0][0] === ';') {
      this._queue.shift();
    }
  }

  isEmpty(): boolean {
    // Fast paths
    if (this.buf.length === 0 && this._queue.length === 0) {
      return true;
    }
    if (this.buf[0] !== '') {
      return false;
    }

    for (const part of this.buf) {
      if (part !== '') {
        return false;
      }
    }

    for (const [part] of this._queue) {
      if (part !== '') {
        return false;
      }
    }

    return true;
  }

  endsWith(suffix: string): boolean {
    // Fast path to avoid iterating over this._queue.
    if (suffix.length === 1) {
      let last;
      if (this._queue.length > 0) {
        const str = this._queue[0][0];
        last = str[str.length - 1];
      } else {
        last = this.last;
      }

      return last === suffix;
    }

    const end =
      this.last + this._queue.reduce((acc, item) => item[0] + acc, '');
    if (suffix.length <= end.length) {
      return end.slice(-suffix.length) === suffix;
    }

    // We assume that everything being matched is at most a single token plus some whitespace,
    // which everything currently is, but otherwise we'd have to expand _last or check _buf.
    return false;
  }

  hasContent(): boolean {
    return this._queue.length > 0 || Boolean(this.last);
  }

  /**
   * Sets a given position as the current source location so generated code after this call
   * will be given this position in the sourcemap.
   */

  source(prop: string, loc: undefined | SourceLocation): void {
    if (prop && !loc) {
      return undefined;
    }

    // @ts-ignore
    const pos = loc ? loc[prop] : undefined;

    this.sourcePosition.identifierName =
      (loc && loc.identifierName) || undefined;
    this.sourcePosition.line = pos ? pos.line : undefined;
    this.sourcePosition.column = pos ? pos.column : undefined;
    this.sourcePosition.filename = (loc && loc.filename) || undefined;
  }

  /**
   * Call a callback with a specific source location and restore on completion.
   */
  withSource(
    prop: string,
    loc: undefined | SourceLocation,
    cb: () => void,
  ): void {
    // Use the call stack to manage a stack of "source location" data.
    const originalLine = this.sourcePosition.line;
    const originalColumn = this.sourcePosition.column;
    const originalFilename = this.sourcePosition.filename;
    const originalIdentifierName = this.sourcePosition.identifierName;

    this.source(prop, loc);

    cb();

    this.sourcePosition.line = originalLine;
    this.sourcePosition.column = originalColumn;
    this.sourcePosition.filename = originalFilename;
    this.sourcePosition.identifierName = originalIdentifierName;
  }

  getCurrentColumn(): Number0 {
    const extra = this._queue.reduce((acc, item) => item[0] + acc, '');
    const lastIndex = extra.lastIndexOf('\n');

    if (lastIndex === -1) {
      return add(this.position.column, extra.length);
    } else {
      return coerce0(extra.length - 1 - lastIndex);
    }
  }

  getCurrentLine(): Number1 {
    const extra = this._queue.reduce((acc, item) => item[0] + acc, '');

    let count = 0;
    for (let i = 0; i < extra.length; i++) {
      if (extra[i] === '\n') {
        count++;
      }
    }

    return add(this.position.line, count);
  }
}
