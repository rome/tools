# `dependencies.ts`

## `can parse npm dependency patterns`

```
npm {
  name: 'foo'
  range: undefined
}
```

## `can parse npm dependency patterns: 1`

```
npm {
  name: '@foo/bar'
  range: undefined
}
```

## `can parse npm dependency patterns: 2`

```
npm {
  name: 'foo'
  range: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 0
    patch: 0
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
}
```

## `can parse npm dependency patterns: 3`

```
npm {
  name: '@foo/bar'
  range: AbsoluteVersion {
    build: Array []
    major: 1
    minor: 0
    patch: 0
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
}
```
