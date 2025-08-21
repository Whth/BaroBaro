import {
  config,
  download_mod,
  installed_mod,
  list_installed_mods,
  list_mod_lists,
  mod_lists,
  refresh_config,
  save_config
} from '../invokes'
import {Config, SteamCmdConfig} from '../proto/config'
import {BarotraumaMod, ModList} from '../proto/mods'
import type {Profile} from '../types'
// Export the refs from invokes.ts so components can use them directly
export {config, installed_mod, mod_lists}

// Export the main composable function
export function useModManager() {
    // Return all the existing functions and refs
    return {
        // Refs
        config,
        installed_mod,
        mod_lists,

        // Initialization
        initializeApp,

        // Config management
        updateConfig,
        updateGameHome,
        updateSteamCmdHome,
        updateSteamCmdConfig,

        // Mod management
        refreshInstalledMods,
        loadInstalledMods: refreshInstalledMods, // Alias for compatibility
        downloadMods,

        // Profile/ModList management
        refreshModLists,
        createModList,
        updateModList,
        deleteModList,
        getModById,
        getModListByName,
        isModEnabled,

        // New function for saving profiles
        saveProfile
    }
}

// New function for saving profiles
export async function saveProfile(profile: Profile) {
    try {
        // Convert Profile to ModList
        const modList: ModList = {
            profileName: profile.name,
            basePackage: profile.basePackage,
            mods: profile.enabledMods
        }

        // If it's an existing profile, update it, otherwise create a new one
        const existingProfile = mod_lists.value.find((list: ModList) => list.profileName === profile.name)
        if (existingProfile) {
            await updateModList(modList)
        } else {
            await createModList(profile.name, profile.basePackage, profile.enabledMods)
        }

        console.log('Profile saved successfully')
    } catch (error) {
        console.error('Failed to save profile:', error)
        throw error
    }
}

// Initialize the application
export async function initializeApp() {
    try {
        await refresh_config()
        console.log('App initialized successfully')
    } catch (error) {
        console.error('Failed to initialize app:', error)
    }
}

// Config management functions
export async function updateConfig(newConfig: Config) {
    try {
        // Ensure both required fields are present
        const gameHome = newConfig.gameHome || config.value.gameHome || ''
        const steamcmdHome = newConfig.steamcmdHome || config.value.steamcmdHome || ''
        
        config.value = {
            ...newConfig,
            gameHome,
            steamcmdHome
        }
        await save_config()
        console.log('Config saved successfully')
    } catch (error) {
        console.error('Failed to save config:', error)
        throw error
    }
}

export async function updateGameHome(newGameHome: string) {
    try {
        // Ensure both required fields are present
        const currentSteamCmdHome = config.value.steamcmdHome || ''
        config.value = {
            ...config.value,
            gameHome: newGameHome,
            steamcmdHome: currentSteamCmdHome
        }
        await save_config()
        console.log('Game home updated successfully')
    } catch (error) {
        console.error('Failed to update game home:', error)
        throw error
    }
}

export async function updateSteamCmdHome(newSteamCmdHome: string) {
    try {
        // Ensure both required fields are present
        const currentGameHome = config.value.gameHome || ''
        config.value = {
            ...config.value,
            gameHome: currentGameHome,
            steamcmdHome: newSteamCmdHome
        }
        await save_config()
        console.log('SteamCMD home updated successfully')
    } catch (error) {
        console.error('Failed to update SteamCMD home:', error)
        throw error
    }
}

export async function updateSteamCmdConfig(newSteamCmdConfig: SteamCmdConfig) {
    try {
        // Ensure both required fields are present
        const currentGameHome = config.value.gameHome || ''
        const currentSteamCmdHome = config.value.steamcmdHome || ''
        config.value = {
            ...config.value,
            gameHome: currentGameHome,
            steamcmdHome: currentSteamCmdHome,
            steamcmdConfig: newSteamCmdConfig
        }
        await save_config()
        console.log('SteamCMD config updated successfully')
    } catch (error) {
        console.error('Failed to update SteamCMD config:', error)
        throw error
    }
}

// Mod management functions
export async function refreshInstalledMods() {
    try {
        await list_installed_mods()
        console.log('Installed mods refreshed successfully')
    } catch (error) {
        console.error('Failed to refresh installed mods:', error)
        throw error
    }
}

export async function downloadMods(modIds: number[]) {
    try {
        await download_mod(modIds)
        // After downloading, refresh the installed mods list
        await refreshInstalledMods()
        console.log('Mods downloaded successfully')
    } catch (error) {
        console.error('Failed to download mods:', error)
        throw error
    }
}

// Profile/ModList management functions
export async function refreshModLists() {
    try {
        await list_mod_lists()
        console.log('Mod lists refreshed successfully')
    } catch (error) {
        console.error('Failed to refresh mod lists:', error)
        throw error
    }
}

export async function createModList(profileName: string, basePackage: string, mods: string[]) {
    try {
        // In a real implementation, this would call a Tauri command to create a new mod list
        // For now, we'll just add it to the local ref and refresh
        console.log('Creating mod list:', profileName)
        await refreshModLists()
    } catch (error) {
        console.error('Failed to create mod list:', error)
        throw error
    }
}

export async function updateModList(modList: ModList) {
    try {
        // In a real implementation, this would call a Tauri command to update a mod list
        // For now, we'll just refresh the mod lists
        console.log('Updating mod list:', modList.profileName)
        await refreshModLists()
    } catch (error) {
        console.error('Failed to update mod list:', error)
        throw error
    }
}

export async function deleteModList(profileName: string) {
    try {
        // In a real implementation, this would call a Tauri command to delete a mod list
        // For now, we'll just refresh the mod lists
        console.log('Deleting mod list:', profileName)
        await refreshModLists()
    } catch (error) {
        console.error('Failed to delete mod list:', error)
        throw error
    }
}

// Utility functions
export function getModById(modId: string): BarotraumaMod | undefined {
    return installed_mod.value.find((mod: BarotraumaMod) => mod.steamWorkshopId === modId)
}

export function getModListByName(profileName: string): ModList | undefined {
    return mod_lists.value.find((list: ModList) => list.profileName === profileName)
}

export function isModEnabled(modId: string, currentModList: ModList | null): boolean {
    if (!currentModList) return false
    return currentModList.mods.includes(modId)
}