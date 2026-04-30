import type {Editor} from "@tiptap/core"

import type {ExtensionDescriptor, SelectionKey, SelectionState} from "./bridge_api.ts"
import {getBridgeBindings} from "./bridge_api.ts"

export function activeSelection(
    entries: ReadonlyArray<readonly [
        key: SelectionKey,
        isActive: (editor: Editor) => boolean,
    ]>,
): Pick<ExtensionDescriptor, "selection_keys" | "selection_state"> {
    return {
        selection_keys: entries.map(([key]) => key),
        selection_state: (editor): Partial<SelectionState> => {
            const state: SelectionState = {}
            for (const [key, isActive] of entries) {
                state[key] = isActive(editor)
            }
            return state
        },
    }
}

export const mappedSelection = activeSelection

export function registerOfficialExtension(descriptor: ExtensionDescriptor): void {
    getBridgeBindings().registerExtension(descriptor)
}
