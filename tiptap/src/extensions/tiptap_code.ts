import {Code} from "@tiptap/extension-code"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "code",
    create: () => Code,
    commands: {
        set_code: (editor) => editor.chain().focus().setCode().run(),
        toggle_code: (editor) => editor.chain().focus().toggleCode().run(),
        unset_code: (editor) => editor.chain().focus().unsetCode().run(),
    },
}

export function register_code(): void {
    registerOfficialExtension(descriptor)
}
