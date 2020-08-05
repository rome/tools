/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/* -*- Mode: js; js-indent-level: 2; -*- */
/*
 * Copyright 2011 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 *
 * Based on the Base 64 VLQ implementation in Closure Compiler:
 * https://code.google.com/p/closure-compiler/source/browse/trunk/src/com/google/debugging/sourcemap/Base64VLQ.java
 *
 * Copyright 2011 The Closure Compiler Authors. All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials provided
 *    with the distribution.
 *  * Neither the name of Google Inc. nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from 'this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

const intToCharMap = Array.from(
	"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
);

/**
 * Encode an integer in the range of 0 to 63 to a single base 64 digit.
 */
export function encode(number: number): string {
	if (0 <= number && number < intToCharMap.length) {
		return intToCharMap[number];
	} else {
		throw new TypeError(`Must be between 0 and 63: ${number}`);
	}
}

// A single base 64 digit can contain 6 bits of data. For the base 64 variable
// length quantities we use in the source map spec, the first bit is the sign,
// the next four bits are the actual value, and the 6th bit is the
// continuation bit. The continuation bit tells us whether there are more
// digits in this value following this digit.
//
//   Continuation
//   |    Sign
//   |    |
//   V    V
//   101011
const VLQ_BASE_SHIFT = 5;

// binary: 100000
const VLQ_BASE = 1 << VLQ_BASE_SHIFT;

// binary: 011111
const VLQ_BASE_MASK = VLQ_BASE - 1;

// binary: 100000
const VLQ_CONTINUATION_BIT = VLQ_BASE;

/**
 * Converts from 'a two-complement value to a value where the sign bit is
 * placed in the least significant bit.  For example, as decimals:
 *   1 becomes 2 (10 binary), -1 becomes 3 (11 binary)
 *   2 becomes 4 (100 binary), -2 becomes 5 (101 binary)
 */
function toVLQSigned(aValue: number): number {
	return aValue < 0 ? (-aValue << 1) + 1 : aValue << 1;
}

/**
 * Converts to a two-complement value from 'a value where the sign bit is
 * placed in the least significant bit.  For example, as decimals:
 *   2 (10 binary) becomes 1, 3 (11 binary) becomes -1
 *   4 (100 binary) becomes 2, 5 (101 binary) becomes -2
 */
// eslint-disable-next-line no-unused-vars
function fromVLQSigned(value: number): number {
	const isNegative = (value & 1) === 1;
	const shifted = value >> 1;
	return isNegative ? -shifted : shifted;
}

/**
 * Returns the base 64 VLQ encoded value.
 */
export function encodeVLQ(value: number): string {
	let encoded: string = "";
	let vlq = toVLQSigned(value);

	do {
		let digit: number = vlq & VLQ_BASE_MASK;
		vlq >>>= VLQ_BASE_SHIFT;
		if (vlq > 0) {
			// There are still more digits in this value, so we must make sure the
			// continuation bit is marked.
			digit |= VLQ_CONTINUATION_BIT;
		}
		encoded += encode(digit);
	} while (vlq > 0);

	return encoded;
}

/**
 * Decode a single base 64 character code digit to an integer. Returns -1 on
 * failure.
 */
export function decode(charCode: number): number {
	const uppercaseA = 65; // 'A'
	const uppercaseZ = 90; // 'Z'
	const lowercaseA = 97; // 'a'
	const lowercaseZ = 122; // 'z'
	const zero = 48; // '0'
	const nine = 57; // '9'
	const plus = 43; // '+'
	const slash = 47; // '/'
	const lowercaseOffset = 26;
	const numberOffset = 52;

	// 0 - 25: ABCDEFGHIJKLMNOPQRSTUVWXYZ
	if (uppercaseA <= charCode && charCode <= uppercaseZ) {
		return charCode - uppercaseA;
	}

	// 26 - 51: abcdefghijklmnopqrstuvwxyz
	if (lowercaseA <= charCode && charCode <= lowercaseZ) {
		return charCode - lowercaseA + lowercaseOffset;
	}

	// 52 - 61: 0123456789
	if (zero <= charCode && charCode <= nine) {
		return charCode - zero + numberOffset;
	}

	// 62: +
	if (charCode === plus) {
		return 62;
	}

	// 63: /
	if (charCode === slash) {
		return 63;
	}

	// Invalid base64 digit.
	return -1;
}

export function decodeVLQ(aStr: string, aIndex: number): [number, number] {
	let strLen = aStr.length;
	let result = 0;
	let shift = 0;
	let continuation = false;

	do {
		if (aIndex >= strLen) {
			throw new Error("Expected more digits in base 64 VLQ value.");
		}

		let digit = decode(aStr.charCodeAt(aIndex++));
		if (digit === -1) {
			throw new Error(`Invalid base64 digit: ${aStr.charAt(aIndex - 1)}`);
		}

		continuation = !!(digit & VLQ_CONTINUATION_BIT);
		digit &= VLQ_BASE_MASK;
		result = result + (digit << shift);
		shift += VLQ_BASE_SHIFT;
	} while (continuation);

	return [fromVLQSigned(result), aIndex];
}
