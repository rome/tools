import {
	DiagnosticAdvice,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {ServerRequest} from "@internal/core";
import {getVCSClient} from "@internal/vcs";

type VSCDiagnostics = [
	noVSCAdvice: DiagnosticAdvice[],
	uncommittedChangesAdvice: DiagnosticAdvice[]
];

export async function checkVSCWorkingDirectory(
	req: ServerRequest,
	[noVSCAdvice, uncommittedChangesAdvice]: VSCDiagnostics,
) {
	const {client} = req;
	const vcsClient = await getVCSClient(client.flags.cwd);
	const location = req.getDiagnosticLocationForClientCwd();
	if (vcsClient === undefined) {
		throw createSingleDiagnosticsError({
			location,
			description: descriptions.VCS.UNCOMMITTED_CHANGES(noVSCAdvice),
		});
	} else {
		const uncommittedFiles = await vcsClient.getUncommittedFiles();
		if (uncommittedFiles.length > 0) {
			throw createSingleDiagnosticsError({
				location,
				description: descriptions.VCS.UNCOMMITTED_CHANGES(
					uncommittedChangesAdvice,
				),
			});
		}
	}
}
