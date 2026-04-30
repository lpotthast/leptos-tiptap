import {Italic} from "@tiptap/extension-italic"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "italic",
    create: () => Italic,
    commands: {
        set_italic: (editor) => editor.chain().focus().setItalic().run(),
        toggle_italic: (editor) => editor.chain().focus().toggleItalic().run(),
        unset_italic: (editor) => editor.chain().focus().unsetItalic().run(),
    },
    ...activeSelection([
        ["italic", (editor) => editor.isActive("italic")],
    ]),
}

export function register_italic(): void {
    registerOfficialExtension(descriptor)
}
