import {Image} from "@tiptap/extension-image"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildImageAttributes(command: {
    src: string
    alt?: string | null
    title?: string | null
}) {
    return {
        src: command.src,
        alt: command.alt ?? undefined,
        title: command.title ?? undefined,
    }
}

const descriptor: ExtensionDescriptor = {
    name: "image",
    create: () => Image,
    commands: {
        set_image: (editor, command) =>
            command.kind === "set_image"
                ? editor.chain().focus().setImage(buildImageAttributes(command)).run()
                : false,
    },
}

export function register_image(): void {
    registerOfficialExtension(descriptor)
}
