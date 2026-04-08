import {Highlight} from "@tiptap/extension-highlight"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "highlight",
    create: () => Highlight,
    commands: {
        toggle_highlight: (editor) => editor.chain().focus().toggleHighlight().run(),
    },
    selection_keys: ["highlight"],
    selection_state: (editor) => ({
        highlight: editor.isActive("highlight"),
    }),
}

export function register_highlight(): void {
    registerOfficialExtension(descriptor)
}
