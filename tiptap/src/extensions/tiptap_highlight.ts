import {Highlight} from "@tiptap/extension-highlight"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

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
            command.kind === "set_highlight"
                ? editor.chain().focus().setHighlight(buildHighlightAttributes(command.attributes)).run()
                : false,
        toggle_highlight: (editor, command) =>
            command.kind === "toggle_highlight"
                ? editor.chain().focus().toggleHighlight(buildHighlightAttributes(command.attributes)).run()
                : false,
        unset_highlight: (editor) => editor.chain().focus().unsetHighlight().run(),
    },
    selection_keys: ["highlight"],
    selection_state: (editor) => ({
        highlight: editor.isActive("highlight"),
    }),
}

export function register_highlight(): void {
    registerOfficialExtension(descriptor)
}
