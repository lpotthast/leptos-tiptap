import Link from "@tiptap/extension-link"

import type {ExtensionDescriptor} from "../bridge_api.ts"
import {activeSelection, registerOfficialExtension} from "../bridge_extension_helpers.ts"

function buildLinkAttributes(
    href: string,
    target?: string | null,
    rel?: string | null,
    className?: string | null,
) {
    const attributes: { href: string; target?: string; rel?: string; class?: string } = {href}
    if (target != null) {
        attributes.target = target
    }
    if (rel != null) {
        attributes.rel = rel
    }
    if (className != null) {
        attributes.class = className
    }
    return attributes
}

const descriptor: ExtensionDescriptor = {
    name: "link",
    create: () => Link,
    commands: {
        set_link: (editor, command) =>
            editor.chain().focus().setLink(
                buildLinkAttributes(command.href, command.target, command.rel, command.class),
            ).run(),
        toggle_link: (editor, command) =>
            editor.chain().focus().toggleLink(
                buildLinkAttributes(command.href, command.target, command.rel, command.class),
            ).run(),
        unset_link: (editor) => editor.chain().focus().unsetLink().run(),
    },
    ...activeSelection([
        ["link", (editor) => editor.isActive("link")],
    ]),
}

export function register_link(): void {
    registerOfficialExtension(descriptor)
}
