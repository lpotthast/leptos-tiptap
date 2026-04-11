import {HardBreak} from "@tiptap/extension-hard-break"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "hard_break",
    create: () => HardBreak,
    commands: {
        set_hard_break: (editor) => editor.chain().focus().setHardBreak().run(),
    },
}

export function register_hard_break(): void {
    registerOfficialExtension(descriptor)
}
