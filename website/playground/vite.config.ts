import { defineConfig } from "vite";
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
});
