import type { SettingsTabProps } from "../tabs/SettingsTab";
import { useState, useEffect } from "react";
import { classNames, createLocalStorage } from "../utils";
import SettingsTab from "../tabs/SettingsTab";

const isCollapsedStore = createLocalStorage("settings-collapsed");

export default function SettingsPane(props: SettingsTabProps) {
	const [isCollapsed, setIsCollapsed] = useState(isCollapsedStore.getBoolean());

	function collapseToggle() {
		setIsCollapsed(!isCollapsed);
	}

	useEffect(() => {
		isCollapsedStore.set(isCollapsed);
	}, [isCollapsed]);

	return (
		<div className="settings-pane">
			{!isCollapsed && (
				<div className="fields">
					<SettingsTab {...props} />
				</div>
			)}
			<div
				className={classNames("collapser", isCollapsed && "collapsed")}
				onMouseDown={collapseToggle}
				onKeyDown={collapseToggle}
			>
				<div className="dot" />
				<div className="dot" />
				<div className="dot" />
			</div>
		</div>
	);
}
