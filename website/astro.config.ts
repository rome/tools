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
import react from "@astrojs/react";

async function inline(
	dir: string,
	regex: RegExp,
	tagBefore: string,
	tagAfter: string,
): Promise<void> {
	const files = await globby(`${dir}/**/*.html`);

	await Promise.all(
		files.map(async (htmlPath) => {
			if (htmlPath.includes("playground")) {
				return;
			}

			const paths: string[] = [];
			let file = await fs.readFile(htmlPath, "utf8");

			file = file.replace(regex, (match, p1) => {
				paths.push(p1);
				return `{{INLINE:${p1}}}`;
			});

			const sources: string[] = await Promise.all(
				paths.map(async (rawPath) => {
					let resolvedPath;
					if (rawPath[0] === "/") {
						resolvedPath = `${dir}${rawPath}`;
					} else {
						resolvedPath = path.resolve(
							path.join(path.dirname(htmlPath), rawPath),
						);
					}
					return await fs.readFile(resolvedPath, "utf8");
				}),
			);

			paths.forEach((p, i) => {
				file = file.replace(
					`{{INLINE:${p}}}`,
					`${tagBefore}${sources[i]}${tagAfter}`,
				);
			});

			await fs.writeFile(htmlPath, file);
		}),
	);
}

function inlineCSS(): AstroIntegration {
	return {
		name: "inlineCSS",
		hooks: {
			"astro:build:done": async ({ dir }) => {
				await inline(
					dir.pathname,
					/<link rel="stylesheet" href="(.*?)"\s*\/?>/g,
					"<style>",
					"</style>",
				);
			},
		},
	};
}

function inlineJS(): AstroIntegration {
	return {
		name: "inlineJS",
		hooks: {
			"astro:build:done": async ({ dir }) => {
				await inline(
					dir.pathname,
					/<script type="module" src="(.*?)"><\/script>/g,
					'<script async defer type="module">',
					"</script>",
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
		react(),
		inlineCSS(),
		inlineJS(),
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

	vite: {
		plugins: [],

		worker: {
			format: "es",
		},

		server: {
			fs: {
				// https://vitejs.dev/config/server-options.html#server-fs-allow
				allow: [process.cwd(), "../npm/wasm-web"],
			},
		},
	},
});
