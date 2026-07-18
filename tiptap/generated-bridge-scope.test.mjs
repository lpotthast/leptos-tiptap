// Loads two copies of the real generated artifacts to verify their bridge state stays isolated.
// This protects pages that host multiple independently bundled leptos-tiptap applications.

import assert from "node:assert/strict"
import {promises as fs} from "node:fs"
import os from "node:os"
import path from "node:path"
import test from "node:test"
import {fileURLToPath, pathToFileURL} from "node:url"

import {BRIDGE_GLOBAL_KEY} from "./bridge-config.mjs"

const moduleDirectory = path.dirname(fileURLToPath(import.meta.url))
const generatedDirectory = path.resolve(moduleDirectory, "../src/js/generated")
const artifactNames = ["bridge_runtime.js", "tiptap_document.js"]

/** @param {string} destination */
async function copyArtifactSet(destination) {
    await fs.mkdir(destination, {recursive: true})
    await Promise.all(artifactNames.map((artifactName) =>
        fs.copyFile(
            path.join(generatedDirectory, artifactName),
            path.join(destination, artifactName),
        )))
}

test("isolates real generated bridge artifacts by their module directory", async (context) => {
    const temporaryRoot = await fs.mkdtemp(path.join(os.tmpdir(), "leptos-tiptap-bridge-scopes-"))
    context.after(() => fs.rm(temporaryRoot, {recursive: true, force: true}))

    const firstDirectory = path.join(temporaryRoot, "first")
    const secondDirectory = path.join(temporaryRoot, "second")
    await Promise.all([
        copyArtifactSet(firstDirectory),
        copyArtifactSet(secondDirectory),
    ])

    const [firstRealDirectory, secondRealDirectory] = await Promise.all([
        fs.realpath(firstDirectory),
        fs.realpath(secondDirectory),
    ])
    const firstRuntimeUrl = pathToFileURL(path.join(firstRealDirectory, "bridge_runtime.js")).href
    const firstExtensionUrl = pathToFileURL(path.join(firstRealDirectory, "tiptap_document.js")).href
    const secondRuntimeUrl = pathToFileURL(path.join(secondRealDirectory, "bridge_runtime.js")).href
    const secondExtensionUrl = pathToFileURL(path.join(secondRealDirectory, "tiptap_document.js")).href

    const firstRuntime = await import(firstRuntimeUrl)
    const firstExtension = await import(firstExtensionUrl)
    firstRuntime.init_bridge_runtime()
    firstExtension.register_document()

    const secondRuntime = await import(secondRuntimeUrl)
    secondRuntime.init_bridge_runtime()
    assert.deepEqual(secondRuntime.__testing.getRegisteredExtensionNames(), [])

    const secondExtension = await import(secondExtensionUrl)
    secondExtension.register_document()

    const bridgeHost = Reflect.get(globalThis, BRIDGE_GLOBAL_KEY)
    assert.equal(typeof bridgeHost?.getBindings, "function")

    const firstScope = new URL(".", firstRuntimeUrl).href
    const secondScope = new URL(".", secondRuntimeUrl).href
    const firstBindings = bridgeHost.getBindings(firstScope)
    const secondBindings = bridgeHost.getBindings(secondScope)

    assert.notStrictEqual(firstBindings, secondBindings)
    assert.notStrictEqual(
        firstBindings?.modules["@tiptap/core"],
        secondBindings?.modules["@tiptap/core"],
    )
    assert.deepEqual(firstRuntime.__testing.getRegisteredExtensionNames(), ["document"])
    assert.deepEqual(secondRuntime.__testing.getRegisteredExtensionNames(), ["document"])
})
