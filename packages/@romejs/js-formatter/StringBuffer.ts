/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Number0,
  Number1,
  ob1Inc,
  ob1Number0,
  ob1Number1,
  ob1Sub,
} from '@romejs/ob1';

export default class StringBuffer {
  static from(parent: StringBuffer): StringBuffer {
    return new StringBuffer(parent);
  }

  constructor(parent?: StringBuffer) {
    if (parent === undefined) {
      this.buffer = [];
      this.index = ob1Number0;
      this.column = ob1Number0;
      this.line = ob1Number1;
    } else {
      // FIXME: This is a aweful for memory usage.
      // It must be replaced by a better algorithm.
      this.buffer = parent.buffer.slice();
      this.index = parent.index;
      this.column = parent.column;
      this.line = parent.line;
    }
  }

  buffer: Array<string>;
  index: Number0;
  column: Number0;
  line: Number1;

  push(segment: string): void {
    for (const ch of segment) {
      this.index = ob1Inc(this.index);
      if (ch === '\n') {
        this.line = ob1Inc(this.line);
        this.column = ob1Number0;
      } else {
        this.column = ob1Inc(this.column);
      }
    }

    this.buffer.push(segment);
  }

  trim(): void {
    let trimmed = 0;

    while (this.buffer.length > 0) {
      const segment = this.buffer[this.buffer.length - 1];
      if (/^ *$/.test(segment)) {
        this.buffer.pop();
        trimmed += segment.length;
      } else {
        break;
      }
    }

    if (trimmed > 0) {
      this.index = ob1Sub(this.index, trimmed);
      this.column = ob1Sub(this.column, trimmed);
    }
  }

  merge(other: StringBuffer): void {
    this.buffer = other.buffer;
    this.index = other.index;
    this.column = other.column;
    this.line = other.line;
  }

  toString(): string {
    return this.buffer.join('');
  }
}
