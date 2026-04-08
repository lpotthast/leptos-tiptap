import {build} from "esbuild"
import {promises as fs} from "node:fs"
import path from "node:path"

const outputDir = path.resolve("../leptos-tiptap/src/js/generated")
const bridgeGlobalKey = "__LEPTOS_TIPTAP_BRIDGE__"

const tiptapCoreEntry = {
    tiptap_core: "./src/tiptap_core.ts",
}

const bridgeRuntimeEntry = {
    bridge_runtime: "./src/bridge_runtime.ts",
}

const extensionEntries = {
    tiptap_blockquote: "./src/extensions/tiptap_blockquote.ts",
    tiptap_bold: "./src/extensions/tiptap_bold.ts",
    tiptap_bullet_list: "./src/extensions/tiptap_bullet_list.ts",
    tiptap_code: "./src/extensions/tiptap_code.ts",
    tiptap_code_block: "./src/extensions/tiptap_code_block.ts",
    tiptap_document: "./src/extensions/tiptap_document.ts",
    tiptap_dropcursor: "./src/extensions/tiptap_dropcursor.ts",
    tiptap_gapcursor: "./src/extensions/tiptap_gapcursor.ts",
    tiptap_hard_break: "./src/extensions/tiptap_hard_break.ts",
    tiptap_heading: "./src/extensions/tiptap_heading.ts",
    tiptap_highlight: "./src/extensions/tiptap_highlight.ts",
    tiptap_history: "./src/extensions/tiptap_history.ts",
    tiptap_horizontal_rule: "./src/extensions/tiptap_horizontal_rule.ts",
    tiptap_image: "./src/extensions/tiptap_image.ts",
    tiptap_italic: "./src/extensions/tiptap_italic.ts",
    tiptap_link: "./src/extensions/tiptap_link.ts",
    tiptap_list_item: "./src/extensions/tiptap_list_item.ts",
    tiptap_ordered_list: "./src/extensions/tiptap_ordered_list.ts",
    tiptap_paragraph: "./src/extensions/tiptap_paragraph.ts",
    tiptap_strike: "./src/extensions/tiptap_strike.ts",
    tiptap_text: "./src/extensions/tiptap_text.ts",
    tiptap_text_align: "./src/extensions/tiptap_text_align.ts",
    tiptap_youtube: "./src/extensions/tiptap_youtube.ts",
}

const exportNameCache = new Map()

function createHostedModulePlugin() {
    return {
        name: "hosted-modules",
        setup(pluginBuild) {
            pluginBuild.onResolve({
                filter: /^@tiptap\/(core|pm\/(dropcursor|gapcursor|history|model|state))$/,
            }, (args) => ({
                path: args.path,
                namespace: "hosted-module",
            }))

            pluginBuild.onLoad({filter: /.*/, namespace: "hosted-module"}, async (args) => ({
                contents: await createHostedModuleContents(args.path),
                loader: "js",
            }))
        },
    }
}

async function createHostedModuleContents(moduleName) {
    let exportNames = exportNameCache.get(moduleName)
    if (exportNames == null) {
        const namespace = await import(moduleName)
        exportNames = Object.keys(namespace)
            .filter((name) => name !== "default")
            .sort()
        exportNameCache.set(moduleName, exportNames)
    }

    const lines = [
        `const bridgeBindings = globalThis.${bridgeGlobalKey};`,
        "if (bridgeBindings == null || bridgeBindings.modules == null) throw new Error(\"leptos-tiptap bridge bindings are unavailable\");",
        `const moduleExports = bridgeBindings.modules[${JSON.stringify(moduleName)}];`,
        `if (moduleExports == null) throw new Error(${JSON.stringify(`leptos-tiptap bridge module "${moduleName}" is unavailable`)});`,
    ]

    for (const exportName of exportNames) {
        lines.push(`export const ${exportName} = moduleExports[${JSON.stringify(exportName)}];`)
    }

    return lines.join("\n")
}

async function validateImportFreeArtifacts(directory) {
    const files = (await fs.readdir(directory))
        .filter((name) => name.endsWith(".js"))

    for (const file of files) {
        const contents = await fs.readFile(path.join(directory, file), "utf8")
        if (/^\s*import\s/m.test(contents)) {
            throw new Error(`Generated artifact ${file} still contains an import statement.`)
        }
    }
}

async function buildEntryPoints(entryPoints, plugins = []) {
    await build({
        entryPoints,
        bundle: true,
        format: "esm",
        minify: true,
        platform: "browser",
        splitting: false,
        target: "es2020",
        outdir: outputDir,
        plugins,
    })
}

await fs.rm(outputDir, {recursive: true, force: true})
await fs.mkdir(outputDir, {recursive: true})

await buildEntryPoints(tiptapCoreEntry)
await buildEntryPoints(bridgeRuntimeEntry)

const hostedModulePlugin = createHostedModulePlugin()

for (const [outfile, entryPoint] of Object.entries(extensionEntries)) {
    await buildEntryPoints({[outfile]: entryPoint}, [hostedModulePlugin])
}

await validateImportFreeArtifacts(outputDir)
