import { PlaygroundProps } from "./types";
import CodeMirror, { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import type { ViewUpdate } from "@codemirror/view";
// import { EditorView } from "@codemirror/view";
// import { romeAst } from "../lang-rome-ast/dist/";
import { romeAst, parser } from "codemirror-lang-rome-ast";
import { romeAst as RomeFormatterIr } from "lang-rome-formatter-ir";
import { javascript } from "@codemirror/lang-javascript";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import { SettingsMenu } from "./SettingsMenu";
import TreeView from "./TreeView";
import { useCallback, useEffect, useRef, useState } from "react";
import MermaidGraph from "./MermaidGraph";
import { EditorSelection } from "@codemirror/state";

export default function DesktopPlayground({
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
	const [clipboardStatus, setClipboardStatus] = useState<
		"success" | "failed" | "normal"
	>("normal");
	const [selectedIndex, setSelectedIndex] = useState(0);
	const extensions = [
		javascript({
			jsx: isJsx,
			typescript: isTypeScript,
		}),
	];

	const romeAstCodeMirrorExtension = [romeAst()];
	const romeFormatterIrCodeMirrorExtension = [RomeFormatterIr()];

	const codemirrorRef: React.Ref<ReactCodeMirrorRef> = useRef(null);
	useEffect(() => {
		if (clipboardStatus !== "normal") {
			setClipboardStatus("normal");
		}
	}, [formatter_ir]);

	const onUpdate = useCallback((viewUpdate: ViewUpdate) => {
		const cursorPosition = viewUpdate.state.selection.ranges[0]?.from ?? 0;
		console.log(codemirrorRef);
		if (codemirrorRef.current) {
			let current = codemirrorRef.current;

			let { view, state } = current;
			let parsedAst = parser.parse(ast);
			// console.log(ast.topNode.type.isError)
			console.log(cursorPosition)
			parsedAst.iterate({
				enter(node) {
					if (node.type.name === "SyntaxToken") {
						let range = node.node.getChild("Range");
						if (!range) {
							return;
						}
						// console.log(range.type.)
						let current = range.firstChild;
						while (current) {
							if (current.type.isError) {
								return;
							}
							current = current.nextSibling;
						}

						const children = range.node.getChildren("Number");
						// console.log(children.at(1));
						let first = children.at(0)?.node;
						let second = children.at(1)?.node;
						if (first && second) {
							let start = +ast.slice(first.from, first.to);
							let end = +ast.slice(second.from, second.to);
							if (cursorPosition >= start && cursorPosition <= end) {
								view?.dispatch({
									scrollIntoView: true,
									selection: EditorSelection.create([
										EditorSelection.range(node.from, node.to),
										EditorSelection.cursor(node.from),
									]),
								});
							}
						}
						// // const lastChild = range.node.getChild('Number');
						// firstChild.from
						// console.log(range.node.getChild(3))
					}
				},
			});
		}
		setPlaygroundState((state) =>
			state.cursorPosition !== cursorPosition
				? {
						...state,
						cursorPosition,
				  }
				: state
		);
	}, []);
	const onChange = useCallback((value) => {
		setPlaygroundState((state) => ({ ...state, code: value }));
	}, []);

	return (
		<div className="divide-y divide-slate-300 h-screen flex flex-col">
			<h1 className="p-4 text-xl">Rome Playground</h1>
			<SettingsMenu
				settings={settings}
				setPlaygroundState={setPlaygroundState}
			/>
			<div className="box-border flex divide-x divide-slate-300 flex-1 overflow-auto">
				<CodeMirror
					value={code}
					className="h-full overflow-y-hidden w-1/2 p-5 h-full"
					height="100%"
					extensions={extensions}
					placeholder="Enter your code here"
					onUpdate={onUpdate}
					onChange={onChange}
				/>
				<Tabs
					className="w-1/2 p-5 flex flex-col"
					selectedTabPanelClassName="flex-1 react-tabs__tab-panel--selected overflow-y-auto"
					onSelect={(index, last) => {
						// console.log(index)
					}}
				>
					<TabList>
						<Tab selectedClassName="bg-slate-300">Formatter</Tab>
						<Tab selectedClassName="bg-slate-300">CST</Tab>
						<Tab selectedClassName="bg-slate-300">AST</Tab>
						<Tab selectedClassName="bg-slate-300">Rome IR</Tab>
						<Tab selectedClassName="bg-slate-300">Prettier IR</Tab>
						<Tab disabled={errors === ""} selectedClassName="bg-slate-300">
							Diagnostics
						</Tab>
						<Tab
							disabled={control_flow_graph === ""}
							selectedClassName="bg-slate-300"
						>
							Control Flow Graph
						</Tab>
					</TabList>
					<TabPanel>
						<div className="h-1/2 flex flex-col pb-4">
							<h1 className="text-lg font-medium pb-2">Rome</h1>
							<CodeMirror
								value={formatted_code}
								extensions={extensions}
								placeholder="Rome Output"
								className="flex-1 overflow-y-auto"
								height="100%"
								readOnly={true}
							/>
						</div>
						<div className="h-1/2 flex flex-col">
							<h1 className="text-lg font-medium pb-2">Prettier</h1>
							<CodeMirror
								value={prettierOutput.code}
								extensions={extensions}
								placeholder="Prettier Output"
								className="flex-1 overflow-y-auto"
								height="100%"
								readOnly={true}
							/>
						</div>
					</TabPanel>
					<TabPanel>
						<TreeView tree={cst} />
					</TabPanel>
					<TabPanel>
						<CodeMirror
							value={ast}
							ref={codemirrorRef}
							extensions={romeAstCodeMirrorExtension}
							className="h-full"
							height="100%"
							readOnly={true}
						/>
					</TabPanel>
					<TabPanel>
						<CodeMirror
							value={formatter_ir}
							extensions={romeFormatterIrCodeMirrorExtension}
							className="h-full"
							height="100%"
							readOnly={true}
						/>
					</TabPanel>
					<TabPanel>
						<CodeMirror
							value={prettierOutput.ir}
							extensions={romeFormatterIrCodeMirrorExtension}
							className="h-full"
							height="100%"
							readOnly={true}
						/>
					</TabPanel>
					<TabPanel>
						<div
							className="overflow-scroll whitespace-pre-wrap text-red-500 text-xs error-panel h-full"
							dangerouslySetInnerHTML={{ __html: errors }}
						/>
					</TabPanel>
					<TabPanel>
						<MermaidGraph graph={control_flow_graph} />
					</TabPanel>
				</Tabs>
			</div>
		</div>
	);
}
