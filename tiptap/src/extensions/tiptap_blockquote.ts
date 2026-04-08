import {Blockquote} from "@tiptap/extension-blockquote"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "blockquote",
    create: () => Blockquote,
    commands: {
        toggle_blockquote: (editor) => editor.chain().focus().toggleBlockquote().run(),
    },
    selection_keys: ["blockquote"],
    selection_state: (editor) => ({
        blockquote: editor.isActive("blockquote"),
    }),
}

export function register_blockquote(): void {
    registerOfficialExtension(descriptor)
}
