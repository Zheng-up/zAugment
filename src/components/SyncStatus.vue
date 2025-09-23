<template>
  <div class="sync-status-component">
    <div class="status-header">
      <div class="header-content">
        <h3>存储状态</h3>
      </div>
    </div>

    <div class="status-content">
      <!-- 存储类型显示 -->
      <div class="storage-info">
        <div class="storage-mode">
          <span class="storage-badge local">本地存储</span>
        </div>
        <div class="sync-hint">数据保存在本地文件</div>
      </div>
    </div>
  </div>
</template>

<script setup>
// 简化的同步状态组件，只显示本地存储状态
// 不需要任何响应式数据或方法，因为现在只是静态显示
</script>

<style scoped>
.sync-status-component {
  background: white;
  border-radius: 6px;
  padding: 10px 5px;
  border: 1px solid #e5e7eb;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  gap: 8px;
  height: fit-content;
  text-align: left;
  width: 100%;
}

.status-header {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-shrink: 0;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-header h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: #374151;
}

.storage-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.storage-mode {
  display: flex;
  justify-content: center;
}

.storage-badge {
  font-size: 11px;
  font-weight: 600;
  padding: 3px 8px;
  border-radius: 12px;
  white-space: nowrap;
}

.sync-hint {
  font-size: 10px;
  color: #6b7280;
  text-align: center;
  font-weight: 500;
}

.storage-badge.local {
  background: #fef3c7;
  color: #92400e;
}

@media (max-width: 768px) {
  .sync-status-component {
    padding: 8px 10px;
    gap: 6px;
  }

  .status-header h3 {
    font-size: 12px;
  }

  .storage-badge {
    font-size: 10px;
    padding: 2px 6px;
  }

  .sync-hint {
    font-size: 9px;
  }
}
</style>
