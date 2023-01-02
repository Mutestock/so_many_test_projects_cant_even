import { listen } from "@tauri-apps/api/event";

export async function menuEventListenerInit(): Promise<void> {
    let eventName: string = "";
    listen<string>("tauri://menu", (event) => {
        eventName = event.payload;
        console.log(`New menu event of type ${eventName}. This should be handled correctly in the future`);
    }).catch((err) => console.log(`Error happened. This should be logged correctly in the future ${eventName}`));
}