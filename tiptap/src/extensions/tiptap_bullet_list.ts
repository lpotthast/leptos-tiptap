import {BulletList} from "@tiptap/extension-bullet-list"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "bullet_list",
    create: () => BulletList,
    commands: {
        toggle_bullet_list: (editor) => editor.chain().focus().toggleBulletList().run(),
    },
    selection_keys: ["bullet_list"],
    selection_state: (editor) => ({
        bullet_list: editor.isActive("bulletList"),
    }),
}

export function register_bullet_list(): void {
    registerOfficialExtension(descriptor)
}
