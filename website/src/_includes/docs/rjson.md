### Rome JSON

Rome JSON (RJSON) is a superset of JSON. It does not add any new data types. It just makes some syntax optional for the sake of readability.

We wanted to allow comments inside Rome configuration files. Existing JSON supersets either add new data types (effecting portability), introduce syntax variants, or offer no way to edit the JSON and retain the original comments. This necessitated the creation of our own JSON parser.

RJSON is a superset, meaning that it is backwards compatible and accepts all existing JSON. All places where RJSON files are allowed, you can alternatively use a regular JSON file where these syntax extensions wont be allowed.

##### Implicit top level object

You can omit the curly braces for a top-level object and we will treat it as an object.

```javascript
foo: "bar"
"bar": "foo"
```

##### Comments

Standard JavaScript comments are supported. Both line and block comments.

```javascript
{
	// Line comment
	/* Block comment */
	foo: "bar"
}
```

##### Multiline strings

Regular double quoted strings can have newlines.

##### Unquoted keys

If a property key is a valid identifier then the quotes can be omitted, just like in regular JavaScript.

```javascript
{
	unquotedKey: true
}
```

##### Optional commas

Commas are not required to separate elements of an array:

```javascript
[
	1
	2
	3
]
```

or an object:

```javascript
{
	a: 1
	b: 2
	c: 3
}
```

##### Numeric separators

You can use [numeric separators](https://github.com/tc39/proposal-numeric-separator) in numbers, just like in regular JavaScript:

**Example**

```javascript
5_000
```
