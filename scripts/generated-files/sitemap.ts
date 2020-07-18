import {ROOT, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory, readFileText} from "@romefrontend/fs";
import {AbsoluteFilePath} from "@romefrontend/path";

type Node = {
	type: "file" | "folder";
	heading: undefined | string;
	link: boolean;
	url: string;
	name: string;
	children: Array<Node>;
};

// Files we don't want to show up in the sitemap
const DENYLIST = ["sitemap.md"];

export async function main() {
	const websiteSource = ROOT.appendList("website", "src");

	await modifyGeneratedFile(
		websiteSource.append("sitemap.md"),
		async () => {
			async function buildTree(folderPath: AbsoluteFilePath): Promise<Node> {
				let children: Array<Node> = [];

				for (const path of await readDirectory(folderPath)) {
					const name = path.getExtensionlessBasename();
					if (name[0] === "_") {
						continue;
					}

					const relative = websiteSource.relative(path);
					if (DENYLIST.includes(relative.join())) {
						continue;
					}

					const url = `/${relative.changeBasename(
						relative.getExtensionlessBasename(),
					).join()}`;

					const stats = await lstat(path);
					if (stats.isFile() && path.hasEndExtension("md")) {
						const file = await readFileText(path);
						const headingMatch = file.match(/# (.*?)\n/);
						const heading = headingMatch == null ? undefined : headingMatch[1];

						children.push({
							type: "file",
							heading,
							link: true,
							children: [],
							url,
							name,
						});
					} else if (stats.isDirectory()) {
						const node = await buildTree(path);
						if (node.children.length > 0 || node.link) {
							children.push(node);
						}
					}
				}

				let index: undefined | Node;
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
					url: folderPath.equal(websiteSource)
						? "/"
						: `/${websiteSource.relative(folderPath).join()}`,
					link: index !== undefined,
					heading: index === undefined ? undefined : index.heading,
					name: folderPath === websiteSource ? "" : folderPath.getBasename(),
					children,
				};
			}

			function pushList(node: Node, level: number) {
				let text = `\`/${node.name}\``;

				if (node.link) {
					text = `[${text}](${node.url})`;
				}

				if (node.heading) {
					text += `: ${node.heading}`;
				}

				const indent = "\t".repeat(level);
				lines.push(`${indent} - ${text}`);

				if (node.type === "folder") {
					for (const child of node.children) {
						pushList(child, level + 1);
					}
				}
			}

			const tree = await buildTree(websiteSource);
			const lines: Array<string> = [];
			pushList(tree, 0);

			return {lines};
		},
	);
}
