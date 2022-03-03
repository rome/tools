import "./App.css";
import CodeEditor from "@uiw/react-textarea-code-editor";
import "react-tabs/style/react-tabs.css";
import init, { run } from "../pkg/rome_playground";
import { Tabs, Tab, TabList, TabPanel } from "react-tabs";
import { useEffect, useState } from "react";
import prettier from "prettier";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import IndentStyleSelect from "./IndentStyleSelect";
import LineWidthInput from "./LineWidthInput";
import { IndentStyle } from "./types";
import SourceTypeSelect from "./SourceTypeSelect";

enum LoadingState {
  Loading,
  Success,
  Error,
}

function formatWithPrettier(
  code: string,
  options: { lineWidth: number; indentStyle: IndentStyle; indentWidth: number }
) {
  try {
    return prettier.format(code, {
      useTabs: options.indentStyle === IndentStyle.Tab,
      tabWidth: options.indentWidth,
      printWidth: options.lineWidth,
      parser: "babel",
      plugins: [parserBabel],
    });
  } catch (err) {
    return code;
  }
}

function App() {
  useEffect(() => {
    init()
      .then(() => {
        setLoadingState(LoadingState.Success);
      })
      .catch(() => {
        setLoadingState(LoadingState.Error);
      });
  }, []);

  const searchParams = new URLSearchParams(window.location.search);
  const [loadingState, setLoadingState] = useState(LoadingState.Loading);
  const [code, setCode] = useState(
    window.location.hash !== "#" ? atob(window.location.hash.substring(1)) : ""
  );
  const [lineWidth, setLineWidth] = useState(
    parseInt(searchParams.get("lineWidth") ?? "80")
  );
  const [indentStyle, setIndentStyle] = useState(
    (searchParams.get("indentStyle") as IndentStyle) ?? IndentStyle.Tab
  );
  const [indentWidth, setIndentWidth] = useState(
    parseInt(searchParams.get("indentWidth") ?? "2")
  );
  const [isTypeScript, setIsTypeScript] = useState(
    searchParams.get("typescript") === "true"
  );
  const [isJsx, setIsJsx] = useState(searchParams.get("jsx") === "true");

  useEffect(() => {
    const url = `${window.location.protocol}//${window.location.host}${
      window.location.pathname
    }?lineWidth=${lineWidth}&indentStyle=${indentStyle}&indentWidth=${indentWidth}&typescript=${isTypeScript}&jsx=${isJsx}#${btoa(
      code
    )}`;
    window.history.pushState({ path: url }, "", url);
  }, [lineWidth, indentStyle, indentWidth, code, isTypeScript, isJsx]);

  switch (loadingState) {
    case LoadingState.Error:
      return <div>Error loading. Please refresh</div>;
    case LoadingState.Loading:
      return (
        <div className="h-screen w-screen flex align-center justify-center">
          Loading...
        </div>
      );
    default:
      const { cst, ast, formatted_code, formatter_ir, errors } = run(
        code,
        lineWidth,
        indentStyle === IndentStyle.Tab,
        indentWidth,
        isTypeScript,
        isJsx
      );
      return (
        <div className="divide-y divide-slate-300">
          <h1 className="p-4 text-xl">Rome Playground</h1>
          <div>
            <LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />
            <IndentStyleSelect
              indentWidth={indentWidth}
              setIndentWidth={setIndentWidth}
              indentStyle={indentStyle}
              setIndentStyle={setIndentStyle}
            />
            <SourceTypeSelect
              isTypeScript={isTypeScript}
              setIsTypeScript={setIsTypeScript}
              isJsx={isJsx}
              setIsJsx={setIsJsx}
            />
          </div>
          <div className="box-border flex h-screen divide-x divide-slate-300">
            <div className="w-1/2 p-5">
              <CodeEditor
                value={code}
                language="js"
                placeholder="Enter JS here"
                onChange={(evn) => {
                  setCode(evn.target.value);
                }}
                style={{
                  fontSize: 12,
                  height: "100vh",
                  fontFamily:
                    "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
                }}
              />
            </div>
            <div className="w-1/2 p-5 flex flex-col">
              <Tabs>
                <TabList>
                  <Tab selectedClassName="bg-slate-300">Formatter</Tab>
                  <Tab selectedClassName="bg-slate-300">CST</Tab>
                  <Tab selectedClassName="bg-slate-300">AST</Tab>
                  <Tab selectedClassName="bg-slate-300">Formatter IR</Tab>
                  <Tab
                    disabled={errors === ""}
                    selectedClassName="bg-slate-300"
                  >
                    Errors
                  </Tab>
                </TabList>
                <TabPanel>
                  <h1>Rome</h1>
                  <CodeEditor
                    value={formatted_code}
                    language="js"
                    placeholder="Rome Output"
                    style={{
                      fontSize: 12,
                      height: "40vh",
                      overflowY: "scroll",
                      fontFamily:
                        "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
                    }}
                  />
                  <h1>Prettier</h1>
                  <CodeEditor
                    value={formatWithPrettier(code, {
                      lineWidth,
                      indentStyle,
                      indentWidth,
                    })}
                    language="js"
                    placeholder="Prettier Output"
                    style={{
                      fontSize: 12,
                      height: "50vh",
                      overflowY: "scroll",
                      fontFamily:
                        "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
                    }}
                  />
                </TabPanel>
                <TabPanel>
                  <pre className="h-screen overflow-y-scroll">{cst}</pre>
                </TabPanel>
                <TabPanel>
                  <pre className="h-screen overflow-y-scroll">{ast}</pre>
                </TabPanel>
                <TabPanel>
                  <pre className="h-screen overflow-y-scroll">
                    {formatter_ir}
                  </pre>
                </TabPanel>
                <TabPanel>
                  <pre className="h-screen overflow-y-scroll whitespace-pre-wrap text-red-500 text-xs">
                    {errors}
                  </pre>
                </TabPanel>
              </Tabs>
            </div>
          </div>
        </div>
      );
  }
}

export default App;
