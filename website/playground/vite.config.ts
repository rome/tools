import { defineConfig, searchForWorkspaceRoot } from "vite";
import react from "@vitejs/plugin-react";
import svgr from "vite-plugin-svgr";

// https://vitejs.dev/config/
export default defineConfig({
	base: process.env.BASE_URL,
	plugins: [react(), svgr()],
	worker: {
		format: "es",
	},
	resolve: {
		dedupe: ["@codemirror/state"],
	},
	server: {
		fs: {
			// https://vitejs.dev/config/server-options.html#server-fs-allow
			allow: [searchForWorkspaceRoot(process.cwd()), "../../npm/wasm-web"],
		},
	},
});
