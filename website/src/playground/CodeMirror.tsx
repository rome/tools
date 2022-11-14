import type { Diagnostic as RomeDiagnostic } from "@rometools/wasm-web";
import type {
	ReactCodeMirrorProps,
	ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import type { Extension } from "@codemirror/state";
import type { Diagnostic as CodeMirrorDiagnostic } from "@codemirror/lint";
import { EditorView } from "@codemirror/view";
import RealCodeMirror from "@uiw/react-codemirror";
import { forwardRef, useEffect, useMemo, useState } from "react";
import { useTheme } from "./utils";
import { lintGutter, setDiagnostics } from "@codemirror/lint";

interface Props extends ReactCodeMirrorProps {
	diagnostics?: RomeDiagnostic[];
}

function getDiagnosticMessage(diagnostic: RomeDiagnostic): string {
	let buf = "";
	for (const elem of diagnostic.message) {
		buf += elem.content;
	}
	return buf;
}

function romeDiagnosticsToCodeMirror(
	rome: RomeDiagnostic[],
): CodeMirrorDiagnostic[] {
	const codeMirror: CodeMirrorDiagnostic[] = [];

	for (const diag of rome) {
		const span = diag.location?.span;
		if (span === undefined) {
			continue;
		}

		let severity: CodeMirrorDiagnostic["severity"];
		switch (diag.severity) {
			case "Error":
			case "Fatal": {
				severity = "error";
				break;
			}

			case "Information": {
				severity = "info";
				break;
			}

			case "Warning": {
				severity = "warning";
				break;
			}

			default: {
				severity = "error";
			}
		}

		codeMirror.push({
			from: span[0],
			to: span[1],
			severity,
			message: getDiagnosticMessage(diag),
		});
	}

	return codeMirror;
}

function getDefaultExtensions(extensions: Extension[] = []) {
	return [EditorView.lineWrapping, ...extensions];
}

export default forwardRef<ReactCodeMirrorRef, Props>(function CodeMirror(
	{ diagnostics, ...props },
	ref,
) {
	const theme = useTheme();

	let [editor, setEditor] = useState<EditorView>();

	function onCreateEditor(editor: EditorView) {
		setEditor(editor);
	}

	const extensions = useMemo(() => {
		if (diagnostics === undefined) {
			return getDefaultExtensions(props.extensions);
		}

		return [lintGutter(), ...getDefaultExtensions(props.extensions)];
	}, [diagnostics, props.extensions]);

	useEffect(() => {
		if (editor !== undefined && diagnostics !== undefined) {
			editor.dispatch(
				setDiagnostics(editor.state, romeDiagnosticsToCodeMirror(diagnostics)),
			);
		}
	}, [editor, diagnostics]);

	return (
		<RealCodeMirror
			{...props}
			extensions={extensions}
			onCreateEditor={onCreateEditor}
			ref={ref}
			theme={theme}
		/>
	);
});
