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
  dec,
} from '@romejs/ob1';

export type BufferSnapshot = {
  mappingsIndex: number;
  lastGenLine: undefined | Number1;
  lastSourceLine: undefined | Number1;
  lastSourceColumn: undefined | Number0;
  lineLengthsIndex: number;
  bufIndex: number;
  last: string;
  position: Position;
  sourcePosition: {
    identifierName: undefined | string;
    line: undefined | Number1;
    column: undefined | Number0;
    filename: undefined | string;
  };
};

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
    this.inputSourceMap = opts.inputSourceMap === undefined
      ? undefined : new SourceMapConsumer(opts.inputSourceMap);

    this.lineLengths = [];
    this.buf = [];
    this.last = '';

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
  lineLengths: Array<Number0>;

  buf: Array<string>;
  last: string;

  position: Position;
  sourcePosition: {
    identifierName: undefined | string;
    line: undefined | Number1;
    column: undefined | Number0;
    filename: undefined | string;
  };

  save(): BufferSnapshot {
    return {
      mappingsIndex: this.mappings.length,
      lastGenLine: this.lastGenLine,
      lastSourceLine: this.lastSourceLine,
      lastSourceColumn: this.lastSourceColumn,
      bufIndex: this.buf.length,
      last: this.last,
      position: {...this.position},
      sourcePosition: {...this.sourcePosition},
      lineLengthsIndex: this.lineLengths.length,
    };
  }

  restore(snapshot: BufferSnapshot) {
    this.mappings = this.mappings.slice(0, snapshot.mappingsIndex);
    this.lastGenLine = snapshot.lastGenLine;
    this.lastSourceLine = snapshot.lastSourceLine;
    this.lastSourceColumn = snapshot.lastSourceColumn;
    this.buf = this.buf.slice(0, snapshot.bufIndex);
    this.last = snapshot.last;
    this.position = snapshot.position;
    this.sourcePosition = snapshot.sourcePosition;
    this.lineLengths = this.lineLengths.slice(0, snapshot.lineLengthsIndex);
  }

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
    if (this.lastGenLine === generatedLine && this.lastSourceLine ===
    originalLine && this.lastSourceColumn === originalColumn) {
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
    let code = this.buf.join('').trimRight();

    if (this.opts.format === 'pretty') {
      code += '\n';
    }

    return code;
  }

  append(str: string): void {
    const {column, filename, identifierName, line} = this.sourcePosition;

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

    for (let i = 0;
    i < str.length;
    i++) {
      this.position.index = inc(this.position.index);

      if (str[i] === '\n') {
        this.lineLengths.push(inc(this.position.column));
        this.position.line = inc(this.position.line);
        this.position.column = number0;
      } else {
        this.position.column = inc(this.position.column);
      }
    }
  }

  isEmpty(): boolean {
    // Fast paths
    if (this.buf.length === 0) {
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

    return true;
  }

  removeTrailing(char: string) {
    while (this.endsWith(char)) {
      const i = this.buf.length - 1;
      if (this.buf[i] === char) {
        this.buf.pop();
      } else {
        this.buf[i] = this.buf[i].slice(0, -1);
      }

      if (char === '\n') {
        this.position.line = dec(this.position.line);

        const lastLine = this.lineLengths.pop();
        this.position.column = lastLine || number0;
      } else {
        this.position.column = dec(this.position.column);
      }
    }
  }

  removeTrailingNewlines() {
    this.removeTrailing('\n');
  }

  endsWith(suffix: string): boolean {
    if (this.buf.length === 0) {
      return false;
    }

    let i = this.buf.length - 1;
    let last = this.buf[i];
    while (last.length < suffix.length && i > 0) {
      i--;
      last = this.buf[i] + last;
    }
    return last.endsWith(suffix);
  }

  hasContent(): boolean {
    return this.last !== '';
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

    this.sourcePosition.identifierName = loc && loc.identifierName || undefined;
    this.sourcePosition.line = pos ? pos.line : undefined;
    this.sourcePosition.column = pos ? pos.column : undefined;
    this.sourcePosition.filename = loc && loc.filename || undefined;
  }

  /**
   * Call a callback with a specific source location and restore on completion.
   */
  withSource(prop: string, loc: undefined | SourceLocation, cb: () => void): void {
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
}
