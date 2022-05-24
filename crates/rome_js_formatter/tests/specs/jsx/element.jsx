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
