import CodeMirror, { RomeExtension } from "../CodeMirror";
interface Props {
	code: string;
	extensions: RomeExtension[];
}

export default function ImportSortingTab({ code, extensions }: Props) {
	return <CodeMirror value={code} extensions={extensions} readOnly={true} />;
}
