import type {Editor, EditorOptions} from "@tiptap/core"

export const BRIDGE_GLOBAL_KEY = "__LEPTOS_TIPTAP_BRIDGE__"

export type ContentFormat = "html" | "json"

export type ErrorKind =
    | "editor_unavailable"
    | "editor_mount_failed"
    | "duplicate_editor_id"
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

export type EmptyResponse = { kind: "empty" }

export type SelectionKey =
    | "h1"
    | "h2"
    | "h3"
    | "h4"
    | "h5"
    | "h6"
    | "paragraph"
    | "bold"
    | "italic"
    | "strike"
    | "blockquote"
    | "highlight"
    | "bullet_list"
    | "ordered_list"
    | "align_left"
    | "align_center"
    | "align_right"
    | "align_justify"
    | "link"
    | "youtube"

export type SelectionState = Partial<Record<SelectionKey, boolean>>

export type ContentPayload =
    | {
        format: "html"
        value: string
    }
    | {
        format: "json"
        value: unknown
    }

export type CreateRequest = {
    id: string
    content: ContentPayload
    editable: boolean
    extensions: string[]
    placeholder?: string | null
}

export type ReadyPayload = {
    generation: number
}

export type TextAlign = "left" | "center" | "right" | "justify"

export type CodeBlockAttributes = {
    language?: string | null
}

export type HighlightAttributes = {
    color?: string | null
}

export type AttributesPayload = Record<string, unknown>

export type Range = {
    from: number
    to: number
}

export type PositionOrRange = number | Range

export type FocusTarget = "start" | "end" | "all" | number | null

export type FocusOptions = {
    scroll_into_view?: boolean
}

export type ParseOptionsPayload = {
    preserve_whitespace?: boolean | "full"
    from?: number
    to?: number
}

export type SetContentOptions = {
    emit_update?: boolean
    parse_options?: ParseOptionsPayload | null
    error_on_invalid_content?: boolean
}

export type InsertContentOptions = {
    parse_options?: ParseOptionsPayload | null
    update_selection?: boolean
    apply_input_rules?: boolean
    apply_paste_rules?: boolean
    error_on_invalid_content?: boolean
}

export type MarkOptions = {
    extend_empty_mark_range?: boolean
}

// Intentionally unsupported core commands/overloads:
// - `command`, `first`, and `for_each`: require executable callbacks over the bridge.
// - `set_meta` with `Plugin` or `PluginKey`: non-serializable runtime objects.
// - `ParseOptions.findPositions`, `topNode`, `topMatch`, and `context`: depend on DOM or ProseMirror instances.
// - `focus(false)`: redundant no-op sentinel; omit the focus command instead.

export type CoreCommand =
    | { kind: "blur" }
    | { kind: "clear_content"; emit_update?: boolean }
    | { kind: "clear_nodes" }
    | { kind: "create_paragraph_near" }
    | { kind: "cut"; range: Range; target_pos: number }
    | { kind: "delete_current_node" }
    | { kind: "delete_node"; type_or_name: string }
    | { kind: "delete_range"; range: Range }
    | { kind: "delete_selection" }
    | { kind: "enter" }
    | { kind: "exit_code" }
    | { kind: "extend_mark_range"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "focus"; target?: FocusTarget; options?: FocusOptions | null }
    | { kind: "insert_content"; content: ContentPayload; options?: InsertContentOptions | null }
    | { kind: "insert_content_at"; position: PositionOrRange; content: ContentPayload; options?: InsertContentOptions | null }
    | { kind: "join_up" }
    | { kind: "join_down" }
    | { kind: "join_backward" }
    | { kind: "join_forward" }
    | { kind: "join_item_backward" }
    | { kind: "join_item_forward" }
    | { kind: "join_textblock_backward" }
    | { kind: "join_textblock_forward" }
    | { kind: "keyboard_shortcut"; name: string }
    | { kind: "lift"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "lift_empty_block" }
    | { kind: "newline_in_code" }
    | { kind: "reset_attributes"; type_or_name: string; attribute_names: string[] }
    | { kind: "scroll_into_view" }
    | { kind: "select_all" }
    | { kind: "select_node_backward" }
    | { kind: "select_node_forward" }
    | { kind: "select_parent_node" }
    | { kind: "select_textblock_end" }
    | { kind: "select_textblock_start" }
    | { kind: "set_mark"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "set_meta"; key: string; value: unknown }
    | { kind: "set_node"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "set_node_selection"; position: number }
    | { kind: "set_text_selection"; position: PositionOrRange }
    | { kind: "split_block"; keep_marks?: boolean }
    | { kind: "toggle_list"; list_type_or_name: string; item_type_or_name: string; keep_marks?: boolean; attributes?: AttributesPayload | null }
    | { kind: "toggle_mark"; type_or_name: string; attributes?: AttributesPayload | null; options?: MarkOptions | null }
    | { kind: "toggle_node"; type_or_name: string; toggle_type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "toggle_wrap"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "undo_input_rule" }
    | { kind: "unset_all_marks" }
    | { kind: "unset_mark"; type_or_name: string; options?: MarkOptions | null }
    | { kind: "update_attributes"; type_or_name: string; attributes: AttributesPayload }
    | { kind: "wrap_in"; type_or_name: string; attributes?: AttributesPayload | null }
    | { kind: "wrap_in_list"; type_or_name: string; attributes?: AttributesPayload | null }

export type ExtensionCommand =
    | { kind: "set_blockquote" }
    | { kind: "toggle_blockquote" }
    | { kind: "unset_blockquote" }
    | { kind: "set_bold" }
    | { kind: "toggle_bold" }
    | { kind: "unset_bold" }
    | { kind: "toggle_bullet_list" }
    | { kind: "set_code" }
    | { kind: "toggle_code" }
    | { kind: "unset_code" }
    | { kind: "set_code_block"; attributes?: CodeBlockAttributes | null }
    | { kind: "toggle_code_block"; attributes?: CodeBlockAttributes | null }
    | { kind: "set_hard_break" }
    | { kind: "set_heading"; level: number }
    | { kind: "toggle_heading"; level: number }
    | { kind: "set_paragraph" }
    | { kind: "set_highlight"; attributes?: HighlightAttributes | null }
    | { kind: "toggle_highlight"; attributes?: HighlightAttributes | null }
    | { kind: "unset_highlight" }
    | { kind: "undo" }
    | { kind: "redo" }
    | { kind: "set_horizontal_rule" }
    | { kind: "set_image"; src: string; alt?: string | null; title?: string | null }
    | { kind: "set_italic" }
    | { kind: "toggle_italic" }
    | { kind: "unset_italic" }
    | { kind: "set_link"; href: string; target?: string | null; rel?: string | null; class?: string | null }
    | { kind: "toggle_link"; href: string; target?: string | null; rel?: string | null; class?: string | null }
    | { kind: "unset_link" }
    | { kind: "split_list_item"; attributes?: Record<string, unknown> | null }
    | { kind: "sink_list_item" }
    | { kind: "lift_list_item" }
    | { kind: "toggle_ordered_list" }
    | { kind: "set_strike" }
    | { kind: "toggle_strike" }
    | { kind: "unset_strike" }
    | { kind: "set_text_align"; alignment: TextAlign }
    | { kind: "toggle_text_align"; alignment: TextAlign }
    | { kind: "unset_text_align" }
    | { kind: "set_youtube_video"; src: string; start?: number | null; width?: number | null; height?: number | null }

export type RuntimeCommand =
    | { kind: "set_editable"; editable: boolean }

export type EditorCommand = CoreCommand | ExtensionCommand | RuntimeCommand

export type CommandKind = EditorCommand["kind"]
export type CoreCommandKind = CoreCommand["kind"]
export type ExtensionCommandKind = ExtensionCommand["kind"]
export type RuntimeCommandKind = RuntimeCommand["kind"]

export type DocumentRequest =
    | { kind: "get_content"; format: ContentFormat }
    | { kind: "set_content"; content: ContentPayload; options?: SetContentOptions | null }

export type DocumentResponse =
    | { kind: "content"; content: ContentPayload }
    | EmptyResponse

export type DocumentCall = {
    id: string
    generation: number
    request: DocumentRequest
}

export type TiptapExtension = NonNullable<EditorOptions["extensions"]>[number]
export type ExtensionCommandHandler<K extends ExtensionCommandKind = ExtensionCommandKind> = (
    editor: Editor,
    command: Extract<ExtensionCommand, { kind: K }>,
) => boolean | void
export type ExtensionCommandHandlers = Partial<{
    [K in ExtensionCommandKind]: ExtensionCommandHandler<K>
}>

export type ExtensionCreateContext = {
    placeholder?: string | null
}

export type ExtensionDescriptor = {
    name: string
    create: (context: ExtensionCreateContext) => TiptapExtension | TiptapExtension[]
    commands?: ExtensionCommandHandlers
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
    return {}
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
