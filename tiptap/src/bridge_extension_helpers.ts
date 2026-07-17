import type {Editor} from "@tiptap/core"

import type {ActiveKey, ActiveState, ExtensionDescriptor} from "./bridge_api.ts"
import {getBridgeBindings} from "./bridge_api.ts"

const bridgeBindings = getBridgeBindings()

export function activeState(
    entries: ReadonlyArray<readonly [
        key: ActiveKey,
        isActive: (editor: Editor) => boolean,
    ]>,
): Pick<ExtensionDescriptor, "active_keys" | "active_state"> {
    return {
        active_keys: entries.map(([key]) => key),
        active_state: (editor): ActiveState => {
            const state: ActiveState = {}
            for (const [key, isActive] of entries) {
                state[key] = isActive(editor)
            }
            return state
        },
    }
}

export const mappedActiveState = activeState

export function registerOfficialExtension(descriptor: ExtensionDescriptor): void {
    bridgeBindings.registerExtension(descriptor)
}
