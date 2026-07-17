import { execFileSync } from 'node:child_process'
import { readFile, writeFile } from 'node:fs/promises'

const selector = process.argv[2] ?? '2'
const packageJsonPath = new URL('./package.json', import.meta.url)
const packageJson = JSON.parse(await readFile(packageJsonPath, 'utf8'))
const packageVersions = JSON.parse(
  execFileSync(
    'npm',
    ['view', `@tiptap/core@${selector}`, 'version', '--json'],
    { encoding: 'utf8' },
  ),
)
const version = Array.isArray(packageVersions)
  ? packageVersions.at(-1)
  : packageVersions

if (typeof version !== 'string' || !/^2\.\d+\.\d+$/.test(version)) {
  throw new Error(
    `Selector ${JSON.stringify(selector)} resolved to ${JSON.stringify(version)}, not a stable Tiptap 2 release`,
  )
}

const tiptapDependencies = Object.keys(packageJson.dependencies).filter(name =>
  name.startsWith('@tiptap/'),
)

for (const name of tiptapDependencies) {
  packageJson.dependencies[name] = version
}
packageJson.version = version

await writeFile(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`)
console.log(`Pinned ${tiptapDependencies.length} Tiptap packages to ${version}`)
