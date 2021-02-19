# `markup`

Utility to make fancy messages to display info or data to the users.

## TOC
[Emphasis](#Emphasis)
[Number](#Number)
[Indent](#Indent)
[ViewLinePrefix](#ViewLinePrefix)
[ViewPointer](#ViewPointer)
[GrammarNumber](#GrammarNumber)
[Hyperlink](#Hyperlink)
[Filelink](#Filelink)
[Inverse](#Inverse)
[Dim](#Dim)
[Filesize](#Filesize)
[Duration](#Duration)
[Italic](#Italic)
[Underline](#Underline)
[Strike](#Strike)
[Token](#Token)
[Error](#Error)
[Success](#Success)
[Locator](#Locator)
[Warn](#Warn)
[Info](#Info)
[Code](#Code)
[Color](#Color)
[Highlight](#Highlight)
[Table](#Table)
[Td](#Td)
[Hr](#Hr)
[Hr](#Hr)
[Pad](#Pad)
[Ol](#Ol)
[Ul](#Ul)

## Syntax
All examples use the following template:
```ts
import {reporter} from "./_utils";
import {markup} from "@internal/markup";

reporter.log(markup`...`);
```

Most attributes are optional.

### Emphasis
```ts
markup`The <emphasis>emphasis</emphasis> tag`
```
![emphasis](https://user-images.githubusercontent.com/33844379/107866611-f7f09b80-6e72-11eb-9d72-2223b616b9b3.png)

### Number
Attribute | Type
--- | ---
approx | boolean

```ts
markup`
The number tag formats numbers: <number>1000000</number> (without: 1000000);  <number>iamnotanumber</number> (without: iamnotanumber)
<number>2.999</number> is <number approx="true">3</number>
`
```

![number](https://user-images.githubusercontent.com/33844379/107887048-b6f49780-6f03-11eb-9c3d-10d370c336e5.png)

### Indent
```ts
markup`
Unindented sentence.
<indent>Indented sentence.</indent>
<indent><indent>Double indented sentence.</indent></indent>
`
```
![indent](https://user-images.githubusercontent.com/33844379/107866795-b7921d00-6e74-11eb-983c-5b3ef7324670.png)

### View
Attribute | Type
--- | ---
extraSoftWrapIndent | number
lineWrap | "none" \| "word-break" \| "char-break"
align | "right" \| "left"

```ts
markup`
<view>
Normal view
</view>
<view extraSoftWrapIndent="3">
Extra soft wrap indent and a lot of words to fill the whole width of the terminal
</view>
<view align="right">
<viewLinePrefix>Prefix</viewLinePrefix>Align right
</view>
<view lineWrap="word-break">
Line wrap word-break myextremelyextralongwordwithtoomanycharacters andanotheroneofthosebecauseilikethem
</view>
<view lineWrap="char-break">
Line wrap char-break myextremelyextralongwordwithtoomanycharacters andanotheroneofthosebecauseilikethem
</view>
<view lineWrap="none">
Line wrap none myextremelyextralongwordwithtoomanycharacters andanotheroneofthosebecauseilikethem
</view>
`
```
![view](https://user-images.githubusercontent.com/33844379/107895596-3ac67800-6f34-11eb-8cef-2385f686c7c8.png)

*Don't know what `extraSoftWrapIndent` does.*

### ViewLinePrefix
Attribute | Type
--- | ---
type | "first" \| "middle" \| "end" \| "pointer"
align | "right" \| "left"

```ts
markup`
<view><viewLinePrefix type="first">Prefix </viewLinePrefix>First line
<viewLinePrefix type="middle">Prefix 2 </viewLinePrefix>Some text
<viewLinePrefix type="end">Prefix 3 </viewLinePrefix>Some more
Last one</view>

<view><viewLinePrefix type="pointer">Prefix </viewLinePrefix>Some text</view>

<view><viewLinePrefix type="first" align="right">Prefix </viewLinePrefix>First line
<viewLinePrefix type="middle">Another prefix </viewLinePrefix>Some text
<viewLinePrefix type="end" align="left">Prefix </viewLinePrefix>Some more
Last one</view>
`
```
![viewLinePrefix](https://user-images.githubusercontent.com/33844379/107898013-ec68a780-6f3a-11eb-93f6-74691ff9ee01.png)

### ViewPointer
Attribute | Type
--- | ---
char | string
line | number
start | number
end | number

### GrammarNumber
Attribute | Type
--- | ---
none | string
singular | string
plural | string

```ts
markup`There is/are <grammarNumber none="nothing" singular="die" plural="dice">0</grammarNumber>;
<grammarNumber none="nothing" singular="die" plural="dice">1</grammarNumber>;
<grammarNumber none="nothing" singular="die" plural="dice">2</grammarNumber>`
```
![grammarNumber](https://user-images.githubusercontent.com/33844379/107899301-30a97700-6f3e-11eb-84bd-2fc19ed5846c.png)


### Hyperlink
Attribute | Type
--- | ---
target | string

```ts
markup`Have you checked out <hyperlink target="https://rome.tools"/>?`
```
![hyperlink](https://user-images.githubusercontent.com/33844379/107899845-c98cc200-6f3f-11eb-96cd-af334ec1aae5.png)

### Filelink
Attribute | Type
--- | ---
target | string
column | number
line | number

```ts
markup`Error is <filelink target="test.ts" line="1" column="10">here</filelink>`
```

*Doesn't seem to do anything?*

### Inverse
```ts
markup`The <inverse>inverse</inverse> tag`
```
![inverse](https://user-images.githubusercontent.com/33844379/107899875-df01ec00-6f3f-11eb-87dc-dc1d35667e3f.png)

### Dim
```ts
markup`The <dim>dim</dim> tag`
```
*Don't have a terminal that supports it*

### Filesize
```ts
markup`The file is <filesize>1024</filesize>`
```
![filesize](https://user-images.githubusercontent.com/33844379/107899903-f2ad5280-6f3f-11eb-944f-53536a32c2dc.png)

### Duration
Attribute | Type
--- | ---
approx | boolean

```ts
markup`<duration>1247</duration> <duration approx="true">1000</duration>`
```
![duration](https://user-images.githubusercontent.com/33844379/108449799-63b47900-7264-11eb-83ea-db7331e59d07.png)

### Italic
```ts
markup`The <italic>italic</italic> tag`
```
![italic](https://user-images.githubusercontent.com/33844379/108402605-0dbce280-721e-11eb-86e8-b38f71743ecd.png)

### Underline
```ts
markup`The <underline>underline</underline> tag`
```
![underline](https://user-images.githubusercontent.com/33844379/108403100-a4899f00-721e-11eb-97ba-487fdec01244.png)

### Strike
```ts
markup`The <strike>strike</strike> tag`
```
*Don't have a terminal that supports it*

### Token
Attribute | Type
--- | ---
type | "boolean" \| "keyword" \| "number" \| "regex" \| "string" \| "comment" \| "operator" \| "punctuation" \| "variable" \| "attr-name" \| "function" \| "attr-value" \| "attr-equals" \| "tag"

```ts
markup`<token type="boolean">true</token> <token type="keyword">function</token> <token type="number">465</token>
<token type="regex">/[abc]+(def).*/</token> <token type="string">hello world</token> <token type="comment">im a comment</token>
<token type="operator">+</token> <token type="punctuation">,</token> <token type="variable">foo</token>
<token type="attr-name">data</token> <token type="function">markup</token> <token type="attr-value">value</token>
<token type="attr-equals">yes</token> <token type="tag">tag</token>`
```
![token](https://user-images.githubusercontent.com/33844379/108448934-e76d6600-7262-11eb-8305-f4d8544cb5f2.png)

*Doesn't seem to do anything?*

### Error
```ts
markup`The <error>error</error> tag`
```
![error](https://user-images.githubusercontent.com/33844379/107899942-0e185d80-6f40-11eb-9455-3e93d1166fff.png)

### Success
```ts
markup`The <success>success</success> tag`
```
![success](https://user-images.githubusercontent.com/33844379/107899969-196b8900-6f40-11eb-86c0-cd845279f210.png)

### Locator
Attribute | Type
--- | ---
id | string

### Warn
```ts
markup`The <warn>warn</warn> tag`
```
![warn](https://user-images.githubusercontent.com/33844379/107899982-24261e00-6f40-11eb-9a47-d08e62575317.png)

### Info
```ts
markup`The <info>info</info> tag`
```
![info](https://user-images.githubusercontent.com/33844379/107900014-3011e000-6f40-11eb-84ed-ed53dd6ad299.png)

### Code
```ts
markup`The <code>code</code> tag`
```
![code](https://user-images.githubusercontent.com/33844379/108412338-0a2f5880-722a-11eb-90d9-bcc188f4982d.png)

### Color
Attribute | Type
--- | ---
fg | "black" \| "brightBlack" \| "red" \| "brightRed" \| "green" \| "brightGreen" \| "yellow" \| "brightYellow" \| "blue" \| "brightBlue" \| "magenta" \| "brightMagenta" \| "cyan" \| "brightCyan" \| "white" \| "brightWhite"
bg | "black" \| "brightBlack" \| "red" \| "brightRed" \| "green" \| "brightGreen" \| "yellow" \| "brightYellow" \| "blue" \| "brightBlue" \| "magenta" \| "brightMagenta" \| "cyan" \| "brightCyan" \| "white" \| "brightWhite"

```ts
markup`The <color fg="green" bg="white">color</color> tag`
```
![color](https://user-images.githubusercontent.com/33844379/108415080-53cd7280-722d-11eb-8e8b-7c77dd350486.png)

### Highlight
Attribute | Type
--- | ---
i | number
legend | boolean

```ts
markup`The <highlight i="1" legend="true">highlight</highlight> tag
The <highlight i="2">highlight</highlight> tag`
```
![highlight](https://user-images.githubusercontent.com/33844379/108430994-8cc41200-7242-11eb-99ab-6fd34d56000b.png)

### Table
#### Td
Attribute | Type
--- | ---
align | "right" \| "left"

```ts
markup`
<table>
<tr><td>Column 1</td><td>Column 2</td><td>Column 3</td></tr>
<tr><td>foo</td><td>yes</td><td align="right">4</td></tr>
<tr><td>bar</td><td>no</td><td>2</td></tr>
</table>`
```
![table](https://user-images.githubusercontent.com/33844379/108447802-d28fd300-7260-11eb-9c5a-5ca23a9551a7.png)

### Hr
```ts
markup`Hey
<hr/>
Hi`
```
![hr](https://user-images.githubusercontent.com/33844379/108432742-28ef1880-7245-11eb-9134-09b625b7afe0.png)

### Pad
Attribute | Type
--- | ---
width | number
align | "right" \| "left"

```ts
markup`<pad width="20" align="right">Hello</pad> world
12345678901234567890
<pad width="20" align="left">Hello</pad> world`
```
![pad](https://user-images.githubusercontent.com/33844379/108434973-d283d900-7248-11eb-9c9c-252ce6b192c4.png)

### Ul
```ts
markup`
<ul>
<li>Item 1</li>
<li>Item 2</li>
<li>Item 3</li>
</ul>`
```
![ul](https://user-images.githubusercontent.com/33844379/108446517-893e8400-725e-11eb-9d31-fb3f32c75207.png)

### Ol
Attribute | Type
--- | ---
reversed | boolean
start | number

```ts
markup`
<ol reversed="true">
<li>Item 1</li>
<li>Item 2</li>
<li>Item 3</li>
</ol>

<ol start="4">
<li>Item 1</li>
<li>Item 2</li>
<li>Item 3</li>
</ol>`
```
![ol](https://user-images.githubusercontent.com/33844379/108446863-26012180-725f-11eb-9bf5-43a1162b98e1.png)
