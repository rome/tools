import {PACKAGES, execDev} from "./_utils";

export async function main(args: Array<string>) {
	await execDev(["bundle", PACKAGES.append("rome").join(), ...args]);
}
