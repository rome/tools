/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, Number1, ob1Inc, ob1Number0, ob1Number1} from '@romejs/ob1';

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
      this.buffer = [];
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

  merge(other: StringBuffer): void {
    this.buffer.push(...other.buffer);
    this.index = other.index;
    this.column = other.column;
    this.line = other.line;
  }

  toString(): string {
    return this.buffer.join('');
  }
}
