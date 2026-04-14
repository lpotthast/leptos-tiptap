import {spawnSync} from "node:child_process"
import {promises as fs} from "node:fs"
import path from "node:path"

const generatedDir = path.resolve("../src/js/generated")
const hostedModulesPath = path.resolve("./src/generated/hosted_modules.ts")

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

async function main() {
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
