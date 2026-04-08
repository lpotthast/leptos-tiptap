import {Code} from "@tiptap/extension-code"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "code",
    create: () => Code,
}

export function register_code(): void {
    registerOfficialExtension(descriptor)
}
