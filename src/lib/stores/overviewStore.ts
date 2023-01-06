import { writable } from "svelte/store";



// When looking at the overview of available nodes
export const currentlySelectedCategory = writable("");

// I.e. categories which are not banned
export const toggledOnCategories = writable([]);