import * as tiptapCore from "@tiptap/core"
import * as pmDropcursor from "@tiptap/pm/dropcursor"
import * as pmGapcursor from "@tiptap/pm/gapcursor"
import * as pmHistory from "@tiptap/pm/history"
import * as pmModel from "@tiptap/pm/model"
import * as pmState from "@tiptap/pm/state"

import {getOrCreateBridgeBindings} from "./bridge_api.ts"

export function init_tiptap_core(): void {
    const bridgeBindings = getOrCreateBridgeBindings()
    bridgeBindings.modules["@tiptap/core"] = tiptapCore
    bridgeBindings.modules["@tiptap/pm/dropcursor"] = pmDropcursor
    bridgeBindings.modules["@tiptap/pm/gapcursor"] = pmGapcursor
    bridgeBindings.modules["@tiptap/pm/history"] = pmHistory
    bridgeBindings.modules["@tiptap/pm/model"] = pmModel
    bridgeBindings.modules["@tiptap/pm/state"] = pmState
}
