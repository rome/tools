import docsearch from "@docsearch/js";

const docsearchContainer = document.querySelector("#docsearch-target") as HTMLElement;

docsearch({
	appId: "ZKNROT3Q65",
	apiKey: "6c573608bd6c44671bfc263fb83992e2",
	indexName: "rome",
	container: docsearchContainer,
});
