import { invoke } from "@tauri-apps/api"

export class TagInvocation {
    name: string

    constructor(name: string) {
        this.name = name
    }

    public static async createTag(name: string): Promise<void> {
        await invoke("cmd_create_tag", {
            tagName: name
        });
    }

    public static async deleteTag(name: string): Promise<void> {
        await invoke("cmd_delete_tag", {
            tagName: name
        });
    }

    public static async readListTag(): Promise<TagInvocation[]> {
        let res = await invoke("cmd_read_list_tag", {}) as any;
        return res.payload.map(x => new TagInvocation(x.payload.name));
    }

    public static async updateTag(oldTagName: string, newTagName: string): Promise<void> {
        await invoke("cmd_update_tag", {
            oldTagName: oldTagName,
            newTagName: newTagName
        })
    }
}