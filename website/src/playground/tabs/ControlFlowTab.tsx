import { useMemo } from "react";
import mermaid from "mermaid";
import { useTheme } from "../utils";

interface Props {
	graph: string;
}

let initialized = false;

export default function ControlFlowTab({ graph }: Props) {
	if (graph === "") {
		return <div className="empty-panel">No control flow graph present</div>;
	}

	const theme = useTheme();

	if (!initialized) {
		initialized = true;
		mermaid.initialize({ startOnLoad: true });
	}

	graph = `%%{init: {'theme':'${
		theme === "dark" ? "dark" : "default"
	}'}}%%\n${graph}`;

	const graphSVG = useMemo(() => {
		return mermaid.render("graph-div", graph);
	}, [graph]);

	return (
		<div className="mermaid" dangerouslySetInnerHTML={{ __html: graphSVG }} />
	);
}
