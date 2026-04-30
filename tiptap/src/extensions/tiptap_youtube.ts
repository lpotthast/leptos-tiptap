import {Youtube} from "@tiptap/extension-youtube"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildYoutubeAttributes(command: {
    src: string
    start?: number | null
    width?: number | null
    height?: number | null
}) {
    const attributes: { src: string; start?: number; width?: number; height?: number } = {
        src: command.src,
    }
    if (command.start != null) {
        attributes.start = command.start
    }
    if (command.width != null) {
        attributes.width = command.width
    }
    if (command.height != null) {
        attributes.height = command.height
    }
    return attributes
}

const descriptor: ExtensionDescriptor = {
    name: "youtube",
    create: () => Youtube,
    commands: {
        set_youtube_video: (editor, command) =>
            editor.chain().focus().setYoutubeVideo(buildYoutubeAttributes(command)).run(),
    },
    ...activeSelection([
        ["youtube", (editor) => editor.isActive("youtube")],
    ]),
}

export function register_youtube(): void {
    registerOfficialExtension(descriptor)
}
