import CodeMirror from "../CodeMirror";
import type { Extension } from "@codemirror/state";

interface Props {
	code: string;
	extensions: Extension[];
}

export default function ImportSortingTab({ code, extensions }: Props) {
	return <CodeMirror value={code} extensions={extensions} readOnly={true} />;
}
