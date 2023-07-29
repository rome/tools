enum ValidString {
  A = 'test',
  B = 'div' + 'ided',
  C = `test2`,
  D = `di` + `ided2`,
  AA = A + ValidString.A,
}

enum ValidNumber {
  A,
  B = 42,
  C = -42,
  D = +42,
  E = 2 + 2,
  F = A + ValidNumber.B,
}

enum ValidQuotedKey {
  'A',
  'B' = 1,
  ['C'],
}

enum ValidFlags {
  A = 1 << 0,
  B = 1 >> 0,
  C = 1 >>> 0,
  D = 1 | 0,
  E = 1 & 0,
  F = 1 ^ 0,
  G = ~1,
}

enum FileAccess {
  None = 0,
  Read = 1,
  Write = 1 << 1,
  All = (1 | (1 << 1)),
}

enum FileAccessWithRef {
  None = 0,
  Read = 1,
  Write = FileAccessWithRef["Read"] << 1,
  All = Read | FileAccessWithRef.Write,
}

enum ValidRef {
  "A",
  "B",
  C = A | B,
}

enum ValidComputedRef {
  ["A"],
  ["B"],
  C = A | B,
}

export {}