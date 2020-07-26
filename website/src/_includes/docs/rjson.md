### Rome JSON

Rome JSON (RJSON) is a superset of JSON. It does not add any new data types. It just makes some syntax optional for the sake of readability.

We wanted to allow comments inside Rome configuration files. Existing JSON supersets either add new data types (effecting portability), or offered no way for a program to edit the JSON, and retain the original comments. So we decided to write our own JSON parser to do this.

##### Implicit top level object

You can omit the curly braces for a top level object and we will treat it as an object.

```json
foo: "bar"
"bar": "foo"
```

##### Comments

Standard JavaScript comments are supported. Both line and block comments.

```json
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

```json
{
	unquotedKey: true
}
```

##### Optional commas

Commas are not required to separate elements of an array:

**Example**

```json
[
	1
	2
	3
]
```

or an object:

```json
{
	a: 1
	b: 2
	c: 3
}
```

##### Numeric separators

You can use [numeric separators](https://github.com/tc39/proposal-numeric-separator) in numbers, just like in regular JavaScript:

**Example**

```json
5_000
```
