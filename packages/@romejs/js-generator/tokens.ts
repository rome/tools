/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

type WordToken = {
  type: 'Word';
  value: string;
};

type NumberToken = {
  type: 'Number';
  value: string;
};

type SpaceToken = {
  type: 'Space';
  optional: boolean;
};

type NewlineToken = {
  type: 'Newline';
};

type DerivedNewlineToken = {
  type: 'DerivedNewline';
  id: number;
};

type OperatorToken = {
  type: 'Operator';
  value: string;
};

type VerbatimToken = {
  type: 'Verbatim';
  value: string;
};

type IndentToken = {
  type: 'Indent';
  tokens: Tokens;
};

export type GroupToken = {
  type: 'Group';
  priority?: boolean;
  breakOnNewline?: boolean;
  unbroken: {
    leading?: Tokens;
    separator?: Tokens;
    trailing?: Tokens;
    trim?: string;
  };
  broken: {
    indentNewline?: boolean;
    indent?: boolean;
    force?: boolean;
    leading?: Tokens;
    separator?: Tokens;
    trailing?: Tokens;
    before?: Tokens;
    after?: Tokens;
  };
  groups: Array<Tokens | {
    tokens: Tokens;
    afterBroken?: Tokens;
    afterUnbroken?: Tokens;
  }>;
};

export type LinkedGroupsToken = {
  type: 'LinkedGroups';
  tokens: Tokens;
};

export type TerminatorlessToken = {
  type: 'Terminatorless';
  tokens: Tokens;
};

export type CommentToken = {
  type: 'Comment';
  value: string;
};

export type Token =
  | GroupToken
  | IndentToken
  | VerbatimToken
  | OperatorToken
  | LinkedGroupsToken
  | NewlineToken
  | SpaceToken
  | NumberToken
  | DerivedNewlineToken
  | TerminatorlessToken
  | CommentToken
  | WordToken;

export type Tokens = Array<Token>;

export const newline: NewlineToken = {
  type: 'Newline',
};

export const space: SpaceToken = {
  type: 'Space',
  optional: true,
};

export const requiredSpace: SpaceToken = {
  type: 'Space',
  optional: false,
};

export function terminatorless(tokens: Tokens): TerminatorlessToken {
  return {
    type: 'Terminatorless',
    tokens,
  };
}

export function breakGroup(
  groups: Array<Tokens>,
  priority: boolean = false,
): GroupToken {
  return group(groups, {
    priority,
    broken: {
      indentNewline: false,
      separator: [newline],
    },
    unbroken: {
      separator: [space],
    },
  });
}

export function group(
  groups: GroupToken['groups'],
  extra: Omit<GroupToken, 'type' | 'groups'>,
): GroupToken {
  return {
    type: 'Group',
    groups,
    ...extra,
  };
}

export function linkedGroups(tokens: Tokens): LinkedGroupsToken {
  return {
    type: 'LinkedGroups',
    tokens,
  };
}

export function derivedNewline(id: number): DerivedNewlineToken {
  return {
    type: 'DerivedNewline',
    id,
  };
}

export function comment(value: string): CommentToken {
  return {
    type: 'Comment',
    value,
  };
}

export function verbatim(value: string): VerbatimToken {
  return {
    type: 'Verbatim',
    value,
  };
}

export function operator(value: string): OperatorToken {
  return {
    type: 'Operator',
    value,
  };
}

export function word(value: string): WordToken {
  return {
    type: 'Word',
    value,
  };
}

export function number(value: string): NumberToken {
  return {
    type: 'Number',
    value,
  };
}

export function indent(tokens: Tokens): IndentToken {
  return {
    type: 'Indent',
    tokens,
  };
}

export function flatten(arr: Array<Tokens>): Tokens {
  let tokens: Tokens = [];

  for (const elem of arr) {
    tokens = tokens.concat(elem);
  }

  return tokens;
}
