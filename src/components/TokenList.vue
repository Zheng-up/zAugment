<template>
  <div class="token-list-container">
    <!-- 统一的账号管理卡片 -->
    <div class="unified-account-card">
      <!-- 卡片头部 - 包含说明信息 -->
      <!-- <div class="card-header">
        <div class="header-main">
          <div class="section-icon">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
              />
            </svg>
          </div>
          <div class="header-text">
            <h3>Augment账号管理</h3>
            <p class="header-description">
              管理您的Augment账号信息，包括租户URL、Token等。
            </p>
          </div>
        </div>
      </div> -->

      <!-- 分割线 -->
      <div v-if="tokens.length > 0" class="card-divider"></div>

      <!-- 内容区域 -->
      <div class="card-content">
        <!-- Empty State -->
        <div v-if="tokens.length === 0 && !isLoading" class="empty-state">
          <div class="empty-state-content">
            <div class="empty-icon">
              <svg
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
                />
              </svg>
            </div>
            <h4>还没有账号</h4>
            <p>
              点击右上角"新增账号"按钮添加您的第一个Augment账号，或在注册页面获取新账号。
            </p>
            <div class="empty-actions">
              <button @click="$emit('add-new-token')" class="btn-empty primary">
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                </svg>
                立即新增
              </button>
            </div>
          </div>
        </div>

        <!-- Loading State -->
        <div v-if="isLoading" class="loading-state">
          <div class="spinner"></div>
          <p>正在加载账号...</p>
        </div>

        <!-- Token List -->
        <div v-if="tokens.length > 0" class="token-list">
          <div class="token-grid">
            <TokenCard
              v-for="token in tokens"
              :key="token.id"
              :ref="(el) => setTokenCardRef(el, token.id)"
              :token="token"
              :is-batch-checking="isRefreshing"
              :statusThresholds="statusThresholds"
              @delete="$emit('delete-token', $event)"
              @copy-success="handleCopySuccess"
              @edit="$emit('edit-token', $event)"
              @token-updated="handleTokenUpdated"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TokenCard from "./TokenCard.vue";

// Props
const props = defineProps({
  tokens: {
    type: Array,
    default: () => [],
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
  hasUnsavedChanges: {
    type: Boolean,
    default: false,
  },
  shouldAutoCheck: {
    type: Boolean,
    default: false,
  },
  statusThresholds: {
    type: Object,
    default: () => ({
      time: { warning: 10, safe: 20 },
      balance: { warning: 1000, safe: 2000 },
    }),
  },
});

// Emits
const emit = defineEmits([
  "save-changes",
  "refresh",
  "delete-token",
  "edit-token",
  "copy-success",
  "token-updated",
  "add-new-token",
  "auto-check-completed",
  "check-all-status",
]);

// Token card refs for portal refresh
const tokenCardRefs = ref({});
const isRefreshing = ref(false);

const setTokenCardRef = (el, tokenId) => {
  if (el) {
    tokenCardRefs.value[tokenId] = el;
  } else {
    delete tokenCardRefs.value[tokenId];
  }
};

const handleTokenUpdated = (updatedToken) => {
  emit("token-updated", updatedToken);
};

const handleCopySuccess = (message, type) => {
  emit("copy-success", message, type);
};

// 检查所有Token的账号状态
const checkAllAccountStatus = async () => {
  if (props.tokens.length === 0) {
    return { success: true, message: "没有需要检查的账号" };
  }

  isRefreshing.value = true;

  try {
    // 准备批量检测的数据，过滤掉标记为跳过检测的账号
    const tokensToCheck = props.tokens.filter((token) => !token.skip_check);

    if (tokensToCheck.length === 0) {
      emit("copy-success", "所有账号都已禁用检测", "info");
      return { success: true, message: "所有账号都已禁用检测" };
    }

    const tokenInfos = tokensToCheck.map((token) => ({
      id: token.id,
      access_token: token.access_token,
      tenant_url: token.tenant_url,
      portal_url: token.portal_url || null,
      auth_session: token.auth_session || null,
    }));

    // 单次批量API调用
    const results = await invoke("batch_check_tokens_status", {
      tokens: tokenInfos,
    });

    // 批量更新tokens状态并获取统计信息
    const stats = updateTokensFromResults(results);

    // 通知父组件保存
    emit("token-updated");

    // 显示检查完成提示（包含详细统计）
    let message = `刷新完成：刷新状态 ${stats.statusSuccess} 个`;
    if (stats.statusFailed > 0) {
      message += `，失败 ${stats.statusFailed} 个`;
    }

    // 添加 Portal 信息统计
    if (stats.portalSuccess > 0 || stats.portalFailed > 0) {
      message += ` | 刷新信息 ${stats.portalSuccess} 个`;
      if (stats.portalFailed > 0) {
        message += `，失败 ${stats.portalFailed} 个`;
      }
    }

    // 根据失败数量决定提示类型
    const messageType = stats.statusFailed > 0 ? "warning" : "success";
    emit("copy-success", message, messageType);

    return { success: true, message: "批量检查完成" };
  } catch (error) {
    console.error("Batch check error:", error);
    emit("copy-success", `批量检查失败: ${error}`, "error");
    return {
      success: false,
      message: `批量检查失败: ${error}`,
    };
  } finally {
    isRefreshing.value = false;
  }
};

// 根据批量检测结果更新tokens状态
const updateTokensFromResults = (results) => {
  // 统计信息
  let statusSuccess = 0;
  let statusFailed = 0;
  let portalSuccess = 0;
  let portalFailed = 0;

  results.forEach((result) => {
    const token = props.tokens.find((t) => t.id === result.token_id);
    if (token) {
      const statusResult = result.status_result;

      // 统计状态检测结果
      if (statusResult.status && statusResult.status !== "ERROR") {
        statusSuccess++;
      } else {
        statusFailed++;
      }

      // 始终更新 access_token、tenant_url 和 portal_url (如果 token 被刷新,这里会是新值)
      token.access_token = result.access_token;
      token.tenant_url = result.tenant_url;

      // 如果后端返回了新的 portal_url，更新它
      if (result.portal_url) {
        token.portal_url = result.portal_url;
        console.log(`Updated token ${token.id} portal_url:`, result.portal_url);
      }

      // 更新ban_status
      token.ban_status = statusResult.status;

      // 自动禁用封禁或过期的账号检测
      if (
        (statusResult.status === "SUSPENDED" ||
          statusResult.status === "EXPIRED") &&
        !token.skip_check
      ) {
        token.skip_check = true;
        // 显示通知
        const autoDisableMsg =
          statusResult.status === "SUSPENDED"
            ? "账号已封禁，已自动禁用检测"
            : "账号已过期，已自动禁用检测";
        emit("copy-success", autoDisableMsg, "error");
      }

      // 更新 suspensions 信息（如果有）
      if (result.suspensions) {
        token.suspensions = result.suspensions;
        console.log(
          `Updated suspensions for token ${token.id}:`,
          result.suspensions
        );
      }

      // 更新Portal信息（如果有）
      if (result.portal_info) {
        token.portal_info = {
          credits_balance: result.portal_info.credits_balance,
          expiry_date: result.portal_info.expiry_date,
        };
        console.log(
          `Updated token ${token.id} portal info:`,
          token.portal_info
        );
        portalSuccess++;
      } else if (result.portal_error) {
        // 批量刷新失败时清空 portal_info
        token.portal_info = null;
        console.warn(
          `Failed to get portal info for token ${token.id}, cleared cache:`,
          result.portal_error
        );
        portalFailed++;
      }

      // 更新时间戳
      token.updated_at = new Date().toISOString();
      console.log(
        `Updated token ${token.id} status to: ${statusResult.status}`
      );
    }
  });

  // 返回统计信息
  return {
    statusSuccess,
    statusFailed,
    portalSuccess,
    portalFailed,
  };
};

// 关闭所有 TokenCard 的弹窗
const closeAllTokenCardModals = () => {
  Object.values(tokenCardRefs.value).forEach((cardRef) => {
    if (cardRef && cardRef.closeAllModals) {
      cardRef.closeAllModals();
    }
  });
};

// Expose methods
defineExpose({
  checkAllAccountStatus,
  closeAllTokenCardModals,
  refreshAllPortalInfo: async () => {
    // 刷新所有有portal_url的账号的额度信息
    const refreshOperations = [];

    props.tokens.forEach((token) => {
      if (token.portal_url && tokenCardRefs.value[token.id]) {
        const cardRef = tokenCardRefs.value[token.id];
        if (cardRef && cardRef.refreshPortalInfo) {
          // 立即调用刷新方法，不等待结果
          const refreshPromise = cardRef
            .refreshPortalInfo()
            .then(() => {
              return { success: true, tokenId: token.id };
            })
            .catch((error) => {
              console.warn(`刷新账号 ${token.id} 额度信息失败:`, error);
              return { success: false, tokenId: token.id, error };
            });

          refreshOperations.push(refreshPromise);
        }
      }
    });

    // 等待所有刷新操作完成并统计结果
    if (refreshOperations.length > 0) {
      const results = await Promise.allSettled(refreshOperations);

      // 统计成功的刷新数量
      let refreshedCount = 0;
      let failedCount = 0;

      results.forEach((result) => {
        if (result.status === "fulfilled" && result.value.success) {
          refreshedCount++;
        } else {
          failedCount++;
        }
      });

      return {
        success: true,
        message: `已刷新 ${refreshedCount} 个账号的额度信息${
          failedCount > 0 ? `，${failedCount} 个失败` : ""
        }`,
        refreshedCount,
        failedCount,
      };
    } else {
      return {
        success: true,
        message: "没有需要刷新额度的账号",
        refreshedCount: 0,
        failedCount: 0,
      };
    }
  },
});

// 生命周期
onMounted(async () => {
  // 只在首次进入应用且有账户时自动检测状态
  if (props.tokens.length > 0 && props.shouldAutoCheck) {
    emit("copy-success", "正在检测Augment账户状态...", "info");

    // 延迟检测，让界面先完成渲染
    setTimeout(() => {
      emit("check-all-status");
      emit("auto-check-completed");
    }, 1000);
  }
});
</script>

<style scoped>
.token-list-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.list-content {
  flex: 1;
  overflow: visible;
  min-height: 0;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
}

.empty-state-content {
  max-width: 400px;
  margin: 0 auto;
}

.empty-icon {
  color: rgba(59, 130, 246, 0.6);
  margin-bottom: 24px;
  filter: drop-shadow(0 2px 4px rgba(59, 130, 246, 0.1));
}

.empty-state h4 {
  color: #1e293b;
  margin: 0 0 12px 0;
  font-size: 1.25rem;
  font-weight: 600;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.empty-state p {
  color: #64748b;
  margin: 0 0 24px 0;
  font-size: 1rem;
  line-height: 1.6;
}

.empty-actions {
  margin-top: 24px;
}

.btn-empty {
  padding: 12px 24px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn-empty.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
}

.btn-empty.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
}

.loading-state {
  text-align: center;
  padding: 80px 40px;
  color: #64748b;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.03) 0%,
    rgba(139, 92, 246, 0.03) 100%
  );
  border-radius: 16px;
  border: 1px solid rgba(59, 130, 246, 0.08);
}

.spinner {
  width: 48px;
  height: 48px;
  border: 3px solid rgba(59, 130, 246, 0.1);
  border-top: 3px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 24px;
  filter: drop-shadow(0 2px 4px rgba(59, 130, 246, 0.1));
}

.loading-state p {
  font-size: 1rem;
  font-weight: 500;
  margin: 0;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.token-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: 10px;
  padding: 10px 0 0;
}

/* Token列表整体布局优化 */
.token-list {
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 响应式布局优化 */
@media (max-width: 1200px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 10px;
  }
}

@media (max-width: 1200px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 10px;
  }
}

@media (max-width: 768px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 4px;
  }

  .empty-state,
  .loading-state {
    padding: 60px 20px;
  }
}

/* 统一账号管理卡片样式 */
.unified-account-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.4);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(20px);
}

.unified-account-card:hover {
  /* transform: translateY(-2px); */
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  border-color: rgba(226, 232, 240, 0.6);
}

/* 卡片头部样式 */
.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 28px 32px;
  background: linear-gradient(
    135deg,
    rgba(249, 250, 251, 0.8) 0%,
    rgba(243, 244, 246, 0.5) 100%
  );
}

.header-main {
  display: flex;
  align-items: flex-start;
  gap: 20px;
  flex: 1;
}

.section-icon {
  width: 56px;
  height: 56px;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1) 0%,
    rgba(139, 92, 246, 0.1) 100%
  );
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid rgba(59, 130, 246, 0.2);
  flex-shrink: 0;
}

.section-icon svg {
  color: #3b82f6;
}

.header-text h3 {
  margin: 0 0 8px 0;
  font-size: 22px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.025em;
}

.header-description {
  color: #64748b;
  font-size: 15px;
  line-height: 1.6;
  margin: 0;
  font-weight: 400;
}

.header-stats {
  display: flex;
  gap: 16px;
  align-items: center;
  margin-top: 8px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  background: rgba(59, 130, 246, 0.1);
  border-radius: 24px;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.stat-label {
  font-size: 13px;
  color: #64748b;
  font-weight: 500;
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  color: #3b82f6;
}

/* 分割线 */
.card-divider {
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(226, 232, 240, 0.5) 50%,
    transparent 100%
  );
  margin: 0 32px;
}

/* 内容区域 */
.card-content {
  padding: 0 10px 10px;
}

/* Button styles */
.btn {
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn.small {
  padding: 6px 10px;
  font-size: 11px;
}

.btn.success {
  background: #22c55e;
  color: white;
}

.btn.success:hover:not(:disabled) {
  background: #16a34a;
}

.btn.secondary {
  background: #64748b;
  color: white;
}

.btn.secondary:hover {
  background: #475569;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
