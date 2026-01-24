import { computed, ref, watch } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { getConfig, setConfig } from "./bridge";

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            name: "Home",
            component: () => import("./views/Home.vue")
        }
    ]
})

export const systemDarkMode = computed(() => {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
});

(() => {
    if (systemDarkMode.value) {
        document.documentElement.classList.add("dark")
    } 
})();

export const progressbar = ref(0);

// function increaseProgress() {
//     progressbar.value += 5;
//     if (progressbar.value > 100) {
//         progressbar.value = 100;
//         setTimeout(() => {
//             progressbar.value = 0;
//             increaseProgress()
//         }, 1500)
//     }
//     setTimeout(() => {
//         if (progressbar.value < 100) {
//             increaseProgress();
//         }
//     }, 100)
// }
// increaseProgress()

export const config = ref(await getConfig());
watch(config, async (newConfig) => {
    console.log(newConfig)
    setConfig(newConfig)  
})