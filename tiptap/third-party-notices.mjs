// Derives third-party notices from the npm modules that esbuild actually includes in the bundles.
// This fail-closed inventory is needed because the generated JavaScript ships inside the Rust crate.

import {createHash} from "node:crypto"
import {promises as fs} from "node:fs"
import path from "node:path"
import {fileURLToPath} from "node:url"

const moduleDirectory = path.dirname(fileURLToPath(import.meta.url))
const defaultOutputPath = path.resolve(moduleDirectory, "../THIRD_PARTY_NOTICES")
const defaultTiptapLicensePath = path.resolve(moduleDirectory, "licenses/tiptap-MIT.txt")
const defaultTiptapLicenseMetadataPath = path.resolve(moduleDirectory, "licenses/tiptap-MIT.json")
const licenseFilePattern = /^(?:licen[cs]e|copying|notice)(?:$|[._-])/i
const sectionSeparator = "=".repeat(79)

/**
 * @typedef {{source: string, contents: string}} LicenseDocument
 * @typedef {{key: string, license: string, documents: LicenseDocument[]}} PackageEntry
 * @typedef {{key: string, documents: LicenseDocument[], packages: PackageEntry[]}} LicenseGroup
 */

/** @param {string} inputPath */
function isNodeModulesPath(inputPath) {
    return inputPath.split(/[\\/]/).includes("node_modules")
}

/**
 * @param {unknown} error
 * @returns {error is NodeJS.ErrnoException}
 */
function isErrnoException(error) {
    return error instanceof Error && "code" in error
}

/**
 * @param {string} inputPath
 * @param {string} workingDirectory
 */
async function findPackageRoot(inputPath, workingDirectory) {
    let directory = path.dirname(path.resolve(workingDirectory, inputPath))

    while (path.basename(directory) !== "node_modules") {
        const packageJsonPath = path.join(directory, "package.json")
        try {
            const packageJson = JSON.parse(await fs.readFile(packageJsonPath, "utf8"))
            if (typeof packageJson.name === "string" && typeof packageJson.version === "string") {
                return directory
            }
        } catch (error) {
            if (!isErrnoException(error) || error.code !== "ENOENT") {
                throw error
            }
        }

        const parent = path.dirname(directory)
        if (parent === directory) {
            break
        }
        directory = parent
    }

    throw new Error(`Could not find the npm package root for bundled input ${inputPath}.`)
}

/** @param {unknown} license */
function describeLicense(license) {
    if (typeof license === "string" && license.length > 0) {
        return license
    }
    if (Array.isArray(license)) {
        const names = license
            .map((entry) => typeof entry === "string" ? entry : entry?.type)
            .filter((entry) => typeof entry === "string" && entry.length > 0)
        if (names.length > 0) {
            return names.join(" OR ")
        }
    }
    return "see included license text"
}

/** @param {string} contents */
function normalizeLicenseText(contents) {
    return `${contents.replace(/^\uFEFF/, "").replace(/\r\n/g, "\n").trimEnd()}\n`
}

/**
 * @param {string} filePath
 * @param {string} description
 */
async function readLicenseText(filePath, description) {
    const contents = normalizeLicenseText(await fs.readFile(filePath, "utf8"))
    if (contents.trim().length === 0) {
        throw new Error(`${description} is empty.`)
    }
    return contents
}

/**
 * @param {{name: string, version: string, license?: unknown}} packageJson
 * @param {string} tiptapLicensePath
 * @param {string} tiptapLicenseMetadataPath
 * @returns {Promise<LicenseDocument>}
 */
async function readTiptapLicenseDocument(
    packageJson,
    tiptapLicensePath,
    tiptapLicenseMetadataPath,
) {
    const metadata = JSON.parse(await fs.readFile(tiptapLicenseMetadataPath, "utf8"))
    if (
        metadata.package !== "@tiptap/core"
        || typeof metadata.version !== "string"
        || metadata.license !== "MIT"
        || metadata.source !== `https://github.com/ueberdosis/tiptap/blob/@tiptap/core@${metadata.version}/LICENSE.md`
        || !/^[a-f0-9]{64}$/.test(metadata.sha256)
    ) {
        throw new Error(`Invalid Tiptap license metadata in ${tiptapLicenseMetadataPath}.`)
    }
    if (packageJson.version !== metadata.version) {
        throw new Error(
            `Bundled npm package ${packageJson.name}@${packageJson.version} does not match `
            + `the vendored Tiptap license version ${metadata.version}.`,
        )
    }
    if (packageJson.license !== metadata.license) {
        throw new Error(
            `Bundled npm package ${packageJson.name}@${packageJson.version} declares `
            + `${String(packageJson.license)}, but the vendored Tiptap license metadata declares `
            + `${metadata.license}.`,
        )
    }

    const contents = await readLicenseText(tiptapLicensePath, "The vendored Tiptap license")
    const sha256 = createHash("sha256").update(contents).digest("hex")
    if (sha256 !== metadata.sha256) {
        throw new Error(
            `The vendored Tiptap license SHA-256 is ${sha256}, expected ${metadata.sha256}.`,
        )
    }

    return {
        source: "tiptap/licenses/tiptap-MIT.txt",
        contents,
    }
}

/**
 * @param {string} packageRoot
 * @param {{name: string, version: string, license?: unknown}} packageJson
 * @param {string} tiptapLicensePath
 * @param {string} tiptapLicenseMetadataPath
 * @returns {Promise<LicenseDocument[]>}
 */
async function readLicenseDocuments(
    packageRoot,
    packageJson,
    tiptapLicensePath,
    tiptapLicenseMetadataPath,
) {
    const entries = await fs.readdir(packageRoot, {withFileTypes: true})
    const licenseFiles = entries
        .filter((entry) => (entry.isFile() || entry.isSymbolicLink()) && licenseFilePattern.test(entry.name))
        .map((entry) => entry.name)
        .sort((left, right) => left.localeCompare(right))

    if (licenseFiles.length > 0) {
        return Promise.all(licenseFiles.map(async (name) => ({
            source: `package ${name}`,
            contents: await readLicenseText(
                path.join(packageRoot, name),
                `License document ${name} for ${packageJson.name}@${packageJson.version}`,
            ),
        })))
    }

    if (packageJson.name.startsWith("@tiptap/") && packageJson.license === "MIT") {
        return [await readTiptapLicenseDocument(
            packageJson,
            tiptapLicensePath,
            tiptapLicenseMetadataPath,
        )]
    }

    throw new Error(
        `Bundled npm package ${packageJson.name} has no license, licence, copying, or notice file.`,
    )
}

/**
 * @param {import("esbuild").Metafile[]} metafiles
 * @param {{workingDirectory?: string, tiptapLicensePath?: string, tiptapLicenseMetadataPath?: string}} options
 */
export async function createThirdPartyNotices(metafiles, options = {}) {
    const workingDirectory = options.workingDirectory ?? process.cwd()
    const tiptapLicensePath = options.tiptapLicensePath ?? defaultTiptapLicensePath
    const tiptapLicenseMetadataPath = options.tiptapLicenseMetadataPath
        ?? defaultTiptapLicenseMetadataPath
    const bundledInputs = new Set()

    for (const metafile of metafiles) {
        for (const output of Object.values(metafile.outputs)) {
            for (const [inputPath, contribution] of Object.entries(output.inputs)) {
                if (contribution.bytesInOutput > 0 && isNodeModulesPath(inputPath)) {
                    bundledInputs.add(inputPath)
                }
            }
        }
    }

    const packageRoots = new Set()
    for (const inputPath of [...bundledInputs].sort((left, right) => left.localeCompare(right))) {
        packageRoots.add(await findPackageRoot(inputPath, workingDirectory))
    }

    /** @type {Map<string, PackageEntry>} */
    const packages = new Map()
    for (const packageRoot of [...packageRoots].sort((left, right) => left.localeCompare(right))) {
        const packageJson = JSON.parse(await fs.readFile(path.join(packageRoot, "package.json"), "utf8"))
        if (typeof packageJson.name !== "string" || typeof packageJson.version !== "string") {
            throw new Error(`Bundled npm package at ${packageRoot} has no string name or version.`)
        }

        const documents = await readLicenseDocuments(
            packageRoot,
            packageJson,
            tiptapLicensePath,
            tiptapLicenseMetadataPath,
        )
        const key = `${packageJson.name}@${packageJson.version}`
        const existing = packages.get(key)
        const next = {
            key,
            license: describeLicense(packageJson.license ?? packageJson.licenses),
            documents,
        }

        if (existing != null && JSON.stringify(existing.documents) !== JSON.stringify(next.documents)) {
            throw new Error(`Bundled copies of ${key} contain conflicting license documents.`)
        }
        packages.set(key, next)
    }

    if (packages.size === 0) {
        throw new Error("The generated bundles did not contain any npm package inputs.")
    }

    /** @type {Map<string, LicenseGroup>} */
    const groups = new Map()
    for (const packageEntry of [...packages.values()].sort((left, right) => left.key.localeCompare(right.key))) {
        const fingerprint = JSON.stringify(packageEntry.documents.map((document) => document.contents))
        const group = groups.get(fingerprint) ?? {
            key: packageEntry.key,
            documents: packageEntry.documents,
            packages: [],
        }
        group.packages.push(packageEntry)
        groups.set(fingerprint, group)
    }

    const lines = [
        "THIRD-PARTY SOFTWARE NOTICES",
        "",
        "leptos-tiptap bundles the npm packages listed below into generated JavaScript",
        "that is shipped in this crate. This file is generated by tiptap/build.mjs",
        "from esbuild's contributed-input metadata. Do not edit it manually.",
        "",
    ]

    const sortedGroups = [...groups.values()].sort((left, right) => left.key.localeCompare(right.key))

    for (const group of sortedGroups) {
        lines.push(sectionSeparator, "Packages covered by the following license text:")
        for (const packageEntry of group.packages) {
            const sources = [...new Set(packageEntry.documents.map((document) => document.source))].join(", ")
            lines.push(`- ${packageEntry.key} (${packageEntry.license}; source: ${sources})`)
        }
        lines.push("")

        for (const [index, document] of group.documents.entries()) {
            if (group.documents.length > 1) {
                lines.push(`License document ${index + 1}:`, "")
            }
            lines.push(document.contents.trimEnd(), "")
        }
    }

    return `${lines.join("\n").trimEnd()}\n`
}

/**
 * @param {import("esbuild").Metafile[]} metafiles
 * @param {{workingDirectory?: string, tiptapLicensePath?: string, tiptapLicenseMetadataPath?: string, outputPath?: string}} options
 */
export async function writeThirdPartyNotices(metafiles, options = {}) {
    const notices = await createThirdPartyNotices(metafiles, options)
    await fs.writeFile(options.outputPath ?? defaultOutputPath, notices)
}
