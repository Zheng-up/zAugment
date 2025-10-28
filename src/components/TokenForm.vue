<template>
  <ModalContainer
    :visible="true"
    :title="isEditing ? '编辑账号' : '添加账号'"
    size="medium"
    :show-close-button="true"
    @close="handleCancel"
  >
    <div class="token-form-content">
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label for="tenantUrl">租户URL *</label>
          <input
            id="tenantUrl"
            v-model="formData.tenantUrl"
            type="url"
            placeholder="https://example.augmentcode.com/"
            required
            :disabled="isLoading"
            autocomplete="off"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          />
          <div v-if="errors.tenantUrl" class="error-message">
            {{ errors.tenantUrl }}
          </div>
        </div>

        <div class="form-group">
          <label for="accessToken">Token *</label>
          <input
            id="accessToken"
            v-model="formData.accessToken"
            type="text"
            placeholder="请输入Token..."
            required
            :disabled="isLoading"
            autocomplete="off"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          />
          <div v-if="errors.accessToken" class="error-message">
            {{ errors.accessToken }}
          </div>
        </div>

        <div class="form-group">
          <label for="portalUrl">Portal URL (可选)</label>
          <input
            id="portalUrl"
            v-model="formData.portalUrl"
            type="url"
            placeholder="https://portal.withorb.com/view?token=xxx"
            :disabled="isLoading"
            autocomplete="off"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          />
          <div class="help-text">用于查看账户余额和过期时间</div>
          <div v-if="errors.portalUrl" class="error-message">
            {{ errors.portalUrl }}
          </div>
        </div>

        <div class="form-group">
          <label for="emailNote">邮箱备注 (可选)</label>
          <input
            id="emailNote"
            v-model="formData.emailNote"
            type="text"
            placeholder="如有Portal URL将自动获取，也可手动输入"
            :disabled="isLoading"
            autocomplete="off"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          />
          <div class="help-text">
            如果填写了Portal URL，系统会自动从API获取邮箱信息；也可手动输入邮箱备注
          </div>
          <div v-if="errors.emailNote" class="error-message">
            {{ errors.emailNote }}
          </div>
        </div>
      </form>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button
          type="button"
          @click="handleCancel"
          class="btn secondary"
          :disabled="isLoading"
        >
          取消
        </button>
        <button
          @click="handleSubmit"
          class="btn primary"
          :disabled="isLoading || !isFormValid"
        >
          <span v-if="isLoading" class="loading-spinner"></span>
          {{ isLoading ? "保存中..." : isEditing ? "更新账号" : "添加账号" }}
        </button>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";

// Props
const props = defineProps({
  token: {
    type: Object,
    default: null,
  },
});

// Emits
const emit = defineEmits([
  "close",
  "success",
  "show-status",
  "update-token",
  "add-token",
]);

// Reactive data
const formData = ref({
  tenantUrl: "",
  accessToken: "",
  portalUrl: "",
  emailNote: "",
});

const errors = ref({
  tenantUrl: "",
  accessToken: "",
  portalUrl: "",
  emailNote: "",
});

const isLoading = ref(false);

// Computed properties
const isEditing = computed(() => !!props.token);

const isFormValid = computed(() => {
  return (
    formData.value.tenantUrl.trim() &&
    formData.value.accessToken.trim() &&
    !errors.value.tenantUrl &&
    !errors.value.accessToken &&
    !errors.value.portalUrl &&
    !errors.value.emailNote
  );
});

// Watch for token prop changes (for editing)
watch(
  () => props.token,
  (newToken) => {
    if (newToken) {
      formData.value = {
        tenantUrl: newToken.tenant_url || "",
        accessToken: newToken.access_token || "",
        portalUrl: newToken.portal_url || "",
        emailNote: newToken.email_note || "",
      };
    } else {
      // Reset form for adding new token
      formData.value = {
        tenantUrl: "",
        accessToken: "",
        portalUrl: "",
        emailNote: "",
      };
    }
    // Clear errors when token changes
    errors.value = {
      tenantUrl: "",
      accessToken: "",
      portalUrl: "",
      emailNote: "",
    };
  },
  { immediate: true }
);

// Methods
const showStatus = (message, type = "info") => {
  emit("show-status", message, type);
};

const validateForm = () => {
  errors.value = {
    tenantUrl: "",
    accessToken: "",
    portalUrl: "",
  };

  // Validate tenant URL
  if (!formData.value.tenantUrl.trim()) {
    errors.value.tenantUrl = "租户URL不能为空";
  } else {
    try {
      new URL(formData.value.tenantUrl);
    } catch {
      errors.value.tenantUrl = "请输入有效的URL格式";
    }
  }

  // Validate access token
  if (!formData.value.accessToken.trim()) {
    errors.value.accessToken = "访问令牌不能为空";
  }

  // Validate portal URL (optional)
  if (formData.value.portalUrl && formData.value.portalUrl.trim()) {
    try {
      new URL(formData.value.portalUrl);
    } catch {
      errors.value.portalUrl = "请输入有效的URL格式";
    }
  }

  return (
    !errors.value.tenantUrl &&
    !errors.value.accessToken &&
    !errors.value.portalUrl
  );
};

// 从Portal URL中提取token的辅助函数
const extractTokenFromPortalUrl = (portalUrl) => {
  try {
    const url = new URL(portalUrl);
    return url.searchParams.get("token");
  } catch {
    return null;
  }
};

const handleSubmit = async () => {
  if (!validateForm()) {
    return;
  }

  isLoading.value = true;
  showStatus(isEditing.value ? "正在更新账号..." : "正在保存账号...", "info");

  try {
    // 尝试自动获取邮箱信息
    let autoEmailNote = formData.value.emailNote
      ? formData.value.emailNote.trim() || null
      : null;

    if (formData.value.portalUrl && formData.value.portalUrl.trim()) {
      try {
        const token = extractTokenFromPortalUrl(
          formData.value.portalUrl.trim()
        );
        if (token) {
          const subscriptionData = await invoke("get_subscriptions_from_link", {
            token,
          });
          const subscriptionInfo = JSON.parse(subscriptionData);

          if (subscriptionInfo.data && subscriptionInfo.data.length > 0) {
            const customerEmail = subscriptionInfo.data[0].customer?.email;
            if (customerEmail) {
              autoEmailNote = customerEmail;
              showStatus(`自动获取到邮箱: ${customerEmail}`, "success");
              // 更新表单中的邮箱字段显示
              formData.value.emailNote = customerEmail;
            }
          }
        }
      } catch (error) {
        console.warn("自动获取邮箱失败:", error);
        showStatus("自动获取邮箱失败，使用手动输入的邮箱", "warning");
      }
    }

    const tokenData = {
      tenantUrl: formData.value.tenantUrl.trim(),
      accessToken: formData.value.accessToken.trim(),
      portalUrl: formData.value.portalUrl
        ? formData.value.portalUrl.trim() || null
        : null,
      emailNote: autoEmailNote,
    };

    if (isEditing.value) {
      // Update existing token - 通知父组件更新内存中的数据
      emit("update-token", {
        id: props.token.id,
        ...tokenData,
      });
    } else {
      // Add new token - 通知父组件添加到内存中的数据
      emit("add-token", tokenData);
    }

    emit("success");
    emit("close");
  } catch (error) {
    showStatus(
      `${isEditing.value ? "更新" : "保存"}Token失败: ${error}`,
      "error"
    );
  } finally {
    isLoading.value = false;
  }
};

const handleCancel = () => {
  emit("close");
};
</script>

<style scoped>
/* TokenForm 内容样式 */
.token-form-content {
  display: flex;
  flex-direction: column;
  max-height: 58vh;
}

.form-group {
  margin-bottom: 24px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
  color: #1e293b;
  font-size: 15px;
  letter-spacing: -0.01em;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 14px 16px;
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 12px;
  font-size: 15px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-sizing: border-box;
  background: rgba(248, 250, 252, 0.5);
  backdrop-filter: blur(5px);
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.5);
  box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.1), 0 1px 3px rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
}

.form-group input:disabled,
.form-group textarea:disabled {
  background: rgba(241, 245, 249, 0.7);
  color: #64748b;
  cursor: not-allowed;
  border-color: rgba(203, 213, 225, 0.5);
}

.form-group textarea {
  resize: vertical;
  min-height: 90px;
  line-height: 1.5;
}

.help-text {
  font-size: 13px;
  color: #64748b;
  margin-top: 6px;
  line-height: 1.4;
}

.error-message {
  color: #dc2626;
  font-size: 13px;
  margin-top: 6px;
  font-weight: 500;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn {
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: auto;
  justify-content: center;
  letter-spacing: -0.01em;
  position: relative;
  overflow: hidden;
  min-height: 44px;
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
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(59, 130, 246, 0.25);
}

.btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.35);
}

.btn.secondary {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.9) 0%,
    rgba(241, 245, 249, 0.8) 100%
  );
  color: #64748b;
  border: 1px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
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

.loading-spinner {
  width: 14px;
  height: 14px;
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
</style>
