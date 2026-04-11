import {Strike} from "@tiptap/extension-strike"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "strike",
    create: () => Strike,
    commands: {
        set_strike: (editor) => editor.chain().focus().setStrike().run(),
        toggle_strike: (editor) => editor.chain().focus().toggleStrike().run(),
        unset_strike: (editor) => editor.chain().focus().unsetStrike().run(),
    },
    selection_keys: ["strike"],
    selection_state: (editor) => ({
        strike: editor.isActive("strike"),
    }),
}

export function register_strike(): void {
    registerOfficialExtension(descriptor)
}
