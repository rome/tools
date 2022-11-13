import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import PrettierHeader from "../components/PrettierHeader";
import RomeHeader from "../components/RomeHeader";
import fastDiff from "fast-diff";

interface Props {
	prettier: string;
	rome: string;
	extensions: any[];
}

function removeWhitespace(str: string): string {
	return str.replace(/\s/g, "");
}

function calculateHint(a: string, b: string): string | JSX.Element {
	if (a === b) {
		return <strong>Exact match</strong>;
	} else if (removeWhitespace(a) === removeWhitespace(b)) {
		return <strong>Only whitespace differences</strong>;
	}

	const diff = fastDiff(a, b);
	let insertions = 0;
	let deletions = 0;

	for (const [type] of diff) {
		if (type === fastDiff.INSERT) {
			insertions++;
		} else if (type === fastDiff.DELETE) {
			deletions++;
		}
	}

	return (
		<>
			<span className="insertions">+{insertions}</span>{" "}
			<span className="deletions">-{deletions}</span>
		</>
	);
}

export default function FormatterCodeTab({
	rome,
	prettier,
	extensions,
}: Props) {
	const hint = calculateHint(prettier, rome);

	return (
		<>
			<Collapsible className="rome" heading={<RomeHeader />}>
				<CodeMirror
					value={rome}
					extensions={extensions}
					placeholder="Rome Output"
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible
				className="prettier"
				heading={
					<>
						<PrettierHeader />
						<span className="diff-hint">{hint}</span>
					</>
				}
			>
				<CodeMirror
					value={prettier}
					extensions={extensions}
					placeholder="Prettier Output"
					readOnly={true}
				/>
			</Collapsible>
		</>
	);
}
