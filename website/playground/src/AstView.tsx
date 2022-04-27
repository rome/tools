import ReactJson from "react-json-view";
import { cleanUpAst } from "./utils";

interface Props { ast: string }

export default function AstView({ ast }: Props) {
	return <ReactJson src={cleanUpAst(JSON.parse(ast))} />;
}
