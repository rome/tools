# `parse.ts`

## `parse`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 1`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array ['prerelease']
  loc: Object {
    filename: undefined
    end: Object {
      column: 16
      index: 16
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 10`

```
VersionRange {
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Wildcard {
    loc: Object {
      filename: undefined
      end: Object {
        column: 3
        index: 2
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
  }
  right: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 2
    patch: 3
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 9
        line: 1
      }
      start: Object {
        column: 5
        index: 4
        line: 1
      }
    }
  }
}
```

## `parse: 11`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 6
      index: 6
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 12`

```
Wildcard {
  loc: Object {
    filename: undefined
    end: Object {
      column: 2
      index: 2
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 13`

```
Wildcard {
  loc: Object {
    filename: undefined
    end: Object {
      column: 0
      index: 0
      line: 1
    }
    start: Object {
      column: 0
      index: 0
      line: 1
    }
  }
}
```

## `parse: 14`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array ['prerelease']
  loc: Object {
    filename: undefined
    end: Object {
      column: 15
      index: 15
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 15`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 1
      index: 1
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 16`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: 2
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 17`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 18`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: 3
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 19`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: 2
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 2`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array [
    'pre'
    2
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 20`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 21`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: 3
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 22`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: 2
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 23`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 24`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: undefined
  patch: 3
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 25`

```
WildcardVersion {
  build: Array []
  major: 1
  minor: 2
  patch: undefined
  prerelease: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 26`

```
VersionRange {
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 2
    patch: 3
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 6
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
  }
  right: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 2
    patch: 4
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 13
        index: 13
        line: 1
      }
      start: Object {
        column: 9
        index: 8
        line: 1
      }
    }
  }
}
```

## `parse: 27`

```
LogicalOr {
  loc: Object {
    filename: undefined
    end: Object {
      column: 8
      index: 8
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: WildcardVersion {
    build: Array []
    major: 1
    minor: 2
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 4
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
  }
  right: WildcardVersion {
    build: Array []
    major: 3
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 8
        index: 8
        line: 1
      }
      start: Object {
        column: 8
        index: 7
        line: 1
      }
    }
  }
}
```

## `parse: 28`

```
LogicalOr {
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 3
        index: 2
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
  }
  right: LogicalOr {
    loc: Object {
      filename: undefined
      end: Object {
        column: 11
        index: 11
        line: 1
      }
      start: Object {
        column: 6
        index: 5
        line: 1
      }
    }
    left: WildcardVersion {
      build: Array []
      major: 2
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 7
          line: 1
        }
        start: Object {
          column: 6
          index: 5
          line: 1
        }
      }
    }
    right: WildcardVersion {
      build: Array []
      major: 3
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 11
          line: 1
        }
        start: Object {
          column: 11
          index: 10
          line: 1
        }
      }
    }
  }
}
```

## `parse: 29`

```
Comparator {
  operator: '>='
  loc: Object {
    filename: undefined
    end: Object {
      column: 7
      index: 7
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 7
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 3`

```
AbsoluteVersion {
  build: Array []
  major: 1
  minor: 2
  patch: 3
  prerelease: Array [
    'pre'
    2
    3
    4
    5
    'foo'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 21
      index: 21
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 30`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 15
      index: 15
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 8
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 8
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 15
        index: 15
        line: 1
      }
      start: Object {
        column: 9
        index: 8
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 15
          index: 15
          line: 1
        }
        start: Object {
          column: 11
          index: 10
          line: 1
        }
      }
    }
  }
}
```

## `parse: 31`

```
Comparator {
  operator: '<='
  loc: Object {
    filename: undefined
    end: Object {
      column: 7
      index: 7
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 7
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 32`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 15
      index: 15
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 8
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 8
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 15
        index: 15
        line: 1
      }
      start: Object {
        column: 9
        index: 8
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 15
          index: 15
          line: 1
        }
        start: Object {
          column: 11
          index: 10
          line: 1
        }
      }
    }
  }
}
```

## `parse: 33`

```
Comparator {
  operator: '>'
  loc: Object {
    filename: undefined
    end: Object {
      column: 6
      index: 6
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 6
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 34`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 8
        index: 7
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 7
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 13
        index: 13
        line: 1
      }
      start: Object {
        column: 8
        index: 7
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 35`

```
Comparator {
  operator: '<'
  loc: Object {
    filename: undefined
    end: Object {
      column: 6
      index: 6
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 6
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 36`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 8
        index: 7
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 7
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 13
        index: 13
        line: 1
      }
      start: Object {
        column: 8
        index: 7
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 37`

```
Comparator {
  operator: '^'
  loc: Object {
    filename: undefined
    end: Object {
      column: 6
      index: 6
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 6
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 38`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 8
        index: 7
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 7
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 13
        index: 13
        line: 1
      }
      start: Object {
        column: 8
        index: 7
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 39`

```
Comparator {
  operator: '~'
  loc: Object {
    filename: undefined
    end: Object {
      column: 6
      index: 6
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 4
    patch: 5
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 6
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 4`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  prerelease: Array []
  build: Array ['build']
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 40`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 8
        index: 7
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 7
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 13
        index: 13
        line: 1
      }
      start: Object {
        column: 8
        index: 7
        line: 1
      }
    }
    version: AbsoluteVersion {
      build: Array []
      major: 1
      minor: 4
      patch: 5
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 41`

```
Comparator {
  operator: '>='
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 42`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 6
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 7
          index: 6
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 11
        index: 11
        line: 1
      }
      start: Object {
        column: 7
        index: 6
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 11
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 43`

```
Comparator {
  operator: '<='
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 44`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 6
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 7
          index: 6
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 11
        index: 11
        line: 1
      }
      start: Object {
        column: 7
        index: 6
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 11
          line: 1
        }
        start: Object {
          column: 9
          index: 8
          line: 1
        }
      }
    }
  }
}
```

## `parse: 45`

```
Comparator {
  operator: '>'
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 4
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 46`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 5
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 6
          index: 5
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 9
        line: 1
      }
      start: Object {
        column: 6
        index: 5
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 9
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 47`

```
Comparator {
  operator: '<'
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 4
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 48`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 5
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 6
          index: 5
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 9
        line: 1
      }
      start: Object {
        column: 6
        index: 5
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 9
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 49`

```
Comparator {
  operator: '^'
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 4
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 5`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  build: Array ['build']
  prerelease: Array ['prerelease']
  loc: Object {
    filename: undefined
    end: Object {
      column: 22
      index: 22
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 50`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 5
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 6
          index: 5
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 9
        line: 1
      }
      start: Object {
        column: 6
        index: 5
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 9
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 51`

```
Comparator {
  operator: '~'
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: 4
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 4
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 52`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 6
        index: 5
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 6
          index: 5
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 9
        index: 9
        line: 1
      }
      start: Object {
        column: 6
        index: 5
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: 4
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 9
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 53`

```
Comparator {
  operator: '>='
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 3
        index: 3
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 54`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 7
      index: 7
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 4
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 4
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>='
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 7
        line: 1
      }
      start: Object {
        column: 5
        index: 4
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 7
          index: 7
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 55`

```
Comparator {
  operator: '<='
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 3
        index: 3
        line: 1
      }
      start: Object {
        column: 3
        index: 2
        line: 1
      }
    }
  }
}
```

## `parse: 56`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 7
      index: 7
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 4
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 4
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<='
    loc: Object {
      filename: undefined
      end: Object {
        column: 7
        index: 7
        line: 1
      }
      start: Object {
        column: 5
        index: 4
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 7
          index: 7
          line: 1
        }
        start: Object {
          column: 7
          index: 6
          line: 1
        }
      }
    }
  }
}
```

## `parse: 57`

```
Comparator {
  operator: '>'
  loc: Object {
    filename: undefined
    end: Object {
      column: 2
      index: 2
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 2
        index: 2
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 58`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 3
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 4
          index: 3
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '>'
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 4
        index: 3
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 5
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
    }
  }
}
```

## `parse: 59`

```
Comparator {
  operator: '<'
  loc: Object {
    filename: undefined
    end: Object {
      column: 2
      index: 2
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 2
        index: 2
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 6`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  build: Array ['build']
  prerelease: Array [
    'pre'
    2
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 17
      index: 17
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 60`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 3
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 4
          index: 3
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '<'
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 4
        index: 3
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 5
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
    }
  }
}
```

## `parse: 61`

```
Comparator {
  operator: '^'
  loc: Object {
    filename: undefined
    end: Object {
      column: 2
      index: 2
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 2
        index: 2
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 62`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 3
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 4
          index: 3
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '^'
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 4
        index: 3
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 5
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
    }
  }
}
```

## `parse: 63`

```
Comparator {
  operator: '~'
  loc: Object {
    filename: undefined
    end: Object {
      column: 2
      index: 2
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  version: WildcardVersion {
    build: Array []
    major: 1
    minor: undefined
    patch: undefined
    prerelease: Array []
    loc: Object {
      filename: undefined
      end: Object {
        column: 2
        index: 2
        line: 1
      }
      start: Object {
        column: 2
        index: 1
        line: 1
      }
    }
  }
}
```

## `parse: 64`

```
LogicalAnd {
  loc: Object {
    filename: undefined
    end: Object {
      column: 5
      index: 5
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
  left: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 4
        index: 3
        line: 1
      }
      start: Object {
        column: 1
        index: 0
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 4
          index: 3
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
    }
  }
  right: Comparator {
    operator: '~'
    loc: Object {
      filename: undefined
      end: Object {
        column: 5
        index: 5
        line: 1
      }
      start: Object {
        column: 4
        index: 3
        line: 1
      }
    }
    version: WildcardVersion {
      build: Array []
      major: 1
      minor: undefined
      patch: undefined
      prerelease: Array []
      loc: Object {
        filename: undefined
        end: Object {
          column: 5
          index: 5
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
    }
  }
}
```

## `parse: 7`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  build: Array ['build']
  prerelease: Array [
    'pre'
    2
    3
    4
    5
    'foo'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 27
      index: 27
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 8`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  build: Array [
    'build'
    2
    3
    4
    'foo'
  ]
  prerelease: Array [
    'pre'
    2
    3
    4
    5
    'foo'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 37
      index: 37
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```

## `parse: 9`

```
AbsoluteVersion {
  major: 1
  minor: 2
  patch: 3
  build: Array ['45build']
  prerelease: Array [
    '45pre'
    '42yes'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 25
      index: 25
      line: 1
    }
    start: Object {
      column: 1
      index: 0
      line: 1
    }
  }
}
```
