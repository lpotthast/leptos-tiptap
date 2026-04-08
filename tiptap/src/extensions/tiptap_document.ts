import {Document} from "@tiptap/extension-document"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "document",
    create: () => Document,
}

export function register_document(): void {
    registerOfficialExtension(descriptor)
}
