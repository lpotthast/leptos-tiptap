import {History} from "@tiptap/extension-history"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "history",
    create: () => History,
    commands: {
        undo: (editor) => editor.chain().focus().undo().run(),
        redo: (editor) => editor.chain().focus().redo().run(),
    },
}

export function register_history(): void {
    registerOfficialExtension(descriptor)
}
