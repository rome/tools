import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import PrettierHeader from "../components/PrettierHeader";
import RomeHeader from "../components/RomeHeader";
import type { PrettierOutput } from "../types";
import { romeAst as RomeFormatterIr } from "lang-rome-formatter-ir";

interface Props {
	prettier: PrettierOutput;
	rome: string;
}

const romeFormatterIrCodeMirrorExtension = [RomeFormatterIr()];

export default function FormatterIRTab({ rome, prettier }: Props) {
	return (
		<>
			<Collapsible className="rome" heading={<RomeHeader />}>
				<CodeMirror
					value={rome}
					extensions={romeFormatterIrCodeMirrorExtension}
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible className="prettier" heading={<PrettierHeader />}>
				{prettier.type === "ERROR" ? (
					<CodeMirror value={prettier.stack} readOnly={true} />
				) : (
					<CodeMirror
						value={prettier.ir}
						extensions={romeFormatterIrCodeMirrorExtension}
						readOnly={true}
					/>
				)}
			</Collapsible>
		</>
	);
}
