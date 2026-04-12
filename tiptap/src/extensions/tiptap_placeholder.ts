import {Placeholder} from "@tiptap/extension-placeholder"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "placeholder",
    create: ({placeholder}) => {
        if (placeholder == null) {
            return Placeholder
        }

        return Placeholder.configure({
            placeholder,
        })
    },
}

export function register_placeholder(): void {
    registerOfficialExtension(descriptor)
}
