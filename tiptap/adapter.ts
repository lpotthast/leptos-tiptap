import {Editor} from "@tiptap/core"
import type {Content, EditorOptions} from "@tiptap/core"
import Highlight from "@tiptap/extension-highlight"
import Image from "@tiptap/extension-image"
import Link from "@tiptap/extension-link"
import TextAlign from "@tiptap/extension-text-align"
import Youtube from "@tiptap/extension-youtube"
import StarterKit from "@tiptap/starter-kit"

type ContentFormat = "html" | "json"
type HeadingLevel = Parameters<Editor["commands"]["toggleHeading"]>[0]["level"]

type ErrorKind =
    | "editor_unavailable"
    | "editor_mount_failed"
    | "invalid_content"
    | "command_rejected"
    | "operation_failed"

type BridgeError = {
    kind: ErrorKind
    message: string
    operation?: string
}

type BridgeResult<T> =
    | {
        ok: true
        value: T
    }
    | {
        ok: false
        error: BridgeError
    }

type SelectionState = {
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

type ContentPayload = {
    format: ContentFormat
    value: string
}

type CreateRequest = {
    id: string
    content: ContentPayload
    editable: boolean
}

type ReadyPayload = {
    generation: number
}

type EditorCommand =
    | { kind: "toggle_heading"; level: HeadingLevel }
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

type CommandKind = EditorCommand["kind"]
type CommandByKind = {
    [K in CommandKind]: Extract<EditorCommand, { kind: K }>
}

type CommandRequest = {
    id: string
    generation: number
    command: EditorCommand
}

type DocumentRequest =
    | { kind: "get_content"; format: ContentFormat }
    | { kind: "set_content"; content: ContentPayload }

type DocumentResponse =
    | { kind: "content"; content: ContentPayload }
    | { kind: "empty" }

type DocumentCall = {
    id: string
    generation: number
    request: DocumentRequest
}

type DocumentLookup = Pick<Document, "getElementById">
type CreateEditorOptions = Partial<EditorOptions>
type EditorFactory = (options: CreateEditorOptions) => Editor
type OnSelection = (selectionState: SelectionState) => void
type CommandHandler = (editorEntry: EditorEntry, command: EditorCommand) => BridgeResult<null>

type EditorEntry = {
    editor: Editor
    onSelection: OnSelection
}

type EditorSlot = {
    generation: number
    entry?: EditorEntry
}

const editorSlots = new Map<string, EditorSlot>()
const tiptapExtensions: NonNullable<EditorOptions["extensions"]> = [
    StarterKit,
    TextAlign.configure({
        types: ["heading", "paragraph"],
    }),
    Highlight,
    Image,
    Link,
    Youtube,
]

let editorFactory: EditorFactory = (options) => new Editor(options)
let documentOverride: DocumentLookup | undefined

function okResult<T>(value: T): BridgeResult<T> {
    return {ok: true, value}
}

function errorResult<T>(kind: ErrorKind, message: string, operation?: string): BridgeResult<T> {
    return {ok: false, error: {kind, message, operation}}
}

function currentDocument(): DocumentLookup {
    return documentOverride ?? globalThis.document
}

function getOrCreateSlot(id: string): EditorSlot {
    const existingSlot = editorSlots.get(id)
    if (existingSlot != null) {
        return existingSlot
    }

    const slot: EditorSlot = {generation: 0}
    editorSlots.set(id, slot)
    return slot
}

function destroyEditorInstance(editor: Editor): void {
    try {
        editor.destroy()
    } catch (error) {
        console.error("Could not destroy Tiptap editor instance.", error)
    }
}

function invalidateSlot(id: string): number {
    const slot = getOrCreateSlot(id)
    slot.generation += 1

    if (slot.entry != null) {
        destroyEditorInstance(slot.entry.editor)
        slot.entry = undefined
    }

    return slot.generation
}

function setEditorEntry(
    id: string,
    generation: number,
    editor: Editor,
    onSelection: OnSelection,
): boolean {
    const slot = getOrCreateSlot(id)
    if (slot.generation !== generation) {
        destroyEditorInstance(editor)
        return false
    }

    slot.entry = {editor, onSelection}
    return true
}

function getEditorEntry(id: string): EditorEntry | undefined {
    return editorSlots.get(id)?.entry
}

function withEditor<T>(
    id: string,
    generation: number,
    operation: string,
    onReady: (editorEntry: EditorEntry) => BridgeResult<T>,
): BridgeResult<T> {
    const slot = editorSlots.get(id)
    const editorEntry = slot?.entry

    if (slot?.generation !== generation || editorEntry == null) {
        const message = `Can not execute ${operation} for Tiptap instance "${id}", as no current editor is registered for this handle.`
        console.error(message)
        return errorResult("editor_unavailable", message, operation)
    }

    return onReady(editorEntry)
}

function requireEditorElement(id: string): BridgeResult<HTMLElement> {
    const editorElement = currentDocument().getElementById(id)
    if (editorElement != null) {
        return okResult(editorElement)
    }

    const message = `Can not create Tiptap instance on element with id "${id}", as the element could not be found in the DOM.`
    console.error(message)
    return errorResult("editor_mount_failed", message)
}

function parseContent(content: ContentPayload): BridgeResult<Content> {
    if (content.format !== "json") {
        return okResult(content.value)
    }

    try {
        return okResult(JSON.parse(content.value) as Content)
    } catch (error) {
        const message = "Could not parse Tiptap JSON content."
        console.error(message, error)
        return errorResult(
            "invalid_content",
            `${message} ${error instanceof Error ? error.message : String(error)}`,
        )
    }
}

function serializeContent(editor: Editor, format: ContentFormat): BridgeResult<ContentPayload> {
    return runOperation(getContentOperationName(format), () => {
        if (format === "html") {
            return {
                format,
                value: editor.getHTML(),
            }
        }

        return {
            format,
            value: JSON.stringify(editor.getJSON()),
        }
    })
}

function emitSelectionState(editor: Editor, onSelection: OnSelection): void {
    onSelection(getSelectionState(editor))
}

function runOperation<T>(operation: string, action: () => T): BridgeResult<T> {
    try {
        return okResult(action())
    } catch (error) {
        const message = `${operation} failed: ${error instanceof Error ? error.message : String(error)}`
        console.error(message, error)
        return errorResult("operation_failed", message, operation)
    }
}

function runCommand(operation: string, command: () => boolean): BridgeResult<null> {
    try {
        if (command() === false) {
            return errorResult(
                "command_rejected",
                `Tiptap rejected the ${operation} command for the current editor state.`,
                operation,
            )
        }

        return okResult(null)
    } catch (error) {
        const message = `${operation} failed: ${error instanceof Error ? error.message : String(error)}`
        console.error(message, error)
        return errorResult("operation_failed", message, operation)
    }
}

function createSelectionCommandHandler<K extends CommandKind>(
    runSelectionCommand: (editor: Editor, command: CommandByKind[K]) => boolean,
): CommandHandler {
    return (editorEntry, command) => {
        const typedCommand = command as CommandByKind[K]
        const result = runCommand(command.kind, () => runSelectionCommand(editorEntry.editor, typedCommand))
        if (!result.ok) {
            return result
        }

        emitSelectionState(editorEntry.editor, editorEntry.onSelection)
        return result
    }
}

function createOperationCommandHandler<K extends CommandKind>(
    runEditorOperation: (editor: Editor, command: CommandByKind[K]) => void,
): CommandHandler {
    return (editorEntry, command) =>
        runOperation(command.kind, () => {
            runEditorOperation(editorEntry.editor, command as CommandByKind[K])
            return null
        })
}

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

function buildYoutubeAttributes(command: CommandByKind["set_youtube_video"]) {
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

function getContentOperationName(format: ContentFormat): string {
    return format === "html" ? "get_content_html" : "get_content_json"
}

const commandHandlers: Record<CommandKind, CommandHandler> = {
    toggle_heading: createSelectionCommandHandler<"toggle_heading">((editor, command) =>
        editor.chain().focus().toggleHeading({level: command.level}).run(),
    ),
    set_paragraph: createSelectionCommandHandler<"set_paragraph">((editor) =>
        editor.chain().focus().setParagraph().run(),
    ),
    toggle_bold: createSelectionCommandHandler<"toggle_bold">((editor) =>
        editor.chain().focus().toggleBold().run(),
    ),
    toggle_italic: createSelectionCommandHandler<"toggle_italic">((editor) =>
        editor.chain().focus().toggleItalic().run(),
    ),
    toggle_strike: createSelectionCommandHandler<"toggle_strike">((editor) =>
        editor.chain().focus().toggleStrike().run(),
    ),
    toggle_blockquote: createSelectionCommandHandler<"toggle_blockquote">((editor) =>
        editor.chain().focus().toggleBlockquote().run(),
    ),
    toggle_highlight: createSelectionCommandHandler<"toggle_highlight">((editor) =>
        editor.chain().focus().toggleHighlight().run(),
    ),
    toggle_bullet_list: createSelectionCommandHandler<"toggle_bullet_list">((editor) =>
        editor.chain().focus().toggleBulletList().run(),
    ),
    toggle_ordered_list: createSelectionCommandHandler<"toggle_ordered_list">((editor) =>
        editor.chain().focus().toggleOrderedList().run(),
    ),
    set_text_align_left: createSelectionCommandHandler<"set_text_align_left">((editor) =>
        editor.chain().focus().setTextAlign("left").run(),
    ),
    set_text_align_center: createSelectionCommandHandler<"set_text_align_center">((editor) =>
        editor.chain().focus().setTextAlign("center").run(),
    ),
    set_text_align_right: createSelectionCommandHandler<"set_text_align_right">((editor) =>
        editor.chain().focus().setTextAlign("right").run(),
    ),
    set_text_align_justify: createSelectionCommandHandler<"set_text_align_justify">((editor) =>
        editor.chain().focus().setTextAlign("justify").run(),
    ),
    set_image: createSelectionCommandHandler<"set_image">((editor, command) =>
        editor.chain().focus().setImage({
            src: command.src,
            alt: command.alt,
            title: command.title,
        }).run(),
    ),
    set_link: createSelectionCommandHandler<"set_link">((editor, command) =>
        editor.chain().focus().setLink(
            buildLinkAttributes(command.href, command.target, command.rel),
        ).run(),
    ),
    toggle_link: createSelectionCommandHandler<"toggle_link">((editor, command) =>
        editor.chain().focus().toggleLink(
            buildLinkAttributes(command.href, command.target, command.rel),
        ).run(),
    ),
    unset_link: createSelectionCommandHandler<"unset_link">((editor) =>
        editor.chain().focus().unsetLink().run(),
    ),
    set_youtube_video: createSelectionCommandHandler<"set_youtube_video">((editor, command) =>
        editor.chain().focus().setYoutubeVideo(buildYoutubeAttributes(command)).run(),
    ),
    set_editable: createOperationCommandHandler<"set_editable">((editor, command) => {
        editor.setEditable(command.editable)
    }),
}

// Called by the Rust bridge.
export function create(
    request: CreateRequest,
    onReady: (ready: ReadyPayload) => void,
    onChange: () => void,
    onSelection: OnSelection,
    onError: (error: BridgeError) => void,
): void {
    const generation = invalidateSlot(request.id)

    const editorElement = requireEditorElement(request.id)
    if (!editorElement.ok) {
        onError(editorElement.error)
        return
    }

    const parsedContent = parseContent(request.content)
    if (!parsedContent.ok) {
        onError(parsedContent.error)
        return
    }

    const createdEditor = runOperation("create_editor", () =>
        editorFactory({
            element: editorElement.value,
            editable: request.editable,
            extensions: tiptapExtensions,
            injectCSS: false,
            content: parsedContent.value,
            onUpdate: onChange,
            onSelectionUpdate: ({editor}) => {
                emitSelectionState(editor, onSelection)
            },
        }),
    )
    if (!createdEditor.ok) {
        onError(createdEditor.error)
        return
    }

    if (!setEditorEntry(request.id, generation, createdEditor.value, onSelection)) {
        return
    }

    emitSelectionState(createdEditor.value, onSelection)
    onReady({generation})
}

// Called by the Rust bridge.
export function destroy(id: string): void {
    invalidateSlot(id)
}

// Called by the Rust bridge.
// noinspection JSUnusedGlobalSymbols
export function command(request: CommandRequest): BridgeResult<null> {
    const {id, generation, command} = request
    return withEditor(id, generation, command.kind, (editorEntry) =>
        commandHandlers[command.kind](editorEntry, command),
    )
}

// Called by the Rust bridge.
// noinspection JSUnusedGlobalSymbols
export function document(call: DocumentCall): BridgeResult<DocumentResponse> {
    const {id, generation, request} = call

    switch (request.kind) {
        case "get_content":
            return withEditor(id, generation, getContentOperationName(request.format), ({editor}) => {
                const serializedContent = serializeContent(editor, request.format)
                if (!serializedContent.ok) {
                    return serializedContent
                }

                return okResult({
                    kind: "content",
                    content: serializedContent.value,
                })
            })
        case "set_content":
            return withEditor(id, generation, request.kind, ({editor, onSelection}) => {
                const parsedContent = parseContent(request.content)
                if (!parsedContent.ok) {
                    return parsedContent
                }

                const result = runCommand(request.kind, () => editor.commands.setContent(parsedContent.value))
                if (!result.ok) {
                    return result
                }

                emitSelectionState(editor, onSelection)
                return okResult({kind: "empty"})
            })
    }
}

function getSelectionState(editor: Editor): SelectionState {
    return {
        h1: editor.isActive("heading", {level: 1}),
        h2: editor.isActive("heading", {level: 2}),
        h3: editor.isActive("heading", {level: 3}),
        h4: editor.isActive("heading", {level: 4}),
        h5: editor.isActive("heading", {level: 5}),
        h6: editor.isActive("heading", {level: 6}),
        paragraph: editor.isActive("paragraph"),
        bold: editor.isActive("bold"),
        italic: editor.isActive("italic"),
        strike: editor.isActive("strike"),
        blockquote: editor.isActive("blockquote"),
        highlight: editor.isActive("highlight"),
        bullet_list: editor.isActive("bulletList"),
        ordered_list: editor.isActive("orderedList"),
        align_left: editor.isActive({textAlign: "left"}),
        align_center: editor.isActive({textAlign: "center"}),
        align_right: editor.isActive({textAlign: "right"}),
        align_justify: editor.isActive({textAlign: "justify"}),
        link: editor.isActive("link"),
        youtube: editor.isActive("youtube"),
    }
}

export const __testing = {
    getEditorEntry,
    reset(): void {
        editorSlots.clear()
        editorFactory = (options) => new Editor(options)
        documentOverride = undefined
    },
    setDocument(nextDocument: DocumentLookup): void {
        documentOverride = nextDocument
    },
    setEditorFactory(nextEditorFactory: EditorFactory): void {
        editorFactory = nextEditorFactory
    },
}
