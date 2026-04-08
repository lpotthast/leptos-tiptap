import {Image} from "@tiptap/extension-image"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "image",
    create: () => Image,
    commands: {
        set_image: (editor, command) =>
            command.kind === "set_image"
                ? editor.chain().focus().setImage({
                    src: command.src,
                    alt: command.alt,
                    title: command.title,
                }).run()
                : false,
    },
}

export function register_image(): void {
    registerOfficialExtension(descriptor)
}
