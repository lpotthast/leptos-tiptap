import type {Content, Editor, EditorOptions} from "@tiptap/core"

import {
    emptySelectionState,
    getOrCreateBridgeBindings,
    type BridgeError,
    type BridgeResult,
    type CommandKind,
    type ContentFormat,
    type ContentPayload,
    type CoreCommand,
    type CoreCommandKind,
    type ExtensionCreateContext,
    type ExtensionCommand,
    type ExtensionCommandKind,
    type FocusOptions,
    type InsertContentOptions,
    type MarkOptions,
    type ParseOptionsPayload,
    type CreateRequest,
    type DocumentCall,
    type DocumentResponse,
    type EmptyResponse,
    type EditorCommand,
    type ErrorKind,
    type ExtensionDescriptor,
    type ReadyPayload,
    type RuntimeCommand,
    type RuntimeCommandKind,
    type SelectionKey,
    type SelectionState,
} from "./bridge_api.ts"
import {installHostedModules} from "./generated/hosted_modules.ts"

type TiptapCoreModule = typeof import("@tiptap/core")
type DocumentLookup = Pick<Document, "getElementById">
type CreateEditorOptions = Partial<EditorOptions>
type EditorConstructor = new (options?: CreateEditorOptions) => Editor
type EditorFactory = (options: CreateEditorOptions) => Editor
type OnSelection = (selectionState: SelectionState) => void
type DescriptorCommandHandler = (editor: Editor, command: ExtensionCommand) => boolean | void
type CoreCommandHandler<K extends CoreCommandKind> = (
    editor: Editor,
    command: Extract<CoreCommand, { kind: K }>,
) => BridgeResult<EmptyResponse>
type RuntimeCommandHandler<K extends RuntimeCommandKind> = (
    editor: Editor,
    command: Extract<RuntimeCommand, { kind: K }>,
) => BridgeResult<EmptyResponse>

type EditorEntry = {
    editor: Editor
    onSelection: OnSelection
    commandHandlers: Map<ExtensionCommandKind, DescriptorCommandHandler>
    selectionContributors: Array<(editor: Editor) => Partial<SelectionState>>
    lastSelectionState?: SelectionState
}

type EditorSlot = {
    generation: number
    entry?: EditorEntry
}

type RuntimeDescriptor = {
    descriptor: ExtensionDescriptor
    selectionKeys: SelectionKey[]
}

type RuntimeConfiguration = {
    extensions: NonNullable<EditorOptions["extensions"]>
    commandHandlers: Map<ExtensionCommandKind, DescriptorCommandHandler>
    selectionContributors: Array<(editor: Editor) => Partial<SelectionState>>
}

const editorSlots = new Map<string, EditorSlot>()
const extensionRegistry = new Map<string, ExtensionDescriptor>()
const bridgeBindings = getOrCreateBridgeBindings()
let nextGeneration = 1

let documentOverride: DocumentLookup | undefined
let editorFactory: EditorFactory = createDefaultEditor

installHostedModules(bridgeBindings.modules)

function getTiptapCoreModule(): TiptapCoreModule {
    const module = bridgeBindings.modules["@tiptap/core"]
    if (module == null) {
        throw new Error("leptos-tiptap tiptap_core runtime is not initialized")
    }

    return module as TiptapCoreModule
}

function createDefaultEditor(options: CreateEditorOptions): Editor {
    const EditorClass = getTiptapCoreModule().Editor as EditorConstructor
    return new EditorClass(options)
}

function okResult<T>(value: T): BridgeResult<T> {
    return {ok: true, value}
}

function emptyResponse(): EmptyResponse {
    return {kind: "empty"}
}

function okEmptyResult(): BridgeResult<EmptyResponse> {
    return okResult(emptyResponse())
}

function errorResult<T>(kind: ErrorKind, message: string, operation?: string): BridgeResult<T> {
    return {ok: false, error: {kind, message, operation}}
}

function extensionError(message: string, operation?: string): BridgeResult<never> {
    console.error(message)
    return errorResult("extension_unavailable", message, operation)
}

function registrationError(message: string): BridgeResult<never> {
    console.error(message)
    return errorResult("extension_registration_failed", message)
}

function currentDocument(): DocumentLookup {
    return documentOverride ?? globalThis.document
}

function destroyEditorInstance(editor: Editor): void {
    try {
        editor.destroy()
    } catch (error) {
        console.error("Could not destroy Tiptap editor instance.", error)
    }
}

function allocateGeneration(): number {
    const generation = nextGeneration
    nextGeneration += 1
    return generation
}

function destroySlot(id: string): void {
    const slot = editorSlots.get(id)
    if (slot == null) {
        return
    }

    if (slot.entry != null) {
        destroyEditorInstance(slot.entry.editor)
    }

    editorSlots.delete(id)
}

function setEditorEntry(
    id: string,
    generation: number,
    editor: Editor,
    onSelection: OnSelection,
    runtimeConfig: RuntimeConfiguration,
): boolean {
    const slot = editorSlots.get(id)
    if (slot == null || slot.generation !== generation) {
        destroyEditorInstance(editor)
        return false
    }

    slot.entry = {
        editor,
        onSelection,
        commandHandlers: runtimeConfig.commandHandlers,
        selectionContributors: runtimeConfig.selectionContributors,
    }
    return true
}

function getEditorEntry(id: string): EditorEntry | undefined {
    return editorSlots.get(id)?.entry
}

function getCurrentEditorEntry(id: string, editor: Editor): EditorEntry | undefined {
    const editorEntry = getEditorEntry(id)
    return editorEntry?.editor === editor ? editorEntry : undefined
}

function emitSelectionForCurrentEditor(id: string, editor: Editor): void {
    const editorEntry = getCurrentEditorEntry(id, editor)
    if (editorEntry != null) {
        emitSelectionState(editorEntry)
    }
}

function registerExtension(descriptor: ExtensionDescriptor): void {
    const existing = extensionRegistry.get(descriptor.name)
    if (existing != null) {
        throw new Error(`Tiptap extension "${descriptor.name}" has already been registered.`)
    }

    extensionRegistry.set(descriptor.name, descriptor)
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

function requireUnusedEditorId(id: string): BridgeResult<EmptyResponse> {
    if (editorSlots.get(id)?.entry == null) {
        return okEmptyResult()
    }

    const message = `Can not create Tiptap instance "${id}", as another live editor is already registered for that id.`
    console.error(message)
    return errorResult("duplicate_editor_id", message)
}

function invalidContentError(message: string, error?: unknown): BridgeResult<never> {
    console.error(message, error)
    return errorResult(
        "invalid_content",
        `${message}${error == null ? "" : ` ${error instanceof Error ? error.message : String(error)}`}`,
    )
}

function parseContent(content: ContentPayload): BridgeResult<Content> {
    if (content.format === "html") {
        if (typeof content.value !== "string") {
            return invalidContentError("Could not parse Tiptap HTML content.")
        }

        return okResult(content.value)
    }

    if (typeof content.value === "string") {
        try {
            return okResult(JSON.parse(content.value) as Content)
        } catch (error) {
            return invalidContentError("Could not parse Tiptap JSON content.", error)
        }
    }

    return okResult(content.value as Content)
}

function serializeContent(editor: Editor, format: ContentFormat): BridgeResult<ContentPayload> {
    return runOperation(getContentOperationName(format), () => (
        format === "html"
            ? {
                format,
                value: editor.getHTML(),
            }
            : {
                format,
                value: editor.getJSON(),
            }
    ))
}

function getSelectionState(editorEntry: EditorEntry): SelectionState {
    const state = emptySelectionState()

    for (const contribute of editorEntry.selectionContributors) {
        Object.assign(state, contribute(editorEntry.editor))
    }

    return state
}

function selectionStatesEqual(left: SelectionState, right: SelectionState): boolean {
    const keys = new Set<SelectionKey>([
        ...(Object.keys(left) as SelectionKey[]),
        ...(Object.keys(right) as SelectionKey[]),
    ])

    for (const key of keys) {
        if ((left[key] ?? false) !== (right[key] ?? false)) {
            return false
        }
    }

    return true
}

function emitSelectionState(editorEntry: EditorEntry, options: { force?: boolean } = {}): void {
    const nextState = getSelectionState(editorEntry)
    if (
        options.force !== true
        && editorEntry.lastSelectionState != null
        && selectionStatesEqual(editorEntry.lastSelectionState, nextState)
    ) {
        return
    }

    editorEntry.lastSelectionState = nextState
    editorEntry.onSelection(nextState)
}

function runOperation<T>(operation: string, work: () => T): BridgeResult<T> {
    try {
        return okResult(work())
    } catch (error) {
        const message = `${operation} failed: ${error instanceof Error ? error.message : String(error)}`
        console.error(message, error)
        return errorResult("operation_failed", message, operation)
    }
}

function runCommand(
    operation: string,
    work: () => boolean | void,
): BridgeResult<EmptyResponse> {
    try {
        const result = work()
        if (result === false) {
            return errorResult(
                "command_rejected",
                `Tiptap rejected the ${operation} command for the current editor state.`,
                operation,
            )
        }

        return okEmptyResult()
    } catch (error) {
        const message = `${operation} failed: ${error instanceof Error ? error.message : String(error)}`
        console.error(message, error)
        return errorResult("operation_failed", message, operation)
    }
}

function toParseOptions(parseOptions?: ParseOptionsPayload | null): Partial<EditorOptions["parseOptions"]> {
    if (parseOptions == null) {
        return {}
    }

    return {
        preserveWhitespace: parseOptions.preserve_whitespace,
        from: parseOptions.from,
        to: parseOptions.to,
    }
}

function toFocusOptions(options?: FocusOptions | null): { scrollIntoView?: boolean } {
    return {
        scrollIntoView: options?.scroll_into_view,
    }
}

function toInsertContentOptions(options?: InsertContentOptions | null) {
    return {
        parseOptions: toParseOptions(options?.parse_options),
        updateSelection: options?.update_selection,
        applyInputRules: options?.apply_input_rules,
        applyPasteRules: options?.apply_paste_rules,
        errorOnInvalidContent: options?.error_on_invalid_content,
    }
}

function toMarkOptions(options?: MarkOptions | null): { extendEmptyMarkRange?: boolean } {
    return {
        extendEmptyMarkRange: options?.extend_empty_mark_range,
    }
}

const coreCommandHandlers: {
    [K in CoreCommandKind]: CoreCommandHandler<K>
} = {
    blur: (editor, command) => runCommand(command.kind, () => editor.commands.blur()),
    clear_content: (editor, command) => runCommand(command.kind, () => editor.commands.clearContent(command.emit_update)),
    clear_nodes: (editor, command) => runCommand(command.kind, () => editor.commands.clearNodes()),
    create_paragraph_near: (editor, command) => runCommand(command.kind, () => editor.commands.createParagraphNear()),
    cut: (editor, command) => runCommand(command.kind, () => editor.commands.cut(command.range, command.target_pos)),
    delete_current_node: (editor, command) => runCommand(command.kind, () => editor.commands.deleteCurrentNode()),
    delete_node: (editor, command) => runCommand(command.kind, () => editor.commands.deleteNode(command.type_or_name)),
    delete_range: (editor, command) => runCommand(command.kind, () => editor.commands.deleteRange(command.range)),
    delete_selection: (editor, command) => runCommand(command.kind, () => editor.commands.deleteSelection()),
    enter: (editor, command) => runCommand(command.kind, () => editor.commands.enter()),
    exit_code: (editor, command) => runCommand(command.kind, () => editor.commands.exitCode()),
    extend_mark_range: (editor, command) =>
        runCommand(command.kind, () => editor.commands.extendMarkRange(command.type_or_name, command.attributes ?? undefined)),
    focus: (editor, command) =>
        runCommand(command.kind, () => editor.commands.focus(command.target ?? null, toFocusOptions(command.options))),
    insert_content: (editor, command) => {
        const parsedContent = parseContent(command.content)
        if (!parsedContent.ok) {
            return parsedContent
        }

        const {from, to} = editor.state.selection

        return runCommand(command.kind, () => editor.commands.insertContentAt(
            {from, to},
            parsedContent.value,
            toInsertContentOptions(command.options),
        ))
    },
    insert_content_at: (editor, command) => {
        const parsedContent = parseContent(command.content)
        if (!parsedContent.ok) {
            return parsedContent
        }

        return runCommand(command.kind, () => editor.commands.insertContentAt(
            command.position,
            parsedContent.value,
            toInsertContentOptions(command.options),
        ))
    },
    join_up: (editor, command) => runCommand(command.kind, () => editor.commands.joinUp()),
    join_down: (editor, command) => runCommand(command.kind, () => editor.commands.joinDown()),
    join_backward: (editor, command) => runCommand(command.kind, () => editor.commands.joinBackward()),
    join_forward: (editor, command) => runCommand(command.kind, () => editor.commands.joinForward()),
    join_item_backward: (editor, command) => runCommand(command.kind, () => editor.commands.joinItemBackward()),
    join_item_forward: (editor, command) => runCommand(command.kind, () => editor.commands.joinItemForward()),
    join_textblock_backward: (editor, command) => runCommand(command.kind, () => editor.commands.joinTextblockBackward()),
    join_textblock_forward: (editor, command) => runCommand(command.kind, () => editor.commands.joinTextblockForward()),
    keyboard_shortcut: (editor, command) => runCommand(command.kind, () => editor.commands.keyboardShortcut(command.name)),
    lift: (editor, command) =>
        runCommand(command.kind, () => editor.commands.lift(command.type_or_name, command.attributes ?? undefined)),
    lift_empty_block: (editor, command) => runCommand(command.kind, () => editor.commands.liftEmptyBlock()),
    newline_in_code: (editor, command) => runCommand(command.kind, () => editor.commands.newlineInCode()),
    reset_attributes: (editor, command) =>
        runCommand(command.kind, () => editor.commands.resetAttributes(command.type_or_name, command.attribute_names)),
    scroll_into_view: (editor, command) => runCommand(command.kind, () => editor.commands.scrollIntoView()),
    select_all: (editor, command) => runCommand(command.kind, () => editor.commands.selectAll()),
    select_node_backward: (editor, command) => runCommand(command.kind, () => editor.commands.selectNodeBackward()),
    select_node_forward: (editor, command) => runCommand(command.kind, () => editor.commands.selectNodeForward()),
    select_parent_node: (editor, command) => runCommand(command.kind, () => editor.commands.selectParentNode()),
    select_textblock_end: (editor, command) => runCommand(command.kind, () => editor.commands.selectTextblockEnd()),
    select_textblock_start: (editor, command) => runCommand(command.kind, () => editor.commands.selectTextblockStart()),
    set_mark: (editor, command) =>
        runCommand(command.kind, () => editor.commands.setMark(command.type_or_name, command.attributes ?? undefined)),
    set_meta: (editor, command) => runCommand(command.kind, () => editor.commands.setMeta(command.key, command.value)),
    set_node: (editor, command) =>
        runCommand(command.kind, () => editor.commands.setNode(command.type_or_name, command.attributes ?? undefined)),
    set_node_selection: (editor, command) => runCommand(command.kind, () => editor.commands.setNodeSelection(command.position)),
    set_text_selection: (editor, command) => runCommand(command.kind, () => editor.commands.setTextSelection(command.position)),
    split_block: (editor, command) => runCommand(command.kind, () => editor.commands.splitBlock({keepMarks: command.keep_marks})),
    toggle_list: (editor, command) => runCommand(command.kind, () => editor.commands.toggleList(
        command.list_type_or_name,
        command.item_type_or_name,
        command.keep_marks,
        command.attributes ?? undefined,
    )),
    toggle_mark: (editor, command) => runCommand(command.kind, () => editor.commands.toggleMark(
        command.type_or_name,
        command.attributes ?? undefined,
        toMarkOptions(command.options),
    )),
    toggle_node: (editor, command) => runCommand(command.kind, () => editor.commands.toggleNode(
        command.type_or_name,
        command.toggle_type_or_name,
        command.attributes ?? undefined,
    )),
    toggle_wrap: (editor, command) =>
        runCommand(command.kind, () => editor.commands.toggleWrap(command.type_or_name, command.attributes ?? undefined)),
    undo_input_rule: (editor, command) => runCommand(command.kind, () => editor.commands.undoInputRule()),
    unset_all_marks: (editor, command) => runCommand(command.kind, () => editor.commands.unsetAllMarks()),
    unset_mark: (editor, command) =>
        runCommand(command.kind, () => editor.commands.unsetMark(command.type_or_name, toMarkOptions(command.options))),
    update_attributes: (editor, command) =>
        runCommand(command.kind, () => editor.commands.updateAttributes(command.type_or_name, command.attributes)),
    wrap_in: (editor, command) =>
        runCommand(command.kind, () => editor.commands.wrapIn(command.type_or_name, command.attributes ?? undefined)),
    wrap_in_list: (editor, command) =>
        runCommand(command.kind, () => editor.commands.wrapInList(command.type_or_name, command.attributes ?? undefined)),
}

const runtimeCommandHandlers: {
    [K in RuntimeCommandKind]: RuntimeCommandHandler<K>
} = {
    set_editable: (editor, command) =>
        runCommand(command.kind, () => {
            editor.setEditable(command.editable)
        }),
}

const coreCommandKinds = new Set<CommandKind>(Object.keys(coreCommandHandlers) as CoreCommandKind[])
const runtimeCommandKinds = new Set<CommandKind>(Object.keys(runtimeCommandHandlers) as RuntimeCommandKind[])

function isCoreCommand(command: EditorCommand): command is CoreCommand {
    return coreCommandKinds.has(command.kind)
}

function isRuntimeCommand(command: EditorCommand): command is RuntimeCommand {
    return runtimeCommandKinds.has(command.kind)
}

function executeCoreCommand(editor: Editor, command: CoreCommand): BridgeResult<EmptyResponse> {
    const handler = coreCommandHandlers[command.kind] as (editor: Editor, command: CoreCommand) => BridgeResult<EmptyResponse>
    return handler(editor, command)
}

function executeRuntimeCommand(editor: Editor, command: RuntimeCommand): BridgeResult<EmptyResponse> {
    const handler = runtimeCommandHandlers[command.kind] as (
        editor: Editor,
        command: RuntimeCommand,
    ) => BridgeResult<EmptyResponse>
    return handler(editor, command)
}

function getContentOperationName(format: ContentFormat): string {
    return format === "html" ? "get_content_html" : "get_content_json"
}

function resolveDescriptors(extensionNames: string[]): BridgeResult<RuntimeDescriptor[]> {
    const descriptors: RuntimeDescriptor[] = []

    for (const extensionName of extensionNames) {
        const descriptor = extensionRegistry.get(extensionName)
        if (descriptor == null) {
            return extensionError(
                `Can not create Tiptap instance, as extension "${extensionName}" is not registered.`,
            )
        }

        descriptors.push({
            descriptor,
            selectionKeys: descriptor.selection_keys ?? [],
        })
    }

    return okResult(descriptors)
}

function buildRuntimeConfiguration(
    extensionNames: string[],
    context: ExtensionCreateContext,
): BridgeResult<RuntimeConfiguration> {
    const resolvedDescriptors = resolveDescriptors(extensionNames)
    if (!resolvedDescriptors.ok) {
        return resolvedDescriptors
    }

    const commandHandlers = new Map<ExtensionCommandKind, DescriptorCommandHandler>()
    const selectionContributors: Array<(editor: Editor) => Partial<SelectionState>> = []
    const extensions: NonNullable<EditorOptions["extensions"]> = []
    const seenSelectionKeys = new Set<SelectionKey>()

    for (const {descriptor, selectionKeys} of resolvedDescriptors.value) {
        let created: ReturnType<ExtensionDescriptor["create"]>
        try {
            created = descriptor.create(context)
        } catch (error) {
            const message = `Can not create Tiptap instance, as extension "${descriptor.name}" failed to initialize.`
            console.error(message, error)
            return errorResult(
                "extension_registration_failed",
                `${message} ${error instanceof Error ? error.message : String(error)}`,
            )
        }

        if (Array.isArray(created)) {
            extensions.push(...created)
        } else {
            extensions.push(created)
        }

        for (const selectionKey of selectionKeys) {
            if (seenSelectionKeys.has(selectionKey)) {
                return registrationError(
                    `Can not create Tiptap instance, as multiple selected extensions contribute selection key "${selectionKey}".`,
                )
            }
            seenSelectionKeys.add(selectionKey)
        }

        if (descriptor.selection_state != null) {
            selectionContributors.push(descriptor.selection_state)
        }

        for (const [commandKind, handler] of Object.entries(descriptor.commands ?? {}) as Array<
            [ExtensionCommandKind, DescriptorCommandHandler | undefined]
        >) {
            if (handler == null) {
                continue
            }

            if (commandHandlers.has(commandKind)) {
                return registrationError(
                    `Can not create Tiptap instance, as multiple selected extensions handle command "${commandKind}".`,
                )
            }

            commandHandlers.set(commandKind, handler)
        }
    }

    return okResult({
        extensions,
        commandHandlers,
        selectionContributors,
    })
}

export function init_bridge_runtime(): void {
    bridgeBindings.registerExtension = registerExtension
}

export function create(
    request: CreateRequest,
    onReady: (ready: ReadyPayload) => void,
    onChange: () => void,
    onSelection: OnSelection,
    onError: (error: BridgeError) => void,
): void {
    const unusedEditorId = requireUnusedEditorId(request.id)
    if (!unusedEditorId.ok) {
        onError(unusedEditorId.error)
        return
    }

    const editorElement = requireEditorElement(request.id)
    if (!editorElement.ok) {
        onError(editorElement.error)
        return
    }

    const runtimeConfig = buildRuntimeConfiguration(request.extensions, {
        placeholder: request.placeholder,
    })
    if (!runtimeConfig.ok) {
        onError(runtimeConfig.error)
        return
    }

    const parsedContent = parseContent(request.content)
    if (!parsedContent.ok) {
        onError(parsedContent.error)
        return
    }

    const generation = allocateGeneration()
    editorSlots.set(request.id, {generation})

    const createdEditor = runOperation("create_editor", () =>
        editorFactory({
            element: editorElement.value,
            editable: request.editable,
            extensions: runtimeConfig.value.extensions,
            injectCSS: false,
            content: parsedContent.value,
            onUpdate: () => {
                onChange()
            },
            onTransaction: ({editor}) => {
                emitSelectionForCurrentEditor(request.id, editor)
            },
        }),
    )
    if (!createdEditor.ok) {
        const slot = editorSlots.get(request.id)
        if (slot?.generation === generation && slot.entry == null) {
            editorSlots.delete(request.id)
        }
        onError(createdEditor.error)
        return
    }

    if (!setEditorEntry(request.id, generation, createdEditor.value, onSelection, runtimeConfig.value)) {
        return
    }

    const editorEntry = getEditorEntry(request.id)
    if (editorEntry != null) {
        onReady({generation})
        emitSelectionState(editorEntry, {force: true})
    } else {
        onReady({generation})
    }
}

export function destroy(id: string): void {
    destroySlot(id)
}

export function command(request: {
    id: string
    generation: number
    command: EditorCommand
}): BridgeResult<EmptyResponse> {
    const {id, generation, command} = request

    return withEditor(id, generation, command.kind, (editorEntry) => {
        if (isRuntimeCommand(command)) {
            return executeRuntimeCommand(editorEntry.editor, command)
        }

        if (isCoreCommand(command)) {
            return executeCoreCommand(editorEntry.editor, command)
        }

        const extensionCommand = command as ExtensionCommand
        const handler = editorEntry.commandHandlers.get(extensionCommand.kind)
        if (handler == null) {
            return extensionError(
                `Can not execute ${extensionCommand.kind} for Tiptap instance "${id}", as no active extension provides this command.`,
                extensionCommand.kind,
            )
        }

        return runCommand(extensionCommand.kind, () => handler(editorEntry.editor, extensionCommand))
    })
}

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
            return withEditor(id, generation, request.kind, (editorEntry) => {
                const parsedContent = parseContent(request.content)
                if (!parsedContent.ok) {
                    return parsedContent
                }

                return runCommand(request.kind, () =>
                    editorEntry.editor.commands.setContent(
                        parsedContent.value,
                        request.options?.emit_update ?? false,
                        toParseOptions(request.options?.parse_options),
                        {
                            errorOnInvalidContent: request.options?.error_on_invalid_content,
                        },
                    ),
                )
            })
    }
}

export const __testing = {
    getEditorEntry,
    getRegisteredExtensionNames(): string[] {
        return [...extensionRegistry.keys()]
    },
    hasRegisteredExtension(name: string): boolean {
        return extensionRegistry.has(name)
    },
    registerExtension,
    reset(): void {
        editorSlots.clear()
        extensionRegistry.clear()
        installHostedModules(bridgeBindings.modules)
        bridgeBindings.registerExtension = registerExtension
        documentOverride = undefined
        editorFactory = createDefaultEditor
        nextGeneration = 1
    },
    getSlotCount(): number {
        return editorSlots.size
    },
    setDocument(nextDocument: DocumentLookup): void {
        documentOverride = nextDocument
    },
    setEditorFactory(nextFactory: EditorFactory): void {
        editorFactory = nextFactory
    },
}
