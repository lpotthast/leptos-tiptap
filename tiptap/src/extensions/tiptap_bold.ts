import {Bold} from "@tiptap/extension-bold"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "bold",
    create: () => Bold,
    commands: {
        set_bold: (editor) => editor.chain().focus().setBold().run(),
        toggle_bold: (editor) => editor.chain().focus().toggleBold().run(),
        unset_bold: (editor) => editor.chain().focus().unsetBold().run(),
    },
    ...activeSelection([
        ["bold", (editor) => editor.isActive("bold")],
    ]),
}

export function register_bold(): void {
    registerOfficialExtension(descriptor)
}
