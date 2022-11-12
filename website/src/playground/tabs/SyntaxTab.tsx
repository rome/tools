import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import React from "react";
import CodeMirror from "../CodeMirror";
import { romeAst } from "codemirror-lang-rome-ast";
import Collapsible from "../Collapsible";

interface Props {
	ast: string;
	cst: string;
}

const romeAstCodeMirrorExtension = [romeAst()];

export default React.forwardRef<ReactCodeMirrorRef, Props>(function SyntaxTab(
	{ ast, cst },
	ref,
) {
	return (
		<>
			<Collapsible heading="AST">
				<CodeMirror
					value={ast}
					ref={ref}
					extensions={romeAstCodeMirrorExtension}
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible heading="CST">
				<CodeMirror value={cst} readOnly={true} />
			</Collapsible>
		</>
	);
});
