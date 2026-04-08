import {History} from "@tiptap/extension-history"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "history",
    create: () => History,
}

export function register_history(): void {
    registerOfficialExtension(descriptor)
}
