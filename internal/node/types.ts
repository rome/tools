// Similar to TS NodeJS.ErrnoException but with proper properties
import {StructuredNodeSystemErrorProperties} from "@internal/v8";

// https://nodejs.org/api/errors.html#errors_class_systemerror
export type NodeSystemError = Error &
	Partial<StructuredNodeSystemErrorProperties>;
