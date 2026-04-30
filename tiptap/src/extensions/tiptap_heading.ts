import {Heading} from "@tiptap/extension-heading"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "heading",
    create: () => Heading,
    commands: {
        set_heading: (editor, command) =>
            editor.chain().focus().setHeading({
                level: command.level as 1 | 2 | 3 | 4 | 5 | 6,
            }).run(),
        toggle_heading: (editor, command) =>
            editor.chain().focus().toggleHeading({
                level: command.level as 1 | 2 | 3 | 4 | 5 | 6,
            }).run(),
    },
    ...activeSelection([
        ["h1", (editor) => editor.isActive("heading", {level: 1})],
        ["h2", (editor) => editor.isActive("heading", {level: 2})],
        ["h3", (editor) => editor.isActive("heading", {level: 3})],
        ["h4", (editor) => editor.isActive("heading", {level: 4})],
        ["h5", (editor) => editor.isActive("heading", {level: 5})],
        ["h6", (editor) => editor.isActive("heading", {level: 6})],
    ]),
}

export function register_heading(): void {
    registerOfficialExtension(descriptor)
}
