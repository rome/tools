import { PlaygroundProps } from "./types";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
//@ts-ignore
import SuccessIcon from "../assets/success.svg?component";
//@ts-ignore
import FailedIcon from "../assets/failed.svg?component";
//@ts-ignore
import CopyIcon from "../assets/copy.svg?component";
import { useCallback, useEffect, useState } from "react";

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

	const extensions = [
		javascript({
			jsx: isJsx,
			typescript: isTypeScript,
		}),
	];

	useEffect(() => {
		if (clipboardStatus !== "normal") {
			setClipboardStatus("normal");
		}
	}, [formatter_ir]);

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

	const onChange = useCallback((value) => {
		setPlaygroundState((state) => ({ ...state, code: value }));
	}, []);

	return (
		<div className="divide-y divide-slate-300">
			<h1 className="p-4 text-xl">Rome Playground</h1>
			<SettingsMenu
				settings={settings}
				setPlaygroundState={setPlaygroundState}
			/>
			<div className="box-border flex h-screen divide-x divide-slate-300">
				<div className="w-1/2 p-5">
					<CodeMirror
						value={code}
						height="70vh"
						extensions={extensions}
						placeholder="Enter your code here"
						onChange={onChange}
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
								Diagnostics
							</Tab>
						</TabList>
						<TabPanel>
							<h1>Rome</h1>
							<CodeMirror
								value={formatted_code}
								extensions={extensions}
								placeholder="Rome Output"
								height="30vh"
								readOnly
							/>
							<h1>Prettier</h1>
							<CodeMirror
								value={prettierOutput.code}
								extensions={extensions}
								placeholder="Rome Output"
								height="30vh"
								readOnly
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
							<button
								className="bg-gray-300 px-2 py-2 text-white absolute right-0 top--1 mr-5 flex items-center rounded-md"
								onClick={copyToClipboard}
							>
								{clipboardStatus === "success" && (
									<SuccessIcon
										style={{
											width: 16,
											height: 16,
											marginRight: 5,
										}}
									/>
								)}
								{clipboardStatus === "failed" && (
									<FailedIcon
										style={{
											width: 16,
											height: 16,
											marginRight: 5,
										}}
									/>
								)}
								<CopyIcon style={{ width: 16, height: 16 }} />
							</button>
							<pre className="h-screen overflow-scroll">{formatter_ir}</pre>
						</TabPanel>
						<TabPanel>
							<pre className="h-screen overflow-scroll">{prettierOutput.ir}</pre>
						</TabPanel>
						<TabPanel>
							<div
								className="h-screen overflow-scroll whitespace-pre-wrap text-red-500 text-xs error-panel"
								dangerouslySetInnerHTML={{ __html: errors }}
							/>
						</TabPanel>
					</Tabs>
				</div>
			</div>
		</div>
	);
}
