import { PlaygroundProps } from "./types";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { getLanguage } from "./utils";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
import "react-toastify/dist/ReactToastify.css";
import { ToastContainer, toast } from "react-toastify";

export default function DesktopPlayground(
	{
		setPlaygroundState,
		playgroundState: { code, treeStyle, ...settings },
		prettierOutput,
		romeOutput: { cst, ast, formatted_code, formatter_ir, errors },
	}: PlaygroundProps,
) {
	const { isJsx, isTypeScript } = settings;
	const language = getLanguage(isJsx, isTypeScript);
	const copyToClipboard = async () => {
		if (!navigator.clipboard) {
			toast("Your browser does not support clipboard, could not copy text", {});
		}
		try {
			await navigator.clipboard.writeText(formatter_ir);
			toast("RomeIR has been successfully copy to your clipboard", {});
		} catch (err: any) {
				toast("Could not copy text: ", err);
		}
	};
	return (
		<div className="divide-y divide-slate-300">
			<h1 className="p-4 text-xl">Rome Playground</h1>
			<SettingsMenu
				settings={settings}
				setPlaygroundState={setPlaygroundState}
			/>
			<div className="box-border flex h-screen divide-x divide-slate-300">
				<div className="w-1/2 p-5">
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
							<TreeView
								treeStyle={treeStyle}
								setPlaygroundState={setPlaygroundState}
								tree={cst}
							/>
						</TabPanel>
						<TabPanel>
							<TreeView
								treeStyle={treeStyle}
								setPlaygroundState={setPlaygroundState}
								tree={ast}
							/>
						</TabPanel>
						<TabPanel>
							<ToastContainer />
							<button className="bg-slate-500 px-4 py-1 text-white absolute right-0 mr-5" onClick={copyToClipboard}>Copy to ClipBoard</button>
							<pre className="h-screen overflow-scroll">{formatter_ir}</pre>
						</TabPanel>
						<TabPanel>
							<pre className="h-screen overflow-scroll">{prettierOutput.ir}</pre>
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
