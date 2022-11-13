import type React from "react";
import { useState, createRef, useEffect } from "react";
import { createLocalStorage } from "../utils";

interface Props {
	children: JSX.Element;
}

const RESIZE_TOLERANCE = 10;
const MINIMUM_SIZE = 200;

const codePaneSizeStore = createLocalStorage("code-width");

export default function CodePane({ children }: Props) {
	let [rawWidth, setWidth] = useState<number | undefined>(
		codePaneSizeStore.getNumber(),
	);
	let [isResizing, setIsResizing] = useState(false);
	let [canResize, setCanResize] = useState(false);

	const width =
		rawWidth === undefined ? undefined : Math.max(MINIMUM_SIZE, rawWidth);

	const ref: React.RefObject<HTMLDivElement> = createRef();

	let cursor = canResize || isResizing ? "col-resize" : undefined;

	useEffect(() => {
		// Detect if we can resize
		function onMouseMove(event: MouseEvent) {
			const container = ref.current;
			if (container == null) {
				return;
			}

			const mouseX = event.pageX;
			const containerX = container.offsetLeft + container.clientWidth;
			const distance = Math.abs(containerX - mouseX);

			if (isResizing) {
				event.preventDefault();
				const width = mouseX - container.offsetLeft;
				setWidth(width);
				codePaneSizeStore.set(width);
			}

			if (distance <= RESIZE_TOLERANCE) {
				setCanResize(true);
			} else if (canResize) {
				setCanResize(false);
			}
		}

		function onContextMenu(e: MouseEvent) {
			if (isResizing || canResize) {
				e.preventDefault();
				codePaneSizeStore.clear();
				setWidth(undefined);
				setCanResize(false);
				setIsResizing(false);
			}
		}

		function onMouseDown() {
			if (canResize) {
				setIsResizing(true);
			}
		}

		function onMouseUp() {
			setIsResizing(false);
		}

		window.addEventListener("contextmenu", onContextMenu);
		window.addEventListener("mousedown", onMouseDown);
		window.addEventListener("mouseup", onMouseUp);
		window.addEventListener("mousemove", onMouseMove);

		return () => {
			window.removeEventListener("contextmenu", onContextMenu);
			window.removeEventListener("mousedown", onMouseDown);
			window.removeEventListener("mouseup", onMouseUp);
			window.removeEventListener("mousemove", onMouseMove);
		};
	});

	useEffect(() => {
		if (cursor === undefined) {
			document.body.style.removeProperty("cursor");
		} else {
			document.body.style.cursor = cursor;
		}

		return () => {
			document.body.style.removeProperty("cursor");
		};
	}, [cursor]);

	return (
		<div
			ref={ref}
			className="code-pane"
			style={{ width, flexShrink: width === undefined ? undefined : 0 }}
		>
			{children}
		</div>
	);
}
