/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Event} from '@romejs/events';
import {isPlainObject, Dict} from '@romejs/typescript-helpers';
import os = require('os');

// BSER uses the local endianness to reduce byte swapping overheads
// (the protocol is expressly local IPC only).  We need to tell node
// to use the native endianness when reading various native values.
const isBigEndian = os.endianness() == 'BE';

type Bufferish = Buffer | string;

// Find the next power-of-2 >= size
function nextPow2(size: number): number {
  return Math.pow(2, Math.ceil(Math.log(size) / Math.LN2));
}

// Expandable buffer that we can provide a size hint for
export class Accumulator {
  constructor(initsize: number = 8_192) {
    this.buffer = Buffer.alloc(nextPow2(initsize));
    this.readOffset = 0;
    this.writeOffset = 0;
  }

  buffer: Buffer;
  readOffset: number;
  writeOffset: number;

  canRead(size: number): boolean {
    return this.readAvail() > size;
  }

  // How much we can write into this buffer without allocating
  writeAvail(): number {
    return this.buffer.length - this.writeOffset;
  }

  // How much we can read
  readAvail(): number {
    return this.writeOffset - this.readOffset;
  }

  // Ensure that we have enough space for size bytes
  reserve(size: number) {
    if (size < this.writeAvail()) {
      return;
    }

    // If we can make room by shunting down, do so
    if (this.readOffset > 0) {
      this.buffer.copy(this.buffer, 0, this.readOffset, this.writeOffset);
      this.writeOffset -= this.readOffset;
      this.readOffset = 0;
    }

    // If we made enough room, no need to allocate more
    if (size < this.writeAvail()) {
      return;
    }

    // Allocate a replacement and copy it in
    const buf = Buffer.alloc(nextPow2(this.buffer.length + size -
    this.writeAvail()));
    this.buffer.copy(buf);
    this.buffer = buf;
  }

  // Append buffer or string.  Will resize as needed
  append(buf: Bufferish) {
    if (Buffer.isBuffer(buf)) {
      this.reserve(buf.length);
      buf.copy(this.buffer, this.writeOffset, 0, buf.length);
      this.writeOffset += buf.length;
    } else {
      const size = Buffer.byteLength(buf);
      this.reserve(size);
      this.buffer.write(buf, this.writeOffset);
      this.writeOffset += size;
    }
  }

  assertReadableSize(size: number) {
    if (this.readAvail() < size) {
      throw new Error(
        `wanted to read ${size} bytes but only have ${this.readAvail()}`,
      );
    }
  }

  peekString(size: number): string {
    this.assertReadableSize(size);
    return this.buffer.toString('utf-8', this.readOffset, this.readOffset + size);
  }

  readString(size: number): string {
    const str = this.peekString(size);
    this.readOffset += size;
    return str;
  }

  peekInt(size: number): number {
    this.assertReadableSize(size);

    switch (size) {
      case 1:
        return this.buffer.readInt8(this.readOffset);

      case 2:
        return isBigEndian
          ? this.buffer.readInt16BE(this.readOffset) : this.buffer.readInt16LE(
            this.readOffset,
          );

      case 4:
        return isBigEndian
          ? this.buffer.readInt32BE(this.readOffset) : this.buffer.readInt32LE(
            this.readOffset,
          );

      case 8:
        throw new Error('64-bit numbers aren\'t supported');

      default:
        throw new Error(`invalid integer size ${size}`);
    }
  }

  readInt(bytes: number): number {
    const ival = this.peekInt(bytes);
    this.readOffset += bytes;
    return ival;
  }

  peekDouble(): number {
    this.assertReadableSize(8);
    return isBigEndian
      ? this.buffer.readDoubleBE(this.readOffset) : this.buffer.readDoubleLE(
        this.readOffset,
      );
  }

  readDouble(): number {
    const dval = this.peekDouble();
    this.readOffset += 8;
    return dval;
  }

  readAdvance(size: number) {
    if (size > 0) {
      this.assertReadableSize(size);
    } else if (size < 0 && this.readOffset + size < 0) {
      throw new Error(
        `advance with negative offset ${size} would seek off the start of the buffer`,
      );
    }

    this.readOffset += size;
  }

  writeByte(value: number) {
    this.reserve(1);
    this.buffer.writeInt8(value, this.writeOffset);
    ++this.writeOffset;
  }

  writeInt(value: number, size: number) {
    this.reserve(size);
    switch (size) {
      case 1:
        this.buffer.writeInt8(value, this.writeOffset);
        break;

      case 2:
        if (isBigEndian) {
          this.buffer.writeInt16BE(value, this.writeOffset);
        } else {
          this.buffer.writeInt16LE(value, this.writeOffset);
        }
        break;

      case 4:
        if (isBigEndian) {
          this.buffer.writeInt32BE(value, this.writeOffset);
        } else {
          this.buffer.writeInt32LE(value, this.writeOffset);
        }
        break;

      default:
        throw new Error(`unsupported integer size ${size}`);
    }
    this.writeOffset += size;
  }

  writeDouble(value: number) {
    this.reserve(8);
    if (isBigEndian) {
      this.buffer.writeDoubleBE(value, this.writeOffset);
    } else {
      this.buffer.writeDoubleLE(value, this.writeOffset);
    }
    this.writeOffset += 8;
  }
}

const BSER_ARRAY = 0;
const BSER_OBJECT = 1;
const BSER_STRING = 2;
const BSER_INT8 = 3;
const BSER_INT16 = 4;
const BSER_INT32 = 5;
const BSER_INT64 = 6;
const BSER_REAL = 7;
const BSER_TRUE = 8;
const BSER_FALSE = 9;
const BSER_NULL = 10;
const BSER_TEMPLATE = 11;
const BSER_SKIP = 12;

const ST_NEED_PDU = 0; // Need to read and decode PDU length
const ST_FILL_PDU = 1; // Know the length, need to read whole content
const MAX_INT8 = 127;
const MAX_INT16 = 32_767;
const MAX_INT32 = 2_147_483_647;

export class BunserBuf {
  constructor() {
    this.acc = new Accumulator();
    this.state = ST_NEED_PDU;

    this.errorEvent = new Event({
      name: 'BunserBuf.error',
    });

    this.valueEvent = new Event({
      name: 'BunserBuf.value',
    });

    this.pduLen = false;
  }

  pduLen: number | boolean;
  errorEvent: Event<Error, void>;
  valueEvent: Event<unknown, void>;
  acc: Accumulator;
  state: 0 | 1;

  append(buf: Bufferish, synchronous: boolean = false) {
    if (synchronous) {
      this.acc.append(buf);
      return this.process(synchronous);
    }

    try {
      this.acc.append(buf);
    } catch (err) {
      this.errorEvent.send(err);
      return;
    }

    // Arrange to decode later.  This allows the consuming

    // application to make progress with other work in the

    // case that we have a lot of subscription updates coming

    // in from a large tree.
    this.processLater();
  }

  processLater() {
    process.nextTick(() => {
      try {
        this.process(false);
      } catch (err) {
        this.errorEvent.send(err);
      }
    });
  }

  // Do something with the buffer to advance our state.

  // If we're running synchronously we'll return either

  // the value we've decoded or undefined if we don't

  // yet have enought data.

  // If we're running asynchronously, we'll emit the value

  // when it becomes ready and schedule another invocation

  // of process on the next tick if we still have data we

  // can process.
  process(synchronous: boolean) {
    if (this.state == ST_NEED_PDU) {
      if (this.acc.readAvail() < 2) {
        return;
      }

      // Validate BSER header
      this.expectCode(0);
      this.expectCode(1);
      this.pduLen = this.decodePDUInt();
      if (this.pduLen === false) {
        // Need more data, walk backwards
        this.acc.readAdvance(-2);
        return;
      }

      // Ensure that we have a big enough buffer to read the rest of the PDU
      this.acc.reserve(this.pduLen);
      this.state = ST_FILL_PDU;
    }

    if (this.state == ST_FILL_PDU) {
      if (this.acc.readAvail() < this.pduLen) {
        // Need more data
        return;
      }

      // We have enough to decode it
      const val = this.decodeAny();
      if (synchronous) {
        return val;
      }
      this.valueEvent.send(val);
      this.state = ST_NEED_PDU;
    }

    if (!synchronous && this.acc.readAvail() > 0) {
      this.processLater();
    }
  }

  raise(reason: string): never {
    const bufferLength = this.acc.buffer.length;
    const readableLength = this.acc.readAvail();
    const readOffset = this.acc.readOffset;
    const buffer = JSON.stringify(this.acc.buffer.slice(
      this.acc.readOffset,
      this.acc.readOffset + 32,
    ).toJSON());

    throw new Error(
      `${reason} in Buffer of length ${bufferLength}, ${readableLength} readable at offset ${readOffset} buffer: ${buffer}`,
    );
  }

  expectCode(expected: number): void {
    const code = this.acc.readInt(1);
    if (code != expected) {
      this.raise(`Expected bser opcode ${expected} but got ${code}`);
    }
  }

  decodeAny(): unknown {
    const code = this.acc.peekInt(1);
    switch (code) {
      case BSER_INT8:
      case BSER_INT16:
      case BSER_INT32:
      case BSER_INT64:
        return this.decodeInt();

      case BSER_REAL:
        this.acc.readAdvance(1);
        return this.acc.readDouble();

      case BSER_TRUE:
        this.acc.readAdvance(1);
        return true;

      case BSER_FALSE:
        this.acc.readAdvance(1);
        return false;

      case BSER_NULL:
        this.acc.readAdvance(1);
        return null;

      case BSER_STRING:
        return this.decodeString();

      case BSER_ARRAY:
        return this.decodeArray();

      case BSER_OBJECT:
        return this.decodeObject();

      case BSER_TEMPLATE:
        return this.decodeTemplate();

      default:
        this.raise(`Unhandled bser opcode ${code}`);
    }
  }

  decodeArray(): Array<unknown> {
    this.expectCode(BSER_ARRAY);
    const nitems = this.decodeInt();
    const arr: Array<unknown> = [];
    for (let i = 0; i < nitems; ++i) {
      arr.push(this.decodeAny());
    }
    return arr;
  }

  decodeObject(): Dict<unknown> {
    this.expectCode(BSER_OBJECT);
    const nitems = this.decodeInt();
    const res: Dict<unknown> = {};
    for (let i = 0; i < nitems; ++i) {
      const key = this.decodeString();
      const val = this.decodeAny();
      res[key] = val;
    }
    return res;
  }

  decodeTemplate(): Array<unknown> {
    this.expectCode(BSER_TEMPLATE);
    const keys = this.decodeArray();
    const nitems = this.decodeInt();
    const arr: Array<unknown> = [];
    for (let i = 0; i < nitems; ++i) {
      const obj: Dict<unknown> = {};
      for (let keyidx = 0; keyidx < keys.length; ++keyidx) {
        if (this.acc.peekInt(1) == BSER_SKIP) {
          this.acc.readAdvance(1);
          continue;
        }
        const val = this.decodeAny();
        obj[String(keys[keyidx])] = val;
      }
      arr.push(obj);
    }
    return arr;
  }

  decodeString() {
    this.expectCode(BSER_STRING);
    const len = this.decodeInt();
    return this.acc.readString(len);
  }

  // This is unusual compared to the other decode functions in that

  // we may not have enough data available to satisfy the read, and

  // we don't want to throw.
  decodePDUInt(): false | number {
    if (this.acc.canRead(1)) {
      const size = this.getDecodeIntSize();
      if (this.acc.canRead(1 + size)) {
        return this.decodeInt();
      }
    }

    return false;
  }

  getDecodeIntSize(): number {
    let size = 0;

    const code = this.acc.peekInt(1);
    switch (code) {
      case BSER_INT8:
        size = 1;
        break;

      case BSER_INT16:
        size = 2;
        break;

      case BSER_INT32:
        size = 4;
        break;

      case BSER_INT64:
        size = 8;
        break;

      default:
        throw this.raise(`invalid bser int encoding ${code}`);
    }

    return size;
  }

  decodeInt(): number {
    this.acc.assertReadableSize(1);
    const size = this.getDecodeIntSize();
    this.acc.readAdvance(1);
    return this.acc.readInt(size);
  }
}

// synchronously BSER decode a string and return the value
export function loadFromBuffer(input: Bufferish): unknown {
  const buf = new BunserBuf();
  const result = buf.append(input, true);

  if (buf.acc.readAvail()) {
    throw Error('Excess data found after input buffer, use BunserBuf instead');
  }

  if (typeof result === 'undefined') {
    throw Error('No bser found in string and no error raised!?');
  }

  return result;
}

function dumpInt(buf: Accumulator, val: number) {
  const abs = Math.abs(val);
  if (abs <= MAX_INT8) {
    buf.writeByte(BSER_INT8);
    buf.writeInt(val, 1);
  } else if (abs <= MAX_INT16) {
    buf.writeByte(BSER_INT16);
    buf.writeInt(val, 2);
  } else if (abs <= MAX_INT32) {
    buf.writeByte(BSER_INT32);
    buf.writeInt(val, 4);
  } else {
    throw new Error('???');
  }
}

function dumpArray(buf: Accumulator, val: Array<unknown>) {
  buf.writeByte(BSER_ARRAY);
  dumpInt(buf, val.length);
  for (let i = 0; i < val.length; ++i) {
    dumpUnknown(buf, val[i]);
  }
}

function dumpObject(buf: Accumulator, val: object | null) {
  if (val === null) {
    buf.writeByte(BSER_NULL);
    return;
  }

  if (Array.isArray(val)) {
    dumpArray(buf, val);
    return;
  }

  if (!isPlainObject(val)) {
    throw new Error('Expected a plain object');
  }

  buf.writeByte(BSER_OBJECT);

  const keys = Object.keys(val);

  // First pass to compute number of defined keys
  let num_keys = keys.length;
  for (let i = 0; i < keys.length; ++i) {
    const key = keys[i];
    const v = val[key];
    if (typeof v === 'undefined') {
      num_keys--;
    }
  }

  dumpInt(buf, num_keys);

  for (let i = 0; i < keys.length; ++i) {
    const key = keys[i];
    const v = val[key];
    if (typeof v === 'undefined') {
      // Don't include it
      continue;
    }

    dumpUnknown(buf, key);
    try {
      dumpUnknown(buf, v);
    } catch (err) {
      throw new Error(
        `${err.message}  (while serializing object property with name ${key}`,
      );
    }
  }
}

function dumpUnknown(buf: Accumulator, val: unknown) {
  switch (typeof val) {
    case 'number':
      // check if it is an integer or a float
      if (isFinite(val) && Math.floor(val) === val) {
        dumpInt(buf, val);
      } else {
        buf.writeByte(BSER_REAL);
        buf.writeDouble(val);
      }
      return;

    case 'bigint':
      throw new Error('bigint isn\'t supported yet');

    case 'string':
      buf.writeByte(BSER_STRING);
      dumpInt(buf, Buffer.byteLength(val));
      buf.append(val);
      return;

    case 'boolean':
      buf.writeByte(val ? BSER_TRUE : BSER_FALSE);
      return;

    case 'object':
      dumpObject(buf, val);
      return;

    default:
      throw new Error(`Cannot serialize type ${typeof val} to BSER`);
  }
}

// BSER encode value and return a buffer of the contents
export function dumpToBuffer(val: unknown): Buffer {
  const buf = new Accumulator();
  // Build out the header
  buf.writeByte(0);
  buf.writeByte(1);
  // Reserve room for an int32 to hold our PDU length
  buf.writeByte(BSER_INT32);
  buf.writeInt(0, 4); // We'll come back and fill this in at the end
  dumpUnknown(buf, val);

  // Compute PDU length
  const off = buf.writeOffset;
  const len = off - 7 /* the header length */;
  buf.writeOffset = 3; // The length value to fill in
  buf.writeInt(len, 4); // write the length in the space we reserved
  buf.writeOffset = off;

  return buf.buffer.slice(0, off);
}
