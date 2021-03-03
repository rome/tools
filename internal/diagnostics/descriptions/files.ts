import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {AbsoluteFilePath, Path} from "@internal/path";
import {DiagnosticAdvice} from "../types";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const files = createDiagnosticsCategory({
	NO_FILE_HANDLER: (path: Path) => {
		let advice: DiagnosticAdvice[] = [];

		if (path.hasAnyExtensions()) {
			advice.push({
				type: "action",
				description: markup`Treat this file extension as a binary asset`,
				command: "config push",
				args: ["files.assetExtensions", path.getDotlessExtensions()],
			});
		}

		return {
			category: DIAGNOSTIC_CATEGORIES["files/missingHandler"],
			message: markup`No file handler found for <emphasis>${path}</emphasis>`,
			advice,
		};
	},
	TOO_BIG: (
		path: AbsoluteFilePath,
		projectPath: AbsoluteFilePath,
		size: bigint,
		maxSize: number,
	) => {
		const relative = projectPath.relative(path).join();

		return {
			category: DIAGNOSTIC_CATEGORIES["files/tooBig"],
			message: markup`Size of <emphasis>${path}</emphasis> is <filesize>${String(
				size,
			)}</filesize> which exceeds the project maximum of <filesize>${String(
				maxSize,
			)}</filesize>`,
			advice: [
				{
					type: "log",
					category: "none",
					text: markup`The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.`,
				},
				{
					type: "action",
					description: markup`Ignore this file from linting`,
					command: "config push",
					args: ["lint.ignore", relative],
				},
				{
					type: "action",
					description: markup`Allow only this specific file to exceed the size limit`,
					command: "config push",
					args: ["files.maxSizeIgnore", relative],
				},
				{
					type: "action",
					description: markup`Increase the project size limit for all files to <filesize>${String(
						size,
					)}</filesize>`,
					command: "config set",
					args: ["files.maxSize", String(size)],
				},
			],
		};
	},
});
