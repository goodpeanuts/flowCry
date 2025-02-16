<script setup>
defineProps({
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
</script>

<template>
  <div class="file-explorer">
    <div class="path-bar">
      Current path: {{ currentRemote }}:{{ currentPath }}
    </div>
    <div class="files-list">
      <div v-for="item in items" 
           :key="item.name"
           class="file-item"
           @click="emit('selectItem', item)">
        <span class="file-icon">{{ item.isDir ? '📁' : '📄' }}</span>
        <span class="file-name">{{ item.name }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-explorer {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
  min-width: 0;
}

.path-bar {
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
  margin-bottom: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.files-list {
  flex: 1;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
}

.file-item:hover {
  background-color: #f8f9fa;
}

.file-icon {
  margin-right: 10px;
}

.file-name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

@media (prefers-color-scheme: dark) {
  .path-bar {
    background-color: #2a2a2a;
  }

  .file-item:hover {
    background-color: #2a2a2a;
  }
}
</style> 