import { Path } from "@internal/path";

// rome-ignore lint/ts/noExplicitAny: Wide extends is required here
export function enhanceNodeInspectClass<T extends new (
	...args: any
) => any>(Func: T, callback: (inst: InstanceType<T>) => string) {
	Func.prototype[Symbol.for("nodejs.util.inspect.custom")] = function() {
		if (this instanceof Func) {
			return callback(this);
		} else {
			throw new Error("Incorrect instance type received");
		}
	};
}
