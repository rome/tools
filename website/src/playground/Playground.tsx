import type { PlaygroundProps, RomeAstSyntacticData } from "./types";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import CodeMirror from "./CodeMirror";
import type { ViewUpdate } from "@codemirror/view";
import * as codeMirrorLangRomeAST from "codemirror-lang-rome-ast";
import { javascript } from "@codemirror/lang-javascript";
import { json } from "@codemirror/lang-json";
import SettingsPane from "./components/SettingsPane";
import {
	createRef,
	useCallback,
	useEffect,
	useMemo,
	useRef,
	useState,
} from "react";
import { EditorSelection } from "@codemirror/state";
import SyntaxTab from "./tabs/SyntaxTab";
import ControlFlowTab from "./tabs/ControlFlowTab";
import FormatterCodeTab from "./tabs/FormatterCodeTab";
import FormatterIRTab from "./tabs/FormatterIRTab";
import {
	getCurrentCode,
	getFileState,
	isJSONFilename,
	isJSXFilename,
	isTypeScriptFilename,
	useWindowSize,
} from "./utils";
import Resizable from "./components/Resizable";
import DiagnosticsPane from "./components/DiagnosticsPane";
import Tabs from "./components/Tabs";
import DiagnosticsConsoleTab from "./tabs/DiagnosticsConsoleTab";
import DiagnosticsListTab from "./tabs/DiagnosticsListTab";
import SettingsTab from "./tabs/SettingsTab";

export default function PlaygroundLoader({
	setPlaygroundState,
	resetPlaygroundState,
	playgroundState,
}: PlaygroundProps) {
	const [clipboardStatus, setClipboardStatus] = useState<
		"success" | "failed" | "normal"
	>("normal");

	const file = getFileState(playgroundState, playgroundState.currentFile);
	const romeOutput = file.rome;
	const prettierOutput = file.prettier;

	const codeMirrorExtensions = useMemo(() => {
		if (isJSONFilename(playgroundState.currentFile)) {
			return [json()];
		} else {
			return [
				javascript({
					jsx: isJSXFilename(playgroundState.currentFile),
					typescript: isTypeScriptFilename(playgroundState.currentFile),
				}),
			];
		}
	}, [playgroundState.currentFile]);

	const romeAstSyntacticDataRef = useRef<RomeAstSyntacticData | null>(null);

	const astPanelCodeMirrorRef = useRef<null | ReactCodeMirrorRef>(null);

	useEffect(() => {
		if (clipboardStatus !== "normal") {
			setClipboardStatus("normal");
		}
	}, [romeOutput.formatter.ir]);

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
		scrollAstNodeIntoView(playgroundState.cursorPosition);
	}, [playgroundState.cursorPosition]);

	// We update the syntactic data of `RomeJsAst` only AstSource(`Display` string of our original AstRepresentation) changed.
	useEffect(() => {
		const ast = romeOutput.syntax.ast;
		const tree = codeMirrorLangRomeAST.parser.parse(ast);
		const rangeMap = new Map();
		romeAstSyntacticDataRef.current = {
			ast: tree,
			rangeMap,
		};
		tree.iterate({
			enter(node) {
				if (node.type.name === "SyntaxToken") {
					const range = node.node.getChild("Range");
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
					const first = children.at(0)?.node;
					const second = children.at(1)?.node;
					if (first && second) {
						const start = +ast.slice(first.from, first.to);
						const end = +ast.slice(second.from, second.to);
						rangeMap.set([start, end], [node.from, node.to]);
					}
				}
			},
		});
	}, [romeOutput.syntax.ast]);

	const onChange = useCallback((value: string) => {
		setPlaygroundState((state) => ({
			...state,
			files: {
				...state.files,
				[state.currentFile]: {
					...getFileState(state, state.currentFile),
					content: value,
				},
			},
		}));
	}, []);

	const { width } = useWindowSize();
	const hasNarrowViewport = width !== undefined && width <= 1000;

	const editorRef = createRef<ReactCodeMirrorRef>();

	const code = getCurrentCode(playgroundState) ?? "";

	const editor = (
		<CodeMirror
			ref={editorRef}
			diagnostics={romeOutput.diagnostics.list}
			value={code}
			extensions={codeMirrorExtensions}
			placeholder="Enter your code here"
			onUpdate={onUpdate}
			onChange={onChange}
			autoFocus={true}
		/>
	);

	const results = (
		<Tabs
			className="results-tabs"
			selectedTab={playgroundState.tab}
			onSelect={(tab) => setPlaygroundState((state) => ({ ...state, tab }))}
			tabs={[
				{
					key: "code",
					title: "Code",
					visible: hasNarrowViewport,
					children: editor,
				},
				{
					key: "diagnostics",
					title: "Diagnostics",
					visible: hasNarrowViewport,
					children: (
						<DiagnosticsListTab
							editorRef={editorRef}
							diagnostics={romeOutput.diagnostics.list}
						/>
					),
				},
				{
					key: "formatter",
					title: "Formatter",
					children: (
						<FormatterCodeTab
							rome={romeOutput.formatter.code}
							prettier={prettierOutput}
							extensions={codeMirrorExtensions}
						/>
					),
				},
				{
					key: "formatter-ir",
					title: "Formatter IR",
					children: (
						<FormatterIRTab
							rome={romeOutput.formatter.ir}
							prettier={prettierOutput}
						/>
					),
				},
				{
					key: "syntax",
					title: "Syntax",
					children: (
						<SyntaxTab
							ast={romeOutput.syntax.ast}
							cst={romeOutput.syntax.cst}
							ref={astPanelCodeMirrorRef}
						/>
					),
				},
				{
					key: "cfg",
					title: "Control Flow Graph",
					children: (
						<ControlFlowTab graph={romeOutput.analysis.controlFlowGraph} />
					),
				},
				{
					key: "Console",
					title: "Console",
					visible: hasNarrowViewport,
					children: (
						<DiagnosticsConsoleTab console={romeOutput.diagnostics.console} />
					),
				},
				{
					key: "settings",
					title: "Settings",
					visible: hasNarrowViewport,
					children: (
						<SettingsTab
							onReset={resetPlaygroundState}
							state={playgroundState}
							setPlaygroundState={setPlaygroundState}
						/>
					),
				},
			]}
		/>
	);

	if (hasNarrowViewport) {
		return results;
	}

	return (
		<>
			<SettingsPane
				onReset={resetPlaygroundState}
				state={playgroundState}
				setPlaygroundState={setPlaygroundState}
			/>

			<div className="code-pane">
				{editor}
				<Resizable
					className="diagnostics-pane"
					name="diagnostics"
					direction="top"
				>
					<DiagnosticsPane
						editorRef={editorRef}
						console={romeOutput.diagnostics.console}
						diagnostics={romeOutput.diagnostics.list}
					/>
				</Resizable>
			</div>

			<Resizable className="results-pane" name="results-pane" direction="left">
				{results}
			</Resizable>
		</>
	);

	function scrollAstNodeIntoView(cursorPosition: number) {
		if (
			astPanelCodeMirrorRef.current == null ||
			romeAstSyntacticDataRef.current == null
		) {
			return;
		}

		const view = astPanelCodeMirrorRef.current.view;
		const rangeMap = romeAstSyntacticDataRef.current.rangeMap;

		for (const [sourceRange, displaySourceRange] of rangeMap.entries()) {
			if (
				cursorPosition >= sourceRange[0] &&
				cursorPosition <= sourceRange[1]
			) {
				view?.dispatch({
					scrollIntoView: true,
					selection: EditorSelection.create([
						EditorSelection.range(displaySourceRange[0], displaySourceRange[1]),
						EditorSelection.cursor(displaySourceRange[0]),
					]),
				});
			}
		}
	}
}
