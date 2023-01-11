import { invoke } from "@tauri-apps/api";

export class ImageInvocation {
    imageTitle: string;
    imagePath: string;
    dateAdded: string;
    dateModified: string;
    nodeName: string;

    constructor(imageTitle: string, imagePath: string, dateAdded: string, dateModified: string, nodeName: string) {
        this.imageTitle = imageTitle;
        this.imagePath = imagePath;
        this.dateAdded = dateAdded;
        this.dateModified = dateModified;
        this.nodeName = nodeName;
    }

    public static async createImage(imageTitle: string, nodeName: string): Promise<void> {
        await invoke("cmd_create_image", {
            imageTitle: imageTitle,
            nodeName: nodeName
        })
    }

    public static async readImage(imageTitle: string): Promise<ImageInvocation> {
        let res = await invoke("cmd_read_image", { imageTitle: imageTitle }) as any;
        res = res.payload;
        return new ImageInvocation(
            res.imageTitle,
            res.imagePath,
            res.dateAdded,
            res.dateModified,
            res.nodeName
        );
    }

    public static async deleteImage(imageTitle: string): Promise<void> {
        await invoke("cmd_delete_image", { imageTitle: imageTitle });
    }

    public static async readListImage(): Promise<ImageInvocation[]> {
        let res = await invoke("cmd_read_list_image", {}) as any[];
        return res.map(x => new ImageInvocation(
            x.payload.imageTitle,
            x.payload.imagePath,
            x.payload.dateAdded,
            x.payload.dateModified,
            x.payload.nodeName
        ))
    }
}