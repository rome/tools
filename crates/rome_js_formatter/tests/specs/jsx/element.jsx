// Single string attribute
<div tooltip="A very long tooltip text that would otherwise make the attribute break onto the same line but it is not because of the single string layout"></div>;

// Not single string because of the new line
a = <div tooltip="A very long tooltip text that would otherwise make the attribute break
					onto the same line but it is not because of the single string layout"></div>;

// Inline
a = <ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent></ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent>;

// IndentAttributes
a = <ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent a b  c d></ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent>;

// Empty
a = <div></div>;
<>


</>;

// Not empty
a = <div> </div>;

// Template
a = <div>{`A Long Tempalte String That uses ${5 + 4} that will eventually break across multiple lines ${40 / 3 * 45}`}</div>;

// Meaningful text after self closing element adds a hard line break
a = <div><pre className="h-screen overflow-y-scroll" />adefg</div>;

// Meaningful text after a non-self closing element should add a soft line break
b = a = <div><pre className="h-screen overflow-y-scroll">a</pre>abcd</div>;

// A word right before a self-closing element inserts a hard line break
a = <div>ab<br/></div>;

// A Word not right before a self-closing element inserts a soft line break.
a = <div>ab<pre>text</pre></div>;

// whitespaces
c = <div>a{' '}{' '}{' '}{' '}{' '}{' '}{' '}{' '}b{' '}{' '}{' '}{' '}{' '}{' '}</div>;

c2 = <div>a{' '}{' '}{' '}{' '}{' '}{' '}{' '}{' '}<div></div>content{' '}{' '}{' '}{' '}{' '}{' '}</div>;

// this group should fit one line jsx whitespaces are hidden
b =
	<div>
		<a></a>

		{' '}

		<a></a>

		{' '}
		1
	</div>;

// this group should break first tag and show only first jsx whitespace
b1 =
	<div>
		<a>
			{`
12312
12312
			`}
		</a>

		{' '}

		<a></a>

		{' '}
		1
	</div>;

// this group fit one line and hide jsx whitespace
b2 =
	<>
		<a>123
		</a>

		{' '}
		1
	</>;

// this group break group and show jsx whitespace
b3 =
	<>
		<a>{`
123`}
		</a>

		{' '}
		1
	</>;

const b4 = <div>
	Text <a data-very-long-prop-breakline-rome-playground data-other>
	some link
</a>{' '}
	| some other text,{' '}
</div>;

b5 =
	<div>
		<br /> long text long text long text long text long text long text long text long text<link>url</link> long text long text
	</div>;

<div><div></div><a> jumps over the lazy dog </a></div>;

const Essay = () => <div>The films of Wong Kar-Wai exemplify the synthesis of French New Wave cinema—specifically the unrelenting
	experimental technique and fascination with American/western culture—with more conventional melodramatic, romantic narratives.</div>;

function Tabs() {
	return <Tabs>
		<TabList>
			<Tab selectedClassName="bg-slate-300">Input</Tab>
			<Tab selectedClassName="bg-slate-300">Settings</Tab>
			<Tab selectedClassName="bg-slate-300">Formatter Output</Tab>
			<Tab selectedClassName="bg-slate-300">CST</Tab>
			<Tab selectedClassName="bg-slate-300">AST</Tab>
			<Tab selectedClassName="bg-slate-300">Rome IR</Tab>
			<Tab selectedClassName="bg-slate-300">Prettier IR</Tab>
			<Tab disabled={errors === ""} selectedClassName="bg-slate-300">
				Errors
			</Tab>
		</TabList>
		<TabPanel>
			<CodeEditor
				value={code}
				language={language}
				placeholder="Enter some code here"
				onChange={(evn) => {
					setPlaygroundState((state) => ({
						...state,
						code: evn.target.value,
					}));
				}}
				style={{
					fontSize: 12,
					height: "100vh",
					fontFamily:
						"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
				}}
			/>
		</TabPanel>
		<TabPanel>
			<SettingsMenu
				setPlaygroundState={setPlaygroundState}
				settings={settings}
			/>
		</TabPanel>
		<TabPanel>
			<h1>Rome</h1>
			<CodeEditor
				value={formatted_code}
				language={language}
				placeholder="Rome Output"
				style={{
					fontSize: 12,
					height: "40vh",
					overflowY: "scroll",
					fontFamily:
						"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
				}}
			/>
			<h1>Prettier</h1>
			<CodeEditor
				value={prettierOutput.code}
				language={language}
				placeholder="Prettier Output"
				style={{
					fontSize: 12,
					height: "50vh",
					overflowY: "scroll",
					fontFamily:
						"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
				}}
			/>
		</TabPanel>
		<TabPanel>
			<TreeView
				tree={cst}
				treeStyle={treeStyle}
				setPlaygroundState={setPlaygroundState}
			/>
		</TabPanel>
		<TabPanel>
			<TreeView
				tree={ast}
				treeStyle={treeStyle}
				setPlaygroundState={setPlaygroundState}
			/>
		</TabPanel>
		<TabPanel>
			<pre className="h-screen overflow-y-scroll">{formatter_ir}</pre>
		</TabPanel>
		<TabPanel>
			<pre className="h-screen overflow-y-scroll">{prettierOutput.ir}</pre>
		</TabPanel>
		<TabPanel>
					<pre className="h-screen overflow-y-scroll whitespace-pre-wrap text-red-500 text-xs">
						{errors}
					</pre>
		</TabPanel>
	</Tabs>;
}

function LoginForm() {
	return <form onChange={
		e => {
			e.preventDefault();
			console.log("Submitted form!")}
	}>
		<input value={username} onChange={e => setUsername(e.target.value)} />
		<input type="password" value={password} onChange={e => setPassword(e.target.value)} />
	</form>
}

function MapoTofuRecipe() {
	return <ul>
		Mapo tofu recipe
		<li>2 packets soft or silken tofu</li>
		<li>1 tablespoon minced garlic</li>
		<li>1 tablespoon minced ginger </li>
		<li>2 tablespoons doubanjiang</li>
		<li>1 tablespoon douchi</li>
		<li>1 tablespoon corn or potato starch</li>
		<li>2 scallions or jiu cai</li>
		<li>6 ounces of ground beef or pork</li>
	</ul>
}

<Route path="/" component={<HomePage />} />;

let component = <div> La Haine dir. Mathieu Kassovitz </div>;

let component = (
	<div> Uncle Boonmee Who Can Recall His Past Lives dir. Apichatpong Weerasethakul </div>
);

(<div>Badlands</div>).property;

let bar = <div>
	{foo(() => <div> the quick brown fox jumps over the lazy dog and then jumps over the lazy cat and then over the lazy fish. </div>)}
</div>;

<Component // here is a comment
	className={bar} index={0} name="Component" // here is another comment
></Component>;

// spacing
let a = <a>{' '}</a>
let b = <a>{" "}</a>

// comments
let a = <a>{ /* comment */ " " /* comment */ }</a>;
let a = <a>{  " "
	/* comment */ }</a>;
let a = <a>{ /* comment */ " " }</a>;

// in array
const breadcrumbItems = [
	(
		<Breadcrumb.Item key="home">
			<Link to="/">Home</Link>
		</Breadcrumb.Item>
	),
].concat(extraBreadcrumbItems);

function Component() {
	return (
		<Test
			prop={value}
			// comment
		/>
	);
}

let b = (
	<section>
		<div>
			aVeryLongCOntentThat
		</div // comment
	>
	</section>
);


let a = (
	</*comment3*/Test
		// comment before attribute
		/*comment1*/prop/*comment2*/=/*comment3*/{/*comment4*/value /*comment5*/}/*comment6*/
		// comment after attribute
	>
		<Test
			// comment before attribute
			/*comment1*/prop/*comment2*/=/*comment3*/{/*comment4*/value /*comment5*/}/*comment6*/
			// comment after attribute
		/>
	</Test>
);

function Component() {
	return (
		<div>
			{fn(data)}{' '}
			<Component />
		</div>
	);
}

// jsx whitespace {' '} adds meaningful jsx text
function Component() {
	return (
		<div>
			{fn(datadatadatadatadatadatadatadatadatadatadatadatadatadatadatadata)}{' '}
			<Component />
		</div>
	);
}

// not jsx whitespace doesn't add meaningful jsx text
function Component() {
	return (
		<div>
			{fn(datadatadatadatadatadatadatadatadatadatadatadatadatadatadatadata)}{'  '}
			<Component />
		</div>
	);
}
