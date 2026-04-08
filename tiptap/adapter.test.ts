import assert from "node:assert/strict"
import test from "node:test"
import type {Editor, EditorOptions} from "@tiptap/core"

import {__testing, command, create, destroy, document} from "./adapter.ts"

class FakeEditor {
    destroyed = false
    editable: boolean | undefined
    content: unknown
    getHtmlError: Error | undefined
    getJsonError: Error | undefined
    setContentError: Error | undefined
    setContentResult = true
    chainRunError: Error | undefined
    chainRunResult = true
    chainCalls: Array<{ name: string; args: unknown[] }> = []

    constructor(readonly options: Pick<Partial<EditorOptions>, "content">) {
        this.content = options.content
    }

    commands = {
        setContent: (nextContent: unknown) => {
            if (this.setContentError != null) {
                throw this.setContentError
            }

            this.content = nextContent
            return this.setContentResult
        },
    }

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

    isActive(): boolean {
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
) {
    return {
        id,
        content: {
            format,
            value,
        },
        editable: true,
    } as const
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

function setupAdapterTest(
    options: {
        elementsById?: Record<string, HTMLElement | null>
        makeEditor?: (options: Pick<Partial<EditorOptions>, "content">) => FakeEditor
    } = {},
): FakeEditor[] {
    __testing.reset()
    __testing.setDocument(createFakeDocument(options.elementsById ?? {id: {} as HTMLElement}))

    const createdEditors: FakeEditor[] = []
    __testing.setEditorFactory((editorOptions) => {
        const editor = options.makeEditor?.({content: editorOptions.content}) ??
            new FakeEditor({content: editorOptions.content})
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

    create(createRequest("id", "<p>second</p>"), () => {
    }, () => {
    }, () => {
    }, () => {
    })
    const secondEditor = createdEditors[1]

    assert.notEqual(secondEditor, firstEditor)
    assert.equal(__testing.getEditorEntry("id")?.editor, secondEditor)
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
            value: JSON.stringify({type: "doc", content: []}),
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

    assert.equal(result.ok, true)
    assert.equal(createdEditors[0]?.editable, false)
    assert.equal(selectionCount, 1)
})

test("returns a generation and emits an initial selection state before ready", () => {
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

    assert.deepEqual(callbackOrder, ["selection", "ready"])
    assert.equal(ready?.generation, 1)
})

test("rejects stale generations for document and command calls", () => {
    setupAdapterTest()

    const firstGeneration = createAndGetGeneration(createRequest("id", "<p>first</p>"))
    destroy("id")
    const secondGeneration = createAndGetGeneration(createRequest("id", "<p>second</p>"))

    assert.notEqual(firstGeneration, secondGeneration)

    const staleDocumentResult = withSuppressedConsoleError(() =>
        document({
            id: "id",
            generation: firstGeneration,
            request: {
                kind: "get_content",
                format: "html",
            },
        }),
    )
    const staleCommandResult = withSuppressedConsoleError(() =>
        command({
            id: "id",
            generation: firstGeneration,
            command: {
                kind: "toggle_bold",
            },
        }),
    )

    assert.equal(staleDocumentResult.ok, false)
    if (staleDocumentResult.ok) {
        throw new Error("stale document request should fail")
    }
    assert.equal(staleDocumentResult.error.kind, "editor_unavailable")

    assert.equal(staleCommandResult.ok, false)
    if (staleCommandResult.ok) {
        throw new Error("stale command request should fail")
    }
    assert.equal(staleCommandResult.error.kind, "editor_unavailable")
})
