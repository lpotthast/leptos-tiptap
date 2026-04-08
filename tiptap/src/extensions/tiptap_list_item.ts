import {ListItem} from "@tiptap/extension-list-item"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "list_item",
    create: () => ListItem,
}

export function register_list_item(): void {
    registerOfficialExtension(descriptor)
}
