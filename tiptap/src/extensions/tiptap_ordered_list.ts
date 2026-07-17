import {OrderedList} from "@tiptap/extension-ordered-list"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeState, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "ordered_list",
    create: () => OrderedList,
    commands: {
        toggle_ordered_list: (editor) => editor.chain().focus().toggleOrderedList().run(),
    },
    ...activeState([
        ["ordered_list", (editor) => editor.isActive("orderedList")],
    ]),
}

export function register_ordered_list(): void {
    registerOfficialExtension(descriptor)
}
