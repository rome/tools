import mermaid from "mermaid";
import { useLayoutEffect, useRef } from "react";

mermaid.initialize({
	startOnLoad: true,
});

interface MermaidGraphProps {
	graph: string;
}

export default function MermaidGraph({ graph }: MermaidGraphProps) {
	const element = useRef<HTMLDivElement>(null);

	useLayoutEffect(() => {
		if (element.current) {
			element.current.removeAttribute("data-processed");
			mermaid.contentLoaded();
			(element.current.firstChild as SVGElement)?.style.removeProperty(
				"max-width",
			);
		}
	}, [graph]);

	if (graph === "") {
		return null;
	}

	return (
		<div className="h-screen overflow-scroll mermaid" ref={element}>{graph}</div>
	);
}
