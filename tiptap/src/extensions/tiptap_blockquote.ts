import {Blockquote} from "@tiptap/extension-blockquote"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "blockquote",
    create: () => Blockquote,
    commands: {
        set_blockquote: (editor) => editor.chain().focus().setBlockquote().run(),
        toggle_blockquote: (editor) => editor.chain().focus().toggleBlockquote().run(),
        unset_blockquote: (editor) => editor.chain().focus().unsetBlockquote().run(),
    },
    ...activeSelection([
        ["blockquote", (editor) => editor.isActive("blockquote")],
    ]),
}

export function register_blockquote(): void {
    registerOfficialExtension(descriptor)
}
