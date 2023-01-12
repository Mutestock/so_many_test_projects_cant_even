import { invoke } from "@tauri-apps/api"

export class NodeVisualInvocation {
    nodeName: string;
    colorCodeHex: string;

    constructor(nodeName: string, colorCodeHex: string) {
        this.nodeName = nodeName
        this.colorCodeHex = colorCodeHex
    }

    public static async readNodeVisualList(): Promise<NodeVisualInvocation[]> {
        let res = await invoke("cmd_read_list_node_visual", {}) as any;
        return res.payload.map(x => new NodeVisualInvocation(x.node_name, x.color_code_hex))
    }
}