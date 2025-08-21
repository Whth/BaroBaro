import { createRouter, createWebHistory } from "vue-router";
import DashboardView from "@/views/dashboard/DashboardView.vue";
import ModsView from "@/views/mods/ModsView.vue";
import SettingsView from "@/views/settings/SettingsView.vue";

const routes = [
	{
		path: "/",
		name: "Dashboard",
		component: DashboardView,
	},
	{
		path: "/mods",
		name: "Mods",
		component: ModsView,
	},
	{
		path: "/settings",
		name: "Settings",
		component: SettingsView,
	},
];

const router = createRouter({
	history: createWebHistory(),
	routes,
});

export default router;
