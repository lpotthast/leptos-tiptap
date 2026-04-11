import {Heading} from "@tiptap/extension-heading"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

const descriptor: ExtensionDescriptor = {
    name: "heading",
    create: () => Heading,
    commands: {
        set_heading: (editor, command) =>
            command.kind === "set_heading"
                ? editor.chain().focus().setHeading({
                    level: command.level as 1 | 2 | 3 | 4 | 5 | 6,
                }).run()
                : false,
        toggle_heading: (editor, command) =>
            command.kind === "toggle_heading"
                ? editor.chain().focus().toggleHeading({
                    level: command.level as 1 | 2 | 3 | 4 | 5 | 6,
                }).run()
                : false,
    },
    selection_keys: ["h1", "h2", "h3", "h4", "h5", "h6"],
    selection_state: (editor) => ({
        h1: editor.isActive("heading", {level: 1}),
        h2: editor.isActive("heading", {level: 2}),
        h3: editor.isActive("heading", {level: 3}),
        h4: editor.isActive("heading", {level: 4}),
        h5: editor.isActive("heading", {level: 5}),
        h6: editor.isActive("heading", {level: 6}),
    }),
}

export function register_heading(): void {
    registerOfficialExtension(descriptor)
}
