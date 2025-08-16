import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/dashboard/DashboardView.vue'
import ModsView from '@/views/mods/ModsView.vue'
import ProfilesView from '@/views/profiles/ProfilesView.vue'
import SettingsView from '@/views/settings/SettingsView.vue'

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: DashboardView
  },
  {
    path: '/mods',
    name: 'Mods',
    component: ModsView
  },
  {
    path: '/profiles',
    name: 'Profiles',
    component: ProfilesView
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router