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
          <h4>粘贴 JSON 或 Session 数据</h4>
          <div class="header-buttons">
            <button
              @click="fillJsonTemplate"
              class="btn-export-control"
              :disabled="isProcessing"
            >
              填入JSON模板
            </button>
            <button
              @click="fillSessionTemplate"
              class="btn-export-control"
              :disabled="isProcessing"
            >
              填入Session模板
            </button>
          </div>
        </div>

        <textarea
          v-model="jsonText"
          placeholder="请粘贴数据..."
          class="json-textarea"
          :disabled="isProcessing"
          rows="5"
          autocomplete="off"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
        ></textarea>
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
                <div class="header-tag-col">标签</div>
                <div class="header-expiry-col">时间</div>
                <div class="header-balance-col">额度</div>
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
                  <div
                    class="account-email-col"
                    :style="getEmailColStyle(token)"
                  >
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
                  <div class="account-tag-col">
                    <span
                      v-if="token.tag_text"
                      class="tag-text"
                      :style="getTagStyle(token)"
                    >
                      {{ token.tag_text }}
                    </span>
                    <span v-else class="tag-empty">-</span>
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
              @click="exportJsonToClipboard"
              class="btn primary"
              :disabled="isProcessing || selectedTokensForExport.size === 0"
            >
              {{ isProcessing ? "导出中..." : "导出JSON" }}
            </button>

            <button
              @click="exportSessionToClipboard"
              class="btn secondary"
              :disabled="isProcessing || selectedTokensForExport.size === 0"
            >
              {{ isProcessing ? "导出中..." : "导出Session" }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
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
  "export-session",
  "request-tag-editor",
]);

// 颜色映射（与 TokenCard.vue 保持一致）
const colorMap = {
  red: "#b91c1c",
  green: "#15803d",
  yellow: "#a16207",
  blue: "#3b82f6",
  black: "#1f2937",
};

// Refs
const activeTab = ref(props.initialTab);
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

// 导入相关方法
const fillJsonTemplate = () => {
  const template = [
    {
      tenant_url: "tenant_url1",
      access_token: "access_token1",
      portal_url: "portal_url1（选填）",
      email_note: "email_note1（选填）",
    },
    {
      tenant_url: "tenant_url2",
      access_token: "access_token2",
      portal_url: "portal_url2（选填）",
      email_note: "email_note2（选填）",
    },
  ];
  jsonText.value = JSON.stringify(template, null, 2);
};

const fillSessionTemplate = () => {
  const template = ["session1", "session2"];
  jsonText.value = JSON.stringify(template, null, 2);
};
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

    // 检测数据类型：JSON 格式（对象数组）或 Session 格式（字符串数组）
    const isSessionFormat = importedData.every(
      (item) => typeof item === "string"
    );
    const isJsonFormat = importedData.every(
      (item) =>
        item && typeof item === "object" && item.tenant_url && item.access_token
    );

    if (isSessionFormat) {
      // Session 格式：在这里批量处理
      await handleSessionImport(importedData);
    } else if (isJsonFormat) {
      // JSON 格式：验证后发送
      emit("import", importedData);
      jsonText.value = "";
    } else {
      // 混合或无效格式
      emit(
        "import-error",
        "导入失败：数据格式不正确。请使用 JSON 格式（包含 tenant_url 和 access_token）或 Session 字符串数组"
      );
    }
  } catch (error) {
    emit("import-error", `格式错误：${error.message}`);
  } finally {
    isProcessing.value = false;
  }
};

// Session 批量导入处理（并行处理）
const handleSessionImport = async (sessions) => {
  if (!Array.isArray(sessions) || sessions.length === 0) {
    emit("import-error", "没有有效的 Session 数据");
    return;
  }

  // 验证所有 Session 非空
  const invalidSessions = [];
  sessions.forEach((session, index) => {
    if (!session || typeof session !== "string" || !session.trim()) {
      invalidSessions.push({
        index: index + 1,
        session: "空 Session",
      });
    }
  });

  // 如果有空的 Session，报错并中止
  if (invalidSessions.length > 0) {
    const errorDetails = invalidSessions
      .map((item) => `第 ${item.index} 个 Session 为空`)
      .join("\n");
    emit("import-error", `Session 不能为空！\n${errorDetails}`);
    return;
  }

  console.log(`开始并行处理 ${sessions.length} 个 Session...`);
  const startTime = Date.now();

  // 并行处理所有 Session
  const promises = sessions.map(async (session, index) => {
    if (!session || typeof session !== "string") {
      return { success: false, error: "无效的 Session 格式" };
    }

    const trimmedSession = session.trim();

    // 验证非空
    if (!trimmedSession) {
      return {
        success: false,
        error: "Session 不能为空",
      };
    }

    try {
      // 调用 Rust 后端处理 Session
      const result = await invoke("add_token_from_session", {
        session: trimmedSession,
      });

      if (result && result.access_token) {
        // 构建 token 对象
        const tokenData = {
          tenant_url: result.tenant_url || "",
          access_token: result.access_token,
          portal_url: result.user_info?.portal_url || "",
          email_note: result.user_info?.email_note || "",
          auth_session: session.trim(), // ✅ 保存原始 Session 字符串
        };

        console.log(
          `Session ${index + 1} 导入成功，已保存 auth_session:`,
          session.trim().substring(0, 50) + "..."
        );
        return { success: true, data: tokenData };
      } else {
        return { success: false, error: "后端返回数据无效" };
      }
    } catch (error) {
      console.error(`处理 Session ${index + 1} 失败:`, error);
      // 映射后端错误标识符到用户友好的提示信息
      let errorMessage = error.message || error || "未知错误";
      if (errorMessage.includes("SESSION_ERROR_OR_ACCOUNT_BANNED")) {
        errorMessage = "Session 无效或账号已被封禁";
      }
      return { success: false, error: errorMessage };
    }
  });

  // 等待所有 Promise 完成
  const results = await Promise.all(promises);

  // 统计结果
  const importedTokens = [];
  let successCount = 0;
  let errorCount = 0;

  results.forEach((result) => {
    if (result.success) {
      importedTokens.push(result.data);
      successCount++;
    } else {
      errorCount++;
    }
  });

  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(2);
  console.log(
    `Session 并行处理完成，耗时: ${duration}秒，成功: ${successCount}，失败: ${errorCount}`
  );

  if (importedTokens.length > 0) {
    // 发送转换后的 token 数据给父组件
    emit("import", importedTokens);
    jsonText.value = "";

    if (errorCount > 0) {
      emit(
        "import-error",
        `成功导入 ${successCount} 个 Session，失败 ${errorCount} 个（耗时 ${duration}秒）`
      );
    }
  } else {
    emit("import-error", `导入失败：${errorCount} 个 Session 处理失败`);
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

const exportJsonToClipboard = async () => {
  if (selectedTokensForExport.value.size === 0) return;

  isProcessing.value = true;
  try {
    const exportData = getSelectedTokensData();
    const jsonString = JSON.stringify(
      exportData.length === 1 ? exportData[0] : exportData,
      null,
      2
    );

    // 获取选中的账号ID列表
    const selectedIds = Array.from(selectedTokensForExport.value);

    emit("export-clipboard", {
      data: jsonString,
      count: selectedTokensForExport.value.size,
      tokenIds: selectedIds, // 传递账号ID列表
    });

    // 清空选中状态，但不关闭弹窗（由父组件决定）
    selectedTokensForExport.value = new Set();
  } catch (error) {
    emit("import-error", `导出失败: ${error}`);
  } finally {
    isProcessing.value = false;
  }
};

const exportSessionToClipboard = async () => {
  if (selectedTokensForExport.value.size === 0) {
    emit("import-error", "请先选择要导出的账号");
    return;
  }

  isProcessing.value = true;
  try {
    const selectedTokens = props.tokens.filter((token) =>
      selectedTokensForExport.value.has(token.id)
    );

    console.log("选中的账号:", selectedTokens);

    // Session 格式：字符串数组
    const sessionData = selectedTokens
      .map((token) => {
        const session = token.auth_session || "";
        console.log(
          `账号 ${token.email_note || token.tenant_url} 的 auth_session:`,
          session
        );
        return session;
      })
      .filter((session) => session); // 过滤掉空字符串

    console.log("提取的 Session 数据:", sessionData);

    if (sessionData.length === 0) {
      emit("import-error", "选中的账号中没有可用的 Session 数据");
      isProcessing.value = false;
      return;
    }

    const jsonString = JSON.stringify(sessionData, null, 2);

    // 获取有 Session 数据的账号ID列表
    const tokenIdsWithSession = selectedTokens
      .filter((token) => token.auth_session)
      .map((token) => token.id);

    // 发出 export-session 事件（用于区分 JSON 和 Session 导出）
    emit("export-session", {
      data: jsonString,
      count: sessionData.length,
      tokenIds: tokenIdsWithSession, // 传递账号ID列表
    });

    // 清空选中状态，但不关闭弹窗（由父组件决定）
    selectedTokensForExport.value = new Set();
  } catch (error) {
    console.error("导出 Session 失败:", error);
    emit("import-error", `导出失败: ${error}`);
  } finally {
    isProcessing.value = false;
  }
};

// 导出辅助函数
const getEmailColStyle = (token) => {
  if (token.tag_color && colorMap[token.tag_color]) {
    return {
      color: colorMap[token.tag_color] + " !important",
    };
  }
  return {};
};

const getTagStyle = (token) => {
  if (token.tag_color && colorMap[token.tag_color]) {
    const color = colorMap[token.tag_color];
    return {
      color: color,
      backgroundColor: `${color}15`, // 15% 透明度
      borderColor: `${color}40`, // 40% 透明度
    };
  }
  return {};
};

const getTokenExpiryDisplay = (token) => {
  // 如果账号已封禁或已过期，显示未知（与 TokenCard 逻辑一致）
  if (token.ban_status === "SUSPENDED" || token.ban_status === "EXPIRED") {
    return "未知";
  }

  // 如果有 portal_url 且有数据，显示实际数据
  if (token.portal_url && token.portal_info && token.portal_info.expiry_date) {
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

  // 其他情况：没有 portal_url 或获取失败，显示未知
  return "未知";
};

const getTokenBalanceDisplay = (token) => {
  // 如果账号已封禁或已过期，显示未知（与 TokenCard 逻辑一致）
  if (token.ban_status === "SUSPENDED" || token.ban_status === "EXPIRED") {
    return "未知";
  }

  // 如果有 portal_url 且有数据，显示实际数据
  if (
    token.portal_url &&
    token.portal_info &&
    typeof token.portal_info.credits_balance === "number"
  ) {
    return token.portal_info.credits_balance.toLocaleString();
  }

  // 其他情况：没有 portal_url 或获取失败，显示未知
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
  min-height: 328px;
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
  grid-template-columns: 20% 20% 2fr 2fr 2fr;
  gap: 12px;
  width: 100%;
  text-align: center;
  align-items: center;
}

.header-email-col,
.header-tag-col,
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

  max-height: 354px;

  padding: 8px 0;
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
  grid-template-columns: 20% 20% 2fr 2fr 2fr;
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

.account-tag-col {
  display: flex;
  justify-content: center;
  align-items: center;
}

.tag-text {
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
  border: 1px solid;
  white-space: nowrap;
}

.tag-empty {
  color: #cbd5e1;
  font-size: 12px;
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
/* @media (max-width: 768px) {
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
} */
</style>
