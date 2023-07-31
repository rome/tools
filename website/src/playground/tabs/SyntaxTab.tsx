import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { romeAst } from "codemirror-lang-rome-ast";
import React from "react";

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
