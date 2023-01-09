import { listen } from "@tauri-apps/api/event";
import { goto } from '$app/navigation';



export async function menuEventListenerInit(): Promise<void> {
    let menuEventName: string = "";
    listen<string>("tauri://menu", (event) => {
        menuEventName = event.payload;
        handleMenuEventRedirection(menuEventName);
    }).catch((err) => console.log(`Error happened ${err}. This should be logged correctly in the future ${menuEventName}`));
}

class EventRerouter {
    private rerouteUrl: string;

    constructor(rerouteUrl: string) {
        this.rerouteUrl = rerouteUrl;
    }

    reroute() {
        goto(this.rerouteUrl)
    }
}

const EventRerouters = {
    "new": null,
    "overview": new EventRerouter("/overview"),
    "about": new EventRerouter("/about"),
    "import": new EventRerouter("/import"),
    "sync": new EventRerouter("/sync"),
    "backup": new EventRerouter("/backup"),
    "new_node": new EventRerouter("/new-node"),
    "new_category": new EventRerouter("/new-category"),
}


function handleMenuEventRedirection(menuEventName: string) {
    let caughtEvent: EventRerouter | null = EventRerouters[menuEventName];
    if (caughtEvent == null || caughtEvent == undefined) {
        return;
    }
    caughtEvent.reroute()
}