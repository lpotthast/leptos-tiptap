import {TextAlign} from "@tiptap/extension-text-align"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const allowedAlignments = ["left", "center", "right", "justify"] as const

const descriptor: ExtensionDescriptor = {
    name: "text_align",
    create: () =>
        TextAlign.configure({
            types: ["heading", "paragraph"],
        }),
    commands: {
        set_text_align: (editor, command) =>
            command.kind === "set_text_align"
                && allowedAlignments.includes(command.alignment)
                ? editor.chain().focus().setTextAlign(command.alignment).run()
                : false,
        toggle_text_align: (editor, command) =>
            command.kind === "toggle_text_align"
                && allowedAlignments.includes(command.alignment)
                ? editor.chain().focus().toggleTextAlign(command.alignment).run()
                : false,
        unset_text_align: (editor) => editor.chain().focus().unsetTextAlign().run(),
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
