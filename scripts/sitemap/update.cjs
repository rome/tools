require("../_setup.cjs");

const {readGeneratedFile, write, readFile} = require("../_utils.cjs");
const {root} = require("../_constants.cjs");
const path = require("path");
const fs = require("fs");

const src = path.join(root, "website", "src");
const sitemapFile = path.join(src, "sitemap.md");
let sitemapTemplate = readGeneratedFile(sitemapFile, false, "html");

const tree = buildTree(src);

function buildTree(folder) {
	let children = [];

	const files = fs.readdirSync(folder).sort();
	for (const name of files) {
		if (name[0] === "_") {
			continue;
		}

		const loc = path.join(folder, name);
		const url = `/${path.relative(src, loc)}`;

		const stats = fs.statSync(loc);
		if (stats.isFile() && name.endsWith(".md")) {
			const file = readFile(loc);
			const headingMatch = file.match(/# (.*?)\n/);
			const heading = headingMatch == null ? undefined : headingMatch[1];

			children.push({
				type: "file",
				heading,
				link: true,
				children: [],
				// Remove .md
				url: url.slice(0, -3),
				name: name.slice(0, -3),
			});
		} else if (stats.isDirectory()) {
			const node = buildTree(loc);
			if (node.children.length > 0 || node.link) {
				children.push(node);
			}
		}
	}

	let index;
	children = children.filter((child) => {
		if (child.name === "index") {
			index = child;
			return false;
		} else {
			return true;
		}
	});

	return {
		type: "folder",
		url: `/${path.relative(src, folder)}`,
		link: index !== undefined,
		heading: index && index.heading,
		name: folder === src ? "" : path.basename(folder),
		children,
	};
}

function pushList(node, level) {
	let text = `\`/${node.name}\``;

	if (node.link) {
		text = `[${text}](${node.url})`;
	}

	if (node.heading) {
		text += `: ${node.heading}`;
	}

	sitemapTemplate += `${"  ".repeat(level)} - ${text}\n`;

	if (node.type === "folder") {
		for (const child of node.children) {
			pushList(child, level + 1);
		}
	}
}

pushList(tree, 0);

write(sitemapFile, sitemapTemplate);
