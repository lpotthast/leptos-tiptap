import Link from "@tiptap/extension-link"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildLinkAttributes(href: string, target?: string | null, rel?: string | null) {
    const attributes: { href: string; target?: string; rel?: string } = {href}
    if (target != null) {
        attributes.target = target
    }
    if (rel != null) {
        attributes.rel = rel
    }
    return attributes
}

const descriptor: ExtensionDescriptor = {
    name: "link",
    create: () => Link,
    commands: {
        set_link: (editor, command) =>
            command.kind === "set_link"
                ? editor.chain().focus().setLink(
                    buildLinkAttributes(command.href, command.target, command.rel),
                ).run()
                : false,
        toggle_link: (editor, command) =>
            command.kind === "toggle_link"
                ? editor.chain().focus().toggleLink(
                    buildLinkAttributes(command.href, command.target, command.rel),
                ).run()
                : false,
        unset_link: (editor) => editor.chain().focus().unsetLink().run(),
    },
    selection_keys: ["link"],
    selection_state: (editor) => ({
        link: editor.isActive("link"),
    }),
}

export function register_link(): void {
    registerOfficialExtension(descriptor)
}
