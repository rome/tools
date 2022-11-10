import { defineConfig } from "astro/config";
import type { AstroIntegration } from "astro";
import mdx from "@astrojs/mdx";
import compress from "astro-compress";
import path from "node:path";
import fs from "node:fs/promises";
import { globby } from "globby";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkToc from "remark-toc";

function inlineCSS(): AstroIntegration {
	return {
		name: "inlineCSS",
		hooks: {
			"astro:build:done": async ({ dir }) => {
				const files = await globby(`${dir.pathname}/**/*.html`);

				await Promise.all(
					files.map(async (htmlPath) => {
						const pageStyles: string[] = [];
						const stylesPaths: string[] = [];

						let file = await fs.readFile(htmlPath, "utf8");

						file = file.replace(
							/<link rel="stylesheet" href="(.*?)"\s*\/?>/g,
							(match, p1) => {
								stylesPaths.push(p1);
								return `{{${p1}}}`;
							},
						);

						await Promise.all(
							stylesPaths.map(async (stylesPath) => {
								if (stylesPath[0] === "/") {
									stylesPath = `${dir.pathname}${stylesPath}`;
								} else {
									stylesPath = path.resolve(
										path.join(path.dirname(htmlPath), stylesPath),
									);
								}
								const styles = await fs.readFile(stylesPath, "utf8");
								pageStyles.push(styles);
							}),
						);

						stylesPaths.forEach((p, i) => {
							file = file.replace(
								`{{${p}}}`,
								`<style>${pageStyles[i]}</style>`,
							);
						});

						await fs.writeFile(htmlPath, file);
					}),
				);
			},
		},
	};
}
// https://astro.build/config
export default defineConfig({
	site: "https://rome.tools",
	output: "static",
	outDir: "build",

	integrations: [
		inlineCSS(),
		mdx(),
		compress({
			path: "./build",
		}),
	],

	build: {
		format: "directory",
	},

	markdown: {
		syntaxHighlight: "prism",
		remarkPlugins: [remarkToc],
		rehypePlugins: [
			rehypeSlug,
			[
				rehypeAutolinkHeadings,
				{
					behavior: "append",
					content: [],
				},
			],
		],
		extendDefaultPlugins: true,
	},
});
