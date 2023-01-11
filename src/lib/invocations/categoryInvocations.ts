import { invoke } from "@tauri-apps/api"


export class CategoryInvocation {
    name: string;
    colorCodeHex: string;
    visibilityToggledOn: boolean;

    constructor(name: string, colorCodeHex: string, visibilityToggledOn: boolean) {
        this.name = name;
        this.colorCodeHex = colorCodeHex;
        this.visibilityToggledOn = visibilityToggledOn
    }

    public static async createCategory(name: string, colorCodeHex: string): Promise<void> {
        await invoke("cmd_create_category", {
            newCategoryName: name,
            colorCodeHex: colorCodeHex
        })
    }

    public static async readCategory(categoryName): Promise<CategoryInvocation> {
        let res = await invoke("cmd_read_category", {
            categoryName: categoryName
        }) as any;
        res = res.payload;
        return new CategoryInvocation(res.name, res.colorCodeHex, res.visibilityToggledOn)
    }

    public static async deleteCategory(categoryName: string): Promise<void> {
        await invoke("cmd_delete_category", {
            categoryName: categoryName
        });
    }
    
    public static async readListCategory(): Promise<CategoryInvocation[]> {
        let res = await invoke("cmd_read_list_category", {}) as any;

        return res.payload.map(x => new CategoryInvocation(
            x.name,
            x.color_code_hex,
            x.visibility_toggled_on
        ))
    }

    public static async categoryToggleVisibility(categoryName: string): Promise<void> {
        await invoke("cmd_category_toggle_visibility", { categoryName: categoryName })
    }
}
