# `@internal/binary-transport`

## Explainer

Rome uses a bridge abstraction to split work between worker processes. There are several different types of bridges, most of which use this binary transport format to communicate with the main process.

Using a binary format allows these processes to communicate with much less overhead which leads to less memory usage and less time spent communicating.

### How it works

When you want to send data from one process to another using this transport format, you must first encode the data, then decode the data on the other side.

Similar to JSON, only certain data types can actually be encoded, although this format supports far more types than JSON does. For a full list of supported data types, see [`RSERValue` in `types.ts`](./types.ts).

The "syntax" of the format is a lot different from JSON though.

```json
{
	"nested": [
		{
			"data": "structure"
		}
	]
}
```

But this format sacrifices readability to have a much more "flat" syntax:

```ruby
[type:object]
[size:1] # number of keys to read

	# key: "nested"
	[size:6]
	[bytes:"nested"]

	# value: [{ data: "structure" }]
	[type:array]
	[size:1] # number of elements in array

		# element: { data: "structure" }
		[type:object]
		[size:1]

			# key: "data"
			[size:4]
			[bytes:"data"]

			# value: "structure"
			[type:string]
			[size:9]
			[bytes:"structure"]
```

Removing the space between them it looks like this:

```ruby
[type:object][size:1][size:6][bytes:"nested"][type:array][size:1][type:object][size:1][size:4][bytes:"data"][type:string][size:9][bytes:"structure"]
```

Further simplifying:

```ruby
object,1,6,"nested",array,1,object,1,4,"data",string,9,"structure"
```

Then convert the "types" like `object`, `array`, and `string` into known "codes":

```ruby
14,1,6,"nested",11,1,14,1,4,"data",2,9,"structure"
```

Then encode string values with utf8:

```ruby
14,1,6,n,e,s,t,e,d,11,1,14,1,4,d,a,t,a,2,9,s,t,r,u,c,t,u,r,e
14,1,6,110,101,115,116,101,100,11,1,14,1,4,100,97,116,97,2,9,115,116,114,117,99,116,117,114,101
```

Formatting our data this way gives us a linear set of instructions to re-construct our encoded data and doesn't require a complex and slow parser.
