import {createRouter, createWebHashHistory} from "vue-router";
import StoreView from "@/views/StoreView.vue";
import LibraryView from "@/views/LibraryView.vue";
import SettingsView from "@/views/SettingsView.vue";

const routes = [
    {path: "/", redirect: "/store"},
    {path: "/store", component: StoreView},
    {path: "/library", component: LibraryView},
    {path: "/settings", component: SettingsView},

];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

export default router;
