import {HardBreak} from "@tiptap/extension-hard-break"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "hard_break",
    create: () => HardBreak,
}

export function register_hard_break(): void {
    registerOfficialExtension(descriptor)
}
