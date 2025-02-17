<script setup>
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import RemotesSidebar from "./components/RemotesSidebar.vue";
import FileExplorer from "./components/FileExplorer.vue";
import { useRemoteStore } from './store/remotes'
import { useDragResizeStore } from './store/dragResize'

// 使用remote store
const remoteStore = useRemoteStore()
const { 
    remotes,
    currentRemote,
    currentPath,
    currentItems,
    isLoading,
    loadRemotes,
    loadFiles
} = remoteStore

// 使用drag resize store
const dragResizeStore = useDragResizeStore()
const {
    isDragging,
    sidebarWidth,
    containerRef,
    handleDrag,
    startDrag,
    stopDrag,
    cleanup
} = dragResizeStore

function handleFileSelect(item) {
  if (item.isDir) {
    const newPath = currentPath.value ? `${currentPath.value}/${item.name}` : item.name;
    loadFiles(currentRemote.value, newPath);
  }
}

onUnmounted(() => {
  cleanup()
});

onMounted(() => {
  loadRemotes();
});
</script>

<template>
  <div class="app-container" 
       @mousemove="handleDrag"
       ref="containerRef">
    <RemotesSidebar 
      :remotes="remotes"
      :current-remote="currentRemote"
      :is-loading="isLoading"
      :style="{
        width: `${sidebarWidth}%`,
        willChange: isDragging ? 'width' : 'auto'
      }"
      @refresh="loadRemotes"
      @select-remote="loadFiles"
      @add-remote="addNewRemote"
    />
    <div class="resize-handle" 
         @mousedown="startDrag"
         :class="{ 'is-dragging': isDragging }">
      <div class="resize-handle-bar"></div>
    </div>
    <FileExplorer 
      :current-remote="currentRemote"
      :current-path="currentPath"
      :items="currentItems"
      @select-item="handleFileSelect"
    />
  </div>
</template>

<style>
.app-container {
  display: flex;
  height: 100vh;
  margin: 0;
  padding: 0;
  position: relative;
  touch-action: none; /* 防止移动设备上的滚动干扰 */
}

.resize-handle {
  width: 8px;
  cursor: ew-resize;
  background: transparent;
  position: relative;
  z-index: 1;
  touch-action: none;
  will-change: transform;
}

.resize-handle:hover .resize-handle-bar,
.resize-handle.is-dragging .resize-handle-bar {
  background-color: #007bff;
}

.resize-handle-bar {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
  width: 2px;
  background-color: #ddd;
  transform: translateX(-50%);
  transition: background-color 0.2s;
}

/* 全局样式 */
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .resize-handle-bar {
    background-color: #404040;
  }

  .resize-handle:hover .resize-handle-bar,
  .resize-handle.is-dragging .resize-handle-bar {
    background-color: #0056b3;
  }
}

/* 在现有样式的基础上添加 */
:root {
  --system-ui: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, 
               "Helvetica Neue", Arial, sans-serif;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: var(--system-ui);
}

/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #666;
}

@media (prefers-color-scheme: dark) {
  ::-webkit-scrollbar-thumb {
    background: #666;
  }
  
  ::-webkit-scrollbar-thumb:hover {
    background: #888;
  }
}
</style>
