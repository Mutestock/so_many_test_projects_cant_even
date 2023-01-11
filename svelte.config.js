import staticAdapter from "@sveltejs/adapter-static";
import preprocessor from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocessor(),
  kit: {
    adapter: staticAdapter(),
  },
};

export default config;
