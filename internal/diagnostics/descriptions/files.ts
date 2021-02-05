import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {AbsoluteFilePath, AnyFilePath} from "@internal/path";
import {DiagnosticAdvice} from "../types";

export const files = createDiagnosticsCategory({
	NO_FILE_HANDLER: (path: AnyFilePath) => {
		let advice: DiagnosticAdvice = [];

		if (path.hasAnyExtensions()) {
			advice.push({
				type: "action",
				instruction: markup`You can treat this file extension as a binary asset by running`,
				noun: markup`Treat this file extension as a binary asset`,
				command: "config push",
				args: ["files.assetExtensions", path.getDotlessExtensions()],
			});
		}

		return {
			category: "files/missingHandler",
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
			category: "files/tooBig",
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
					instruction: markup`You can ignore this file from linting by running`,
					noun: markup`Ignore this file from linting`,
					command: "config push",
					args: ["lint.ignore", relative],
				},
				{
					type: "action",
					instruction: markup`Or can allow this specific file to exceed the size limit with`,
					noun: markup`Allow only this specific file to exceed the limit`,
					command: "config push",
					args: ["files.maxSizeIgnore", relative],
				},
				{
					type: "action",
					instruction: markup`Or just increase the size limit for all files to <filesize>${String(
						size,
					)}</filesize>`,
					noun: markup`Increase project max file size limit`,
					command: "config set",
					args: ["files.maxSize", String(size)],
				},
			],
		};
	},
});
