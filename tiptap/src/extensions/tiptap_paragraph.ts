import {Paragraph} from "@tiptap/extension-paragraph"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "paragraph",
    create: () => Paragraph,
    commands: {
        set_paragraph: (editor) => editor.chain().focus().setParagraph().run(),
    },
    ...activeSelection([
        ["paragraph", (editor) => editor.isActive("paragraph")],
    ]),
}

export function register_paragraph(): void {
    registerOfficialExtension(descriptor)
}
