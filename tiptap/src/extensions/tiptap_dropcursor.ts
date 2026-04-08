import {Dropcursor} from "@tiptap/extension-dropcursor"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "dropcursor",
    create: () => Dropcursor,
}

export function register_dropcursor(): void {
    registerOfficialExtension(descriptor)
}
