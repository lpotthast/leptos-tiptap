import { createHash } from 'node:crypto'
import { execFileSync } from 'node:child_process'
import { readFile, writeFile } from 'node:fs/promises'

const selector = process.argv[2] ?? '2'
const packageJsonPath = new URL('./package.json', import.meta.url)
const tiptapLicensePath = new URL('./licenses/tiptap-MIT.txt', import.meta.url)
const tiptapLicenseMetadataPath = new URL('./licenses/tiptap-MIT.json', import.meta.url)
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

const declaredLicense = JSON.parse(
  execFileSync(
    'npm',
    ['view', `@tiptap/core@${version}`, 'license', '--json'],
    { encoding: 'utf8' },
  ),
)
if (declaredLicense !== 'MIT') {
  throw new Error(
    `@tiptap/core@${version} declares ${JSON.stringify(declaredLicense)}, not MIT`,
  )
}

const licenseRevision = `@tiptap/core@${version}`
const licenseSource = `https://github.com/ueberdosis/tiptap/blob/${licenseRevision}/LICENSE.md`
const rawLicenseSource = `https://raw.githubusercontent.com/ueberdosis/tiptap/${licenseRevision}/LICENSE.md`
const licenseResponse = await fetch(rawLicenseSource)
if (!licenseResponse.ok) {
  throw new Error(
    `Could not download the Tiptap license for ${version}: HTTP ${licenseResponse.status}`,
  )
}
const tiptapLicense = `${(await licenseResponse.text()).replace(/\r\n/g, '\n').trimEnd()}\n`
if (tiptapLicense.trim().length === 0) {
  throw new Error(`Downloaded an empty Tiptap license for ${version}`)
}
const tiptapLicenseMetadata = {
  package: '@tiptap/core',
  version,
  license: declaredLicense,
  source: licenseSource,
  sha256: createHash('sha256').update(tiptapLicense).digest('hex'),
}

const tiptapDependencies = Object.keys(packageJson.dependencies).filter(name =>
  name.startsWith('@tiptap/'),
)

for (const name of tiptapDependencies) {
  packageJson.dependencies[name] = version
}
packageJson.version = version

await Promise.all([
  writeFile(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`),
  writeFile(tiptapLicensePath, tiptapLicense),
  writeFile(tiptapLicenseMetadataPath, `${JSON.stringify(tiptapLicenseMetadata, null, 2)}\n`),
])
console.log(`Pinned ${tiptapDependencies.length} Tiptap packages to ${version}`)
