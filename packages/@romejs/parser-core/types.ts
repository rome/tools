/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, Number1, ob1Coerce1, ob1Number0Neg1} from '@romejs/ob1';

//# Node types
export type NodeBase = {
  type: string;
  loc?: SourceLocation;
};

export type SimpleNode<Type extends string> = NodeBase & {type: Type};

export type ComplexNode<Type extends string, Data> = NodeBase & Data & {
  type: Type;
};

export type ValueNode<Type extends string, Value> = NodeBase & {
  type: Type;
  value: Value;
};

//# Token types
export type TokenBase = {
  type: string;
  start: Number0;
  end: Number0;
};

export type TokensShape = {
  Invalid: InvalidToken;
  EOF: EOFToken;
  SOF: SOFToken;
  [type: string]: TokenBase;
};

export type SimpleToken<Type extends string> = TokenBase & {type: Type};

export type ComplexToken<Type extends string, Data> = TokenBase & Data & {
  type: Type;
};

export type ValueToken<Type extends string, Value> = TokenBase & {
  type: Type;
  value: Value;
};

export type EOFToken = SimpleToken<'EOF'>;

export type SOFToken = SimpleToken<'SOF'>;

export type InvalidToken = SimpleToken<'Invalid'>;

export type BaseTokens = {
  Invalid: InvalidToken;
  EOF: EOFToken;
  SOF: SOFToken;
};

//# Other types
export type SourceLocation = {
  filename?: string;
  identifierName?: string;
  start: Position;
  end: Position;
};

export type Position = {
  index: Number0;
  line: Number1;
  column: Number0;
};

export const UNKNOWN_POSITION: Position = {
  index: ob1Number0Neg1,
  line: ob1Coerce1(-1),
  column: ob1Number0Neg1,
};
