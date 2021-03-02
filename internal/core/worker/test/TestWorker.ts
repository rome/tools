import {AssembledBundle} from "@internal/core/common/types/bundler";
import {serializeAssembled} from "@internal/core/server/bundler/utils";
import {AbsoluteFilePathMap} from "@internal/path";
import Worker from "../Worker";
import TestWorkerFile from "./TestWorkerFile";

export default class TestWorker {
	constructor(worker: Worker) {
		this.worker = worker;

		this.runners = new AbsoluteFilePathMap();
		this.compiled = new AbsoluteFilePathMap();
	}

	private worker: Worker;

	private runners: AbsoluteFilePathMap<TestWorkerFile>;
	private compiled: AbsoluteFilePathMap<string>;

	public serializeAssembled(assembled: AssembledBundle): string {
		return serializeAssembled(
			assembled,
			(path) => {
				return this.compiled.get(path);
			},
		);
	}

	public init() {
		const {bridge} = this.worker;

		bridge.events.testPrepare.subscribe(async (opts) => {
			const runner = new TestWorkerFile(this.worker, this, opts);
			this.runners.set(opts.path, runner);
			return await runner.prepare();
		});

		bridge.events.testGetConsoleAdvice.subscribe(async (path) => {
			const runner = this.runners.assert(path);
			return runner.getConsoleAdvice();
		});

		bridge.events.testGetRawSnapshot.subscribe(async ({path, snapshotPath}) => {
			return this.runners.assert(path).snapshotManager.getRawSnapshot(snapshotPath);
		});

		bridge.events.testRun.subscribe(async (opts) => {
			const {path} = opts;
			const runner = this.runners.assert(path);
			return await runner.run(opts);
		});

		bridge.events.testReceiveCompiledDependency.subscribe((map) => {
			for (const [path, content] of map) {
				this.compiled.set(path, content);
			}
		});
	}
}
