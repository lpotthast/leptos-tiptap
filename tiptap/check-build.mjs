import {spawnSync} from "node:child_process"
import {promises as fs} from "node:fs"
import path from "node:path"

const generatedDir = path.resolve("../src/js/generated")
const hostedModulesPath = path.resolve("./src/generated/hosted_modules.ts")
const thirdPartyNoticesPath = path.resolve("../THIRD_PARTY_NOTICES")
const bridgeApiPath = path.resolve("./src/bridge_api.ts")
const extensionsDir = path.resolve("./src/extensions")
const rustExtensionsPath = path.resolve("../src/api/extensions.rs")
const rustFfiPath = path.resolve("../src/runtime/ffi.rs")
const rustProtocolPath = path.resolve("../src/protocol/mod.rs")
const rustRegistrationPath = path.resolve("../src/runtime/registration.rs")
const rustSelectionPath = path.resolve("../src/api/types/selection.rs")

/**
 * @param {string} directory
 * @param {string} root
 * @returns {Promise<Map<string, string>>}
 */
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

/**
 * @param {Map<string, string>} before
 * @param {Map<string, string>} after
 * @param {string} label
 */
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

/** @param {string[]} values */
function uniqueSorted(values) {
    return [...new Set(values)].sort()
}

/**
 * @param {string} contents
 * @param {string} startMarker
 * @param {string} endMarker
 * @param {string} label
 */
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

/**
 * @param {string} contents
 * @param {RegExp} expression
 */
function matches(contents, expression) {
    return [...contents.matchAll(expression)].flatMap((match) =>
        match[1] == null ? [] : [match[1]])
}

/** @param {string} value */
function toSnakeCase(value) {
    return value.replace(/[A-Z]/g, (character, index) =>
        `${index === 0 ? "" : "_"}${character.toLowerCase()}`)
}

/**
 * @param {string} label
 * @param {string} leftName
 * @param {string[]} leftValues
 * @param {string} rightName
 * @param {string[]} rightValues
 */
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

// Extracts the brace-delimited body that follows a `pub(crate) enum X {` declaration
// in a Rust source string, walking braces to handle nested ones.
/**
 * @param {string} contents
 * @param {string} declaration
 */
function extractRustEnumBody(contents, declaration) {
    const start = contents.indexOf(declaration)
    if (start === -1) {
        throw new Error(`Could not find Rust declaration: ${declaration}`)
    }

    const bodyStart = contents.indexOf("{", start) + 1
    let depth = 1
    let i = bodyStart
    while (i < contents.length && depth > 0) {
        const character = contents[i]
        if (character === "{") {
            depth += 1
        } else if (character === "}") {
            depth -= 1
            if (depth === 0) {
                break
            }
        }
        i += 1
    }

    if (depth !== 0) {
        throw new Error(`Unterminated Rust enum body for: ${declaration}`)
    }

    return contents.slice(bodyStart, i)
}

// Parses a Rust enum body into a Map<snake_case_name, sorted_field_names[]>.
// Unit variants map to an empty array; struct variants map to their named fields.
/**
 * @param {string} body
 * @returns {Map<string, string[]>}
 */
function parseRustVariantFields(body) {
    const variants = new Map()
    const variantRegex = /(?:#\[[^\]]+\]\s*)*([A-Z][A-Za-z0-9]*)\s*(?:\{([^}]*)\}|,)/g
    const fieldRegex = /([a-z_][a-z0-9_]*)\s*:(?!:)/g

    let variantMatch
    while ((variantMatch = variantRegex.exec(body)) !== null) {
        const variantName = variantMatch[1]
        const fieldsBlock = variantMatch[2]
        if (variantName == null) {
            throw new Error("Rust variant parser matched without a variant name")
        }
        const fields = []

        if (fieldsBlock != null) {
            let fieldMatch
            while ((fieldMatch = fieldRegex.exec(fieldsBlock)) !== null) {
                const field = fieldMatch[1]
                if (field != null) {
                    fields.push(field)
                }
            }
            fieldRegex.lastIndex = 0
        }

        variants.set(toSnakeCase(variantName), uniqueSorted(fields))
    }

    return variants
}

// Parses a TypeScript discriminated-union section into a Map<kind, sorted_field_names[]>.
// Each variant is expected to be written as `| { kind: "x"; field?: T; ... }`.
// `kind` itself is filtered out of the field set since it is the discriminator.
/**
 * @param {string} section
 * @returns {Map<string, string[]>}
 */
function parseTsVariantFields(section) {
    const variants = new Map()
    const variantRegex = /\|\s*\{\s*kind:\s*"([^"]+)"([^}]*)\}/g
    const fieldRegex = /([a-z_][a-z0-9_]*)\s*\??\s*:/g

    let variantMatch
    while ((variantMatch = variantRegex.exec(section)) !== null) {
        const kind = variantMatch[1]
        const remainder = variantMatch[2]
        if (kind == null || remainder == null) {
            throw new Error("TypeScript variant parser matched without a kind or body")
        }
        const fields = []

        let fieldMatch
        while ((fieldMatch = fieldRegex.exec(remainder)) !== null) {
            const name = fieldMatch[1]
            if (name != null && name !== "kind") {
                fields.push(name)
            }
        }
        fieldRegex.lastIndex = 0

        variants.set(kind, uniqueSorted(fields))
    }

    return variants
}

// Asserts that, for every kind shared by both maps, the field sets match exactly.
// Kinds that exist only on one side are intentionally ignored here: the kind-set
// drift check above already covers that case.
/**
 * @param {string} label
 * @param {string} leftName
 * @param {Map<string, string[]>} leftMap
 * @param {string} rightName
 * @param {Map<string, string[]>} rightMap
 */
function assertSameVariantFields(label, leftName, leftMap, rightName, rightMap) {
    const sharedKinds = [...leftMap.keys()].filter((kind) => rightMap.has(kind)).sort()
    const drifts = []

    for (const kind of sharedKinds) {
        const left = leftMap.get(kind)
        const right = rightMap.get(kind)
        if (left == null || right == null) {
            throw new Error(`Shared variant ${kind} disappeared during comparison`)
        }
        const leftSet = new Set(left)
        const rightSet = new Set(right)
        const missingFromLeft = right.filter((value) => !leftSet.has(value))
        const missingFromRight = left.filter((value) => !rightSet.has(value))

        if (missingFromLeft.length === 0 && missingFromRight.length === 0) {
            continue
        }

        const lines = [`  ${kind}:`]
        if (missingFromLeft.length > 0) {
            lines.push(`    missing from ${leftName}: ${missingFromLeft.join(", ")}`)
        }
        if (missingFromRight.length > 0) {
            lines.push(`    missing from ${rightName}: ${missingFromRight.join(", ")}`)
        }
        drifts.push(lines.join("\n"))
    }

    if (drifts.length === 0) {
        return
    }

    throw new Error(`${label} field drift detected.\n${drifts.join("\n")}`)
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

    const tsCommandFields = parseTsVariantFields(tsCommandSection)
    const rustCommandFields = parseRustVariantFields(
        extractRustEnumBody(protocol, "pub(crate) enum EditorCommand {"),
    )
    assertSameVariantFields(
        "Editor command payload contract",
        "tiptap/src/bridge_api.ts",
        tsCommandFields,
        "src/protocol/mod.rs",
        rustCommandFields,
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

    const tsDocumentFields = parseTsVariantFields(tsDocumentSection)
    const rustDocumentFields = parseRustVariantFields(
        extractRustEnumBody(protocol, "pub(crate) enum DocumentRequest {"),
    )
    assertSameVariantFields(
        "Document request payload contract",
        "tiptap/src/bridge_api.ts",
        tsDocumentFields,
        "src/protocol/mod.rs",
        rustDocumentFields,
    )

    const tsActiveKeySection = extractBetween(
        bridgeApi,
        "export type ActiveKey =",
        "export type ActiveState =",
        "TypeScript active keys",
    )
    const rustActiveKeys = [...parseRustVariantFields(
        extractRustEnumBody(rustSelection, "pub enum TiptapActiveKey {"),
    ).keys()]
    assertSameSet(
        "Active key wire contract",
        "tiptap/src/bridge_api.ts",
        matches(tsActiveKeySection, /\|\s*"([^"]+)"/g),
        "src/api/types/selection.rs",
        rustActiveKeys,
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
    const thirdPartyNoticesBefore = await fs.readFile(thirdPartyNoticesPath, "utf8")

    const result = spawnSync(process.execPath, ["build.mjs"], {
        stdio: "inherit",
    })
    if (result.status !== 0) {
        process.exit(result.status ?? 1)
    }

    const generatedAfter = await snapshotDirectory(generatedDir)
    const hostedModulesAfter = await fs.readFile(hostedModulesPath, "utf8")
    const thirdPartyNoticesAfter = await fs.readFile(thirdPartyNoticesPath, "utf8")
    const changedFiles = [
        ...diffSnapshots(generatedBefore, generatedAfter, "generated"),
    ]

    if (hostedModulesBefore !== hostedModulesAfter) {
        changedFiles.push("src/generated/hosted_modules.ts")
    }
    if (thirdPartyNoticesBefore !== thirdPartyNoticesAfter) {
        changedFiles.push("../THIRD_PARTY_NOTICES")
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
