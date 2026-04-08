import type {Editor, EditorOptions} from "@tiptap/core"

export const BRIDGE_GLOBAL_KEY = "__LEPTOS_TIPTAP_BRIDGE__"

export type ContentFormat = "html" | "json"

export type ErrorKind =
    | "editor_unavailable"
    | "editor_mount_failed"
    | "invalid_content"
    | "command_rejected"
    | "operation_failed"
    | "extension_unavailable"
    | "extension_registration_failed"

export type BridgeError = {
    kind: ErrorKind
    message: string
    operation?: string
}

export type BridgeResult<T> =
    | {
        ok: true
        value: T
    }
    | {
        ok: false
        error: BridgeError
    }

export type SelectionState = {
    h1: boolean
    h2: boolean
    h3: boolean
    h4: boolean
    h5: boolean
    h6: boolean
    paragraph: boolean
    bold: boolean
    italic: boolean
    strike: boolean
    blockquote: boolean
    highlight: boolean
    bullet_list: boolean
    ordered_list: boolean
    align_left: boolean
    align_center: boolean
    align_right: boolean
    align_justify: boolean
    link: boolean
    youtube: boolean
}

export type SelectionKey = keyof SelectionState

export type ContentPayload = {
    format: ContentFormat
    value: string
}

export type CreateRequest = {
    id: string
    content: ContentPayload
    editable: boolean
    extensions: string[]
}

export type ReadyPayload = {
    generation: number
}

export type EditorCommand =
    | { kind: "toggle_heading"; level: number }
    | { kind: "set_paragraph" }
    | { kind: "toggle_bold" }
    | { kind: "toggle_italic" }
    | { kind: "toggle_strike" }
    | { kind: "toggle_blockquote" }
    | { kind: "toggle_highlight" }
    | { kind: "toggle_bullet_list" }
    | { kind: "toggle_ordered_list" }
    | { kind: "set_text_align_left" }
    | { kind: "set_text_align_center" }
    | { kind: "set_text_align_right" }
    | { kind: "set_text_align_justify" }
    | { kind: "set_image"; src: string; alt: string; title: string }
    | { kind: "set_link"; href: string; target?: string | null; rel?: string | null }
    | { kind: "toggle_link"; href: string; target?: string | null; rel?: string | null }
    | { kind: "unset_link" }
    | { kind: "set_youtube_video"; src: string; start?: number | null; width?: number | null; height?: number | null }
    | { kind: "set_editable"; editable: boolean }

export type CommandKind = EditorCommand["kind"]

export type DocumentRequest =
    | { kind: "get_content"; format: ContentFormat }
    | { kind: "set_content"; content: ContentPayload }

export type DocumentResponse =
    | { kind: "content"; content: ContentPayload }
    | { kind: "empty" }

export type DocumentCall = {
    id: string
    generation: number
    request: DocumentRequest
}

export type TiptapExtension = NonNullable<EditorOptions["extensions"]>[number]

export type ExtensionDescriptor = {
    name: string
    create: () => TiptapExtension | TiptapExtension[]
    commands?: Partial<Record<CommandKind, (editor: Editor, command: EditorCommand) => boolean | void>>
    selection_keys?: SelectionKey[]
    selection_state?: (editor: Editor) => Partial<SelectionState>
}

export type BridgeBindings = {
    modules: Record<string, unknown>
    registerExtension: (descriptor: ExtensionDescriptor) => void
}

type BridgeGlobal = typeof globalThis & {
    [BRIDGE_GLOBAL_KEY]?: BridgeBindings
}

export function emptySelectionState(): SelectionState {
    return {
        h1: false,
        h2: false,
        h3: false,
        h4: false,
        h5: false,
        h6: false,
        paragraph: false,
        bold: false,
        italic: false,
        strike: false,
        blockquote: false,
        highlight: false,
        bullet_list: false,
        ordered_list: false,
        align_left: false,
        align_center: false,
        align_right: false,
        align_justify: false,
        link: false,
        youtube: false,
    }
}

export function getOrCreateBridgeBindings(): BridgeBindings {
    const bridge = globalThis as BridgeGlobal
    const existing = bridge[BRIDGE_GLOBAL_KEY]
    if (existing != null) {
        return existing
    }

    const bindings: BridgeBindings = {
        modules: {},
        registerExtension: () => {
            throw new Error("leptos-tiptap bridge runtime is not initialized")
        },
    }

    bridge[BRIDGE_GLOBAL_KEY] = bindings
    return bindings
}

export function getBridgeBindings(): BridgeBindings {
    return getOrCreateBridgeBindings()
}
