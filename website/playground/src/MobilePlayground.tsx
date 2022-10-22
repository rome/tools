import { useCallback, useMemo } from "react";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import CodeMirror from "@uiw/react-codemirror";
import type { ViewUpdate } from "@codemirror/view";
import { javascript } from "@codemirror/lang-javascript";
import { PlaygroundProps } from "./types";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
import MermaidGraph from "./MermaidGraph";

export function MobilePlayground({
	setPlaygroundState,
	playgroundState: { code, ...settings },
	prettierOutput,
	romeOutput: {
		cst,
		ast,
		formatted_code,
		formatter_ir,
		errors,
		control_flow_graph,
	},
}: PlaygroundProps) {
	const { isJsx, isTypeScript } = settings;

	const onUpdate = useCallback((viewUpdate: ViewUpdate) => {
		const cursorPosition = viewUpdate.state.selection.ranges[0]?.from ?? 0;
		setPlaygroundState((state) =>
			state.cursorPosition !== cursorPosition
				? {
						...state,
						cursorPosition,
				  }
				: state,
		);
	}, []);
	const onChange = useCallback((value) => {
		setPlaygroundState((state) => ({ ...state, code: value }));
	}, []);

	const extensions = useMemo(() => [
		javascript({
			jsx: isJsx,
			typescript: isTypeScript,
		}),
	], [isJsx, isTypeScript]);

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
					<Tab
						disabled={control_flow_graph === ""}
						selectedClassName="bg-slate-300"
					>
						Control Flow Graph
					</Tab>
				</TabList>
				<TabPanel>
					<CodeMirror
						value={code}
						extensions={extensions}
						placeholder="Enter your code here"
						onUpdate={onUpdate}
						onChange={onChange}
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
					<CodeMirror
						value={formatted_code}
						extensions={extensions}
						placeholder="Rome Output"
						readOnly={true}
						style={{
							fontSize: 12,
							height: "40vh",
							overflowY: "scroll",
							fontFamily:
								"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
						}}
					/>
					<h1>Prettier</h1>
					<CodeMirror
						value={prettierOutput.code}
						extensions={extensions}
						readOnly={true}
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
					<TreeView tree={cst} />
				</TabPanel>
				<TabPanel>
					<TreeView tree={ast} />
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
				<TabPanel>
					<MermaidGraph graph={control_flow_graph} />
				</TabPanel>
			</Tabs>
		</div>
	);
}
