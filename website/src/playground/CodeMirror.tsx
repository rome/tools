import type {ReactCodeMirrorProps, ReactCodeMirrorRef} from "@uiw/react-codemirror";
import RealCodeMirror from "@uiw/react-codemirror";
import {forwardRef} from "react";
import { useTheme } from "./utils";

export default forwardRef<ReactCodeMirrorRef, ReactCodeMirrorProps>(function CodeMirror(props, ref) {
  const theme = useTheme();

  return <RealCodeMirror {...props} ref={ref} theme={theme} />;
});
