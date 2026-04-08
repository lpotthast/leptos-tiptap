import {Text} from "@tiptap/extension-text"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "text",
    create: () => Text,
}

export function register_text(): void {
    registerOfficialExtension(descriptor)
}
