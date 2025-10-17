<template>
  <div class="outlook-manager-container" @contextmenu.prevent>
    <!-- 统一的邮箱管理卡片 -->
    <div class="unified-account-card">
      <!-- 内容区域 -->
      <div class="card-content">
        <!-- 账户列表 -->
        <div class="accounts-section">
          <!-- Loading State -->
          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <p>正在加载账户...</p>
          </div>

          <!-- Empty State -->
          <div v-else-if="accounts.length === 0" class="empty-state">
            <div class="empty-state-content">
              <div class="empty-icon">
                <svg
                  width="48"
                  height="48"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
                  />
                </svg>
              </div>
              <h4>还没有邮箱账户</h4>
              <p>
                添加outlook邮箱账户后即可开始使用邮件管理功能，用于快捷获取验证码。账户数据仅保存在本次会话内，关闭软件后将自动清空。
              </p>
            </div>
          </div>

          <!-- Accounts List -->
          <div v-else class="accounts-list">
            <div class="accounts-grid">
              <div
                v-for="account in accounts"
                :key="account"
                class="account-card"
              >
                <div class="account-info">
                  <div class="account-header">
                    <div class="account-avatar">
                      {{ getAccountInitial(account) }}
                    </div>
                    <div class="account-details">
                      <div class="account-email-row">
                        <div class="account-email">{{ account }}</div>
                        <button
                          @click="copyEmail(account)"
                          class="copy-email-btn"
                          title="复制邮箱地址"
                        >
                          <svg
                            width="14"
                            height="14"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                          >
                            <path
                              d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
                            />
                          </svg>
                        </button>
                      </div>
                      <div
                        class="account-status"
                        :class="getStatusClass(account)"
                      >
                        {{ getStatusText(account) }}
                      </div>
                    </div>
                  </div>
                  <!-- 获取验证码按钮及验证码显示 -->
                  <div class="code-actions">
                    <button
                      @click="getLatestCode(account)"
                      :disabled="isGettingCodes[account]"
                      :class="[
                        'btn-card',
                        'success',
                        'code-btn',
                        { loading: isGettingCodes[account] },
                      ]"
                    >
                      <svg
                        v-if="!isGettingCodes[account]"
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                        />
                      </svg>
                      <div v-else class="loading-spinner"></div>
                      {{ isGettingCodes[account] ? "获取中..." : "获取验证码" }}
                    </button>
                    <template v-if="lastCodes[account]">
                      <span
                        class="code-badge"
                        @click="copyCode(lastCodes[account])"
                        title="点击复制验证码"
                      >
                        {{ lastCodes[account] }}
                      </span>
                    </template>
                  </div>
                </div>
                <div class="account-actions">
                  <button @click="viewEmails(account)" class="btn-card primary">
                    <svg
                      width="14"
                      height="14"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
                      />
                    </svg>
                    查看邮件
                  </button>

                  <button
                    @click="checkStatus(account)"
                    :disabled="isCheckingStatus"
                    class="btn-card secondary"
                  >
                    <svg
                      width="14"
                      height="14"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z"
                      />
                    </svg>
                    状态
                  </button>
                  <button
                    @click="showDeleteConfirm(account)"
                    class="btn-card danger"
                  >
                    <svg
                      width="14"
                      height="14"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z"
                      />
                    </svg>
                    移除
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 删除确认弹窗 -->
  <ModalContainer
    :visible="showDeleteModal"
    title="确认移除账户"
    size="small"
    @close="showDeleteModal = false"
  >
    <div class="delete-confirm-content">
      <div class="confirm-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M12,2C17.53,2 22,6.47 22,12C22,17.53 17.53,22 12,22C6.47,22 2,17.53 2,12C2,6.47 6.47,2 12,2M15.59,7L12,10.59L8.41,7L7,8.41L10.59,12L7,15.59L8.41,17L12,13.41L15.59,17L17,15.59L13.41,12L17,8.41L15.59,7Z"
          />
        </svg>
      </div>
      <h4>确认移除账户</h4>
      <p>
        确定要移除邮箱账户 <strong>{{ accountToDelete }}</strong> 吗？
      </p>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button @click="showDeleteModal = false" class="btn secondary">
          取消
        </button>
        <button @click="confirmDelete" class="btn danger">确认移除</button>
      </div>
    </template>
  </ModalContainer>

  <!-- 邮件查看器 -->
  <ModalContainer
    :visible="showEmailViewer"
    :title="`邮件管理 - ${selectedEmail}`"
    size="large"
    @close="showEmailViewer = false"
  >
    <EmailManagerContent
      v-if="showEmailViewer"
      :email="selectedEmail"
      @show-status="showStatus"
    />
  </ModalContainer>

  <!-- 添加账户弹窗 -->
  <ModalContainer
    :visible="showAddAccountModal"
    title="添加outlook邮箱"
    size="medium"
    @close="showAddAccountModal = false"
  >
    <div class="add-account-modal-content">
      <!-- 导入格式说明 -->
      <div class="format-section">
        <div class="format-header">
          <div class="format-title">
            <h4>选择导入格式</h4>
            <p>点击选择格式，每行一个账户：</p>
          </div>
          <div class="format-status">
            当前选择：格式{{ selectedFormat === "format1" ? "1" : "2" }}
          </div>
        </div>
        <div class="format-examples">
          <div
            class="format-item clickable"
            :class="{ active: selectedFormat === 'format1' }"
            @click="selectedFormat = 'format1'"
          >
            <strong>格式1：</strong> 邮箱地址----密码----Client ID----Refresh
            Token
          </div>
          <div
            class="format-item clickable"
            :class="{ active: selectedFormat === 'format2' }"
            @click="selectedFormat = 'format2'"
          >
            <strong>格式2：</strong> 邮箱地址----密码----Refresh Token----Client
            ID
          </div>
        </div>
      </div>

      <!-- 账户信息输入 -->
      <div class="form-group">
        <label>账户信息:</label>
        <textarea
          v-model="accountInput"
          rows="3"
          :placeholder="getInputPlaceholder()"
          class="form-textarea"
        ></textarea>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button @click="showAddAccountModal = false" class="btn secondary">
          取消
        </button>
        <button
          @click="handleAddAccounts"
          :disabled="!canAddAccount || isAdding"
          :class="['btn', 'primary', { loading: isAdding }]"
        >
          {{
            isAdding
              ? "添加中..."
              : accountInput.includes("\n")
              ? "批量添加"
              : "添加账户"
          }}
        </button>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref, computed, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";
import EmailManagerContent from "./EmailManagerContent.vue";

const props = defineProps({
  importFormat: {
    type: String,
    default: "format1",
  },
});

const emit = defineEmits([
  "show-status",
  "accounts-count-changed",
  "accounts-status-changed",
]);
// 最近验证码记录（仅UI显示用）
const lastCodes = ref({});

// 顶层：获取最新一封邮件验证码
const getLatestCode = async (email) => {
  // 设置加载状态
  isGettingCodes.value[email] = true;

  try {
    const res = await invoke("outlook_get_emails", {
      email,
      folder: "inbox",
      page: 1,
      pageSize: 1,
    });

    if (!res || !res.emails || res.emails.length === 0) {
      showStatus(`${email} 暂无邮件可获取验证码`, "warning");
      return;
    }

    const first = res.emails[0];
    let code = first.verification_code || null;

    if (!code) {
      try {
        const details = await invoke("outlook_get_email_details", {
          email,
          messageId: first.message_id,
        });
        const text =
          details.body_plain || details.body_html || details.text_body || "";
        const htmlStripped =
          typeof text === "string"
            ? text.replace(/<[^>]*>/g, " ").replace(/&[^;]+;/g, " ")
            : "";
        const subject = details.subject || "";
        const candidates = [htmlStripped, subject]
          .filter(Boolean)
          .map((t) => t.match(/\b(\d{4,8})\b/g) || [])
          .flat();
        if (candidates.length > 0) {
          code = candidates[0];
        }
      } catch (e) {
        console.warn("获取详情以提取验证码失败:", e);
      }
    }

    if (code) {
      // 复制
      try {
        await navigator.clipboard.writeText(code);
      } catch {
        const ta = document.createElement("textarea");
        ta.value = code;
        document.body.appendChild(ta);
        ta.select();
        document.execCommand("copy");
        document.body.removeChild(ta);
      }

      showStatus(`验证码: ${code}（已复制）`, "success");
      // 保存到UI状态
      lastCodes.value[email] = code;
    } else {
      showStatus(`${email} 未找到验证码`, "warning");
    }
  } catch (error) {
    showStatus(`获取验证码失败: ${error}`, "error");
  } finally {
    // 清除加载状态
    isGettingCodes.value[email] = false;
  }
};

// 响应式数据
const accounts = ref([]);
const accountStatuses = ref({});
const accountStatusTimestamps = ref({}); // 存储状态检测时间戳
const isLoading = ref(false);
const isAdding = ref(false);
const isCheckingStatus = ref(false);
const showEmailViewer = ref(false);
const showDeleteModal = ref(false);
const selectedEmail = ref("");
const accountToDelete = ref("");
const isGettingCodes = ref({}); // 跟踪每个账户的验证码获取状态
const showAddAccountModal = ref(false); // 控制添加账户弹窗

const accountInput = ref("");
const selectedFormat = ref("format1"); // 默认使用格式1

// 计算属性
const canAddAccount = computed(() => {
  if (!accountInput.value.trim()) return false;

  // 检查每一行是否都符合格式
  const lines = accountInput.value
    .trim()
    .split("\n")
    .filter((line) => line.trim());
  return lines.every((line) => {
    const parts = line.trim().split("----");
    return parts.length === 4 && parts.every((part) => part.trim());
  });
});

// 方法
const showStatus = (message, type = "info") => {
  emit("show-status", message, type);
};

// 会话内存存储：仅更新内存对象
const saveStatusToLocal = (email, status) => {
  const now = Date.now();
  accountStatuses.value = { ...accountStatuses.value, [email]: status };
  accountStatusTimestamps.value = {
    ...accountStatusTimestamps.value,
    [email]: now,
  };
  console.log(`✅ 状态已更新(内存): ${email} -> ${status}`);
};

// 从本地存储加载状态
const loadStatusFromLocal = () => {
  // 会话内模式：直接返回当前内存对象（首次为空）
  return { ...accountStatuses.value };
};

// 清理过期状态（会话内存，仅操作内存对象）
const cleanExpiredStatuses = () => {
  const now = Date.now();
  Object.keys(accountStatusTimestamps.value).forEach((email) => {
    const timestamp = accountStatusTimestamps.value[email] || 0;
    if (now - timestamp >= STATUS_EXPIRE_TIME) {
      delete accountStatuses.value[email];
      delete accountStatusTimestamps.value[email];
    }
  });
};

// 获取输入提示文本
const getInputPlaceholder = () => {
  return `支持单个或批量添加，每行一个账户 示例：\nxxx@outlook.com----password----${
    selectedFormat.value === "format1"
      ? "client_id----refresh_token"
      : "refresh_token----client_id"
  }\nxxx@outlook.com----password----${
    selectedFormat.value === "format1"
      ? "client_id----refresh_token"
      : "refresh_token----client_id"
  }`;
};

const refreshAccounts = async () => {
  isLoading.value = true;
  try {
    // 获取账户列表
    const accountList = await invoke("outlook_get_all_accounts");
    accounts.value = accountList;

    // 清理过期状态
    cleanExpiredStatuses();

    // 从本地存储加载状态
    const savedStatuses = loadStatusFromLocal();

    // 初始化状态，优先使用本地存储的有效状态，否则设为unknown
    accountStatuses.value = {};
    accountList.forEach((email) => {
      accountStatuses.value[email] = savedStatuses[email] || "unknown";
    });
  } catch (error) {
    showStatus(`刷新失败: ${error}`, "error");
  } finally {
    isLoading.value = false;
  }
};

// 账户解析函数 - 根据用户选择的格式解析
const parseAccountData = (input, format = null) => {
  const actualFormat = format || props.importFormat;
  const parts = input.trim().split("----");

  if (parts.length !== 4) {
    throw new Error(
      `格式错误，请按照选择的格式输入：\n${
        actualFormat === "format1"
          ? "邮箱地址----密码----Client ID----Refresh Token"
          : "邮箱地址----密码----Refresh Token----Client ID"
      }`
    );
  }

  const [email, password, third, fourth] = parts.map((part) => part.trim());

  if (!email || !password || !third || !fourth) {
    throw new Error("邮箱、密码、Refresh Token 和 Client ID 都不能为空");
  }

  // 简单的邮箱验证
  if (!email.includes("@")) {
    throw new Error("邮箱格式不正确");
  }

  let refreshToken, clientId;
  if (actualFormat === "format1") {
    // 格式1：邮箱----密码----Client ID----Refresh Token
    clientId = third;
    refreshToken = fourth;
  } else {
    // 格式2：邮箱----密码----Refresh Token----Client ID
    refreshToken = third;
    clientId = fourth;
  }

  return { email, password, refreshToken, clientId };
};

// 添加账户（支持单个和批量）
const addAccounts = async () => {
  if (!accountInput.value.trim()) return;

  isAdding.value = true;

  try {
    const lines = accountInput.value
      .trim()
      .split("\n")
      .filter((line) => line.trim());
    let successCount = 0;
    let errorCount = 0;
    const errors = [];

    for (let i = 0; i < lines.length; i++) {
      try {
        const { email, refreshToken, clientId } = parseAccountData(
          lines[i],
          selectedFormat.value
        );

        await invoke("outlook_save_credentials", {
          email,
          refreshToken,
          clientId,
        });

        successCount++;
      } catch (error) {
        errorCount++;
        errors.push(`第 ${i + 1} 行: ${error.message}`);
      }
    }

    // 注意：不在这里重置表单，由调用者决定

    // 刷新账户列表
    await refreshAccounts();

    // 自动检测新添加账户的状态
    if (successCount > 0) {
      const addedEmails = [];
      for (let i = 0; i < lines.length; i++) {
        try {
          const { email } = parseAccountData(lines[i], selectedFormat.value);
          addedEmails.push(email);
        } catch (error) {
          // 忽略解析错误的行
        }
      }

      // 异步检测新账户状态（不阻塞用户操作）
      setTimeout(async () => {
        for (const email of addedEmails) {
          try {
            const status = await invoke("outlook_check_account_status", {
              email,
            });
            // 保存状态到本地存储
            saveStatusToLocal(email, status.status);
          } catch (error) {
            // 保存错误状态到本地存储
            saveStatusToLocal(email, "error");
          }
        }
      }, 1000);
    }

    // 显示结果
    if (lines.length === 1) {
      if (successCount > 0) {
        showStatus("账户添加成功，正在后台检测状态...", "success");
      } else {
        showStatus(`添加失败: ${errors[0]}`, "error");
      }
    } else {
      if (successCount > 0 && errorCount === 0) {
        showStatus(
          `批量添加成功！共添加 ${successCount} 个账户，正在后台检测状态...`,
          "success"
        );
      } else if (successCount > 0 && errorCount > 0) {
        showStatus(
          `部分添加成功！成功 ${successCount} 个，失败 ${errorCount} 个，正在后台检测状态...\n错误详情：\n${errors
            .slice(0, 3)
            .join("\n")}${errors.length > 3 ? "\n..." : ""}`,
          "warning"
        );
      } else {
        showStatus(
          `批量添加失败！共 ${errorCount} 个错误\n${errors
            .slice(0, 3)
            .join("\n")}${errors.length > 3 ? "\n..." : ""}`,
          "error"
        );
      }
    }
  } catch (error) {
    showStatus(`操作失败: ${error}`, "error");
  } finally {
    isAdding.value = false;
  }
};

// 批量刷新所有账户状态
const refreshAllStatuses = async () => {
  if (accounts.value.length === 0) return;

  isCheckingStatus.value = true;
  let checkedCount = 0;

  try {
    for (const account of accounts.value) {
      try {
        const status = await invoke("outlook_check_account_status", {
          email: account,
        });

        // 保存状态到本地存储
        saveStatusToLocal(account, status.status);
        checkedCount++;
      } catch (error) {
        // 保存错误状态到本地存储
        saveStatusToLocal(account, "error");
      }
    }

    showStatus(`批量状态检查完成！已检查 ${checkedCount} 个账户`, "success");
  } catch (error) {
    showStatus(`批量状态检查失败: ${error}`, "error");
  } finally {
    isCheckingStatus.value = false;
  }
};

// 显示删除确认弹窗
const showDeleteConfirm = (email) => {
  accountToDelete.value = email;
  showDeleteModal.value = true;
};

// 确认删除账户
const confirmDelete = async () => {
  try {
    const deleted = await invoke("outlook_delete_account", {
      email: accountToDelete.value,
    });
    if (deleted) {
      await refreshAccounts();
      delete accountStatuses.value[accountToDelete.value];
      showStatus("邮箱账户已成功移除", "success");
    } else {
      showStatus("账户不存在", "warning");
    }
  } catch (error) {
    showStatus(`移除失败: ${error}`, "error");
  } finally {
    showDeleteModal.value = false;
    accountToDelete.value = "";
  }
};

const checkStatus = async (email) => {
  isCheckingStatus.value = true;
  try {
    const status = await invoke("outlook_check_account_status", { email });

    // 保存状态到本地存储
    saveStatusToLocal(email, status.status);

    showStatus(`${email} 状态: ${status.status}`, "info");
  } catch (error) {
    // 保存错误状态到本地存储
    saveStatusToLocal(email, "error");

    showStatus(`状态检查失败: ${error}`, "error");
  } finally {
    isCheckingStatus.value = false;
  }
};

const viewEmails = (email) => {
  selectedEmail.value = email;
  showEmailViewer.value = true;
};

const getStatusClass = (email) => {
  const status = accountStatuses.value[email];
  return {
    "status-active": status === "active",
    "status-inactive": status === "inactive",
    "status-error": status === "error",
    "status-unknown": status === "unknown" || !status,
  };
};

const getStatusText = (email) => {
  const status = accountStatuses.value[email];
  switch (status) {
    case "active":
      return "活跃";
    case "inactive":
      return "非活跃";
    case "error":
      return "错误";
    case "unknown":
      return "未知";
    default:
      return "未知";
  }
};

const getAccountInitial = (email) => {
  return email.charAt(0).toUpperCase();
};

// 复制邮箱地址
const copyEmail = async (email) => {
  try {
    await navigator.clipboard.writeText(email);
    showStatus(`已复制邮箱地址: ${email}`, "success");
  } catch (error) {
    // 降级处理：如果clipboard API不可用，使用传统方法
    try {
      const textArea = document.createElement("textarea");
      textArea.value = email;
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand("copy");
      document.body.removeChild(textArea);
      showStatus(`已复制邮箱地址: ${email}`, "success");
    } catch (fallbackError) {
      showStatus("复制失败，请手动复制", "error");
    }
  }
};

// 复制验证码
const copyCode = async (code) => {
  try {
    await navigator.clipboard.writeText(code);
    showStatus(`验证码已复制: ${code}`, "success");
  } catch (error) {
    // 降级处理
    try {
      const textArea = document.createElement("textarea");
      textArea.value = code;
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand("copy");
      document.body.removeChild(textArea);
      showStatus(`验证码已复制: ${code}`, "success");
    } catch (fallbackError) {
      showStatus("复制失败，请手动复制", "error");
    }
  }
};

// 处理弹窗中的添加账户操作
const handleAddAccounts = async () => {
  await addAccounts();
  // 添加成功后关闭弹窗并清空输入
  if (!isAdding.value) {
    showAddAccountModal.value = false;
    accountInput.value = "";
  }
};

// 对外暴露的方法
const openAddAccountModal = () => {
  showAddAccountModal.value = true;
};

// 初始化邮箱管理（用于软件启动时自动检测）
const initializeEmailManagement = async () => {
  // 已移除首次进入时的自动加载与检测逻辑
};

// 计算账户状态统计
const accountsStatusStats = computed(() => {
  const stats = {
    total: accounts.value.length,
    active: 0,
    inactive: 0,
    error: 0,
    unknown: 0,
  };

  accounts.value.forEach((account) => {
    const status = accountStatuses.value[account];
    switch (status) {
      case "active":
        stats.active++;
        break;
      case "inactive":
        stats.inactive++;
        break;
      case "error":
        stats.error++;
        break;
      default:
        stats.unknown++;
        break;
    }
  });

  return stats;
});

// 监听账户数量变化并通知父组件
watch(
  () => accounts.value.length,
  (newCount) => {
    emit("accounts-count-changed", newCount);
  },
  { immediate: true }
);

// 监听账户状态统计变化并通知父组件
watch(
  accountsStatusStats,
  (newStats) => {
    emit("accounts-status-changed", newStats);
  },
  { deep: true, immediate: true }
);

// 暴露方法供父组件调用
defineExpose({
  openAddAccountModal,
  refreshAllStatuses,
  initializeEmailManagement,
  refreshAccounts, // 添加账户状态刷新方法
});

// 生命周期
onMounted(async () => {
  // 首次进入不再自动加载账户或检测状态
});
</script>

<style scoped>
/* 主容器 - 完全模仿TokenList */
.outlook-manager-container {
  width: 100%;
  height: 100%;
  display: flex;
  /* 验证码展示样式 */
  .code-actions {
    margin-top: 16px;
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .code-btn {
    padding: 10px 14px;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 600;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    border: none;
    color: white;
    box-shadow: 0 2px 8px rgba(16, 185, 129, 0.25);
    transition: all 0.25s ease;
  }
  .code-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.35);
  }

  .code-btn.loading {
    opacity: 0.8;
    cursor: wait;
  }

  .loading-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .code-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none !important;
    box-shadow: 0 2px 8px rgba(16, 185, 129, 0.15) !important;
  }
  .code-badge {
    display: inline-flex;
    align-items: center;
    padding: 6px 12px;
    border-radius: 12px;
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.1) 0%,
      rgba(139, 92, 246, 0.05) 100%
    );
    color: #2563eb;
    font-weight: 700;
    cursor: pointer;
    border: 1px solid rgba(59, 130, 246, 0.25);
    transition: all 0.25s ease;
    font-family: "SF Mono", "Monaco", "Cascadia Code", monospace;
    letter-spacing: 0.5px;
    position: relative;
    overflow: hidden;
  }
  .code-badge::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.3),
      transparent
    );
    transition: left 0.5s ease;
  }
  .code-badge:hover {
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.15) 0%,
      rgba(139, 92, 246, 0.08) 100%
    );
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2);
  }
  .code-badge:hover::before {
    left: 100%;
  }

  .account-status {
    gap: 8px;
  }
  .code-divider {
    color: #cbd5e1;
  }
  .account-code {
    color: #0ea5e9;
    cursor: pointer;
    font-weight: 600;
  }
  .account-code:hover {
    text-decoration: underline;
  }

  flex-direction: column;
}

/* 统一账号管理卡片样式 - 与TokenList完全一致 */
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

/* 卡片标题栏 */
.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 28px 32px 20px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
}

.card-header .header-main {
  display: flex;
  align-items: flex-start;
  gap: 20px;
  flex: 1;
}

.card-header .section-icon {
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

.card-header .section-icon svg {
  color: #3b82f6;
}

.card-header .header-text h3 {
  margin: 0 0 8px 0;
  font-size: 22px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.025em;
}

.card-header .header-description {
  color: #64748b;
  font-size: 15px;
  line-height: 1.6;
  margin: 0;
  font-weight: 400;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.btn-header {
  padding: 12px 20px;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all 0.25s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
}

.btn-header.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn-header.primary::before {
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
  transition: left 0.5s ease;
}

.btn-header.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
}

.btn-header.primary:hover::before {
  left: 100%;
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

/* 内容区域 - 完全模仿TokenList */
.card-content {
  padding: 0 10px 10px;
}

/* 存储信息提醒 */
.session-notice {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.1) 0%,
    rgba(5, 150, 105, 0.05) 100%
  );
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 8px;
  margin-bottom: 20px;
  font-size: 13px;
  color: #065f46;
}

.notice-icon {
  font-size: 16px;
  flex-shrink: 0;
}

/* 表单组件 */
.form-group {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
  color: #374151;
  font-size: 14px;
}

.form-textarea {
  width: 100%;
  padding: 8px 6px;
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 10px;
  font-size: 14px;
  line-height: 1.5;
  transition: all 0.2s ease;
  box-sizing: border-box;
  resize: none;
  background: rgba(255, 255, 255, 0.8);
}

.form-textarea:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  background: rgba(255, 255, 255, 0.95);
}

.input-hint {
  margin-top: 10px;
}

.format-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.format-title {
  font-weight: 500;
  color: #6b7280;
  font-size: 12px;
}

.format-examples {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.format-item {
  font-size: 11px;
  color: #9ca3af;
  padding: 4px 8px;
  background: rgba(59, 130, 246, 0.05);
  border-radius: 4px;
  border: 1px solid rgba(59, 130, 246, 0.1);
}

.form-actions {
  margin-top: 20px;
}

.btn-action {
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
}

.btn-action.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn-action.primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
}

.btn-action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.btn-action.loading {
  opacity: 0.7;
  cursor: wait;
}

.section-header-simple {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.section-header-simple h4 {
  margin: 0;
  color: #1e293b;
  font-size: 18px;
  font-weight: 600;
}

.btn-small {
  padding: 8px 12px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn-small.secondary {
  background: rgba(107, 114, 128, 0.1);
  color: #4b5563;
  border: 1px solid rgba(107, 114, 128, 0.2);
}

.btn-small.secondary:hover:not(:disabled) {
  background: rgba(107, 114, 128, 0.15);
  transform: translateY(-1px);
}

/* Empty State - 与TokenList一致 */
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

/* Loading State - 与TokenList一致 */
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

/* 账户网格 */
.accounts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 20px;
  margin-top: 12px;
}

.account-card {
  background: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.95) 0%,
    rgba(255, 255, 255, 0.85) 100%
  );
  border: 1px solid rgba(226, 232, 240, 0.4);
  border-radius: 16px;
  padding: 24px;
  position: relative;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(12px);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05), 0 1px 2px rgba(0, 0, 0, 0.03);
  overflow: hidden;
  animation: fadeInUp 0.6s ease-out;
  animation-fill-mode: both;
}

/* 为不同位置的卡片添加延迟动画 */
.account-card:nth-child(1) {
  animation-delay: 0.1s;
}
.account-card:nth-child(2) {
  animation-delay: 0.2s;
}
.account-card:nth-child(3) {
  animation-delay: 0.3s;
}
.account-card:nth-child(4) {
  animation-delay: 0.4s;
}
.account-card:nth-child(5) {
  animation-delay: 0.5s;
}
.account-card:nth-child(6) {
  animation-delay: 0.6s;
}
.account-card:nth-child(n + 7) {
  animation-delay: 0.7s;
}

.account-card::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, #3b82f6 0%, #8b5cf6 50%, #06b6d4 100%);
  border-radius: 16px 16px 0 0;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.account-card:hover {
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.12);
  transform: translateY(-1px);
}

.account-info {
  margin-bottom: 20px;
}

.account-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 16px;
}

.account-avatar {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 18px;
  flex-shrink: 0;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  position: relative;
  overflow: hidden;
}

.account-avatar::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    145deg,
    transparent 0%,
    rgba(255, 255, 255, 0.1) 100%
  );
  border-radius: 14px;
}

.account-details {
  flex: 1;
  min-width: 0;
}

.account-email-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.account-email {
  font-weight: 600;
  color: #1e293b;
  font-size: 15px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  letter-spacing: -0.01em;
}

.copy-email-btn {
  background: rgba(59, 130, 246, 0.08);
  border: 1px solid rgba(59, 130, 246, 0.15);
  border-radius: 8px;
  padding: 6px;
  cursor: pointer;
  color: #3b82f6;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  position: relative;
  overflow: hidden;
}

.copy-email-btn::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.4),
    transparent
  );
  transition: left 0.5s ease;
}

.copy-email-btn:hover {
  background: rgba(59, 130, 246, 0.12);
  border-color: rgba(59, 130, 246, 0.25);
  transform: scale(1.05);
  box-shadow: 0 3px 12px rgba(59, 130, 246, 0.25);
}

.copy-email-btn:hover::before {
  left: 100%;
}

.copy-email-btn:active {
  transform: scale(0.98);
}

.account-status {
  font-size: 12px;
  padding: 4px 12px;
  border-radius: 16px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-weight: 600;
  letter-spacing: 0.025em;
  text-transform: uppercase;
  font-size: 10px;
  position: relative;
  overflow: hidden;
}

.status-active {
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.12) 0%,
    rgba(5, 150, 105, 0.08) 100%
  );
  color: #047857;
  border: 1px solid rgba(16, 185, 129, 0.25);
  box-shadow: 0 2px 4px rgba(16, 185, 129, 0.1);
}

.status-inactive {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.12) 0%,
    rgba(220, 38, 38, 0.08) 100%
  );
  color: #b91c1c;
  border: 1px solid rgba(239, 68, 68, 0.25);
  box-shadow: 0 2px 4px rgba(239, 68, 68, 0.1);
}

.status-error {
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.12) 0%,
    rgba(217, 119, 6, 0.08) 100%
  );
  color: #b45309;
  border: 1px solid rgba(245, 158, 11, 0.25);
  box-shadow: 0 2px 4px rgba(245, 158, 11, 0.1);
}

.status-unknown {
  background: linear-gradient(
    135deg,
    rgba(107, 114, 128, 0.12) 0%,
    rgba(75, 85, 99, 0.08) 100%
  );
  color: #4b5563;
  border: 1px solid rgba(107, 114, 128, 0.25);
  box-shadow: 0 2px 4px rgba(107, 114, 128, 0.1);
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.account-actions {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  margin-top: 4px;
}

.btn-card {
  padding: 10px 0px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  justify-content: center;
  position: relative;
  overflow: hidden;
  min-height: 36px;
}

.btn-card.primary {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1) 0%,
    rgba(139, 92, 246, 0.1) 100%
  );
  color: #3b82f6;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.btn-card.primary::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(59, 130, 246, 0.1),
    transparent
  );
  transition: left 0.5s ease;
}

.btn-card.primary:hover {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.15) 0%,
    rgba(139, 92, 246, 0.15) 100%
  );
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.25);
  border-color: rgba(59, 130, 246, 0.3);
}

.btn-card.primary:hover::before {
  left: 100%;
}

.btn-card.secondary {
  background: linear-gradient(
    135deg,
    rgba(107, 114, 128, 0.08) 0%,
    rgba(75, 85, 99, 0.06) 100%
  );
  color: #4b5563;
  border: 1px solid rgba(107, 114, 128, 0.2);
}

.btn-card.secondary::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(107, 114, 128, 0.1),
    transparent
  );
  transition: left 0.5s ease;
}

.btn-card.secondary:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(107, 114, 128, 0.12) 0%,
    rgba(75, 85, 99, 0.08) 100%
  );
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(107, 114, 128, 0.15);
  border-color: rgba(107, 114, 128, 0.3);
}

.btn-card.secondary:hover::before {
  left: 100%;
}

.btn-card.danger {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.08) 0%,
    rgba(220, 38, 38, 0.06) 100%
  );
  color: #dc2626;
  border: 1px solid rgba(239, 68, 68, 0.2);
}

.btn-card.danger::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(239, 68, 68, 0.1),
    transparent
  );
  transition: left 0.5s ease;
}

.btn-card.danger:hover {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.12) 0%,
    rgba(220, 38, 38, 0.08) 100%
  );
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(239, 68, 68, 0.25);
  border-color: rgba(239, 68, 68, 0.3);
}

.btn-card.danger:hover::before {
  left: 100%;
}

.btn-card.success {
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.08) 0%,
    rgba(5, 150, 105, 0.06) 100%
  );
  color: #059669;
  border: 1px solid rgba(16, 185, 129, 0.2);
}

.btn-card.success::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(16, 185, 129, 0.1),
    transparent
  );
  transition: left 0.5s ease;
}

.btn-card.success:hover {
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.12) 0%,
    rgba(5, 150, 105, 0.08) 100%
  );
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(16, 185, 129, 0.25);
  border-color: rgba(16, 185, 129, 0.3);
}

.btn-card.success:hover::before {
  left: 100%;
}

.btn-card:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

/* 响应式布局 */
@media (max-width: 1400px) {
  .accounts-grid {
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 18px;
  }
}

@media (max-width: 1200px) {
  .accounts-grid {
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 16px;
  }

  .account-card {
    padding: 20px;
  }
}

@media (max-width: 900px) {
  .accounts-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }
}

@media (max-width: 768px) {
  .accounts-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .card-header {
    flex-direction: column;
    align-items: stretch;
    gap: 16px;
    padding: 20px 24px 16px;
  }

  .card-header .header-main {
    flex-direction: column;
    gap: 12px;
    text-align: center;
  }

  .header-actions {
    justify-content: center;
  }

  .account-card {
    padding: 20px;
  }

  .account-header {
    gap: 14px;
    margin-bottom: 14px;
  }

  .account-avatar {
    width: 48px;
    height: 48px;
    border-radius: 12px;
  }

  .account-actions {
    gap: 8px;
    margin-top: 16px;
  }

  .btn-card {
    flex: 1;
    min-width: 0;
  }

  .account-email-row {
    gap: 8px;
  }

  .account-email {
    font-size: 14px;
  }

  .copy-email-btn {
    width: 26px;
    height: 26px;
  }

  .code-actions {
    margin-top: 14px;
    gap: 8px;
  }

  .code-btn {
    padding: 8px 12px;
    font-size: 12px;
  }

  .code-badge {
    padding: 5px 10px;
    font-size: 11px;
  }
}

@media (max-width: 480px) {
  .account-card {
    padding: 6px;
  }

  .account-header {
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
  }

  .account-details {
    width: 100%;
  }

  .account-email-row {
    justify-content: center;
    flex-wrap: wrap;
  }

  .account-actions {
    flex-direction: column;
    gap: 8px;
  }

  .btn-card {
    flex: none;
    width: 100%;
  }

  .code-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .code-btn {
    width: 100%;
    justify-content: center;
  }

  .code-badge {
    align-self: center;
  }

  .format-header {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .format-status {
    align-self: flex-start;
  }
}

/* 删除确认弹窗样式 */
.delete-confirm-content {
  text-align: center;
  padding: 10px;
}

.confirm-icon {
  color: #dc2626;
  margin: 0 20px;
  filter: drop-shadow(0 2px 4px rgba(220, 38, 38, 0.1));
}

.delete-confirm-content h4 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.delete-confirm-content p {
  margin: 0 0 12px 0;
  color: #64748b;
  line-height: 1.6;
}

.warning-text {
  font-size: 13px;
  color: #dc2626;
  font-weight: 500;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 6px;
}

.btn {
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
  min-height: 44px;
  justify-content: center;
}

.btn::before {
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

.btn:hover::before {
  left: 100%;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.btn.secondary {
  background: linear-gradient(
    135deg,
    rgba(156, 163, 175, 0.1) 0%,
    rgba(107, 114, 128, 0.05) 100%
  );
  color: #6b7280;
  border: 1px solid rgba(156, 163, 175, 0.5);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
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

.btn.secondary:hover:not(:disabled)::before {
  left: 100%;
}

.btn.secondary:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(156, 163, 175, 0.15) 0%,
    rgba(107, 114, 128, 0.1) 100%
  );
  border-color: rgba(107, 114, 128, 0.4);
  transform: translateY(-2px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1);
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(59, 130, 246, 0.25);
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
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.35);
}

.btn.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(239, 68, 68, 0.25);
  position: relative;
  overflow: hidden;
}

.btn.danger::before {
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

.btn.danger:hover:not(:disabled)::before {
  left: 100%;
}

.btn.danger:hover:not(:disabled) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.35);
}

.btn svg {
  transition: transform 0.25s ease;
}

.btn:hover:not(:disabled) svg {
  transform: scale(1.1);
}
.info-hint {
  display: flex;
  align-items: flex-start;
  gap: 10px;

  padding: 12px 16px;
  background: rgba(239, 246, 255, 0.5);
  border: 1px solid rgba(191, 219, 254, 0.5);
  border-radius: 10px;
  color: #2563eb;
  font-size: 13px;
  line-height: 1.5;
  backdrop-filter: blur(8px);
}

.info-hint svg {
  flex-shrink: 0;
  opacity: 0.8;
  margin-top: 1px;
}

/* 添加账户弹窗样式 */
.add-account-modal-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: auto;
}

/* 针对添加账户弹窗的特殊样式覆盖 */
:deep(.modal-body) {
  overflow: hidden !important;
  padding: 24px !important;
}

.format-section {
  background: rgba(59, 130, 246, 0.05);
  border: 1px solid rgba(59, 130, 246, 0.1);
  border-radius: 12px;
  padding: 16px;
}

.format-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0px;
  gap: 20px;
}

.format-title h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}

.format-title p {
  margin: 0;
  color: #64748b;
  font-size: 14px;
  line-height: 1.5;
}

.format-status {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  padding: 8px 12px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: 1px solid rgba(59, 130, 246, 0.2);
  white-space: nowrap;
}

.format-section p {
  margin: 0 0 16px 0;
  color: #64748b;
  font-size: 14px;
  line-height: 1.5;
}

.format-examples {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
}

.format-item {
  font-size: 13px;
  color: #374151;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.5);
  font-family: "SF Mono", "Monaco", "Cascadia Code", monospace;
  transition: all 0.2s ease;
}

.format-item.active {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.3);
  color: #3b82f6;
  font-weight: 500;
}

.format-item.clickable {
  cursor: pointer;
  user-select: none;
}

.format-item.clickable:hover {
  background: rgba(59, 130, 246, 0.05);
  border-color: rgba(59, 130, 246, 0.2);
  transform: translateY(-1px);
}

.format-item strong {
  color: #3b82f6;
  font-weight: 600;
}

.format-note {
  margin: 0 !important;
  font-size: 13px;
  color: #059669;
  font-weight: 500;
  background: rgba(16, 185, 129, 0.1);
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid rgba(16, 185, 129, 0.2);
}
</style>
