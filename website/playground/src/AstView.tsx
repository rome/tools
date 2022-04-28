import ReactJson from "react-json-view";

interface Props {
	ast: string;
}

export default function AstView({ ast }: Props) {
	return <ReactJson src={JSON.parse(ast)} />;
}
