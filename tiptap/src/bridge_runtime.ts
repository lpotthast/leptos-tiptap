import type {Content, Editor, EditorOptions} from "@tiptap/core"

import {
    emptySelectionState,
    getOrCreateBridgeBindings,
    type BridgeError,
    type BridgeResult,
    type CommandKind,
    type ContentFormat,
    type ContentPayload,
    type CreateRequest,
    type DocumentCall,
    type DocumentResponse,
    type EditorCommand,
    type ErrorKind,
    type ExtensionDescriptor,
    type ReadyPayload,
    type SelectionKey,
    type SelectionState,
} from "./bridge_api.ts"

type TiptapCoreModule = typeof import("@tiptap/core")
type DocumentLookup = Pick<Document, "getElementById">
type CreateEditorOptions = Partial<EditorOptions>
type EditorConstructor = new (options?: CreateEditorOptions) => Editor
type EditorFactory = (options: CreateEditorOptions) => Editor
type OnSelection = (selectionState: SelectionState) => void
type DescriptorCommandHandler = (editor: Editor, command: EditorCommand) => boolean | void

type EditorEntry = {
    editor: Editor
    onSelection: OnSelection
    commandHandlers: Map<CommandKind, DescriptorCommandHandler>
    selectionContributors: Array<(editor: Editor) => Partial<SelectionState>>
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
    commandHandlers: Map<CommandKind, DescriptorCommandHandler>
    selectionContributors: Array<(editor: Editor) => Partial<SelectionState>>
}

const editorSlots = new Map<string, EditorSlot>()
const extensionRegistry = new Map<string, ExtensionDescriptor>()
const bridgeBindings = getOrCreateBridgeBindings()

let documentOverride: DocumentLookup | undefined
let editorFactory: EditorFactory = createDefaultEditor

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
    runtimeConfig: RuntimeConfiguration,
): boolean {
    const slot = getOrCreateSlot(id)
    if (slot.generation !== generation) {
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
    return runOperation(getContentOperationName(format), () => (
        format === "html"
            ? {
                format,
                value: editor.getHTML(),
            }
            : {
                format,
                value: JSON.stringify(editor.getJSON()),
            }
    ))
}

function emitSelectionState(editorEntry: EditorEntry): void {
    editorEntry.onSelection(getSelectionState(editorEntry))
}

function getSelectionState(editorEntry: EditorEntry): SelectionState {
    const state = emptySelectionState()

    for (const contribute of editorEntry.selectionContributors) {
        Object.assign(state, contribute(editorEntry.editor))
    }

    return state
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
): BridgeResult<null> {
    try {
        const result = work()
        if (result === false) {
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

function buildRuntimeConfiguration(extensionNames: string[]): BridgeResult<RuntimeConfiguration> {
    const resolvedDescriptors = resolveDescriptors(extensionNames)
    if (!resolvedDescriptors.ok) {
        return resolvedDescriptors
    }

    const commandHandlers = new Map<CommandKind, DescriptorCommandHandler>()
    const selectionContributors: Array<(editor: Editor) => Partial<SelectionState>> = []
    const extensions: NonNullable<EditorOptions["extensions"]> = []
    const seenSelectionKeys = new Set<SelectionKey>()

    for (const {descriptor, selectionKeys} of resolvedDescriptors.value) {
        const created = descriptor.create()
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
            [CommandKind, DescriptorCommandHandler | undefined]
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
    const generation = invalidateSlot(request.id)

    const editorElement = requireEditorElement(request.id)
    if (!editorElement.ok) {
        onError(editorElement.error)
        return
    }

    const runtimeConfig = buildRuntimeConfiguration(request.extensions)
    if (!runtimeConfig.ok) {
        onError(runtimeConfig.error)
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
            extensions: runtimeConfig.value.extensions,
            injectCSS: false,
            content: parsedContent.value,
            onUpdate: onChange,
            onSelectionUpdate: ({editor}) => {
                const editorEntry = getEditorEntry(request.id)
                if (editorEntry != null && editorEntry.editor === editor) {
                    emitSelectionState(editorEntry)
                }
            },
        }),
    )
    if (!createdEditor.ok) {
        onError(createdEditor.error)
        return
    }

    if (!setEditorEntry(request.id, generation, createdEditor.value, onSelection, runtimeConfig.value)) {
        return
    }

    const editorEntry = getEditorEntry(request.id)
    if (editorEntry != null) {
        emitSelectionState(editorEntry)
    }
    onReady({generation})
}

export function destroy(id: string): void {
    invalidateSlot(id)
}

export function command(request: {
    id: string
    generation: number
    command: EditorCommand
}): BridgeResult<null> {
    const {id, generation, command} = request

    if (command.kind === "set_editable") {
        return withEditor(id, generation, command.kind, ({editor}) =>
            runOperation(command.kind, () => {
                editor.setEditable(command.editable)
                return null
            }),
        )
    }

    return withEditor(id, generation, command.kind, (editorEntry) => {
        const handler = editorEntry.commandHandlers.get(command.kind)
        if (handler == null) {
            return extensionError(
                `Can not execute ${command.kind} for Tiptap instance "${id}", as no active extension provides this command.`,
                command.kind,
            )
        }

        const result = runCommand(command.kind, () => handler(editorEntry.editor, command))
        if (result.ok) {
            emitSelectionState(editorEntry)
        }
        return result
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

                const result = runCommand(request.kind, () =>
                    editorEntry.editor.commands.setContent(parsedContent.value),
                )
                if (!result.ok) {
                    return result
                }

                emitSelectionState(editorEntry)
                return okResult({kind: "empty"})
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
        bridgeBindings.registerExtension = registerExtension
        documentOverride = undefined
        editorFactory = createDefaultEditor
    },
    setDocument(nextDocument: DocumentLookup): void {
        documentOverride = nextDocument
    },
    setEditorFactory(nextFactory: EditorFactory): void {
        editorFactory = nextFactory
    },
}
