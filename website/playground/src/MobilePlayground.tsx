import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { createSetter, getLanguage } from "./utils";
import { PlaygroundProps } from "./types";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
import ReactJson from "react-json-view";

export function MobilePlayground({
	setPlaygroundState,
	playgroundState: { code, treeStyle, ...settings },
	prettierOutput,
	romeOutput: { cst, ast, formatted_code, formatter_ir, errors },
}: PlaygroundProps) {
	const { isJsx, isTypeScript } = settings;
	const language = getLanguage(isJsx, isTypeScript);
	return (
		<div className="p-1">
			<h1 className="p-3 text-xl pb-5">Rome Playground</h1>
			<Tabs>
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
					<ReactJson src={prettierOutput.ir} />
				</TabPanel>
				<TabPanel>
					<pre className="h-screen overflow-y-scroll whitespace-pre-wrap text-red-500 text-xs">
						{errors}
					</pre>
				</TabPanel>
			</Tabs>
		</div>
	);
}
