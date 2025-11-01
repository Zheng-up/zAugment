<template>
  <ModalContainer
    :visible="visible"
    title="API 服务器管理"
    size="medium"
    @close="$emit('close')"
  >
    <div class="api-server-settings">
      <!-- 状态显示 -->
      <div class="status-section">
        <div class="status-item">
          <span class="status-label">服务器状态</span>
          <div class="status-value">
            <span :class="['status-badge', serverStatus.running ? 'running' : 'stopped']">
              <span class="status-dot"></span>
              {{ serverStatus.running ? '运行中' : '已停止' }}
            </span>
          </div>
        </div>

        <div v-if="serverStatus.running" class="status-item">
          <span class="status-label">服务地址</span>
          <div class="status-value">
            <code class="code-text">http://localhost:{{ serverStatus.port }}</code>
            <button @click="copyToClipboard(`http://localhost:${serverStatus.port}`)" class="btn-copy-small" title="复制地址">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
              </svg>
            </button>
          </div>
        </div>
      </div>

      <!-- 控制按钮 -->
      <div class="control-section">
        <button v-if="!serverStatus.running" @click="startServer" :disabled="isLoading" class="btn-control primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z"/>
          </svg>
          {{ isLoading ? '启动中...' : '启动服务器' }}
        </button>

        <button v-if="serverStatus.running" @click="stopServer" :disabled="isLoading" class="btn-control danger">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 6h12v12H6z"/>
          </svg>
          {{ isLoading ? '停止中...' : '停止服务器' }}
        </button>

        <button @click="testConnection" :disabled="isTesting || !serverStatus.running" class="btn-control secondary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
          </svg>
          {{ isTesting ? '测试中...' : '测试连接' }}
        </button>
        <!-- 测试结果 -->
        <div v-if="testResult" :class="['test-result', testResult.success ? 'success' : 'error']">
          <svg v-if="testResult.success" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"/>
          </svg>
          <span>{{ testResult.message }}</span>
        </div>
      </div>



<!-- API 端点列表 -->
<div class="endpoints-list">
    <!-- 健康检查端点 -->
    <div class="endpoint-item">
      <div class="endpoint-header">
        <span class="method-badge get">GET</span>
        <code class="endpoint-path">/api/health</code>
      </div>
      <p class="endpoint-description">健康检查端点，返回服务器状态和版本信息</p>
      <div class="endpoint-example">
        <div class="example-header">
          <span>示例</span>
          <button @click="copyToClipboard(healthExample)" class="btn-copy-example">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
            </svg>
            复制
          </button>
        </div>
        <pre><code>{{ healthExample }}</code></pre>
      </div>
    </div>

    <!-- 单个 Session 导入端点 -->
    <div class="endpoint-item">
      <div class="endpoint-header">
        <span class="method-badge post">POST</span>
        <code class="endpoint-path">/api/import/session</code>
      </div>
      <p class="endpoint-description">导入单个 Session，支持 detailed_response 参数控制返回详细信息</p>
      <div class="endpoint-example">
        <div class="example-header">
          <span>示例</span>
          <button @click="copyToClipboard(sessionExample)" class="btn-copy-example">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
            </svg>
            复制
          </button>
        </div>
        <pre><code>{{ sessionExample }}</code></pre>
      </div>
    </div>

    <!-- 批量 Session 导入端点 -->
    <div class="endpoint-item">
      <div class="endpoint-header">
        <span class="method-badge post">POST</span>
        <code class="endpoint-path">/api/import/sessions</code>
      </div>
      <p class="endpoint-description">批量导入 Session，最多支持 5 个并发导入</p>
      <div class="endpoint-example">
        <div class="example-header">
          <span>示例</span>
          <button @click="copyToClipboard(sessionsExample)" class="btn-copy-example">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
            </svg>
            复制
          </button>
        </div>
        <pre><code>{{ sessionsExample }}</code></pre>
      </div>
    </div>
  </div>
      </div>

     
    
  </ModalContainer>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ModalContainer from './ModalContainer.vue';

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
});

// Emits
const emit = defineEmits(['close', 'show-status']);

// 状态管理
const serverStatus = ref({
  running: false,
  port: 8766
});

const isLoading = ref(false);
const isTesting = ref(false);
const testResult = ref(null);

// 定时器
let statusCheckInterval = null;

// API 示例代码
const healthExample = `curl http://localhost:8766/api/health`;

const sessionExample = `curl -X POST http://localhost:8766/api/import/session \\
  -H "Content-Type: application/json" \\
  -d '{
    "session": "your_session_string_here",
    "detailed_response": false
  }'`;

const sessionsExample = `curl -X POST http://localhost:8766/api/import/sessions \\
  -H "Content-Type: application/json" \\
  -d '{
    "sessions": [
      "session_1",
      "session_2",
      "session_3"
    ],
    "detailed_response": false
  }'`;

// 方法
const checkServerStatus = async () => {
  try {
    const status = await invoke('get_api_server_status');
    serverStatus.value = status;
  } catch (error) {
    console.error('Failed to check server status:', error);
  }
};

const startServer = async () => {
  isLoading.value = true;
  testResult.value = null;
  try {
    await invoke('start_api_server');
    await checkServerStatus();
    emit('show-status', 'API 服务器已启动', 'success');
  } catch (error) {
    emit('show-status', `启动失败: ${error}`, 'error');
  } finally {
    isLoading.value = false;
  }
};

const stopServer = async () => {
  isLoading.value = true;
  testResult.value = null;
  try {
    await invoke('stop_api_server');
    await checkServerStatus();
    emit('show-status', 'API 服务器已停止', 'success');
  } catch (error) {
    emit('show-status', `停止失败: ${error}`, 'error');
  } finally {
    isLoading.value = false;
  }
};

const testConnection = async () => {
  isTesting.value = true;
  testResult.value = null;
  try {
    const response = await fetch(`http://localhost:${serverStatus.value.port}/api/health`);
    const data = await response.json();
    testResult.value = {
      success: true,
      message: `连接成功！服务器版本: ${data.version}`
    };
  } catch (error) {
    testResult.value = {
      success: false,
      message: `连接失败: ${error.message}`
    };
  } finally {
    isTesting.value = false;
  }
};

const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
    emit('show-status', '已复制到剪贴板', 'success');
  } catch (error) {
    emit('show-status', '复制失败', 'error');
  }
};

// 生命周期
onMounted(async () => {
  await checkServerStatus();
  // 每 3 秒检查一次服务器状态
  statusCheckInterval = setInterval(checkServerStatus, 3000);
});

onUnmounted(() => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval);
  }
});
</script>

<style scoped>
.api-server-settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
  max-height: 60vh;
  padding: 10px 0;
}

/* 分隔线 */
.divider {
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(226, 232, 240, 0.6), transparent);
  margin: 8px 0;
}

/* 状态部分 */
.status-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
 
}

.status-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: rgba(248, 250, 252, 0.6);
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.4);
}

.status-label {
  font-size: 14px;
  font-weight: 500;
  color: #475569;
}

.status-value {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
}

.status-badge.running {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.status-badge.stopped {
  background: rgba(148, 163, 184, 0.1);
  color: #64748b;
  border: 1px solid rgba(148, 163, 184, 0.2);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.code-text {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  padding: 4px 8px;
  background: rgba(100, 116, 139, 0.1);
  border-radius: 4px;
  color: #3b82f6;
}

.btn-copy-small {
  padding: 4px;
  background: none;
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 4px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy-small:hover {
  background: rgba(59, 130, 246, 0.05);
  border-color: #3b82f6;
  color: #3b82f6;
}

/* 控制按钮 */
.control-section {
  display: flex;
  gap: 12px;
  align-items: center;
}

.btn-control {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
  white-space: nowrap;
  min-width: 100px;
}

.btn-control.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
}

.btn-control.primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.btn-control.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
}

.btn-control.danger:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

.btn-control.secondary {
  background: white;
  color: #475569;
  border: 1px solid rgba(226, 232, 240, 0.6);
}

.btn-control.secondary:hover:not(:disabled) {
  background: rgba(59, 130, 246, 0.05);
  border-color: #3b82f6;
  color: #3b82f6;
}

.btn-control:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 测试结果 */
.test-result {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
}

.test-result.success {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.test-result.error {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
  border: 1px solid rgba(239, 68, 68, 0.2);
}

/* 端点列表 */
.endpoints-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.endpoint-item {
  padding: 20px;
  background: rgba(248, 250, 252, 0.6);
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.4);
}

.endpoint-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.method-badge {
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.method-badge.get {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.method-badge.post {
  background: rgba(59, 130, 246, 0.1);
  color: #2563eb;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.endpoint-path {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  padding: 4px 8px;
  background: rgba(100, 116, 139, 0.1);
  border-radius: 4px;
  color: #475569;
}

.endpoint-description {
  color: #64748b;
  font-size: 14px;
  line-height: 1.6;
  margin: 0 0 16px 0;
}

.endpoint-example {
  background: rgba(15, 23, 42, 0.95);
  border-radius: 6px;
  overflow: hidden;
  border: 1px solid rgba(100, 116, 139, 0.2);
}

.example-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(100, 116, 139, 0.2);
}

.example-header span {
  font-size: 12px;
  color: #94a3b8;
  font-weight: 500;
}

.btn-copy-example {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 4px;
  color: #60a5fa;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy-example:hover {
  background: rgba(59, 130, 246, 0.2);
  border-color: #60a5fa;
}

.endpoint-example pre {
  margin: 0;
  padding: 12px;
  overflow-x: auto;
}

.endpoint-example code {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  color: #e2e8f0;
}

</style>

