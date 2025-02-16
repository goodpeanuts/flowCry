import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core"

export const useRemoteStore = () => {
    const remotes = ref([])
    const currentRemote = ref('')
    const currentPath = ref('')
    const currentItems = ref([])
    const isLoading = ref(false)

    // 获取remote列表
    async function loadRemotes() {
        if (isLoading.value) return
        
        isLoading.value = true
        try {
            const result = await invoke("get_remotes")
            remotes.value = result
        } catch (error) {
            console.error("Failed to load remotes:", error)
        } finally {
            isLoading.value = false
        }
    }

    // 获取文件列表
    async function loadFiles(remote, path = '') {
        try {
            currentRemote.value = remote
            currentPath.value = path
            const result = await invoke("list_files", { 
                remote: remote,
                path: path 
            })
            currentItems.value = result
        } catch (error) {
            console.error("Failed to load files:", error)
        }
    }

    return {
        remotes,
        currentRemote,
        currentPath,
        currentItems,
        isLoading,
        loadRemotes,
        loadFiles
    }
} 