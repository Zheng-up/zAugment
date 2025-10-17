<template>
  <ModalContainer
    :visible="visible"
    :tabs="modalTabs"
    :active-tab="activeTab"
    size="medium"
    @close="handleClose"
    @tab-change="handleTabChange"
  >
    <!-- 导入内容 -->
    <div v-if="activeTab === 'import'" class="tab-content">
      <div class="import-section">
        <div class="section-header">
          <h4>粘贴JSON数据或导入</h4>
          <div class="header-buttons">
            <button
              @click="handleFileImport"
              class="btn-export-control"
              :disabled="isProcessing"
            >
              本地导入
            </button>
          </div>
        </div>

        <textarea
          v-model="jsonText"
          placeholder="请粘贴JSON格式的账号数据..."
          class="json-textarea"
          :disabled="isProcessing"
          rows="5"
          autocomplete="off"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
        ></textarea>

        <!-- 数据格式说明 -->
        <div class="format-info">
          <h5>数据格式示例：</h5>
          <div class="format-example">
            <pre><code>{
    "tenant_url": "租户URL",
    "access_token": "Token",
    "portal_url": "View usage URL"
}</code></pre>
          </div>
          <div class="format-notes">
            <p><strong>必需字段：</strong> tenant_url, access_token</p>
            <p>
              <strong>支持格式：</strong> JSON数组
              <code>[{},{},{}]</code> 或对象序列 <code>{},{},{}</code>
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- 导出内容 -->
    <div v-if="activeTab === 'export'" class="tab-content">
      <div class="export-section">
        <!-- 账号列表 -->
        <div class="export-accounts-list">
          <div class="export-table-container">
            <!-- 表头 -->
            <div class="export-table-header">
              <div class="header-info-grid">
                <div class="header-email-col">邮箱/租户</div>
                <div class="header-expiry-col">剩余时间</div>
                <div class="header-balance-col">剩余额度</div>
                <div class="header-status-col">状态</div>
              </div>
            </div>

            <div class="export-accounts-container">
              <div
                v-for="token in tokens"
                :key="token.id"
                class="export-account-item"
                :class="{ selected: selectedTokensForExport.has(token.id) }"
                @click="toggleTokenSelection(token.id)"
                :disabled="isProcessing"
              >
                <div class="account-info-grid">
                  <div class="account-email-col">
                    <span class="account-email">
                      {{ token.email_note || "无邮箱号" }}
                    </span>
                    <span v-if="!token.email_note" class="no-email-hint">
                      {{
                        token.tenant_url.length > 25
                          ? token.tenant_url.slice(0, 25) + "..."
                          : token.tenant_url
                      }}
                    </span>
                  </div>
                  <div class="account-expiry-col">
                    {{ getTokenExpiryDisplay(token) }}
                  </div>
                  <div class="account-balance-col">
                    {{ getTokenBalanceDisplay(token) }}
                  </div>
                  <div class="account-status-col">
                    <span
                      class="account-status"
                      :class="getTokenStatusClass(token)"
                    >
                      {{ getTokenStatusDisplay(token) }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <!-- 导入按钮 -->
        <button
          v-if="activeTab === 'import'"
          @click="handleImportFromText"
          class="btn primary"
          :disabled="isProcessing || !jsonText.trim()"
        >
          <span v-if="isProcessing" class="loading-spinner"></span>
          {{ isProcessing ? "导入中..." : "开始导入" }}
        </button>

        <!-- 导出按钮 -->
        <div v-if="activeTab === 'export'" class="export-footer-layout">
          <div class="export-left">
            <div class="export-controls-row">
              <button
                @click="toggleSelectAll"
                class="btn-export-control"
                :disabled="isProcessing"
              >
                {{ isAllSelected ? "取消" : "全选" }}
              </button>
            </div>
            <div class="export-info-row">
              <span class="selected-count">
                已选择 {{ selectedTokensForExport.size }} /
                {{ tokens.length }} 个账号
              </span>
            </div>
          </div>
          <div class="export-actions">
            <button
              @click="exportToClipboard"
              class="btn primary"
              :disabled="isProcessing || selectedTokensForExport.size === 0"
            >
              {{ isProcessing ? "导出中..." : "导出到剪贴板" }}
            </button>

            <button
              @click="exportToFile"
              class="btn secondary"
              :disabled="isProcessing || selectedTokensForExport.size === 0"
            >
              {{ isProcessing ? "导出中..." : "导出到本地" }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </ModalContainer>

  <!-- 隐藏的文件输入 -->
  <input
    ref="fileInput"
    type="file"
    accept=".json"
    style="display: none"
    @change="handleFileSelected"
    autocomplete="off"
  />
</template>

<script setup>
import { ref, computed } from "vue";
import ModalContainer from "./ModalContainer.vue";

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  tokens: {
    type: Array,
    default: () => [],
  },
  initialTab: {
    type: String,
    default: "import",
    validator: (value) => ["import", "export"].includes(value),
  },
});

// Emits
const emit = defineEmits([
  "close",
  "import",
  "import-error",
  "export-clipboard",
  "export-file",
]);

// Refs
const activeTab = ref(props.initialTab);
const fileInput = ref(null);
const jsonText = ref("");
const isProcessing = ref(false);
const selectedTokensForExport = ref(new Set());

// Computed
const modalTabs = computed(() => [
  { key: "import", title: "导入账号" },
  { key: "export", title: "导出账号" },
]);

const isAllSelected = computed(() => {
  return (
    props.tokens.length > 0 &&
    selectedTokensForExport.value.size === props.tokens.length
  );
});

// Methods
const handleTabChange = (tabKey) => {
  if (!isProcessing.value) {
    activeTab.value = tabKey;
  }
};

const handleClose = () => {
  if (!isProcessing.value) {
    jsonText.value = "";
    selectedTokensForExport.value = new Set();
    activeTab.value = props.initialTab;
    emit("close");
  }
};

const handleFileImport = () => {
  if (fileInput.value) {
    fileInput.value.click();
  }
};

const handleFileSelected = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  try {
    const text = await file.text();
    jsonText.value = text;
  } catch (error) {
    console.error("读取文件失败:", error);
  } finally {
    if (fileInput.value) {
      fileInput.value.value = "";
    }
  }
};

// 导入相关方法
const handleImportFromText = async () => {
  if (!jsonText.value.trim()) return;

  isProcessing.value = true;
  try {
    let importedData;
    const trimmedText = jsonText.value.trim();

    // JSON解析逻辑
    try {
      if (trimmedText.startsWith("[") && trimmedText.endsWith("]")) {
        importedData = JSON.parse(trimmedText);
      } else if (trimmedText.startsWith("{")) {
        const endsWithObject =
          trimmedText.endsWith("}") || /}\s*,?\s*$/.test(trimmedText);

        if (endsWithObject) {
          if (trimmedText.includes("},{") || /}\s*,\s*{/.test(trimmedText)) {
            const cleanedForArray = trimmedText.replace(/,\s*$/, "");
            const wrappedText = `[${cleanedForArray}]`;
            importedData = JSON.parse(wrappedText);
          } else {
            const cleanedSingle = trimmedText.replace(/,\s*$/, "");
            const singleObject = JSON.parse(cleanedSingle);
            importedData = [singleObject];
          }
        } else {
          throw new Error(
            "数据必须以 { 开头并以 } 结尾，或者是 JSON 数组格式 [...]"
          );
        }
      } else {
        throw new Error(
          "数据必须以 { 开头并以 } 结尾，或者是 JSON 数组格式 [...]"
        );
      }
    } catch (parseError) {
      // 尝试清理特殊字符
      const cleanedText = trimmedText
        .replace(/[\u200B-\u200D\uFEFF]/g, "")
        .replace(/[""]/g, '"')
        .replace(/['']/g, "'");

      if (cleanedText !== trimmedText) {
        try {
          if (
            cleanedText.startsWith("{") &&
            cleanedText.endsWith("}") &&
            cleanedText.includes("},{")
          ) {
            const wrappedCleanText = `[${cleanedText}]`;
            importedData = JSON.parse(wrappedCleanText);
          } else if (cleanedText.startsWith("{") && cleanedText.endsWith("}")) {
            const singleObject = JSON.parse(cleanedText);
            importedData = [singleObject];
          } else if (cleanedText.startsWith("[") && cleanedText.endsWith("]")) {
            importedData = JSON.parse(cleanedText);
          } else {
            importedData = JSON.parse(cleanedText);
          }
        } catch (secondError) {
          throw parseError;
        }
      } else {
        throw parseError;
      }
    }

    if (!Array.isArray(importedData)) {
      importedData = [importedData];
    }

    // 验证数据
    const invalidItems = importedData.filter(
      (item) =>
        !item ||
        typeof item !== "object" ||
        !item.tenant_url ||
        !item.access_token
    );

    if (invalidItems.length > 0) {
      emit(
        "import-error",
        `导入失败：发现 ${invalidItems.length} 个无效的账号数据，请检查必需字段 tenant_url 和 access_token`
      );
      return;
    }

    emit("import", importedData);
    jsonText.value = "";
  } catch (error) {
    emit("import-error", `JSON格式错误：${error.message}`);
  } finally {
    isProcessing.value = false;
  }
};

// 导出相关方法
const toggleTokenSelection = (tokenId) => {
  if (selectedTokensForExport.value.has(tokenId)) {
    selectedTokensForExport.value.delete(tokenId);
  } else {
    selectedTokensForExport.value.add(tokenId);
  }
};

const toggleSelectAll = () => {
  if (isAllSelected.value) {
    selectedTokensForExport.value = new Set();
  } else {
    selectedTokensForExport.value = new Set(
      props.tokens.map((token) => token.id)
    );
  }
};

const getSelectedTokensData = () => {
  const selectedTokens = props.tokens.filter((token) =>
    selectedTokensForExport.value.has(token.id)
  );

  return selectedTokens.map((token) => ({
    tenant_url: token.tenant_url,
    access_token: token.access_token,
    portal_url: token.portal_url || "",
    email_note: token.email_note || "",
  }));
};

const exportToClipboard = async () => {
  if (selectedTokensForExport.value.size === 0) return;

  isProcessing.value = true;
  try {
    const exportData = getSelectedTokensData();
    const jsonString = JSON.stringify(
      exportData.length === 1 ? exportData[0] : exportData,
      null,
      2
    );

    emit("export-clipboard", {
      data: jsonString,
      count: selectedTokensForExport.value.size,
    });

    // 清空选中状态，但不关闭弹窗（由父组件决定）
    selectedTokensForExport.value = new Set();
  } catch (error) {
    emit("import-error", `导出失败: ${error}`);
  } finally {
    isProcessing.value = false;
  }
};

const exportToFile = async () => {
  if (selectedTokensForExport.value.size === 0) return;

  isProcessing.value = true;
  try {
    const exportData = getSelectedTokensData();
    const jsonString = JSON.stringify(
      exportData.length === 1 ? exportData[0] : exportData,
      null,
      2
    );

    emit("export-file", {
      data: jsonString,
      count: selectedTokensForExport.value.size,
    });

    // 清空选中状态，但不关闭弹窗（由父组件决定）
    selectedTokensForExport.value = new Set();
  } catch (error) {
    emit("import-error", `导出失败: ${error}`);
  } finally {
    isProcessing.value = false;
  }
};

// 导出辅助函数
const getTokenExpiryDisplay = (token) => {
  if (token.portal_info && token.portal_info.expiry_date) {
    const expiryDate = new Date(token.portal_info.expiry_date);
    const now = new Date();
    const diffTime = expiryDate - now;

    if (diffTime < 0) return "已过期";

    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
    const diffHours = Math.floor(
      (diffTime % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)
    );
    const diffMinutes = Math.floor((diffTime % (1000 * 60 * 60)) / (1000 * 60));

    if (diffDays === 0 && diffHours === 0) {
      return `${diffMinutes}分`;
    } else if (diffDays === 0) {
      return `${diffHours}时${diffMinutes}分`;
    } else {
      return `${diffDays}天${diffHours}时${diffMinutes}分`;
    }
  }
  return "未知";
};

const getTokenBalanceDisplay = (token) => {
  if (
    token.portal_info &&
    typeof token.portal_info.credits_balance === "number"
  ) {
    return token.portal_info.credits_balance.toLocaleString();
  }
  return "未知";
};

const getTokenStatusDisplay = (token) => {
  // 使用与主界面一致的综合状态判断逻辑
  const comprehensiveStatus = getComprehensiveStatus(
    token.ban_status,
    token.portal_info
  );

  switch (comprehensiveStatus) {
    case "SUSPENDED":
      return "已封禁";
    case "INVALID_TOKEN":
      return "Token失效";
    case "EXPIRED":
      return "已过期";
    case "ACTIVE":
      return "正常";
    default:
      return "正常"; // 与主界面保持一致，默认显示正常
  }
};

// 综合状态判断：结合API状态和Portal信息（与TokenCard.vue保持一致）
const getComprehensiveStatus = (apiStatus, portalData) => {
  // 优先级1：API检测到的明确状态（包括EXPIRED）
  if (
    apiStatus === "SUSPENDED" ||
    apiStatus === "INVALID_TOKEN" ||
    apiStatus === "EXPIRED"
  ) {
    return apiStatus;
  }

  // 优先级2：API状态为ACTIVE，直接返回
  if (apiStatus === "ACTIVE") {
    return "ACTIVE";
  }

  // 优先级3：API状态未知，检查Portal数据
  if (portalData) {
    // 检查额度是否确实为0（严格等于0）
    if (portalData.credits_balance === 0) {
      return "EXPIRED";
    }

    // 检查是否明确过期
    if (portalData.expiry_date) {
      const now = new Date();
      const expiry = new Date(portalData.expiry_date);
      if (expiry <= now) {
        return "EXPIRED";
      }
    }

    // 如果Portal数据显示有额度且未过期，认为是正常的
    if (portalData.credits_balance > 0) {
      return "ACTIVE";
    }
  }

  // 优先级4：无法确定状态时，返回API状态或默认为ACTIVE
  return apiStatus || "ACTIVE";
};

const getTokenStatusClass = (token) => {
  // 使用与主界面一致的综合状态判断逻辑
  const comprehensiveStatus = getComprehensiveStatus(
    token.ban_status,
    token.portal_info
  );

  switch (comprehensiveStatus) {
    case "SUSPENDED":
      return "status-banned";
    case "INVALID_TOKEN":
      return "status-invalid";
    case "EXPIRED":
      return "status-expired";
    case "ACTIVE":
      return "status-active";
    default:
      return "status-active"; // 与主界面保持一致，默认显示正常样式
  }
};
</script>

<style scoped>
/* Tab 内容样式 */
.tab-content {
  min-height: 300px;
}

/* 导入部分样式 */
.import-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.section-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}

.header-buttons {
  display: flex;
  gap: 8px;
}

.json-textarea {
  width: 100%;
  min-height: 120px;
  padding: 14px 16px;
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 12px;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: none; /* 禁用高度调节 */
  overflow-y: auto; /* 允许垂直滚动 */
  transition: all 0.3s ease;
  background: rgba(248, 250, 252, 0.5);
  box-sizing: border-box;
  /* 隐藏滚动条但保留滚动功能 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

/* 隐藏 Webkit 浏览器（Chrome, Safari）的滚动条 */
.json-textarea::-webkit-scrollbar {
  display: none;
}

.json-textarea:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.5);
  box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.1);
  background: rgba(255, 255, 255, 0.9);
}

.json-textarea:disabled {
  background: rgba(241, 245, 249, 0.7);
  color: #64748b;
  cursor: not-allowed;
  opacity: 0.6;
}

.format-info {
  background: rgba(248, 250, 252, 0.8);
  border: 2px solid rgba(226, 232, 240, 0.3);
  border-radius: 12px;
  padding: 20px;
}

.format-info h5 {
  margin: 0 0 16px 0;
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
}

.format-example pre {
  margin: 0;
  padding: 16px;
  background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
  border-radius: 8px;
  overflow-x: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.format-example code {
  color: #e2e8f0;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 13px;
  line-height: 1.5;
}

.format-notes {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 16px;
}

.format-notes p {
  margin: 0;
  font-size: 13px;
  color: #64748b;
  line-height: 1.4;
}

/* 导出部分样式 */
.export-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.export-accounts-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.export-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 0;
  background: rgba(248, 250, 252, 0.5);
  border-radius: 8px;
}

.export-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-export-control {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  position: relative;
  overflow: hidden;
}

.btn-export-control::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.5s;
}

.btn-export-control:hover::before {
  left: 100%;
}

.btn-export-control:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.btn-export-control:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.selected-count {
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
}
.export-table-container {
  display: flex;
  flex-direction: column;
  gap: 0;
}
/* 表头样式 */
.export-table-header {
  background: rgba(209, 230, 250, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.3);
  border-bottom: none;
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 4px;
}

.header-info-grid {
  display: grid;
  grid-template-columns: 33% 1fr 1fr 1fr;
  gap: 12px;
  width: 100%;
  text-align: center;
  align-items: center;
}

.header-email-col,
.header-expiry-col,
.header-balance-col,
.header-status-col {
  font-size: 12px;
  font-weight: 600;
  color: #64748b;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.header-expiry-col,
.header-balance-col,
.header-status-col {
  text-align: center;
}

.export-accounts-container {
  display: flex;
  flex-direction: column;
  gap: 0;
  /* 不要 */
  /* max-height: 350px;
  overflow-y: auto; */
  /* padding: 8px; */
  border: 1px solid rgba(226, 232, 240, 0.3);
  border-top: none;
  border-radius: 0 0 8px 8px;
  background: rgba(255, 255, 255, 0.3);
}

.export-account-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.2);
  background: rgba(242, 242, 242, 0.609);
  transition: all 0.15s ease;
  cursor: pointer;
  user-select: none;
  border-radius: 6px;
  margin-bottom: 8px;
  border: 1px solid rgba(226, 232, 240, 0.2);
}

.export-account-item:hover:not(:disabled):not(.selected) {
  background: rgba(59, 130, 246, 0.08);
  border-color: rgba(59, 130, 246, 0.2);
}

.export-account-item.selected {
  background: rgba(59, 130, 246, 0.1);
  border-color: #b5d1ffbe;
  box-shadow: 0 0 0 1px rgba(102, 158, 249, 0.2);
}

.export-account-item:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.account-info-grid {
  display: grid;
  grid-template-columns: 33% 1fr 1fr 1fr;
  gap: 12px;
  width: 100%;
  align-items: center;
}

.account-email-col {
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow: hidden;
}

.account-email {
  font-weight: 500;
  color: #1f2937;
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.no-email-hint {
  font-weight: 400;
  color: #6b7280;
  font-size: 11px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.account-expiry-col,
.account-balance-col {
  color: #6b7280;
  font-size: 12px;
  text-align: center;
  white-space: nowrap;
}

.account-status-col {
  text-align: center;
}

.account-status {
  font-weight: 500;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
}

.account-status.status-active {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.account-status.status-expired {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.account-status.status-banned {
  background: rgba(239, 68, 68, 0.2);
  color: #dc2626;
}

.account-status.status-invalid {
  background: rgba(245, 158, 11, 0.1);
  color: #d97706;
}

.account-status.status-unknown {
  background: rgba(156, 163, 175, 0.1);
  color: #6b7280;
}

/* 底部按钮样式 */
.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  align-items: center;
}

/* 底部布局 */
.export-footer-layout {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  flex: 1;
}

.export-left {
  display: flex;
  align-items: center;
  height: 100%;
}

.export-controls-row {
  display: flex;
  align-items: center;
  margin-right: 10px;
}

.export-info-row {
  display: flex;
  align-items: center;
}

.export-actions {
  display: flex;
  gap: 12px;
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s ease;
  min-height: 44px;
  position: relative;
  overflow: hidden;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

.btn.secondary {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.9) 0%,
    rgba(241, 245, 249, 0.8) 100%
  );
  color: #64748b;
  border: 1px solid rgba(226, 232, 240, 0.5);
  position: relative;
  overflow: hidden;
}

.btn.secondary::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.5s;
}

.btn.secondary:hover::before {
  left: 100%;
}

.btn.secondary:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(241, 245, 249, 0.95) 0%,
    rgba(226, 232, 240, 0.9) 100%
  );
  border-color: rgba(203, 213, 225, 0.7);
  color: #475569;
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.btn.primary {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  border: none;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  position: relative;
  overflow: hidden;
}

.btn.primary::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.5s;
}

.btn.primary:hover:not(:disabled)::before {
  left: 100%;
}

.btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tab-content {
    min-height: 250px;
  }

  .export-controls {
    flex-wrap: wrap;
    gap: 8px;
  }

  .header-info-grid {
    grid-template-columns: 1fr;
    gap: 4px;
  }

  .header-email-col,
  .header-expiry-col,
  .header-balance-col,
  .header-status-col {
    text-align: left;
    font-size: 11px;
  }

  .modal-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .export-footer-layout {
    flex-direction: column;
    align-items: stretch;
    gap: 16px;
  }

  .export-left {
    order: 2;
  }

  .export-actions {
    flex-direction: column;
    order: 1;
  }

  .btn {
    justify-content: center;
  }

  .account-info-grid {
    grid-template-columns: 1fr;
    gap: 4px;
  }

  .account-email-col,
  .account-expiry-col,
  .account-balance-col,
  .account-status-col {
    text-align: left;
  }

  .account-email {
    font-size: 12px;
  }

  .account-expiry-col,
  .account-balance-col {
    font-size: 11px;
  }

  .no-email-hint {
    font-size: 10px;
  }
}
</style>
