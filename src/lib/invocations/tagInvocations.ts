import { invoke } from "@tauri-apps/api"

export class TagInvocation {
    name: string

    constructor(name: string) {
        this.name = name
    }

    public async createTag(): Promise<void> {
        await invoke("cmd_create_tag", {
            tagName: "this.name"
        });
    }

    public async deleteTag(): Promise<void> {
        await invoke("cmd_delete_tag", {
            tagName: "this.name"
        });
    }

    public async readListTag(): Promise<TagInvocation[]> {
        let res = await invoke("cmd_read_list_tag", {}) as any[];
        return res.map(x => new TagInvocation(x.name));
    }

    public async updateTag(oldTagName: string, newTagName: string): Promise<void> {
        await invoke("cmd_update_tag", {
            oldTagName: oldTagName,
            newTagName: newTagName
        })
    }
}