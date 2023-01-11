import { invoke } from "@tauri-apps/api";

export class CommentInvocation {
    uuid: string;
    nodeName: string;
    content: string;
    dateAdded: string;
    dateModified: string;

    constructor(uuid: string, nodeName: string, content: string, dateAdded: string, dateModified: string) {
        this.uuid = uuid;
        this.nodeName = nodeName;
        this.content = content;
        this.dateAdded = dateAdded;
        this.dateModified = dateModified;
    }

    public static async createComment(nodeName: string, content: string): Promise<void> {
        await invoke("cmd_create_comment", {
            nodeName: nodeName,
            content: content
        })
    }

    public static async updateCommentContentByNodeName(nodeName: string, content: string): Promise<void> {
        await invoke("cmd_update_comment_content_by_node_name", {
            nodeName: nodeName,
            content: content
        });
    }

    public static async readCommentByNodeName(nodeName: string): Promise<CommentInvocation> {
        let res = await invoke("cmd_read_comment_by_node_name", {
            nodeName: nodeName
        }) as any;
        res = res.payload;

        return new CommentInvocation(
            res.uuid,
            res.nodeName,
            res.content,
            res.dateAdded,
            res.dateModified
        )
    }

    public static async deleteCommentByNodeName(nodeName: string): Promise<void> {
        await invoke("cmd_delete_comment_by_node_name", { nodeName: nodeName })
    }

    public static async readListComment(): Promise<CommentInvocation[]> {
        let res = await invoke("cmd_read_list_comment") as any[]
        return res.payload.map(x => new CommentInvocation(
            x.uuid,
            x.nodeName,
            x.content,
            x.dateAdded,
            x.dateModified)
        )
    }
}