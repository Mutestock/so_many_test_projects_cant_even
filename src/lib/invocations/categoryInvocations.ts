import { invoke } from "@tauri-apps/api"

// Again, currently typescript type checking just isn't clicking with svelte for some reason.

export async function invokeReadAllCategories() {
    let response = await invoke("cmd_read_list_category", {})
    return response.payload
}

export async function invokeToggleCategory(category) {
    let response = await invoke("cmd_category_toggle_visibility", {
        categoryName: category,
    });
    return response.payload
}

export async function invokeCreateCategory(newCategoryName, colorCodeHex) {
    let response = await invoke("cmd_create_category", {
        newCategoryName: newCategoryName,
        colorCodeHex: colorCodeHex
    });
    return response.payload
}


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

    public static async readcategory(categoryName): Promise<CategoryInvocation> {
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
        let res = await invoke("cmd_read_list_category", {}) as any[];
        console.log(res);
        
        return res.payload.map(x => new CategoryInvocation(
            x.name,
            x.colorCodeHex,
            x.visibilityToggledOn
        ))
    }

    public static async categoryToggleVisibility(categoryName: string): Promise<void> {
        await invoke("cmd_category_toggle_visibility", { categoryName: categoryName })
    }
}