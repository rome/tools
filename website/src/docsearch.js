import docsearch from "@docsearch/js";

const docsearchContainer = document.querySelector("#docsearch-target");

docsearch({
  appId: "ZKNROT3Q65",
  apiKey: "6c573608bd6c44671bfc263fb83992e2",
  indexName: "rome",
  container: docsearchContainer,
  debug: false, // Set debug to true if you want to inspect the dropdown
});
