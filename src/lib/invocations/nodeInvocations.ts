import { invoke } from "@tauri-apps/api"

export async function invokeReadAllNodesWithToggledOnCategories() {
    let response = await invoke("cmd_read_list_toggled_on", {})
    return response.payload
}


export class NodeInvocation {
    name: string;
    dateAdded: string;
    dateModified: string;
    primaryImagePath: string | null;
    category: string;

    constructor(name: string,
        dateAdded: string,
        dateModified: string,
        primaryImagePath: string | null,
        category: string) {

        this.name = name;
        this.dateAdded = dateAdded;
        this.dateModified = dateModified;
        this.primaryImagePath = primaryImagePath
        this.category = category
    }

    public static async createNode(category: string, name: string): Promise<void> {
        await invoke("cmd_create_node", {
            category: category,
            name: name
        })
    }

    public static async readNode(name: string): Promise<NodeInvocation> {
        let res = invoke("cmd_read_node", {
            name: name
        }) as any;
        res = res.payload;

        return new NodeInvocation(
            res.name,
            res.dateAdded,
            res.dateModified,
            res.primaryImagePath,
            res.category
        );
    }

    public static async deleteNode(name: string): Promise<void> {
        await invoke("cmd_delete_node", {
            name: name
        })
    }

    public static async readNodeList(): Promise<NodeInvocation[]> {
        let res = invoke("cmd_read_list_node", {}) as any;
        return res.payload.map(x => new NodeInvocation(
            x.name,
            x.dateAdded,
            x.dateModified,
            x.primaryImagePath,
            x.category
        ))
    }

    public static async readNodesByCategory(category: string): Promise<NodeInvocation[]> {
        let res = invoke("cmd_read_nodes_by_category", {
            category: category
        }) as any;
        return res.payload.map(x => new NodeInvocation(
            x.name,
            x.dateAdded,
            x.dateModified,
            x.primaryImagePath,
            x.category
        ))
    }

    public static async readNodeListToggledOn(): Promise<NodeInvocation[]> {
        let res = invoke("cmd_read_list_toggled_on", {}) as any;
        return res.payload.map(x => new NodeInvocation(
            x.name,
            x.dateAdded,
            x.dateModified,
            x.primaryImagePath,
            x.category
        ))
    }
}