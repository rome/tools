import { PlaygroundProps } from "./types";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { getLanguage } from "./utils";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
//@ts-ignore
import SuccessIcon from "../assets/success.svg?component";
//@ts-ignore
import FailedIcon from "../assets/failed.svg?component";
//@ts-ignore
import CopyIcon from "../assets/copy.svg?component";
import { useEffect, useState } from "react";

export default function DesktopPlayground(
	{
		setPlaygroundState,
		playgroundState: { code, treeStyle, ...settings },
		prettierOutput,
		romeOutput: { cst, ast, formatted_code, formatter_ir, errors },
	}: PlaygroundProps,
) {
	const { isJsx, isTypeScript } = settings;
	const [clipboardStatus, setClipboardStatus] = useState<
		"success" | "failed" | "normal"
	>("normal");
	const language = getLanguage(isJsx, isTypeScript);

	useEffect(
		() => {
			if (clipboardStatus !== "normal") {
				setClipboardStatus("normal");
			}
		},
		[formatter_ir],
	);

	const copyToClipboard = async () => {
		if (!navigator.clipboard) {
			setClipboardStatus("failed");
			console.error(
				"Your browser does not support clipboard, could not copy the text",
			);
		}
		try {
			await navigator.clipboard.writeText(formatter_ir);
			setClipboardStatus("success");
		} catch (err: any) {
			setClipboardStatus("failed");
			console.error(err.toString());
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
							<button className="bg-gray-300 px-2 py-2 text-white absolute right-0 top--1 mr-5 flex items-center rounded-md" onClick={copyToClipboard}>
								{clipboardStatus === 'success' && <SuccessIcon style={{width: 16, height: 16, marginRight: 5}} />}
								{clipboardStatus === 'failed' && <FailedIcon style={{width: 16, height: 16, marginRight: 5}} />}
								<CopyIcon style={{width: 16, height: 16}} />
							</button>
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
