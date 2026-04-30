import {CodeBlock} from "@tiptap/extension-code-block"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildCodeBlockAttributes(attributes?: { language?: string | null } | null) {
    if (attributes?.language == null) {
        return undefined
    }

    return {
        language: attributes.language,
    }
}

const descriptor: ExtensionDescriptor = {
    name: "code_block",
    create: () => CodeBlock,
    commands: {
        set_code_block: (editor, command) =>
            editor.chain().focus().setCodeBlock(buildCodeBlockAttributes(command.attributes)).run(),
        toggle_code_block: (editor, command) =>
            editor.chain().focus().toggleCodeBlock(buildCodeBlockAttributes(command.attributes)).run(),
    },
}

export function register_code_block(): void {
    registerOfficialExtension(descriptor)
}
