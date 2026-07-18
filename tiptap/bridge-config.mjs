// Defines the versioned global bridge key and per-artifact scope shared by every bundle.
// Keeping this contract centralized prevents co-loaded builds from sharing incompatible state.

export const BRIDGE_PROTOCOL_VERSION = 1
export const BRIDGE_GLOBAL_KEY = `__LEPTOS_TIPTAP_BRIDGE_V${BRIDGE_PROTOCOL_VERSION}__`

/**
 * Scope bridge state to the directory containing one generated artifact set.
 * wasm-bindgen copies an application's bridge runtime and extension modules into
 * the same directory, while independently bundled applications receive distinct
 * artifact directories.
 *
 * @param {string} moduleUrl
 */
export function bridgeScopeKey(moduleUrl) {
  return new URL('.', moduleUrl).href
}
