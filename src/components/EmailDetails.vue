<template>
  <div class="email-details-content">
    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <p>加载邮件详情中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p>{{ error }}</p>
      <button @click="loadEmailDetails" class="btn primary">重新加载</button>
    </div>

    <div v-else-if="emailDetails" class="email-content">
      <!-- 邮件正文 -->
      <div class="email-body">
        <div v-if="emailDetails.body_html" class="body-section">
          <div class="html-content" v-html="emailDetails.body_html"></div>
        </div>

        <div
          v-if="!emailDetails.body_html && !emailDetails.body_plain"
          class="no-content"
        >
          <p>此邮件没有可显示的内容</p>
          <p class="no-content-hint">可能是特殊格式或编码问题</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  email: {
    type: String,
    required: true,
  },
  messageId: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(["show-status"]);

// 响应式数据
const emailDetails = ref(null);
const isLoading = ref(false);
const error = ref("");

// 方法
const showStatus = (message, type = "info") => {
  emit("show-status", message, type);
};

const loadEmailDetails = async () => {
  isLoading.value = true;
  error.value = "";

  try {
    console.log("=== 开始获取邮件详情 ===");
    console.log("邮箱:", props.email);
    console.log("消息ID:", props.messageId);

    emailDetails.value = await invoke("outlook_get_email_details", {
      email: props.email,
      messageId: props.messageId,
    });

    console.log("=== 邮件详情返回数据 ===");
    console.log("完整返回数据:", emailDetails.value);
    console.log("数据类型:", typeof emailDetails.value);
    console.log("数据结构:", JSON.stringify(emailDetails.value, null, 2));
    console.log("=== 邮件详情数据结束 ===");
  } catch (err) {
    console.error("=== 获取邮件详情失败 ===");
    console.error("错误信息:", err);
    console.error("错误类型:", typeof err);
    console.error("=== 错误信息结束 ===");

    error.value = `加载邮件详情失败: ${err}`;
    showStatus(error.value, "error");
  } finally {
    isLoading.value = false;
  }
};

const formatDate = (dateString) => {
  try {
    const date = new Date(dateString);
    return date.toLocaleString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
  } catch {
    return dateString;
  }
};

// 生命周期
onMounted(() => {
  loadEmailDetails();
});
</script>

<style scoped>
/* 邮件详情内容样式 */
.email-details-content {
  padding: 0;
  height: 100%;
  overflow-y: auto;
}

.loading-state,
.error-state {
  text-align: center;
  padding: 60px 20px;
  color: #6b7280;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(59, 130, 246, 0.1);
  border-top: 4px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.email-body {
  line-height: 1.6;
}

.body-section {
  margin-bottom: 24px;
}

.body-section h4 {
  margin: 0 0 16px 0;
  color: #374151;
  font-size: 18px;
  font-weight: 600;
}

.html-content {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(255, 255, 255, 0.8) 100%
  );
  border: 1px solid rgba(226, 232, 240, 0.3);
  border-radius: 12px;
  padding: 20px;
  max-height: 400px;
  overflow-y: auto;
  word-wrap: break-word;
  backdrop-filter: blur(10px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

.plain-content {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(255, 255, 255, 0.8) 100%
  );
  border: 1px solid rgba(226, 232, 240, 0.3);
  border-radius: 12px;
  padding: 20px;
  font-family: ui-monospace, "SF Mono", "Monaco", "Inconsolata", "Roboto Mono",
    monospace;
  font-size: 14px;
  white-space: pre-wrap;
  word-wrap: break-word;
  max-height: 400px;
  overflow-y: auto;
  margin: 0;
  line-height: 1.6;
  backdrop-filter: blur(10px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

.no-content {
  text-align: center;
  padding: 40px;
  color: #9ca3af;
  font-style: italic;
}

.no-content-hint {
  font-size: 12px;
  margin-top: 8px;
  color: #d1d5db;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn.primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
}

@media (max-width: 768px) {
  .email-subject {
    font-size: 20px;
  }

  .meta-row {
    flex-direction: column;
    gap: 4px;
  }

  .meta-label {
    min-width: auto;
  }
}
</style>
