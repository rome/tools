# `parse.ts`

## `pattern`

```
Pattern {
  comment: ''
  negate: true
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
  ]
}
```

## `pattern: 1`

```
Pattern {
  comment: ''
  names: Array []
  negate: false
  root: false
  segments: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 0
      index: 0
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
}
```

## `pattern: 10`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 9
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 7
              index: 6
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
      ]
    }
    Segment {
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
      parts: Array [
        Word {
          value: 'bar'
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
      ]
    }
  ]
}
```

## `pattern: 11`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 10
      index: 10
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    WildcardSegment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 10
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 10
          line: 1
        }
        start: Object {
          column: 8
          index: 7
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
              index: 10
              line: 1
            }
            start: Object {
              column: 8
              index: 7
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 12`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 8
      index: 8
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 8
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
              index: 8
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 13`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 8
      index: 8
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 8
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
        Wildcard {
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
      ]
    }
  ]
}
```

## `pattern: 14`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
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
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
              index: 9
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 9
              line: 1
            }
            start: Object {
              column: 9
              index: 8
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 15`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
              index: 9
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 14
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
        Word {
          value: 'foob'
          loc: Object {
            filename: undefined
            end: Object {
              column: 13
              index: 13
              line: 1
            }
            start: Object {
              column: 10
              index: 9
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 16`

```
Pattern {
  comment: ' foobar'
  names: Array []
  negate: false
  root: false
  segments: Array []
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 8
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
}
```

## `pattern: 17`

```
Pattern {
  comment: ' foobar'
  negate: false
  root: false
  names: Array [
    'foo'
    'bar '
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 17
      index: 16
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 9
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar '
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
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
      ]
    }
  ]
}
```

## `pattern: 18`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar\\#foobar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 15
      index: 15
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 16
          index: 15
          line: 1
        }
        start: Object {
          column: 1
          index: 0
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 15
          index: 15
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar\\#foobar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 15
              index: 15
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 19`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    '\\#foobar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 12
      index: 12
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 12
          line: 1
        }
        start: Object {
          column: 1
          index: 0
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 12
          index: 12
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Word {
          value: '\\#foobar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 12
              index: 12
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 2`

```
Pattern {
  comment: ''
  negate: false
  root: true
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 8
      index: 8
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 9
          index: 8
          line: 1
        }
        start: Object {
          column: 2
          index: 1
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 8
          line: 1
        }
        start: Object {
          column: 6
          index: 5
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
              index: 8
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 20`

```
Pattern {
  comment: ''
  negate: false
  root: true
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 10
      index: 10
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 10
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 10
          line: 1
        }
        start: Object {
          column: 8
          index: 7
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
              index: 10
              line: 1
            }
            start: Object {
              column: 8
              index: 7
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 21`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 12
          index: 11
          line: 1
        }
        start: Object {
          column: 4
          index: 3
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 4
              index: 3
              line: 1
            }
          }
        }
      ]
    }
    Segment {
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
      parts: Array [
        Word {
          value: 'bar'
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
      ]
    }
  ]
}
```

## `pattern: 22`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 12
      index: 12
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 12
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 12
          index: 12
          line: 1
        }
        start: Object {
          column: 10
          index: 9
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 12
              index: 12
              line: 1
            }
            start: Object {
              column: 10
              index: 9
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 23`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['bar']
  loc: Object {
    filename: undefined
    end: Object {
      column: 13
      index: 13
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 14
          index: 13
          line: 1
        }
        start: Object {
          column: 5
          index: 4
          line: 1
        }
      }
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 11
              index: 10
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 13
          line: 1
        }
        start: Object {
          column: 11
          index: 10
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 13
              index: 13
              line: 1
            }
            start: Object {
              column: 11
              index: 10
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 24`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'hello'
    'world'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 12
      index: 12
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 13
          index: 12
          line: 1
        }
        start: Object {
          column: 1
          index: 0
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'hello'
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
        }
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 12
          index: 12
          line: 1
        }
        start: Object {
          column: 8
          index: 7
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'world'
          loc: Object {
            filename: undefined
            end: Object {
              column: 12
              index: 12
              line: 1
            }
            start: Object {
              column: 8
              index: 7
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 3`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 9
      index: 9
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 9
          line: 1
        }
        start: Object {
          column: 3
          index: 2
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
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
      parts: Array [
        Word {
          value: 'bar'
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
      ]
    }
  ]
}
```

## `pattern: 4`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 10
      index: 10
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 11
          index: 10
          line: 1
        }
        start: Object {
          column: 4
          index: 3
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
              index: 7
              line: 1
            }
            start: Object {
              column: 4
              index: 3
              line: 1
            }
          }
        }
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 10
          index: 10
          line: 1
        }
        start: Object {
          column: 8
          index: 7
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 10
              index: 10
              line: 1
            }
            start: Object {
              column: 8
              index: 7
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```

## `pattern: 5`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['bar']
  loc: Object {
    filename: undefined
    end: Object {
      column: 11
      index: 11
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 12
          index: 11
          line: 1
        }
        start: Object {
          column: 4
          index: 3
          line: 1
        }
      }
      parts: Array [
        Wildcard {
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
              index: 7
              line: 1
            }
            start: Object {
              column: 4
              index: 3
              line: 1
            }
          }
        }
        Word {
          value: 'foo'
          loc: Object {
            filename: undefined
            end: Object {
              column: 9
              index: 8
              line: 1
            }
            start: Object {
              column: 5
              index: 4
              line: 1
            }
          }
        }
      ]
    }
    Segment {
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
      parts: Array [
        Word {
          value: 'bar'
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
      ]
    }
  ]
}
```

## `pattern: 6`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 3
      index: 3
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
  ]
}
```

## `pattern: 7`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array ['foo']
  loc: Object {
    filename: undefined
    end: Object {
      column: 4
      index: 4
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
  ]
}
```

## `pattern: 8`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 7
      index: 7
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
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
      parts: Array [
        Word {
          value: 'bar'
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
        }
      ]
    }
  ]
}
```

## `pattern: 9`

```
Pattern {
  comment: ''
  negate: false
  root: false
  names: Array [
    'foo'
    'bar'
  ]
  loc: Object {
    filename: undefined
    end: Object {
      column: 8
      index: 8
      line: 1
    }
    start: Object {
      column: 0
      index: -1
      line: 1
    }
  }
  segments: Array [
    Segment {
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
      parts: Array [
        Word {
          value: 'foo'
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
      ]
    }
    Segment {
      loc: Object {
        filename: undefined
        end: Object {
          column: 8
          index: 8
          line: 1
        }
        start: Object {
          column: 6
          index: 5
          line: 1
        }
      }
      parts: Array [
        Word {
          value: 'bar'
          loc: Object {
            filename: undefined
            end: Object {
              column: 8
              index: 8
              line: 1
            }
            start: Object {
              column: 6
              index: 5
              line: 1
            }
          }
        }
      ]
    }
  ]
}
```
