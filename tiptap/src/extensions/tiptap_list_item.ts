import {ListItem} from "@tiptap/extension-list-item"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "list_item",
    create: () => ListItem,
    commands: {
        split_list_item: (editor, command) =>
            editor.chain().focus().splitListItem("listItem", command.attributes ?? {}).run(),
        sink_list_item: (editor) => editor.chain().focus().sinkListItem("listItem").run(),
        lift_list_item: (editor) => editor.chain().focus().liftListItem("listItem").run(),
    },
}

export function register_list_item(): void {
    registerOfficialExtension(descriptor)
}
