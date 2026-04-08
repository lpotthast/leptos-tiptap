import {TextAlign} from "@tiptap/extension-text-align"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "text_align",
    create: () =>
        TextAlign.configure({
            types: ["heading", "paragraph"],
        }),
    commands: {
        set_text_align_left: (editor) => editor.chain().focus().setTextAlign("left").run(),
        set_text_align_center: (editor) => editor.chain().focus().setTextAlign("center").run(),
        set_text_align_right: (editor) => editor.chain().focus().setTextAlign("right").run(),
        set_text_align_justify: (editor) => editor.chain().focus().setTextAlign("justify").run(),
    },
    selection_keys: ["align_left", "align_center", "align_right", "align_justify"],
    selection_state: (editor) => ({
        align_left: editor.isActive({textAlign: "left"}),
        align_center: editor.isActive({textAlign: "center"}),
        align_right: editor.isActive({textAlign: "right"}),
        align_justify: editor.isActive({textAlign: "justify"}),
    }),
}

export function register_text_align(): void {
    registerOfficialExtension(descriptor)
}
