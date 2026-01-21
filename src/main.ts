import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./constants";

createApp(App).use(router).mount("#app");
