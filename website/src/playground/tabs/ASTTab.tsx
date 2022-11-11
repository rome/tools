import CodeMirror from "../CodeMirror";
import { romeAst } from "codemirror-lang-rome-ast";
import { ReactCodeMirrorRef } from "@uiw/react-codemirror";

interface Props {
  innerRef: ReactCodeMirrorRef;
  ast: string;
}

const romeAstCodeMirrorExtension = [romeAst()];

export default function ASTTab({ast, innerRef}: Props) {
  return <CodeMirror
    value={ast}
    ref={innerRef}
    extensions={romeAstCodeMirrorExtension}
    readOnly={true}
  />;
}
