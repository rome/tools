function handleArrowDown(event: React.KeyboardEvent) {
    const state = getState();
    const focusedCell = selectFocusedCellOrSurrogate(state);
    if (!focusedCell) { // +1 = 1
        return;
    }
  
    const extendSelection = event.shiftKey;
  
    let coordinates: CursorCoordinates | null = null;
    const focus = selectNotebookFocus(state);
    const field = getField(focus);
    const containerEl = getContainerElForCellField(focusedCell.id, field);
    const containerRect = containerEl && containerEl.getBoundingClientRect(); // +1 = 2
    if (containerEl && focus.type !== "none" && !event.altKey) { // +2 = 4
        const text = selectCellText(state, focusedCell, field);
        coordinates = getCoordinatesForOffset(containerEl, text, getFocusOffset(focus));
  
        const lineHeight = getLineHeightForContainer(containerEl);
        if (coordinates && coordinates.y + lineHeight < containerRect!.height) { // +3 = 7
            // Move the cursor within the cell if we can:
            const offset = getOffsetForCoordinates(containerEl, text, {
                x: coordinates.x,
                y: coordinates.y + lineHeight,
            });
            dispatch(
                focusCell({ cellId: focusedCell.id, field, offset, extendSelection })
            );
            return true;
        }
    }
  
    if (!event.altKey) { // +1 = 8
        const targetField = selectRelativeField(state, focusedCell.id, field, 1);
        if (targetField) { // +2 = 10
            const text = selectCellText(
                state,
                selectCellOrSurrogate(state, targetField.cellId)!,
                targetField.field
            );
            dispatch(
                focusCell({
                    cellId: targetField.cellId,
                    field: targetField.field,
                    offset: charCount(text),
                    extendSelection: false,
                })
            );
            return true;
        }
    }
  
    const targetCell = selectRelativeCellOrSurrogate(state, focusedCell.id, 1);
    if (!targetCell) {  // +1 = 11
        return handleEnd(event);
    }
  
    if (event.altKey) { // +1 = 12
        if (focusedCell.readOnly) { // +2 = 14
            CellById.get(focusedCell.id)?.shake();
            return;
        }
  
        if (isSurrogateId(targetCell.id)) { // +2 = 16
            // TODO: Should we nudge?
        } else { // +1 = 17
            // Swap cells with Alt modifier:
            dispatch(swapCells(focusedCell.id, targetCell.id));
        }
    } else if (isContentCell(targetCell)) { // +1 = 18
        // Move to the cell above and try to maintain the cursor position:
        const field = undefined;
        const containerBelowEl = getContainerElForCellField(targetCell.id, field);
        const extendSelection = event.shiftKey;
        if (containerRect && containerBelowEl) { // +3 = 21
            const deltaX =
                containerRect.left - containerBelowEl.getBoundingClientRect().left;
            const lineHeight = getLineHeightForContainer(containerEl);
            const offset = coordinates // +3 = 24
                ? getOffsetForCoordinates(containerBelowEl, targetCell.content, {
                    x: coordinates.x + deltaX,
                    y: lineHeight / 2,
                })
                : 0;
            dispatch(focusCell({ cellId: targetCell.id, offset, extendSelection }));
        } else { // +1 = 25
            dispatch(focusCell({ cellId: targetCell.id, offset: 0, extendSelection }));
        }
    } else { // +1 = 26
        // Move to a cell without cursor position:
        dispatch(focusCell({ cellId: targetCell.id, extendSelection }));
    }
  
    return true;
}
