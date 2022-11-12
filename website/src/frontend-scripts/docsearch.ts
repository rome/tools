import docsearch from "@docsearch/js";
import { matchesDark, setCurrentTheme } from "./util";

const docsearchContainer = document.querySelector(
	"#docsearch-target",
) as HTMLElement;

// We need to explicitly set data-theme as docsearch explicitly matches for it
if (matchesDark?.matches) {
	setCurrentTheme("dark");
}

docsearch({
	appId: "ZKNROT3Q65",
	apiKey: "6c573608bd6c44671bfc263fb83992e2",
	indexName: "rome",
	container: docsearchContainer,
});
