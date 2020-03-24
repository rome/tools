/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Inspired by the original Metro ob1 library by Moti Zilberman @motiz88

// These weird number intersections are to create fake opaque types

// https://github.com/Microsoft/TypeScript/issues/15807

// A type representing 0-based offsets.
export type Number0 = {_tag: 'ZERO_OFFSET'};

// A type representing 1-based offsets.
export type Number1 = {_tag: 'ONE_OFFSET'};

type AnyNumber = Number0 | Number1;

export type UnknownNumber = AnyNumber | number;

// Add two offsets or numbers.
export function add(a: Number1, b: number): Number1;
export function add(a: number, b: Number1): Number1;
export function add(a: Number0, b: number): Number0;
export function add(a: number, b: Number0): Number0;
export function add(a: Number1, b: Number0): Number1;
export function add(a: Number0, b: Number1): Number1;
export function add(a: Number0, b: Number0): Number0;
export function add(a: number, b: number): Number0;
export function add(a: number | AnyNumber, b: number | AnyNumber): AnyNumber {
  // @ts-ignore
  return a + b;
}

// Subtract a number or 0-based offset from a 1/0-based offset.
export function sub(a: Number1, b: number): Number1;
export function sub(a: Number0, b: number): Number0;
export function sub(a: number, b: Number0): Number0;
export function sub(a: Number0, b: number): Number0;
export function sub(a: Number1, b: Number0): Number1;
export function sub(a: Number0, b: Number0): Number0;
export function sub(a: Number1, b: Number1): Number1;
export function sub(a: number | AnyNumber, b: number | AnyNumber): AnyNumber {
  // @ts-ignore
  return a - b;
}

// Get the underlying number of a 0-based offset, casting away the opaque type.
export function get0(x: Number0): number;
export function get0(x: undefined): undefined;
export function get0(x: undefined | Number0): undefined | number {
  // @ts-ignore
  return x;
}

// Get the underlying number of a 1-based offset, casting away the opaque type.
export function get1(x: Number1): number;
export function get1(x: undefined): undefined;
export function get1(x: undefined | Number1): undefined | number {
  // @ts-ignore
  return x;
}

// Coerce a number into a 0-offset
export function coerce0(x: number): Number0;
export function coerce0(x: undefined): undefined;
export function coerce0(x: undefined | number): Number0 | undefined;
export function coerce0(x: undefined | number | AnyNumber): Number0 | undefined {
  return (x as Number0);
}

export const number0 = coerce0(0);
export const number0Neg1 = coerce0(-1);

// Coerce a number into a 1-offset
export function coerce1(x: number): Number1;
export function coerce1(x: undefined): undefined;
export function coerce1(x: undefined | number): Number1 | undefined;
export function coerce1(x: undefined | number | AnyNumber): Number1 | undefined {
  return (x as Number1);
}

export const number1 = coerce1(1);
export const number1Neg1 = coerce1(-1);

// Add 1 to a 0-based offset, thus converting it to 1-based.
export function coerce0to1(x: Number0): Number1 {
  // @ts-ignore
  return (x + 1 as Number1);
}

// Subtract 1 from a 1-based offset, thus converting it to 0-based.
export function coerce1to0(x: Number1 | number): Number0 {
  // @ts-ignore
  return (x - 1 as Number0);
}

// Increment
export function inc(a: Number0): Number0;
export function inc(a: Number1): Number1;
export function inc(x: AnyNumber): AnyNumber {
  // @ts-ignore
  return x + 1;
}

// Decrement
export function dec(a: Number0): Number0;
export function dec(a: Number1): Number1;
export function dec(x: AnyNumber): AnyNumber {
  // @ts-ignore
  return x - 1;
}
