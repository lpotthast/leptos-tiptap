import {Bold} from "@tiptap/extension-bold"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "bold",
    create: () => Bold,
    commands: {
        toggle_bold: (editor) => editor.chain().focus().toggleBold().run(),
    },
    selection_keys: ["bold"],
    selection_state: (editor) => ({
        bold: editor.isActive("bold"),
    }),
}

export function register_bold(): void {
    registerOfficialExtension(descriptor)
}
