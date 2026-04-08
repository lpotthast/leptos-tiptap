import {Paragraph} from "@tiptap/extension-paragraph"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "paragraph",
    create: () => Paragraph,
    commands: {
        set_paragraph: (editor) => editor.chain().focus().setParagraph().run(),
    },
    selection_keys: ["paragraph"],
    selection_state: (editor) => ({
        paragraph: editor.isActive("paragraph"),
    }),
}

export function register_paragraph(): void {
    registerOfficialExtension(descriptor)
}
