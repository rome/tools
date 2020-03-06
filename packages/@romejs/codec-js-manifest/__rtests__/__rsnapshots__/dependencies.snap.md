# `dependencies.ts`

## `can parse npm dependency patterns`

```javascript
npm {
  name: 'foo'
  range: undefined
}
```

## `can parse npm dependency patterns: 1`

```javascript
npm {
  name: '@foo/bar'
  range: undefined
}
```

## `can parse npm dependency patterns: 2`

```javascript
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

```javascript
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
