import type {ExtensionDescriptor} from "./bridge_api.ts"
import {getBridgeBindings} from "./bridge_api.ts"

export function registerOfficialExtension(descriptor: ExtensionDescriptor): void {
    getBridgeBindings().registerExtension(descriptor)
}
