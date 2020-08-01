import {UnknownObject} from "@internal/typescript-helpers";
import Path from "./Path";

type SetStateCallback<State> =
	| Partial<State>
	| ((state: State) => Partial<State>);

type FindState<State> = (state: State) => boolean;

interface SetStateOptions<State> {
	required?: boolean;
	find?: FindState<State>;
}

export interface VisitorStateExit<State extends UnknownObject> {
	owns(): boolean;
	get(find?: FindState<State>): State;
	getOptional(find?: FindState<State>): undefined | State;
}

export interface VisitorStateEnter<State extends UnknownObject> extends VisitorStateExit<State> {
	reset(state: State): void;
	set(state: SetStateCallback<State>, opts?: SetStateOptions<State>): void;
}

export type AnyVisitorState = VisitorState<UnknownObject>;

type StackItem<State> = [State, Path];

export default class VisitorState<State extends UnknownObject>
	implements VisitorStateEnter<State> {
	constructor() {
		this.stack = [];
		this.currentIndex = -1;
		this.pushQueue = undefined;
		this.currentPath = undefined;
	}

	stack: Array<StackItem<State>>;
	currentIndex: number;
	pushQueue: undefined | State;
	currentPath: undefined | Path;

	setCurrentPath(path: Path) {
		this.currentPath = path;
	}

	owns(): boolean {
		return this.has() && this.stack[this.currentIndex][1] === this.currentPath;
	}

	has() {
		return this.currentIndex >= 0;
	}

	getIndex(find?: FindState<State>): number {
		let index = this.currentIndex;

		if (find !== undefined) {
			for (; index >= 0; index--) {
				if (find(this.stack[index][0])) {
					break;
				}
			}
		}

		return index;
	}

	getOptional(find?: FindState<State>): undefined | State {
		const index = this.getIndex(find);
		if (index === -1) {
			throw new Error("VisitorState: Could not find stack");
		} else {
			return this.stack[index][0];
		}
	}

	get(find?: FindState<State>): State {
		if (!this.has()) {
			throw new Error(
				"VisitorState.get: Nothing on the stack. Did you mean to use getOptional?",
			);
		}

		const index = this.getIndex(find);
		if (index === -1) {
			throw new Error("VisitorState.get: Could not find item on the stack");
		}

		return this.stack[index][0];
	}

	reset(state: State) {
		this.pushQueue = state;
	}

	pop() {
		this.currentIndex--;
		this.stack.pop();
	}

	checkPushed(): boolean {
		const {pushQueue: queueState} = this;
		if (queueState === undefined) {
			return false;
		} else {
			const {currentPath} = this;
			if (currentPath === undefined) {
				throw new Error("VisitorState.checkPushed: No current path found");
			}
			this.stack.push([queueState, currentPath]);
			this.currentIndex++;
			this.pushQueue = undefined;
			return true;
		}
	}

	set(partial: SetStateCallback<State>, opts: SetStateOptions<State> = {}) {
		if (!this.has()) {
			if (opts.required) {
				throw new Error("VisitorState.set: Nothing on the stack");
			} else {
				return;
			}
		}

		// Find stack item
		const stackIndex = this.getIndex(opts.find);
		if (stackIndex === -1) {
			if (opts.required) {
				throw new Error("VisitorState.set: Could not find stack");
			} else {
				return;
			}
		}

		const [state, path] = this.stack[stackIndex];

		// Optionally
		if (typeof partial === "function") {
			partial = partial(state);
		}

		// Allow passing in the existing state as a noop
		if (partial === state) {
			return;
		}

		// Update the state
		// NB: Wonder if we'd get away with an `Object.assign`...?
		const newState: State = {
			...state,
			...partial,
		};
		this.stack[stackIndex] = [newState, path];
	}
}
