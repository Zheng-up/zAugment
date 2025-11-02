<template>
  <ModalContainer
    :visible="visible"
    title="导入列表"
    size="medium"
    @close="$emit('close')"
  >
    <div class="import-list-modal">
      <!-- 顶部栏：统计信息 + 操作按钮 -->
      <div class="top-bar">
        <!-- 统计信息 -->
        <div class="stats-section">
          <div class="stat-item">
            <span class="stat-value">{{ stats.total }}</span>
            <span class="stat-label">总计</span>
          </div>
          <div class="stat-item success">
            <span class="stat-value">{{ stats.success }}</span>
            <span class="stat-label">成功</span>
          </div>
          <div class="stat-item failed">
            <span class="stat-value">{{ stats.failed }}</span>
            <span class="stat-label">失败</span>
          </div>
          <div class="stat-item pending">
            <span class="stat-value">{{ stats.pending }}</span>
            <span class="stat-label">进行中</span>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="actions-section">
          <button
            @click="retryAllFailed"
            :disabled="stats.failed === 0 || isRetrying"
            class="btn-action primary"
            title="重试全部失败"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
            </svg>
          </button>

          <!-- 清空按钮 + Popover 确认框 -->
          <div class="popover-wrapper">
            <button
              @click="showClearConfirm = !showClearConfirm"
              :disabled="stats.total === 0"
              class="btn-action secondary"
              title="清空列表"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
              </svg>
            </button>

            <!-- Popover 确认框 -->
            <div v-if="showClearConfirm" class="popover-confirm">
              <div class="popover-arrow"></div>
              <p class="popover-message">确定要清空所有导入记录吗？</p>
              <div class="popover-actions">
                <button @click="showClearConfirm = false" class="btn-cancel">取消</button>
                <button @click="confirmClearAll" class="btn-confirm">确定</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 导入记录列表 -->
      <div class="records-list">
        <div v-if="records.length === 0" class="empty-state">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
          </svg>
          <p>暂无导入记录</p>
        </div>

        <div v-else class="records-container">
          <div
            v-for="record in records"
            :key="record.id"
            class="record-item"
            :class="record.status"
          >
            <!-- 状态图标 -->
            <div class="record-status">
              <svg v-if="record.status === 'success'" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
              </svg>
              <svg v-else-if="record.status === 'failed'" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
              <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="currentColor" class="spinning">
                <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
              </svg>
            </div>

            <!-- Session 信息 -->
            <div class="record-content">
              <div class="record-session">
                <code>{{ maskSession(record.session) }}</code>
              </div>
              <div class="record-meta">
                <span class="record-time">{{ formatTime(record.importTime) }}</span>
                <span v-if="record.error" class="record-error">{{ record.error }}</span>
                <span v-if="record.message" class="record-message">{{ record.message }}</span>
              </div>
            </div>

            <!-- 操作按钮 -->
            <div class="record-actions">
              <button
                @click="copySession(record.session)"
                class="btn-icon"
                title="复制 Session"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                </svg>
              </button>
              <button
                v-if="record.status === 'failed'"
                @click="retryRecord(record)"
                :disabled="isRetrying"
                class="btn-icon retry"
                title="重试"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </ModalContainer>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useImportRecords, ImportStatus } from '../stores/importRecords';
import ModalContainer from './ModalContainer.vue';

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
});

// 监听弹窗关闭，同时关闭 Popover
watch(() => props.visible, (newVal) => {
  if (!newVal) {
    showClearConfirm.value = false;
  }
});

// Emits
const emit = defineEmits(['close', 'show-status']);

// 导入记录管理
const {
  records,
  stats,
  clearAll: clearAllRecords,
  getFailedRecords,
  updateRecord
} = useImportRecords();

// 状态
const isRetrying = ref(false);
const showClearConfirm = ref(false);

// 遮罩 Session（只显示前后各10个字符）
const maskSession = (session) => {
  if (!session || session.length <= 20) return session;
  return `${session.substring(0, 10)}...${session.substring(session.length - 10)}`;
};

// 格式化时间
const formatTime = (time) => {
  const date = new Date(time);
  const now = new Date();
  const diff = now - date;
  
  // 小于1分钟
  if (diff < 60000) {
    return '刚刚';
  }
  
  // 小于1小时
  if (diff < 3600000) {
    const minutes = Math.floor(diff / 60000);
    return `${minutes}分钟前`;
  }
  
  // 小于24小时
  if (diff < 86400000) {
    const hours = Math.floor(diff / 3600000);
    return `${hours}小时前`;
  }
  
  // 显示完整时间
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
};

// 复制 Session
const copySession = async (session) => {
  try {
    await navigator.clipboard.writeText(session);
    emit('show-status', 'Session 已复制到剪贴板', 'success');
  } catch (error) {
    emit('show-status', '复制失败', 'error');
  }
};

// 重试单条记录
const retryRecord = async (record) => {
  isRetrying.value = true;
  try {
    // 更新状态为进行中
    updateRecord(record.id, ImportStatus.PENDING, null, null);
    
    // 调用导入 API
    const response = await fetch('http://localhost:8766/api/import/session', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        session: record.session,
        detailed_response: false
      })
    });
    
    const result = await response.json();
    
    if (response.ok && result.success) {
      updateRecord(record.id, ImportStatus.SUCCESS, null, result.message);
      emit('show-status', '重试成功', 'success');
    } else {
      updateRecord(record.id, ImportStatus.FAILED, result.error || '导入失败', null);
      emit('show-status', '重试失败', 'error');
    }
  } catch (error) {
    updateRecord(record.id, ImportStatus.FAILED, error.message, null);
    emit('show-status', `重试失败: ${error.message}`, 'error');
  } finally {
    isRetrying.value = false;
  }
};

// 重试所有失败的记录
const retryAllFailed = async () => {
  const failedRecords = getFailedRecords();
  if (failedRecords.length === 0) return;

  isRetrying.value = true;

  try {
    // 先将所有失败记录状态更新为进行中
    failedRecords.forEach(record => {
      updateRecord(record.id, ImportStatus.PENDING, null, null);
    });

    // 使用批量导入 API
    const sessions = failedRecords.map(r => r.session);
    const response = await fetch('http://localhost:8766/api/import/sessions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        sessions: sessions,
        detailed_response: true
      })
    });

    const result = await response.json();

    if (response.ok) {
      // 根据返回结果更新每条记录的状态
      if (result.results) {
        result.results.forEach((importResult, index) => {
          const record = failedRecords[index];
          if (importResult.success) {
            updateRecord(record.id, ImportStatus.SUCCESS, null, '重试成功');
          } else {
            updateRecord(record.id, ImportStatus.FAILED, importResult.error || '重试失败', null);
          }
        });
      }

      emit('show-status', `批量重试完成: ${result.successful}/${result.total} 成功`, 'success');
    } else {
      // 如果批量 API 失败，回退到单个重试
      for (const record of failedRecords) {
        await retryRecord(record);
      }
    }
  } catch (error) {
    emit('show-status', `批量重试失败: ${error.message}`, 'error');
    // 将所有记录状态恢复为失败
    failedRecords.forEach(record => {
      updateRecord(record.id, ImportStatus.FAILED, '批量重试失败', null);
    });
  } finally {
    isRetrying.value = false;
  }
};

// 确认清空列表
const confirmClearAll = () => {
  showClearConfirm.value = false;
  clearAllRecords();
  emit('show-status', '已清空导入列表', 'success');
};

// 点击外部关闭 Popover
const handleClickOutside = (event) => {
  if (showClearConfirm.value) {
    // 检查点击是否在 popover 内部
    const popoverConfirm = event.target.closest('.popover-confirm');
    const popoverButton = event.target.closest('.popover-wrapper .btn-action');

    // 如果点击的不是 popover 内部也不是触发按钮，则关闭
    if (!popoverConfirm && !popoverButton) {
      showClearConfirm.value = false;
    }
  }
};

// 监听点击事件
onMounted(() => {
  // 使用 capture 阶段捕获点击事件
  document.addEventListener('click', handleClickOutside, true);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside, true);
});
</script>

<style scoped>
.import-list-modal {
  display: flex;
  flex-direction: column;
  gap: 12px;
  height: 450px;
}

/* 顶部栏 */
.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: 8px 12px;
  background: rgba(248, 250, 252, 0.5);
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.5);
}

/* 统计信息 */
.stats-section {
  display: flex;
  gap: 20px;
  flex: 1;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.stat-label {
  font-size: 11px;
  color: #94a3b8;
  font-weight: 500;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  line-height: 1;
  color: #1e293b;
}

.stat-item.success .stat-value {
  color: #22c55e;
}

.stat-item.failed .stat-value {
  color: #ef4444;
}

.stat-item.pending .stat-value {
  color: #3b82f6;
}

/* 操作按钮 */
.actions-section {
  display: flex;
  gap: 8px;
}

/* Popover 包装器 */
.popover-wrapper {
  position: relative;
}

.btn-action {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  width: 32px;
  height: 32px;
}

.btn-action.primary {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
}

.btn-action.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: scale(1.05);
}

.btn-action.secondary {
  background: rgba(226, 232, 240, 0.5);
  color: #64748b;
  border: 1px solid rgba(203, 213, 225, 0.5);
}

.btn-action.secondary:hover:not(:disabled) {
  background: rgba(203, 213, 225, 0.7);
  transform: scale(1.05);
}

.btn-action:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  transform: none;
}

/* Popover 确认框 */
.popover-confirm {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  width: 200px;
  padding: 12px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(226, 232, 240, 0.8);
  z-index: 1000;
  animation: popoverFadeIn 0.2s ease;
}

@keyframes popoverFadeIn {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.popover-arrow {
  position: absolute;
  top: -6px;
  right: 8px;
  width: 12px;
  height: 12px;
  background: white;
  border-left: 1px solid rgba(226, 232, 240, 0.8);
  border-top: 1px solid rgba(226, 232, 240, 0.8);
  transform: rotate(45deg);
}

.popover-message {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: #334155;
  line-height: 1.5;
}

.popover-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn-cancel,
.btn-confirm {
  padding: 4px 12px;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel {
  background: transparent;
  color: #64748b;
}

.btn-cancel:hover {
  background: rgba(226, 232, 240, 0.5);
}

.btn-confirm {
  background: #3b82f6;
  color: white;
}

.btn-confirm:hover {
  background: #2563eb;
}

/* 记录列表 */
.records-list {
  flex: 1;
  overflow-y: auto;
 
  /* 隐藏滚动条但保留滚动功能 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

/* 隐藏 Webkit 浏览器（Chrome, Safari）的滚动条 */
.records-list::-webkit-scrollbar {
  display: none;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100px 20px;
  text-align: center;
}

.empty-state svg {
  color: #cbd5e1;
  opacity: 0.4;
  margin-bottom: 12px;
}

.empty-state p {
  font-size: 13px;
  color: #94a3b8;
  margin: 0;
}

.records-container {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.record-item {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 10px;
  padding: 4px 12px;
  background: rgba(255, 255, 255, 0.5);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.record-item:hover {
  background: rgba(255, 255, 255, 0.8);
  border-color: rgba(99, 102, 241, 0.3);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.04);
}

/* 成功状态：淡绿色背景 */
.record-item.success {
  background: rgba(34, 197, 94, 0.08);
  border-color: rgba(34, 197, 94, 0.2);
}

.record-item.success:hover {
  background: rgba(34, 197, 94, 0.12);
  border-color: rgba(34, 197, 94, 0.3);
}

/* 失败状态：淡红色背景 */
.record-item.failed {
  background: rgba(239, 68, 68, 0.08);
  border-color: rgba(239, 68, 68, 0.2);
}

.record-item.failed:hover {
  background: rgba(239, 68, 68, 0.12);
  border-color: rgba(239, 68, 68, 0.3);
}

/* 进行中状态：淡蓝色背景 */
.record-item.pending {
  background: rgba(59, 130, 246, 0.08);
  border-color: rgba(59, 130, 246, 0.2);
}

.record-item.pending:hover {
  background: rgba(59, 130, 246, 0.12);
  border-color: rgba(59, 130, 246, 0.3);
}

.record-status {
  flex-shrink: 0;
}

.record-status svg {
  display: block;
}

.record-item.success .record-status svg {
  color: #22c55e;
}

.record-item.failed .record-status svg {
  color: #ef4444;
}

.record-item.pending .record-status svg {
  color: #3b82f6;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.record-content {
  flex: 1;
  min-width: 0;
}

.record-session code {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 11px;
  color: #475569;
  background: rgba(255, 255, 255, 0.6);
  padding: 4px 8px;
  border-radius: 4px;
  letter-spacing: 0.2px;
  border: 1px solid rgba(226, 232, 240, 0.4);
}

.record-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 6px;
  font-size: 11px;
}

.record-time {
  color: #94a3b8;
}

.record-error {
  color: #ef4444;
  font-weight: 500;
}

.record-message {
  color: #22c55e;
  font-weight: 500;
}

.record-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: 1px solid rgba(226, 232, 240, 0.4);
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.6);
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-icon:hover:not(:disabled) {
  background: rgba(99, 102, 241, 0.1);
  border-color: rgba(99, 102, 241, 0.4);
  color: #6366f1;
  transform: scale(1.05);
}

.btn-icon.retry:hover:not(:disabled) {
  background: rgba(34, 197, 94, 0.1);
  border-color: rgba(34, 197, 94, 0.4);
  color: #22c55e;
}

.btn-icon:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  transform: none;
}
</style>

