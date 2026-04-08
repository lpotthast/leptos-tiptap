import {Gapcursor} from "@tiptap/extension-gapcursor"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "gapcursor",
    create: () => Gapcursor,
}

export function register_gapcursor(): void {
    registerOfficialExtension(descriptor)
}
