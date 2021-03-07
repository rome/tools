import {createAbsoluteFilePath, createUIDPath} from "./factories";
import os = require("os");

export const HOME_PATH = createAbsoluteFilePath(os.userInfo().homedir);
export const TEMP_PATH = createAbsoluteFilePath(os.tmpdir());
export const CWD_PATH = createAbsoluteFilePath(process.cwd());
export const UNKNOWN_PATH = createUIDPath("unknown");
