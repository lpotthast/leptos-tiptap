import {HorizontalRule} from "@tiptap/extension-horizontal-rule"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "horizontal_rule",
    create: () => HorizontalRule,
    commands: {
        set_horizontal_rule: (editor) => editor.chain().focus().setHorizontalRule().run(),
    },
}

export function register_horizontal_rule(): void {
    registerOfficialExtension(descriptor)
}
