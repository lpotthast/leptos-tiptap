import {Highlight} from "@tiptap/extension-highlight"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildHighlightAttributes(attributes?: { color?: string | null } | null) {
    if (attributes?.color == null) {
        return undefined
    }

    return {
        color: attributes.color,
    }
}

const descriptor: ExtensionDescriptor = {
    name: "highlight",
    create: () => Highlight,
    commands: {
        set_highlight: (editor, command) =>
            editor.chain().focus().setHighlight(buildHighlightAttributes(command.attributes)).run(),
        toggle_highlight: (editor, command) =>
            editor.chain().focus().toggleHighlight(buildHighlightAttributes(command.attributes)).run(),
        unset_highlight: (editor) => editor.chain().focus().unsetHighlight().run(),
    },
    ...activeSelection([
        ["highlight", (editor) => editor.isActive("highlight")],
    ]),
}

export function register_highlight(): void {
    registerOfficialExtension(descriptor)
}
