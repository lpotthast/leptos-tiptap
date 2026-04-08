import {Italic} from "@tiptap/extension-italic"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "italic",
    create: () => Italic,
    commands: {
        toggle_italic: (editor) => editor.chain().focus().toggleItalic().run(),
    },
    selection_keys: ["italic"],
    selection_state: (editor) => ({
        italic: editor.isActive("italic"),
    }),
}

export function register_italic(): void {
    registerOfficialExtension(descriptor)
}
