import {ROOT, execDev} from "./_utils";

export async function main(args: Array<string>) {
	await execDev(["bundle", ROOT.append("internal", "rome").join(), ...args]);
}
