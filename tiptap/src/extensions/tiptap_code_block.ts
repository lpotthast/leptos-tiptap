import {CodeBlock} from "@tiptap/extension-code-block"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "code_block",
    create: () => CodeBlock,
}

export function register_code_block(): void {
    registerOfficialExtension(descriptor)
}
