import assert from "node:assert/strict"
import {createHash} from "node:crypto"
import {promises as fs} from "node:fs"
import os from "node:os"
import path from "node:path"
import test from "node:test"

import {createThirdPartyNotices} from "./third-party-notices.mjs"

/** @param {import("node:test").TestContext} context */
async function temporaryDirectory(context) {
    const root = await fs.mkdtemp(path.join(os.tmpdir(), "leptos-tiptap-notices-"))
    context.after(() => fs.rm(root, {recursive: true, force: true}))
    return root
}

/**
 * @param {Array<[string, number]>} inputs
 * @returns {import("esbuild").Metafile}
 */
function metafile(inputs) {
    return {
        inputs: {},
        outputs: {
            "generated.js": {
                bytes: 1,
                inputs: Object.fromEntries(inputs.map(([inputPath, bytesInOutput]) => [
                    inputPath,
                    {bytesInOutput},
                ])),
                exports: [],
                entryPoint: "src/example.ts",
                imports: [],
            },
        },
    }
}

/**
 * @param {string} root
 * @param {string} name
 * @param {string} version
 * @param {string} license
 * @param {string | undefined} licenseText
 */
async function writePackage(root, name, version, license, licenseText = undefined) {
    const packageRoot = path.join(root, "node_modules", ...name.split("/"))
    await fs.mkdir(packageRoot, {recursive: true})
    await fs.writeFile(
        path.join(packageRoot, "package.json"),
        `${JSON.stringify({name, version, license})}\n`,
    )
    await fs.writeFile(path.join(packageRoot, "index.js"), "export const value = 1\n")
    if (licenseText != null) {
        await fs.writeFile(path.join(packageRoot, "LICENSE"), licenseText)
    }
}

/**
 * @param {string} root
 * @param {string} version
 * @param {string} contents
 * @param {string | undefined} sha256
 */
async function writeTiptapLicense(root, version, contents, sha256 = undefined) {
    const tiptapLicensePath = path.join(root, "tiptap-MIT.txt")
    const tiptapLicenseMetadataPath = path.join(root, "tiptap-MIT.json")
    const normalizedContents = `${contents.replace(/\r\n/g, "\n").trimEnd()}\n`
    await fs.writeFile(tiptapLicensePath, contents)
    await fs.writeFile(tiptapLicenseMetadataPath, `${JSON.stringify({
        package: "@tiptap/core",
        version,
        license: "MIT",
        source: `https://github.com/ueberdosis/tiptap/blob/@tiptap/core@${version}/LICENSE.md`,
        sha256: sha256 ?? createHash("sha256").update(normalizedContents).digest("hex"),
    })}\n`)
    return {tiptapLicensePath, tiptapLicenseMetadataPath}
}

test("lists only packages that contribute code and groups identical licenses", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "included-a", "1.0.0", "MIT", "shared license\n")
    await writePackage(root, "included-b", "2.0.0", "MIT", "shared license\n")
    await writePackage(root, "tree-shaken", "3.0.0", "MIT", "unused license\n")

    const notices = await createThirdPartyNotices([
        metafile([
            ["node_modules/included-a/index.js", 10],
            ["node_modules/included-b/index.js", 5],
            ["node_modules/tree-shaken/index.js", 0],
        ]),
    ], {workingDirectory: root})

    assert.match(notices, /- included-a@1\.0\.0 \(MIT; source: package LICENSE\)/)
    assert.match(notices, /- included-b@2\.0\.0 \(MIT; source: package LICENSE\)/)
    assert.doesNotMatch(notices, /tree-shaken/)
    assert.equal(notices.match(/shared license/g)?.length, 1)
})

test("uses the versioned, checksummed upstream license for Tiptap packages", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "@tiptap/core", "2.27.2", "MIT")
    await fs.mkdir(path.join(root, "node_modules/@tiptap/core/subpath"))
    await fs.writeFile(
        path.join(root, "node_modules/@tiptap/core/subpath/package.json"),
        `${JSON.stringify({module: "./index.js"})}\n`,
    )
    await fs.writeFile(
        path.join(root, "node_modules/@tiptap/core/subpath/index.js"),
        "export const value = 1\n",
    )
    const licenseOptions = await writeTiptapLicense(root, "2.27.2", "upstream Tiptap license\n")

    const notices = await createThirdPartyNotices([
        metafile([["node_modules/@tiptap/core/subpath/index.js", 10]]),
    ], {workingDirectory: root, ...licenseOptions})

    assert.match(notices, /- @tiptap\/core@2\.27\.2 \(MIT; source: tiptap\/licenses\/tiptap-MIT\.txt\)/)
    assert.match(notices, /upstream Tiptap license/)
})

test("rejects a Tiptap package version not covered by the vendored license", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "@tiptap/core", "2.28.0", "MIT")
    const licenseOptions = await writeTiptapLicense(root, "2.27.2", "upstream Tiptap license\n")

    await assert.rejects(
        createThirdPartyNotices([
            metafile([["node_modules/@tiptap/core/index.js", 10]]),
        ], {workingDirectory: root, ...licenseOptions}),
        /@tiptap\/core@2\.28\.0 does not match the vendored Tiptap license version 2\.27\.2/,
    )
})

test("rejects a Tiptap license that does not match its recorded checksum", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "@tiptap/core", "2.27.2", "MIT")
    const licenseOptions = await writeTiptapLicense(
        root,
        "2.27.2",
        "changed Tiptap license\n",
        "0".repeat(64),
    )

    await assert.rejects(
        createThirdPartyNotices([
            metafile([["node_modules/@tiptap/core/index.js", 10]]),
        ], {workingDirectory: root, ...licenseOptions}),
        /vendored Tiptap license SHA-256.*expected 0000000000000000000000000000000000000000000000000000000000000000/,
    )
})

test("rejects a bundled package without a license notice", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "unlicensed", "1.0.0", "MIT")

    await assert.rejects(
        createThirdPartyNotices([
            metafile([["node_modules/unlicensed/index.js", 10]]),
        ], {workingDirectory: root}),
        /unlicensed has no license, licence, copying, or notice file/,
    )
})

test("rejects an empty bundled license document", async (context) => {
    const root = await temporaryDirectory(context)
    await writePackage(root, "empty-license", "1.0.0", "MIT", " \n\t")

    await assert.rejects(
        createThirdPartyNotices([
            metafile([["node_modules/empty-license/index.js", 10]]),
        ], {workingDirectory: root}),
        /License document LICENSE for empty-license@1\.0\.0 is empty/,
    )
})
