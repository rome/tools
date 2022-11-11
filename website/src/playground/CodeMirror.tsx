import RealCodeMirror from "@uiw/react-codemirror";
import {useEffect, useState} from "react";

export default function CodeMirror(props) {
  const [theme, setTheme] = useState(getCurrentTheme());

  useEffect(() => {
    function onColorSchemeChange() {
      setTheme(getCurrentTheme());
    }

    window.addEventListener("colorschemechange", onColorSchemeChange);

    return function cleanup() {
      window.removeEventListener("colorschemechange", onColorSchemeChange);
    };
  });

  return <RealCodeMirror {...props} theme={theme} />;
}
