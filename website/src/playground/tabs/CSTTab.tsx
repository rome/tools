import CodeMirror from "../CodeMirror";

interface Props {
  cst: string;
}

export default function CSTTab({cst}: Props) {
  return <CodeMirror
  value={cst}
  readOnly={true}
/>;
}
