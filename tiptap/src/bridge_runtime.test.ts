import assert from "node:assert/strict"
import test from "node:test"
import type {Editor, EditorOptions} from "@tiptap/core"

import type {CreateRequest, EditorCommand, SelectionState} from "./bridge_api.ts"
import {__testing, command, create, destroy, document} from "./bridge_runtime.ts"
import {register_blockquote} from "./extensions/tiptap_blockquote.ts"
import {register_bold} from "./extensions/tiptap_bold.ts"
import {register_bullet_list} from "./extensions/tiptap_bullet_list.ts"
import {register_code} from "./extensions/tiptap_code.ts"
import {register_code_block} from "./extensions/tiptap_code_block.ts"
import {register_document} from "./extensions/tiptap_document.ts"
import {register_dropcursor} from "./extensions/tiptap_dropcursor.ts"
import {register_gapcursor} from "./extensions/tiptap_gapcursor.ts"
import {register_hard_break} from "./extensions/tiptap_hard_break.ts"
import {register_heading} from "./extensions/tiptap_heading.ts"
import {register_highlight} from "./extensions/tiptap_highlight.ts"
import {register_history} from "./extensions/tiptap_history.ts"
import {register_horizontal_rule} from "./extensions/tiptap_horizontal_rule.ts"
import {register_image} from "./extensions/tiptap_image.ts"
import {register_italic} from "./extensions/tiptap_italic.ts"
import {register_link} from "./extensions/tiptap_link.ts"
import {register_list_item} from "./extensions/tiptap_list_item.ts"
import {register_ordered_list} from "./extensions/tiptap_ordered_list.ts"
import {register_paragraph} from "./extensions/tiptap_paragraph.ts"
import {register_placeholder} from "./extensions/tiptap_placeholder.ts"
import {register_strike} from "./extensions/tiptap_strike.ts"
import {register_text} from "./extensions/tiptap_text.ts"
import {register_text_align} from "./extensions/tiptap_text_align.ts"
import {register_youtube} from "./extensions/tiptap_youtube.ts"

const DEFAULT_EXTENSION_NAMES: string[] = [
    "blockquote",
    "bold",
    "bullet_list",
    "code",
    "code_block",
    "document",
    "dropcursor",
    "gapcursor",
    "hard_break",
    "heading",
    "history",
    "horizontal_rule",
    "italic",
    "list_item",
    "ordered_list",
    "paragraph",
    "placeholder",
    "strike",
    "text",
    "text_align",
    "highlight",
    "image",
    "link",
    "youtube",
]

function registerDefaultExtensions(): void {
    if (__testing.hasRegisteredExtension("blockquote")) {
        return
    }

    register_blockquote()
    register_bold()
    register_bullet_list()
    register_code()
    register_code_block()
    register_document()
    register_dropcursor()
    register_gapcursor()
    register_hard_break()
    register_heading()
    register_history()
    register_horizontal_rule()
    register_italic()
    register_list_item()
    register_ordered_list()
    register_paragraph()
    register_placeholder()
    register_strike()
    register_text()
    register_text_align()
    register_highlight()
    register_image()
    register_link()
    register_youtube()
}

class FakeEditor {
    destroyed = false
    editable: boolean | undefined
    content: unknown
    extensions: NonNullable<EditorOptions["extensions"]> | undefined
    onUpdate: NonNullable<EditorOptions["onUpdate"]> | undefined
    onSelectionUpdate: NonNullable<EditorOptions["onSelectionUpdate"]> | undefined
    onTransaction: NonNullable<EditorOptions["onTransaction"]> | undefined
    activeStates: Record<string, boolean> = {}
    state = {
        selection: {
            from: 1,
            to: 1,
        },
    }
    getHtmlError: Error | undefined
    getJsonError: Error | undefined
    setContentError: Error | undefined
    setContentResult = true
    commandError: Error | undefined
    commandResult = true
    chainRunError: Error | undefined
    chainRunResult = true
    chainCalls: Array<{ name: string; args: unknown[] }> = []
    commandCalls: Array<{ name: string; args: unknown[] }> = []

    constructor(readonly options: Pick<Partial<EditorOptions>, "content" | "extensions" | "onUpdate" | "onSelectionUpdate" | "onTransaction">) {
        this.content = options.content
        this.extensions = options.extensions
        this.onUpdate = options.onUpdate
        this.onSelectionUpdate = options.onSelectionUpdate
        this.onTransaction = options.onTransaction
    }

    commands = new Proxy({}, {
        get: (_target, property) => (...args: unknown[]) => {
            const name = String(property)
            this.commandCalls.push({name, args})

            if (this.commandError != null) {
                throw this.commandError
            }

            if (name === "setContent") {
                if (this.setContentError != null) {
                    throw this.setContentError
                }

                this.content = args[0]
                return this.setContentResult
            }

            return this.commandResult
        },
    })

    destroy(): void {
        this.destroyed = true
    }

    getHTML(): string {
        if (this.getHtmlError != null) {
            throw this.getHtmlError
        }

        return "<p>fake</p>"
    }

    getJSON(): unknown {
        if (this.getJsonError != null) {
            throw this.getJsonError
        }

        return {type: "doc", content: []}
    }

    setEditable(editable: boolean): void {
        this.editable = editable
    }

    isActive(nameOrAttributes?: string | Record<string, unknown>, attributes?: Record<string, unknown>): boolean {
        if (typeof nameOrAttributes === "string") {
            if (nameOrAttributes === "heading" && typeof attributes?.level === "number") {
                return this.activeStates[`heading:${attributes.level}`] ?? false
            }

            return this.activeStates[nameOrAttributes] ?? false
        }

        if (nameOrAttributes != null && typeof nameOrAttributes.textAlign === "string") {
            return this.activeStates[`textAlign:${nameOrAttributes.textAlign}`] ?? false
        }

        return false
    }

    chain(): unknown {
        let proxy: unknown

        proxy = new Proxy({
            run: () => {
                if (this.chainRunError != null) {
                    throw this.chainRunError
                }

                return this.chainRunResult
            },
        }, {
            get: (target, property) => {
                if (property === "run") {
                    return target.run
                }

                return (...args: unknown[]) => {
                    this.chainCalls.push({name: String(property), args})
                    return proxy
                }
            },
        })

        return proxy
    }

    emitSelectionUpdate(): void {
        this.onSelectionUpdate?.({editor: this as unknown as Editor} as never)
    }

    emitTransaction(): void {
        this.onTransaction?.({editor: this as unknown as Editor} as never)
    }

    emitUpdate(): void {
        this.onUpdate?.({editor: this as unknown as Editor} as never)
    }
}

function createFakeDocument(
    elementsById: Record<string, HTMLElement | null> = {},
): Pick<Document, "getElementById"> {
    return {
        getElementById(id: string): HTMLElement | null {
            return elementsById[id] ?? null
        },
    }
}

function createRequest(
    id = "id",
    value = "<p>hello</p>",
    format: "html" | "json" = "html",
): CreateRequest {
    return {
        id,
        content: {
            format,
            value,
        },
        editable: true,
        extensions: [...DEFAULT_EXTENSION_NAMES],
    }
}

function withSuppressedConsoleError<T>(run: () => T): T {
    const originalConsoleError = console.error
    console.error = () => {
    }

    try {
        return run()
    } finally {
        console.error = originalConsoleError
    }
}

function withCapturedConsoleError<T>(run: () => T): { result: T; callCount: number } {
    const originalConsoleError = console.error
    let callCount = 0
    console.error = () => {
        callCount += 1
    }

    try {
        return {
            result: run(),
            callCount,
        }
    } finally {
        console.error = originalConsoleError
    }
}

function setupAdapterTest(
    options: {
        elementsById?: Record<string, HTMLElement | null>
        makeEditor?: (options: Pick<Partial<EditorOptions>, "content" | "extensions" | "onUpdate" | "onSelectionUpdate" | "onTransaction">) => FakeEditor
    } = {},
): FakeEditor[] {
    __testing.reset()
    registerDefaultExtensions()
    __testing.setDocument(createFakeDocument(options.elementsById ?? {id: {} as HTMLElement}))

    const createdEditors: FakeEditor[] = []
    __testing.setEditorFactory((editorOptions) => {
        const editor = options.makeEditor?.({
            content: editorOptions.content,
            extensions: editorOptions.extensions,
            onUpdate: editorOptions.onUpdate,
            onSelectionUpdate: editorOptions.onSelectionUpdate,
            onTransaction: editorOptions.onTransaction,
        }) ??
            new FakeEditor({
                content: editorOptions.content,
                extensions: editorOptions.extensions,
                onUpdate: editorOptions.onUpdate,
                onSelectionUpdate: editorOptions.onSelectionUpdate,
                onTransaction: editorOptions.onTransaction,
            })
        createdEditors.push(editor)
        return editor as unknown as Editor
    })

    return createdEditors
}

function createAndGetGeneration(request = createRequest()): number {
    let ready: { generation: number } | undefined

    create(
        request,
        (payload) => {
            ready = payload
        },
        () => {
        },
        () => {
        },
        () => {
        },
    )

    assert.notEqual(ready, undefined)
    if (ready == null) {
        throw new Error("create should provide a generation")
    }

    return ready.generation
}

function assertCommandDispatch(
    dispatchedCommand: EditorCommand,
    expectedChainCalls: Array<{ name: string; args: unknown[] }>,
): void {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()
    const result = command({
        id: "id",
        generation,
        command: dispatchedCommand,
    })

    assert.equal(result.ok, true)
    assert.deepEqual(editor.chainCalls, expectedChainCalls)
    assert.deepEqual(editor.commandCalls, [])
}

function assertCoreCommandDispatch(
    dispatchedCommand: EditorCommand,
    expectedCommandCalls: Array<{ name: string; args: unknown[] }>,
): void {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()
    const result = command({
        id: "id",
        generation,
        command: dispatchedCommand,
    })

    assert.equal(result.ok, true)
    assert.deepEqual(editor.commandCalls, expectedCommandCalls)
    assert.deepEqual(editor.chainCalls, [])
}

test("reports mount failure immediately when the editor element is missing", () => {
    __testing.reset()
    __testing.setDocument(createFakeDocument())

    const errors: Array<{ kind: string; message: string }> = []

    withSuppressedConsoleError(() => {
        create(
            createRequest(),
            () => {
                throw new Error("create should not succeed")
            },
            () => {
            },
            () => {
            },
            (error) => errors.push(error),
        )
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "editor_mount_failed")
    assert.equal(__testing.getEditorEntry("id"), undefined)
})

test("reports invalid JSON during create without constructing an editor", () => {
    const createdEditors = setupAdapterTest()
    const errors: Array<{ kind: string; message: string }> = []

    withSuppressedConsoleError(() => {
        create(
            createRequest("id", "{", "json"),
            () => {
                throw new Error("create should not succeed")
            },
            () => {
            },
            () => {
            },
            (error) => errors.push(error),
        )
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "invalid_content")
    assert.equal(createdEditors.length, 0)
    assert.equal(__testing.getEditorEntry("id"), undefined)
})

test("rejects duplicate live ids before replacement content validation", () => {
    const createdEditors = setupAdapterTest()
    const errors: Array<{ kind: string; message: string }> = []

    create(
        createRequest("id", "<p>first</p>"),
        () => {
        },
        () => {
        },
        () => {
        },
        (error) => errors.push(error),
    )

    const firstEditor = createdEditors[0]
    assert.notEqual(firstEditor, undefined)
    assert.equal(__testing.getEditorEntry("id")?.editor, firstEditor as unknown as Editor)

    withSuppressedConsoleError(() => {
        create(
            createRequest("id", "{", "json"),
            () => {
                throw new Error("replacement create should not succeed")
            },
            () => {
            },
            () => {
            },
            (error) => errors.push(error),
        )
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "duplicate_editor_id")
    assert.equal(firstEditor?.destroyed, false)
    assert.equal(__testing.getEditorEntry("id")?.editor, firstEditor as unknown as Editor)
})

test("rejects duplicate live ids before replacement extension validation", () => {
    const createdEditors = setupAdapterTest()
    const errors: Array<{ kind: string; message: string }> = []

    create(
        createRequest("id", "<p>first</p>"),
        () => {
        },
        () => {
        },
        () => {
        },
        (error) => errors.push(error),
    )

    const firstEditor = createdEditors[0]
    assert.notEqual(firstEditor, undefined)

    withSuppressedConsoleError(() => {
        create(
            {
                ...createRequest("id", "<p>second</p>"),
                extensions: ["missing_extension"],
            },
            () => {
                throw new Error("replacement create should not succeed")
            },
            () => {
            },
            () => {
            },
            (error) => errors.push(error),
        )
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "duplicate_editor_id")
    assert.equal(firstEditor?.destroyed, false)
    assert.equal(__testing.getEditorEntry("id")?.editor, firstEditor as unknown as Editor)
})

test("reports extension initialization failures through onError without throwing", () => {
    setupAdapterTest()
    __testing.registerExtension({
        name: "throwing_extension",
        create: () => {
            throw new Error("boom")
        },
    })

    const errors: Array<{ kind: string; message: string }> = []

    withSuppressedConsoleError(() => {
        assert.doesNotThrow(() => {
            create(
                {
                    id: "id",
                    content: {
                        format: "html",
                        value: "<p>hello</p>",
                    },
                    editable: true,
                    extensions: ["throwing_extension"],
                },
                () => {
                    throw new Error("create should not succeed")
                },
                () => {
                },
                () => {
                },
                (error) => errors.push(error),
            )
        })
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "extension_registration_failed")
    assert.match(errors[0]?.message ?? "", /throwing_extension/)
    assert.match(errors[0]?.message ?? "", /boom/)
    assert.equal(__testing.getSlotCount(), 0)
})

test("supports base schema extensions when they are requested explicitly", () => {
    const createdEditors = setupAdapterTest()
    const errors: Array<{ kind: string; message: string }> = []

    let generation: number | undefined
    create(
        {
            ...createRequest(),
            extensions: ["document", "paragraph", "text"],
        },
        (payload) => {
            generation = payload.generation
        },
        () => {
        },
        () => {
        },
        (error) => errors.push(error),
    )

    assert.equal(errors.length, 0)
    assert.equal(generation, 1)

    const extensionNames = createdEditors[0]?.extensions?.map(
        (extension) => (extension as {name?: string}).name,
    )
    assert.deepEqual(extensionNames, ["doc", "paragraph", "text"])

    const result = command({
        id: "id",
        generation: generation ?? 0,
        command: {
            kind: "set_paragraph",
        },
    })

    assert.equal(result.ok, true)
})

test("emits sparse selection state for extensions without contributors", () => {
    setupAdapterTest()

    let latestSelection: SelectionState | undefined

    create(
        {
            ...createRequest(),
            extensions: ["document", "text"],
        },
        () => {
        },
        () => {
        },
        (selectionState) => {
            latestSelection = selectionState
        },
        () => {
        },
    )

    assert.deepEqual(latestSelection, {})
})

test("configures placeholder extension from create request", () => {
    const createdEditors = setupAdapterTest()

    create(
        {
            ...createRequest(),
            extensions: ["document", "paragraph", "text", "placeholder"],
            placeholder: "Start typing here...",
        },
        () => {
        },
        () => {
        },
        () => {
        },
        () => {
        },
    )

    const placeholderExtension = createdEditors[0]?.extensions?.find(
        (extension) => (extension as { name?: string }).name === "placeholder",
    ) as { options?: { placeholder?: unknown } } | undefined

    assert.equal(placeholderExtension?.options?.placeholder, "Start typing here...")
})

test("dispatches parity commands to the expected chained editor methods", () => {
    const cases: Array<{
        name: string
        command: EditorCommand
        chainCalls: Array<{ name: string; args: unknown[] }>
    }> = [
        {
            name: "set_blockquote",
            command: {kind: "set_blockquote"},
            chainCalls: [{name: "focus", args: []}, {name: "setBlockquote", args: []}],
        },
        {
            name: "unset_blockquote",
            command: {kind: "unset_blockquote"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetBlockquote", args: []}],
        },
        {
            name: "set_bold",
            command: {kind: "set_bold"},
            chainCalls: [{name: "focus", args: []}, {name: "setBold", args: []}],
        },
        {
            name: "unset_bold",
            command: {kind: "unset_bold"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetBold", args: []}],
        },
        {
            name: "set_code",
            command: {kind: "set_code"},
            chainCalls: [{name: "focus", args: []}, {name: "setCode", args: []}],
        },
        {
            name: "toggle_code_block",
            command: {kind: "toggle_code_block", attributes: {language: "rust"}},
            chainCalls: [{name: "focus", args: []}, {name: "toggleCodeBlock", args: [{language: "rust"}]}],
        },
        {
            name: "set_hard_break",
            command: {kind: "set_hard_break"},
            chainCalls: [{name: "focus", args: []}, {name: "setHardBreak", args: []}],
        },
        {
            name: "set_heading",
            command: {kind: "set_heading", level: 2},
            chainCalls: [{name: "focus", args: []}, {name: "setHeading", args: [{level: 2}]}],
        },
        {
            name: "set_highlight",
            command: {kind: "set_highlight", attributes: {color: "#ff0"}},
            chainCalls: [{name: "focus", args: []}, {name: "setHighlight", args: [{color: "#ff0"}]}],
        },
        {
            name: "unset_highlight",
            command: {kind: "unset_highlight"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetHighlight", args: []}],
        },
        {
            name: "undo",
            command: {kind: "undo"},
            chainCalls: [{name: "focus", args: []}, {name: "undo", args: []}],
        },
        {
            name: "redo",
            command: {kind: "redo"},
            chainCalls: [{name: "focus", args: []}, {name: "redo", args: []}],
        },
        {
            name: "set_horizontal_rule",
            command: {kind: "set_horizontal_rule"},
            chainCalls: [{name: "focus", args: []}, {name: "setHorizontalRule", args: []}],
        },
        {
            name: "set_italic",
            command: {kind: "set_italic"},
            chainCalls: [{name: "focus", args: []}, {name: "setItalic", args: []}],
        },
        {
            name: "unset_italic",
            command: {kind: "unset_italic"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetItalic", args: []}],
        },
        {
            name: "split_list_item",
            command: {kind: "split_list_item", attributes: {checked: true}},
            chainCalls: [{name: "focus", args: []}, {name: "splitListItem", args: ["listItem", {checked: true}]}],
        },
        {
            name: "sink_list_item",
            command: {kind: "sink_list_item"},
            chainCalls: [{name: "focus", args: []}, {name: "sinkListItem", args: ["listItem"]}],
        },
        {
            name: "lift_list_item",
            command: {kind: "lift_list_item"},
            chainCalls: [{name: "focus", args: []}, {name: "liftListItem", args: ["listItem"]}],
        },
        {
            name: "set_strike",
            command: {kind: "set_strike"},
            chainCalls: [{name: "focus", args: []}, {name: "setStrike", args: []}],
        },
        {
            name: "unset_strike",
            command: {kind: "unset_strike"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetStrike", args: []}],
        },
        {
            name: "set_text_align",
            command: {kind: "set_text_align", alignment: "left"},
            chainCalls: [{name: "focus", args: []}, {name: "setTextAlign", args: ["left"]}],
        },
        {
            name: "toggle_text_align",
            command: {kind: "toggle_text_align", alignment: "center"},
            chainCalls: [{name: "focus", args: []}, {name: "toggleTextAlign", args: ["center"]}],
        },
        {
            name: "unset_text_align",
            command: {kind: "unset_text_align"},
            chainCalls: [{name: "focus", args: []}, {name: "unsetTextAlign", args: []}],
        },
        {
            name: "set_image",
            command: {kind: "set_image", src: "https://example.com/test.png"},
            chainCalls: [{name: "focus", args: []}, {name: "setImage", args: [{src: "https://example.com/test.png", alt: undefined, title: undefined}]}],
        },
        {
            name: "set_link",
            command: {
                kind: "set_link",
                href: "https://example.com",
                target: "_blank",
                rel: "noopener",
                class: "link",
            },
            chainCalls: [{name: "focus", args: []}, {name: "setLink", args: [{href: "https://example.com", target: "_blank", rel: "noopener", class: "link"}]}],
        },
        {
            name: "toggle_link",
            command: {
                kind: "toggle_link",
                href: "https://example.com",
                target: "_blank",
                rel: "noopener",
                class: "link",
            },
            chainCalls: [{name: "focus", args: []}, {name: "toggleLink", args: [{href: "https://example.com", target: "_blank", rel: "noopener", class: "link"}]}],
        },
    ]

    for (const testCase of cases) {
        assertCommandDispatch(testCase.command, testCase.chainCalls)
    }
})

test("dispatches core commands directly to editor.commands without implicit focus", () => {
    const cases: Array<{
        name: string
        command: EditorCommand
        commandCalls: Array<{ name: string; args: unknown[] }>
    }> = [
        {
            name: "blur",
            command: {kind: "blur"},
            commandCalls: [{name: "blur", args: []}],
        },
        {
            name: "clear_content",
            command: {kind: "clear_content", emit_update: true},
            commandCalls: [{name: "clearContent", args: [true]}],
        },
        {
            name: "clear_nodes",
            command: {kind: "clear_nodes"},
            commandCalls: [{name: "clearNodes", args: []}],
        },
        {
            name: "create_paragraph_near",
            command: {kind: "create_paragraph_near"},
            commandCalls: [{name: "createParagraphNear", args: []}],
        },
        {
            name: "cut",
            command: {kind: "cut", range: {from: 1, to: 3}, target_pos: 7},
            commandCalls: [{name: "cut", args: [{from: 1, to: 3}, 7]}],
        },
        {
            name: "delete_node",
            command: {kind: "delete_node", type_or_name: "paragraph"},
            commandCalls: [{name: "deleteNode", args: ["paragraph"]}],
        },
        {
            name: "delete_range",
            command: {kind: "delete_range", range: {from: 2, to: 4}},
            commandCalls: [{name: "deleteRange", args: [{from: 2, to: 4}]}],
        },
        {
            name: "extend_mark_range",
            command: {kind: "extend_mark_range", type_or_name: "link", attributes: {href: "https://example.com"}},
            commandCalls: [{name: "extendMarkRange", args: ["link", {href: "https://example.com"}]}],
        },
        {
            name: "focus",
            command: {kind: "focus", target: "end", options: {scroll_into_view: false}},
            commandCalls: [{name: "focus", args: ["end", {scrollIntoView: false}]}],
        },
        {
            name: "insert_content",
            command: {
                kind: "insert_content",
                content: {format: "html", value: "<p>inserted</p>"},
                options: {update_selection: false},
            },
            commandCalls: [{name: "insertContentAt", args: [{from: 1, to: 1}, "<p>inserted</p>", {
                parseOptions: {},
                updateSelection: false,
                applyInputRules: undefined,
                applyPasteRules: undefined,
                errorOnInvalidContent: undefined,
            }]}],
        },
        {
            name: "insert_content_at",
            command: {
                kind: "insert_content_at",
                position: {from: 1, to: 2},
                content: {format: "json", value: {type: "paragraph"}},
                options: {error_on_invalid_content: true},
            },
            commandCalls: [{name: "insertContentAt", args: [{from: 1, to: 2}, {type: "paragraph"}, {
                parseOptions: {},
                updateSelection: undefined,
                applyInputRules: undefined,
                applyPasteRules: undefined,
                errorOnInvalidContent: true,
            }]}],
        },
        {
            name: "join_up",
            command: {kind: "join_up"},
            commandCalls: [{name: "joinUp", args: []}],
        },
        {
            name: "join_item_forward",
            command: {kind: "join_item_forward"},
            commandCalls: [{name: "joinItemForward", args: []}],
        },
        {
            name: "keyboard_shortcut",
            command: {kind: "keyboard_shortcut", name: "Mod-b"},
            commandCalls: [{name: "keyboardShortcut", args: ["Mod-b"]}],
        },
        {
            name: "lift",
            command: {kind: "lift", type_or_name: "blockquote"},
            commandCalls: [{name: "lift", args: ["blockquote", undefined]}],
        },
        {
            name: "reset_attributes",
            command: {kind: "reset_attributes", type_or_name: "heading", attribute_names: ["level"]},
            commandCalls: [{name: "resetAttributes", args: ["heading", ["level"]]}],
        },
        {
            name: "select_textblock_start",
            command: {kind: "select_textblock_start"},
            commandCalls: [{name: "selectTextblockStart", args: []}],
        },
        {
            name: "set_mark",
            command: {kind: "set_mark", type_or_name: "highlight", attributes: {color: "#ff0"}},
            commandCalls: [{name: "setMark", args: ["highlight", {color: "#ff0"}]}],
        },
        {
            name: "set_meta",
            command: {kind: "set_meta", key: "source", value: {toolbar: true}},
            commandCalls: [{name: "setMeta", args: ["source", {toolbar: true}]}],
        },
        {
            name: "set_node",
            command: {kind: "set_node", type_or_name: "paragraph"},
            commandCalls: [{name: "setNode", args: ["paragraph", undefined]}],
        },
        {
            name: "set_text_selection",
            command: {kind: "set_text_selection", position: 9},
            commandCalls: [{name: "setTextSelection", args: [9]}],
        },
        {
            name: "split_block",
            command: {kind: "split_block", keep_marks: true},
            commandCalls: [{name: "splitBlock", args: [{keepMarks: true}]}],
        },
        {
            name: "toggle_list",
            command: {kind: "toggle_list", list_type_or_name: "bulletList", item_type_or_name: "listItem", keep_marks: true, attributes: {tight: true}},
            commandCalls: [{name: "toggleList", args: ["bulletList", "listItem", true, {tight: true}]}],
        },
        {
            name: "toggle_mark",
            command: {kind: "toggle_mark", type_or_name: "bold", options: {extend_empty_mark_range: true}},
            commandCalls: [{name: "toggleMark", args: ["bold", undefined, {extendEmptyMarkRange: true}]}],
        },
        {
            name: "toggle_node",
            command: {kind: "toggle_node", type_or_name: "heading", toggle_type_or_name: "paragraph", attributes: {level: 2}},
            commandCalls: [{name: "toggleNode", args: ["heading", "paragraph", {level: 2}]}],
        },
        {
            name: "toggle_wrap",
            command: {kind: "toggle_wrap", type_or_name: "blockquote"},
            commandCalls: [{name: "toggleWrap", args: ["blockquote", undefined]}],
        },
        {
            name: "undo_input_rule",
            command: {kind: "undo_input_rule"},
            commandCalls: [{name: "undoInputRule", args: []}],
        },
        {
            name: "unset_all_marks",
            command: {kind: "unset_all_marks"},
            commandCalls: [{name: "unsetAllMarks", args: []}],
        },
        {
            name: "unset_mark",
            command: {kind: "unset_mark", type_or_name: "link", options: {extend_empty_mark_range: true}},
            commandCalls: [{name: "unsetMark", args: ["link", {extendEmptyMarkRange: true}]}],
        },
        {
            name: "update_attributes",
            command: {kind: "update_attributes", type_or_name: "image", attributes: {src: "https://example.com/a.png"}},
            commandCalls: [{name: "updateAttributes", args: ["image", {src: "https://example.com/a.png"}]}],
        },
        {
            name: "wrap_in",
            command: {kind: "wrap_in", type_or_name: "blockquote"},
            commandCalls: [{name: "wrapIn", args: ["blockquote", undefined]}],
        },
        {
            name: "wrap_in_list",
            command: {kind: "wrap_in_list", type_or_name: "orderedList"},
            commandCalls: [{name: "wrapInList", args: ["orderedList", undefined]}],
        },
    ]

    for (const testCase of cases) {
        assertCoreCommandDispatch(testCase.command, testCase.commandCalls)
    }
})

test("rejects invalid text alignment values", () => {
    setupAdapterTest()

    const generation = createAndGetGeneration()

    const result = withSuppressedConsoleError(() =>
        command({
            id: "id",
            generation,
            command: {
                kind: "set_text_align",
                alignment: "diagonal" as never,
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("set_text_align should fail for invalid alignment")
    }

    assert.equal(result.error.kind, "command_rejected")
    assert.equal(result.error.operation, "set_text_align")
})

test("reports invalid JSON on document set_content distinctly from editor availability", () => {
    setupAdapterTest()

    const generation = createAndGetGeneration()

    const result = withSuppressedConsoleError(() =>
        document({
            id: "id",
            generation,
            request: {
                kind: "set_content",
                content: {
                    format: "json",
                    value: "{",
                },
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("set_content should fail")
    }

    assert.equal(result.error.kind, "invalid_content")
    assert.match(result.error.message, /Could not parse Tiptap JSON content/)
})

test("forwards document set_content options to editor.commands.setContent", () => {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()

    const result = document({
        id: "id",
        generation,
        request: {
            kind: "set_content",
            content: {
                format: "html",
                value: "<p>updated</p>",
            },
            options: {
                emit_update: true,
                parse_options: {
                    preserve_whitespace: "full",
                    from: 1,
                    to: 2,
                },
                error_on_invalid_content: true,
            },
        },
    })

    assert.equal(result.ok, true)
    assert.deepEqual(editor.commandCalls, [{
        name: "setContent",
        args: [
            "<p>updated</p>",
            true,
            {
                preserveWhitespace: "full",
                from: 1,
                to: 2,
            },
            {
                errorOnInvalidContent: true,
            },
        ],
    }])
    assert.deepEqual(editor.chainCalls, [])
})

test("forwards insert_content invalid-content handling options to editor.commands.insertContent", () => {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()

    const result = command({
        id: "id",
        generation,
        command: {
            kind: "insert_content",
            content: {
                format: "html",
                value: "<p>updated</p>",
            },
            options: {
                error_on_invalid_content: true,
            },
        },
    })

    assert.equal(result.ok, true)
    assert.deepEqual(editor.commandCalls, [{
        name: "insertContentAt",
        args: [
            {from: 1, to: 1},
            "<p>updated</p>",
            {
                parseOptions: {},
                updateSelection: undefined,
                applyInputRules: undefined,
                applyPasteRules: undefined,
                errorOnInvalidContent: true,
            },
        ],
    }])
})

test("cleans up the registry on destroy and supports recreation with the same id", () => {
    const createdEditors = setupAdapterTest()

    create(createRequest("id", "<p>first</p>"), () => {
    }, () => {
    }, () => {
    }, () => {
    })
    const firstEditor = createdEditors[0]

    destroy("id")

    assert.equal(firstEditor?.destroyed, true)
    assert.equal(__testing.getEditorEntry("id"), undefined)
    assert.equal(__testing.getSlotCount(), 0)

    create(createRequest("id", "<p>second</p>"), () => {
    }, () => {
    }, () => {
    }, () => {
    })
    const secondEditor = createdEditors[1]

    assert.notEqual(secondEditor, firstEditor)
    assert.equal(__testing.getEditorEntry("id")?.editor, secondEditor)
})

test("rejects duplicate live editor ids without destroying the existing editor", () => {
    const createdEditors = setupAdapterTest()
    const errors: Array<{ kind: string; message: string }> = []

    create(createRequest("id", "<p>first</p>"), () => {
    }, () => {
    }, () => {
    }, (error) => errors.push(error))
    const firstEditor = createdEditors[0]

    withSuppressedConsoleError(() => {
        create(createRequest("id", "<p>second</p>"), () => {
            throw new Error("duplicate create should not succeed")
        }, () => {
        }, () => {
        }, (error) => errors.push(error))
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "duplicate_editor_id")
    assert.equal(firstEditor?.destroyed, false)
    assert.equal(createdEditors.length, 1)
    assert.equal(__testing.getEditorEntry("id")?.editor, firstEditor as unknown as Editor)
})

test("does not allocate registry slots when destroying unknown ids", () => {
    setupAdapterTest()

    destroy("missing")

    assert.equal(__testing.getSlotCount(), 0)
})

test("does not retain tombstone slots across create destroy cycles with unique ids", () => {
    const createdEditors = setupAdapterTest({
        elementsById: Object.fromEntries(
            Array.from({length: 5}, (_value, index) => [`editor-${index}`, {} as HTMLElement]),
        ),
    })

    for (let index = 0; index < 5; index += 1) {
        const id = `editor-${index}`
        create(createRequest(id, `<p>${index}</p>`), () => {
        }, () => {
        }, () => {
        }, () => {
        })

        assert.equal(__testing.getSlotCount(), 1)
        destroy(id)
        assert.equal(__testing.getSlotCount(), 0)
    }

    assert.equal(createdEditors.length, 5)
    assert.equal(createdEditors.every((editor) => editor.destroyed), true)
})

test("surfaces document read exceptions as operation_failed errors", () => {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    editor.getHtmlError = new Error("boom")
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()

    const result = withSuppressedConsoleError(() =>
        document({
            id: "id",
            generation,
            request: {
                kind: "get_content",
                format: "html",
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("document get_content should fail")
    }

    assert.equal(result.error.kind, "operation_failed")
    assert.equal(result.error.operation, "get_content_html")
    assert.match(result.error.message, /boom/)
})

test("returns json document content in a content payload", () => {
    setupAdapterTest()

    const generation = createAndGetGeneration()

    const result = document({
        id: "id",
        generation,
        request: {
            kind: "get_content",
            format: "json",
        },
    })

    assert.equal(result.ok, true)
    if (!result.ok) {
        throw new Error("document get_content should succeed")
    }

    assert.deepEqual(result.value, {
        kind: "content",
        content: {
            format: "json",
            value: {type: "doc", content: []},
        },
    })
})

test("surfaces rejected commands as command_rejected errors", () => {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    editor.chainRunResult = false
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()

    const result = withSuppressedConsoleError(() =>
        command({
            id: "id",
            generation,
            command: {
                kind: "toggle_bold",
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("toggle_bold should fail")
    }

    assert.equal(result.error.kind, "command_rejected")
    assert.equal(result.error.operation, "toggle_bold")
})

test("surfaces rejected core commands as command_rejected errors", () => {
    const editor = new FakeEditor({content: "<p>hello</p>"})
    editor.commandResult = false
    setupAdapterTest({
        makeEditor: () => editor,
    })

    const generation = createAndGetGeneration()

    const result = withSuppressedConsoleError(() =>
        command({
            id: "id",
            generation,
            command: {
                kind: "toggle_mark",
                type_or_name: "bold",
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("toggle_mark should fail")
    }

    assert.equal(result.error.kind, "command_rejected")
    assert.equal(result.error.operation, "toggle_mark")
})

test("updates editability without emitting an extra selection state", () => {
    const createdEditors = setupAdapterTest()

    let generation: number | undefined
    let selectionCount = 0

    create(
        createRequest(),
        (payload) => {
            generation = payload.generation
        },
        () => {
        },
        () => {
            selectionCount += 1
        },
        () => {
        },
    )

    const result = command({
        id: "id",
        generation: generation ?? 0,
        command: {
            kind: "set_editable",
            editable: false,
        },
    })

    assert.deepEqual(result, {ok: true, value: {kind: "empty"}})
    assert.equal(createdEditors[0]?.editable, false)
    assert.equal(selectionCount, 1)
})

test("does not emit selection state for successful commands unless Tiptap reports a selection change", () => {
    const createdEditors = setupAdapterTest()

    let generation: number | undefined
    let selectionCount = 0

    create(
        createRequest(),
        (payload) => {
            generation = payload.generation
        },
        () => {
        },
        () => {
            selectionCount += 1
        },
        () => {
        },
    )

    const commandResult = command({
        id: "id",
        generation: generation ?? 0,
        command: {
            kind: "set_text_selection",
            position: 9,
        },
    })

    assert.equal(commandResult.ok, true)
    assert.equal(selectionCount, 1)

    createdEditors[0]?.emitSelectionUpdate()
    assert.equal(selectionCount, 1)
})

test("does not emit selection state for set_content unless Tiptap reports a selection change", () => {
    const createdEditors = setupAdapterTest()

    let generation: number | undefined
    let selectionCount = 0

    create(
        createRequest(),
        (payload) => {
            generation = payload.generation
        },
        () => {
        },
        () => {
            selectionCount += 1
        },
        () => {
        },
    )

    const result = document({
        id: "id",
        generation: generation ?? 0,
        request: {
            kind: "set_content",
            content: {
                format: "html",
                value: "<p>updated</p>",
            },
        },
    })

    assert.equal(result.ok, true)
    assert.equal(selectionCount, 1)

    createdEditors[0]?.emitSelectionUpdate()
    assert.equal(selectionCount, 1)
})

test("emits initial selection state after ready", () => {
    setupAdapterTest()

    const callbackOrder: string[] = []
    let ready: { generation: number } | undefined

    create(
        createRequest(),
        (payload) => {
            callbackOrder.push("ready")
            ready = payload
        },
        () => {
        },
        () => {
            callbackOrder.push("selection")
        },
        () => {
        },
    )

    assert.deepEqual(callbackOrder, ["ready", "selection"])
    assert.equal(ready?.generation, 1)
})

test("emits selection state when a transaction changes active mark state", () => {
    const createdEditors = setupAdapterTest()

    let latestSelection = false
    let selectionCount = 0

    create(
        createRequest(),
        () => {
        },
        () => {
        },
        (selectionState) => {
            latestSelection = selectionState.bold ?? false
            selectionCount += 1
        },
        () => {
        },
    )

    const editor = createdEditors[0]
    assert.equal(selectionCount, 1)
    assert.equal(latestSelection, false)

    if (editor == null) {
        throw new Error("editor should have been created")
    }

    editor.activeStates.bold = true
    editor.emitTransaction()

    assert.equal(selectionCount, 2)
    assert.equal(latestSelection, true)

    editor.emitTransaction()

    assert.equal(selectionCount, 2)
})

test("treats missing selection keys as false when comparing sparse states", () => {
    const createdEditors = setupAdapterTest()
    let contributedSelection: SelectionState = {}

    __testing.registerExtension({
        name: "conditional_selection",
        create: () => ({name: "conditional_selection"} as never),
        selection_keys: ["bold"],
        selection_state: () => contributedSelection,
    })

    let latestSelection: SelectionState | undefined
    let selectionCount = 0

    create(
        {
            ...createRequest(),
            extensions: ["conditional_selection"],
        },
        () => {
        },
        () => {
        },
        (selectionState) => {
            latestSelection = selectionState
            selectionCount += 1
        },
        () => {
        },
    )

    const editor = createdEditors[0]
    if (editor == null) {
        throw new Error("editor should have been created")
    }

    assert.equal(selectionCount, 1)
    assert.deepEqual(latestSelection, {})

    contributedSelection = {bold: false}
    editor.emitTransaction()
    assert.equal(selectionCount, 1)

    contributedSelection = {bold: true}
    editor.emitTransaction()
    assert.equal(selectionCount, 2)
    assert.deepEqual(latestSelection, {bold: true})

    contributedSelection = {}
    editor.emitTransaction()
    assert.equal(selectionCount, 3)
    assert.deepEqual(latestSelection, {})

    contributedSelection = {bold: false}
    editor.emitTransaction()
    assert.equal(selectionCount, 3)
})

test("rejects stale generations for document and command calls", () => {
    setupAdapterTest()

    const firstGeneration = createAndGetGeneration(createRequest("id", "<p>first</p>"))
    destroy("id")
    const secondGeneration = createAndGetGeneration(createRequest("id", "<p>second</p>"))

    assert.notEqual(firstGeneration, secondGeneration)
    assert.equal(__testing.getSlotCount(), 1)

    const staleDocumentCall = withCapturedConsoleError(() =>
        document({
            id: "id",
            generation: firstGeneration,
            request: {
                kind: "get_content",
                format: "html",
            },
        }),
    )
    const staleCommandCall = withCapturedConsoleError(() =>
        command({
            id: "id",
            generation: firstGeneration,
            command: {
                kind: "toggle_bold",
            },
        }),
    )
    const staleDocumentResult = staleDocumentCall.result
    const staleCommandResult = staleCommandCall.result

    assert.equal(staleDocumentResult.ok, false)
    if (staleDocumentResult.ok) {
        throw new Error("stale document request should fail")
    }
    assert.equal(staleDocumentResult.error.kind, "editor_unavailable")
    assert.equal(staleDocumentCall.callCount, 0)

    assert.equal(staleCommandResult.ok, false)
    if (staleCommandResult.ok) {
        throw new Error("stale command request should fail")
    }
    assert.equal(staleCommandResult.error.kind, "editor_unavailable")
    assert.equal(staleCommandCall.callCount, 0)
})

test("rejects duplicate extension registration", () => {
    __testing.reset()

    __testing.registerExtension({
        name: "duplicate",
        create: () => {
            throw new Error("should not be called")
        },
    })

    assert.throws(() => __testing.registerExtension({
        name: "duplicate",
        create: () => {
            throw new Error("should not be called")
        },
    }))
})

test("reports missing requested extensions during create", () => {
    __testing.reset()
    __testing.setDocument(createFakeDocument({id: {} as HTMLElement}))

    const errors: Array<{ kind: string; message: string }> = []

    withSuppressedConsoleError(() => {
        create(
            {
                ...createRequest(),
                extensions: ["missing"],
            },
            () => {
                throw new Error("create should not succeed")
            },
            () => {
            },
            () => {
            },
            (error) => errors.push(error),
        )
    })

    assert.equal(errors.length, 1)
    assert.equal(errors[0]?.kind, "extension_unavailable")
    assert.match(errors[0]?.message ?? "", /not registered/)
})

test("reports commands from inactive extensions as extension_unavailable", () => {
    setupAdapterTest()

    const generation = createAndGetGeneration({
        ...createRequest(),
        extensions: ["document", "paragraph", "text"],
    })

    const result = withSuppressedConsoleError(() =>
        command({
            id: "id",
            generation,
            command: {
                kind: "toggle_bold",
            },
        }),
    )

    assert.equal(result.ok, false)
    if (result.ok) {
        throw new Error("toggle_bold should fail when bold is inactive")
    }

    assert.equal(result.error.kind, "extension_unavailable")
    assert.equal(result.error.operation, "toggle_bold")
})
