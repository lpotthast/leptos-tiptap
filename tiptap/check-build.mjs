import {spawnSync} from "node:child_process"
import {promises as fs} from "node:fs"
import path from "node:path"

const generatedDir = path.resolve("../src/js/generated")
const hostedModulesPath = path.resolve("./src/generated/hosted_modules.ts")
const bridgeApiPath = path.resolve("./src/bridge_api.ts")
const extensionsDir = path.resolve("./src/extensions")
const rustExtensionsPath = path.resolve("../src/api/extensions.rs")
const rustFfiPath = path.resolve("../src/runtime/ffi.rs")
const rustProtocolPath = path.resolve("../src/protocol/mod.rs")
const rustRegistrationPath = path.resolve("../src/runtime/registration.rs")
const rustSelectionPath = path.resolve("../src/api/types/selection.rs")

async function snapshotDirectory(directory, root = directory) {
    const entries = await fs.readdir(directory, {withFileTypes: true})
    const snapshot = new Map()

    for (const entry of entries) {
        const entryPath = path.join(directory, entry.name)
        if (entry.isDirectory()) {
            const nested = await snapshotDirectory(entryPath, root)
            for (const [relativePath, contents] of nested) {
                snapshot.set(relativePath, contents)
            }
            continue
        }

        snapshot.set(path.relative(root, entryPath), await fs.readFile(entryPath, "utf8"))
    }

    return snapshot
}

function diffSnapshots(before, after, label) {
    const changedPaths = new Set([...before.keys(), ...after.keys()])
    const diffs = []

    for (const changedPath of changedPaths) {
        if (before.get(changedPath) !== after.get(changedPath)) {
            diffs.push(`${label}/${changedPath}`)
        }
    }

    return diffs
}

function uniqueSorted(values) {
    return [...new Set(values)].sort()
}

function extractBetween(contents, startMarker, endMarker, label) {
    const start = contents.indexOf(startMarker)
    if (start === -1) {
        throw new Error(`Could not find ${label} start marker: ${startMarker}`)
    }

    const end = contents.indexOf(endMarker, start + startMarker.length)
    if (end === -1) {
        throw new Error(`Could not find ${label} end marker: ${endMarker}`)
    }

    return contents.slice(start, end)
}

function matches(contents, expression) {
    return [...contents.matchAll(expression)].map((match) => match[1])
}

function toSnakeCase(value) {
    return value.replace(/[A-Z]/g, (character, index) =>
        `${index === 0 ? "" : "_"}${character.toLowerCase()}`)
}

function assertSameSet(label, leftName, leftValues, rightName, rightValues) {
    const left = uniqueSorted(leftValues)
    const right = uniqueSorted(rightValues)
    const leftSet = new Set(left)
    const rightSet = new Set(right)
    const missingFromLeft = right.filter((value) => !leftSet.has(value))
    const missingFromRight = left.filter((value) => !rightSet.has(value))

    if (missingFromLeft.length === 0 && missingFromRight.length === 0) {
        return
    }

    const lines = [`${label} drift detected.`]
    if (missingFromLeft.length > 0) {
        lines.push(`Missing from ${leftName}: ${missingFromLeft.join(", ")}`)
    }
    if (missingFromRight.length > 0) {
        lines.push(`Missing from ${rightName}: ${missingFromRight.join(", ")}`)
    }

    throw new Error(lines.join("\n"))
}

async function discoverExtensionNames() {
    const files = await fs.readdir(extensionsDir)
    return files
        .filter((name) => name.startsWith("tiptap_") && name.endsWith(".ts"))
        .map((name) => name.slice("tiptap_".length, -".ts".length))
}

async function validateBridgeDrift() {
    const [
        bridgeApi,
        protocol,
        rustExtensions,
        rustFfi,
        rustRegistration,
        rustSelection,
    ] = await Promise.all([
        fs.readFile(bridgeApiPath, "utf8"),
        fs.readFile(rustProtocolPath, "utf8"),
        fs.readFile(rustExtensionsPath, "utf8"),
        fs.readFile(rustFfiPath, "utf8"),
        fs.readFile(rustRegistrationPath, "utf8"),
        fs.readFile(rustSelectionPath, "utf8"),
    ])

    const tsCommandSection = extractBetween(
        bridgeApi,
        "export type CoreCommand =",
        "export type DocumentRequest =",
        "TypeScript command union",
    )
    const rustCommandSection = extractBetween(
        protocol,
        "pub(crate) fn operation_name(&self)",
        "pub(crate) enum DocumentRequest",
        "Rust command operation names",
    )
    assertSameSet(
        "Editor command wire contract",
        "tiptap/src/bridge_api.ts",
        matches(tsCommandSection, /kind:\s*"([^"]+)"/g),
        "src/protocol/mod.rs",
        matches(rustCommandSection, /Some\("([^"]+)"\)/g),
    )

    const tsDocumentSection = extractBetween(
        bridgeApi,
        "export type DocumentRequest =",
        "export type DocumentResponse =",
        "TypeScript document request union",
    )
    const rustDocumentSection = extractBetween(
        protocol,
        "pub(crate) enum DocumentRequest",
        "impl DocumentRequest",
        "Rust document request enum",
    )
    assertSameSet(
        "Document request wire contract",
        "tiptap/src/bridge_api.ts",
        matches(tsDocumentSection, /kind:\s*"([^"]+)"/g),
        "src/protocol/mod.rs",
        matches(rustDocumentSection, /^\s+([A-Z][A-Za-z0-9]*)\s*\{/gm).map(toSnakeCase),
    )

    const tsSelectionSection = extractBetween(
        bridgeApi,
        "export type SelectionKey =",
        "export type SelectionState =",
        "TypeScript selection keys",
    )
    assertSameSet(
        "Selection key wire contract",
        "tiptap/src/bridge_api.ts",
        matches(tsSelectionSection, /\|\s*"([^"]+)"/g),
        "src/api/types/selection.rs",
        matches(rustSelection, /\bpub\s+([a-z0-9_]+):\s+bool\b/g),
    )

    const rustExtensionNameSection = extractBetween(
        rustExtensions,
        "pub fn name(self) -> &'static str",
        "pub fn all_enabled() -> Vec<Self>",
        "Rust extension names",
    )
    const extensionNames = await discoverExtensionNames()
    const ffiExtensionNames = matches(rustFfi, /\/src\/js\/generated\/tiptap_([a-z0-9_]+)\.js/g)
    const registeredExtensionNames = matches(
        rustRegistration,
        /register_extension\("([^"]+)",\s*ffi::register_[a-z0-9_]+\)/g,
    )

    assertSameSet(
        "Extension entrypoint contract",
        "tiptap/src/extensions",
        extensionNames,
        "src/api/extensions.rs",
        matches(rustExtensionNameSection, /Self::[A-Za-z0-9_]+\s*=>\s*"([^"]+)"/g),
    )
    assertSameSet(
        "Extension FFI contract",
        "src/runtime/ffi.rs",
        ffiExtensionNames,
        "tiptap/src/extensions",
        extensionNames,
    )
    assertSameSet(
        "Extension registration contract",
        "src/runtime/registration.rs",
        registeredExtensionNames,
        "tiptap/src/extensions",
        extensionNames,
    )
}

async function main() {
    await validateBridgeDrift()

    const generatedBefore = await snapshotDirectory(generatedDir)
    const hostedModulesBefore = await fs.readFile(hostedModulesPath, "utf8")

    const result = spawnSync(process.execPath, ["build.mjs"], {
        stdio: "inherit",
    })
    if (result.status !== 0) {
        process.exit(result.status ?? 1)
    }

    const generatedAfter = await snapshotDirectory(generatedDir)
    const hostedModulesAfter = await fs.readFile(hostedModulesPath, "utf8")
    const changedFiles = [
        ...diffSnapshots(generatedBefore, generatedAfter, "generated"),
    ]

    if (hostedModulesBefore !== hostedModulesAfter) {
        changedFiles.push("src/generated/hosted_modules.ts")
    }

    if (changedFiles.length > 0) {
        console.error("Generated Tiptap artifacts changed during build:")
        for (const changedFile of changedFiles.sort()) {
            console.error(`- ${changedFile}`)
        }
        process.exit(1)
    }
}

await main()
