import { PlaygroundProps } from "./types";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { getLanguage } from "./utils";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";

export default function DesktopPlayground(
	{
		playgroundState: { code, setCode, ...settings },
		prettierOutput,
		romeOutput: { cst, ast, formatted_code, formatter_ir, errors },
	}: PlaygroundProps,
) {
	const { isJsx, isTypeScript } = settings;
	const language = getLanguage(isJsx, isTypeScript);
	return (
		<div className="divide-y divide-slate-300">
			<h1 className="p-4 text-xl">Rome Playground</h1>
			<SettingsMenu settings={settings} />
			<div className="box-border flex h-screen divide-x divide-slate-300">
				<div className="w-1/2 p-5">
					<CodeEditor
						value={code}
						language={language}
						placeholder="Enter some code here"
						onChange={(evn) => {
							setCode(evn.target.value);
						}}
						style={{
							fontSize: 12,
							height: "100vh",
							fontFamily:
								"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
						}}
					/>
				</div>
				<div className="w-1/2 p-5 flex flex-col">
					<Tabs>
						<TabList>
							<Tab selectedClassName="bg-slate-300">Formatter</Tab>
							<Tab selectedClassName="bg-slate-300">CST</Tab>
							<Tab selectedClassName="bg-slate-300">AST</Tab>
							<Tab selectedClassName="bg-slate-300">Rome IR</Tab>
							<Tab selectedClassName="bg-slate-300">Prettier IR</Tab>
							<Tab disabled={errors === ""} selectedClassName="bg-slate-300">
								Errors
							</Tab>
						</TabList>
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
							<TreeView tree={JSON.parse(cst)} />
						</TabPanel>
						<TabPanel>
							<TreeView tree={JSON.parse(ast)} />
						</TabPanel>
						<TabPanel>
							<pre className="h-screen overflow-scroll">{formatter_ir}</pre>
						</TabPanel>
						<TabPanel>
							<TreeView tree={prettierOutput.ir} />
						</TabPanel>
						<TabPanel>
							<pre className="h-screen overflow-scroll whitespace-pre-wrap text-red-500 text-xs">
								{errors}
							</pre>
						</TabPanel>
					</Tabs>
				</div>
			</div>
		</div>
	);
}
