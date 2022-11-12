import type { PlaygroundProps, RomeAstSyntacticData } from "./types";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import CodeMirror from "./CodeMirror";
import type { ViewUpdate } from "@codemirror/view";
import * as codeMirrorLangRomeAST from "codemirror-lang-rome-ast";
import { javascript } from "@codemirror/lang-javascript";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import SettingsPane from "./components/SettingsPane";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { EditorSelection } from "@codemirror/state";
import SyntaxTab from "./tabs/SyntaxTab";
import ControlFlowTab from "./tabs/ControlFlowTab";
import DiagnosticsTab from "./tabs/DiagnosticsTab";
import FormatterCodeTab from "./tabs/FormatterCodeTab";
import FormatterIRTab from "./tabs/FormatterIRTab";
import { useWindowSize } from "./utils";

export default function PlaygroundLoader({
	setPlaygroundState,
	resetPlaygroundState,
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
	const { jsx: isJsx, typescript: isTypeScript } = settings;
	const [clipboardStatus, setClipboardStatus] = useState<
		"success" | "failed" | "normal"
	>("normal");
	const extensions = useMemo(
		() => [
			javascript({
				jsx: isJsx,
				typescript: isTypeScript,
			}),
		],
		[isJsx, isTypeScript],
	);

	const romeAstSyntacticDataRef = useRef<RomeAstSyntacticData | null>(null);

	const astPanelCodeMirrorRef = useRef<null | ReactCodeMirrorRef>(null);

	useEffect(() => {
		if (clipboardStatus !== "normal") {
			setClipboardStatus("normal");
		}
	}, [formatter_ir]);

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

	useEffect(() => {
		scrollAstNodeIntoView(settings.cursorPosition);
	}, [settings.cursorPosition]);

	// We update the syntactic data of `RomeJsAst` only AstSource(`Display` string of our original AstRepresentation) changed.
	useEffect(() => {
		let tree = codeMirrorLangRomeAST.parser.parse(ast);
		let rangeMap = new Map();
		romeAstSyntacticDataRef.current = {
			ast: tree,
			rangeMap,
		};
		tree.iterate({
			enter(node) {
				if (node.type.name === "SyntaxToken") {
					let range = node.node.getChild("Range");
					if (!range) {
						return;
					}
					let current = range.firstChild;
					// Checking if current node is broken
					while (current) {
						if (current.type.isError) {
							return;
						}
						current = current.nextSibling;
					}

					const children = range.node.getChildren("Number");
					let first = children.at(0)?.node;
					let second = children.at(1)?.node;
					if (first && second) {
						let start = +ast.slice(first.from, first.to);
						let end = +ast.slice(second.from, second.to);
						rangeMap.set([start, end], [node.from, node.to]);
					}
				}
			},
		});
	}, [ast]);

	const onChange = useCallback((value) => {
		setPlaygroundState((state) => ({ ...state, code: value }));
	}, []);

	const { width } = useWindowSize();
	const hasNarrowViewport = width !== undefined && width <= 1000;

	const editor = (
		<CodeMirror
			value={code}
			extensions={extensions}
			placeholder="Enter your code here"
			onUpdate={onUpdate}
			onChange={onChange}
		/>
	);

	const settingsPane = (
		<SettingsPane onReset={resetPlaygroundState} settings={settings} setPlaygroundState={setPlaygroundState} />
	);

	return (
		<>
			{!hasNarrowViewport && settingsPane}

			{!hasNarrowViewport && <div className="code-pane">{editor}</div>}

			<Tabs
				className="preview-pane"
				selectedTabPanelClassName="react-tabs__tab-panel--selected"
			>
				<TabList>
					{hasNarrowViewport && <Tab>Code</Tab>}
					{hasNarrowViewport && <Tab>Settings</Tab>}
					<Tab>Formatter</Tab>
					<Tab>Diagnostics</Tab>
					<Tab>Syntax</Tab>
					<Tab>IR</Tab>
					<Tab>Control Flow Graph</Tab>
				</TabList>
				{hasNarrowViewport && <TabPanel>{editor}</TabPanel>}
				{hasNarrowViewport && <TabPanel>{settingsPane}</TabPanel>}
				<TabPanel>
					<FormatterCodeTab
						rome={formatted_code}
						prettier={prettierOutput.code}
						extensions={extensions}
					/>
				</TabPanel>
				<TabPanel>
					<DiagnosticsTab errors={errors} />
				</TabPanel>
				<TabPanel>
					<SyntaxTab ast={ast} cst={cst} ref={astPanelCodeMirrorRef} />
				</TabPanel>
				<TabPanel>
					<FormatterIRTab rome={formatter_ir} prettier={prettierOutput.ir} />
				</TabPanel>
				<TabPanel>
					<ControlFlowTab graph={control_flow_graph} />
				</TabPanel>
			</Tabs>
		</>
	);

	function scrollAstNodeIntoView(cursorPosition: number) {
		if (astPanelCodeMirrorRef.current && romeAstSyntacticDataRef.current) {
			let codemirror = astPanelCodeMirrorRef.current;
			let syntacticData = romeAstSyntacticDataRef.current;
			let { view } = codemirror;
			let { rangeMap } = syntacticData;
			for (let [sourceRange, displaySourceRange] of rangeMap.entries()) {
				if (
					cursorPosition >= sourceRange[0] &&
					cursorPosition <= sourceRange[1]
				) {
					view?.dispatch({
						scrollIntoView: true,
						selection: EditorSelection.create([
							EditorSelection.range(
								displaySourceRange[0],
								displaySourceRange[1],
							),
							EditorSelection.cursor(displaySourceRange[0]),
						]),
					});
				}
			}
		}
	}
}
