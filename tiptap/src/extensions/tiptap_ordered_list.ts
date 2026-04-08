import {OrderedList} from "@tiptap/extension-ordered-list"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "ordered_list",
    create: () => OrderedList,
    commands: {
        toggle_ordered_list: (editor) => editor.chain().focus().toggleOrderedList().run(),
    },
    selection_keys: ["ordered_list"],
    selection_state: (editor) => ({
        ordered_list: editor.isActive("orderedList"),
    }),
}

export function register_ordered_list(): void {
    registerOfficialExtension(descriptor)
}
