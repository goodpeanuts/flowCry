<script setup>
import { ref } from "vue";

const props = defineProps({
  remotes: {
    type: Array,
    required: true
  },
  currentRemote: {
    type: String,
    default: ''
  },
  isLoading: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['refresh', 'selectRemote', 'addRemote']);
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <h3>Remotes</h3>
      <button class="refresh-btn" 
              @click="emit('refresh')" 
              title="Refresh remotes list"
              :disabled="isLoading">
        🔄
      </button>
    </div>
    <div class="remotes-list">
      <div v-for="remote in remotes" 
           :key="remote.name"
           @click="emit('selectRemote', remote.name)"
           :class="['remote-item', { active: remote.name === currentRemote }]">
        <div class="remote-name">{{ remote.name }}</div>
        <div class="remote-type">{{ remote.type_ }}</div>
      </div>
    </div>
    <button class="add-remote-btn" @click="emit('addRemote')">
      + Add Remote
    </button>
  </div>
</template>

<style scoped>
.sidebar {
  background-color: #f0f0f0;
  display: flex;
  flex-direction: column;
  border-right: none;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  border-bottom: 1px solid #ddd;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1.1em;
}

.refresh-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 5px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.2s;
}

.refresh-btn:hover {
  background-color: #e0e0e0;
}

.refresh-btn:active {
  transform: rotate(180deg);
}

.remotes-list {
  flex: 1;
  overflow-y: auto;
  padding: 0 10px 10px 10px;
}

.remote-item {
  padding: 10px;
  margin: 5px 0;
  cursor: pointer;
  border-radius: 4px;
}

.remote-item:hover {
  background-color: #e0e0e0;
}

.remote-item.active {
  background-color: #007bff;
  color: white;
}

.remote-name {
  font-weight: bold;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.remote-type {
  font-size: 0.8em;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.add-remote-btn {
  margin: 10px;
  padding: 10px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.add-remote-btn:hover {
  background-color: #218838;
}

@media (prefers-color-scheme: dark) {
  .sidebar {
    background-color: #2a2a2a;
    border-right-color: #404040;
  }

  .remote-item:hover {
    background-color: #3a3a3a;
  }

  .remote-type {
    color: #999;
  }

  .sidebar-header {
    border-bottom-color: #404040;
  }

  .refresh-btn:hover {
    background-color: #3a3a3a;
  }
}
</style> 