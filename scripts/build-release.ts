import {PUBLIC_PACKAGES, execDev} from "./_utils";

export async function main(args: Array<string>) {
	await execDev(["bundle", PUBLIC_PACKAGES.append("rome").join(), ...args]);
}
