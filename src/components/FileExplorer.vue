<script setup>
import { computed } from 'vue';

const props = defineProps({
  currentRemote: {
    type: String,
    default: ''
  },
  currentPath: {
    type: String,
    default: ''
  },
  items: {
    type: Array,
    required: true
  }
});

const emit = defineEmits(['selectItem']);

const sortedItems = computed(() => {
  return [...props.items].sort((a, b) => {
    // 先按类型排序（文件夹在前）
    if (a.isDir !== b.isDir) {
      return b.isDir ? 1 : -1;
    }
    // 再按名称排序
    return a.name.localeCompare(b.name);
  });
});

// 获取文件图标
function getFileIcon(name, isDir) {
  if (isDir) return '📁';
  
  const ext = name.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'pdf': return '📕';
    case 'doc':
    case 'docx': return '📘';
    case 'xls':
    case 'xlsx': return '📗';
    case 'ppt':
    case 'pptx': return '📙';
    case 'jpg':
    case 'jpeg':
    case 'png':
    case 'gif': return '🖼️';
    case 'mp3':
    case 'wav': return '🎵';
    case 'mp4':
    case 'avi':
    case 'mov': return '🎬';
    case 'zip':
    case 'rar':
    case '7z': return '📦';
    default: return '📄';
  }
}
</script>

<template>
  <div class="file-explorer">
    <div class="path-bar">
      <span class="path-segment" @click="emit('selectItem', { name: '', isDir: true })">
        {{ currentRemote }}:
      </span>
      <template v-for="(segment, index) in currentPath.split('/')" :key="index">
        <span class="path-separator">/</span>
        <span class="path-segment" 
              @click="emit('selectItem', { 
                name: currentPath.split('/').slice(0, index + 1).join('/'), 
                isDir: true 
              })">
          {{ segment }}
        </span>
      </template>
    </div>
    
    <div class="files-grid">
      <div v-for="item in sortedItems" 
           :key="item.name"
           class="file-item"
           @click="emit('selectItem', item)"
           @dblclick="item.isDir && emit('selectItem', item)">
        <div class="file-icon">{{ getFileIcon(item.name, item.isDir) }}</div>
        <div class="file-name">{{ item.name }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-explorer {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  user-select: none;
}

.path-bar {
  padding: 8px 16px;
  background-color: var(--path-bar-bg, #f0f0f0);
  border-bottom: 1px solid var(--border-color, #ddd);
  white-space: nowrap;
  overflow-x: auto;
}

.path-segment {
  cursor: pointer;
  color: var(--text-color, #333);
}

.path-segment:hover {
  color: var(--primary-color, #007bff);
}

.path-separator {
  margin: 0 4px;
  color: var(--separator-color, #666);
}

.files-grid {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
  align-content: start;
}

.file-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.file-item:hover {
  background-color: var(--item-hover-bg, rgba(0, 0, 0, 0.05));
}

.file-icon {
  font-size: 32px;
  margin-bottom: 4px;
}

.file-name {
  text-align: center;
  font-size: 12px;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  word-break: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .file-explorer {
    --path-bar-bg: #1e1e1e;
    --border-color: #333;
    --text-color: #e0e0e0;
    --separator-color: #888;
    --primary-color: #3b82f6;
    --item-hover-bg: rgba(255, 255, 255, 0.1);
  }

  .file-item:hover {
    background-color: var(--item-hover-bg);
  }
}
</style> 