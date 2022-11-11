import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import PrettierHeader from "../components/PrettierHeader";
import RomeHeader from "../components/RomeHeader";
import { romeAst as RomeFormatterIr } from "lang-rome-formatter-ir";

interface Props {
  prettier: string;
  rome: string;
}

const romeFormatterIrCodeMirrorExtension = [RomeFormatterIr()];

export default function FormatterIRTab({rome, prettier}: Props) {
  return <>
    <Collapsible heading={<RomeHeader />}>
      <CodeMirror
        value={rome}
        extensions={romeFormatterIrCodeMirrorExtension}
        readOnly={true}
      />
    </Collapsible>
    <Collapsible heading={<PrettierHeader />}>
      <CodeMirror
        value={prettier}
        extensions={romeFormatterIrCodeMirrorExtension}
        readOnly={true}
      />
    </Collapsible>
  </>;
}
