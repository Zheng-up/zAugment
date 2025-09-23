<template>
  <div class="token-form-content">
    <!-- 邮件内容区域 - 可滚动 -->
    <div class="form-body">
      <!-- 邮件列表 -->
      <div v-if="isLoading" class="loading-state">
        <div class="spinner"></div>
        <p>加载邮件中...</p>
      </div>

      <div v-else-if="emails.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
            />
          </svg>
        </div>
        <p>该文件夹中暂无邮件</p>
        <p class="empty-hint">请检查网络连接或稍后重试</p>
      </div>

      <div v-else class="emails-list">
        <div
          v-for="emailItem in emails"
          :key="emailItem.message_id"
          class="email-item"
          @click="selectEmail(emailItem)"
          :class="{ selected: selectedEmailId === emailItem.message_id }"
        >
          <div class="email-sender">
            <div class="sender-info">
              <div class="email-subject">{{ emailItem.subject }}</div>
              <div class="email-date">{{ formatDate(emailItem.date) }}</div>
            </div>
          </div>
          <div class="email-content">
            <!-- 获取到的验证码 -->
            <div class="email-meta-row">
              <div class="email-folder">{{ emailItem.folder }}</div>
              <div v-if="emailItem.verification_code" class="verification-code">
                <span class="code-label">验证码:</span>
                <span
                  class="code-value"
                  @click="copyVerificationCode(emailItem.verification_code)"
                >
                  {{ emailItem.verification_code }}
                </span>
              </div>
            </div>
          </div>
          <div class="email-actions">
            <div
              role="button"
              tabindex="0"
              @click.stop="
                !isExtractingCode && extractVerificationCode(emailItem)
              "
              @keydown.enter.stop="
                !isExtractingCode && extractVerificationCode(emailItem)
              "
              @keydown.space.prevent.stop="
                !isExtractingCode && extractVerificationCode(emailItem)
              "
              :class="[
                'btn',
                'success',
                'small',
                { 'is-disabled': isExtractingCode },
              ]"
              title="获取验证码"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              {{ isExtractingCode ? "获取中..." : "获取验证码" }}
            </div>
            <div
              role="button"
              tabindex="0"
              @click.stop="viewEmailDetails(emailItem)"
              @keydown.enter.stop="viewEmailDetails(emailItem)"
              @keydown.space.prevent.stop="viewEmailDetails(emailItem)"
              class="btn primary small"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12 4.5C7 4.5 2.73 7.61 1 12C2.73 16.39 7 19.5 12 19.5S21.27 16.39 23 12C21.27 7.61 17 4.5 12 4.5ZM12 17C9.24 17 7 14.76 7 12S9.24 7 12 7S17 9.24 17 12S14.76 17 12 17ZM12 9C10.34 9 9 10.34 9 12S10.34 15 12 15S15 13.66 15 12S13.66 9 12 9Z"
                />
              </svg>
              查看详情
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 固定底部工具栏 -->
    <div class="form-footer">
      <div class="toolbar-left">
        <div
          style="margin-top: 10px"
          tabindex="0"
          @click="!isLoading && refreshEmails()"
          @keydown.enter="!isLoading && refreshEmails()"
          @keydown.space.prevent="!isLoading && refreshEmails()"
          :class="['btn', 'primary', { 'is-disabled': isLoading }]"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z"
            />
          </svg>
          {{ isLoading ? "加载中..." : "刷新" }}
        </div>
      </div>

      <div class="toolbar-center">
        <div
          style="margin-top: 10px"
          tabindex="0"
          @click="!(currentPage <= 1 || isLoading) && previousPage()"
          @keydown.enter="!(currentPage <= 1 || isLoading) && previousPage()"
          @keydown.space.prevent="
            !(currentPage <= 1 || isLoading) && previousPage()
          "
          :class="[
            'btn',
            'secondary',
            'small',
            { 'is-disabled': currentPage <= 1 || isLoading },
          ]"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M15.41,16.58L10.83,12L15.41,7.41L14,6L8,12L14,18L15.41,16.58Z"
            />
          </svg>
          上一页
        </div>
        <span class="page-info"
          >第 {{ currentPage }} 页 / 共 {{ totalPages }} 页</span
        >
        <div
          style="margin-top: 10px"
          tabindex="0"
          @click="!(currentPage >= totalPages || isLoading) && nextPage()"
          @keydown.enter="
            !(currentPage >= totalPages || isLoading) && nextPage()
          "
          @keydown.space.prevent="
            !(currentPage >= totalPages || isLoading) && nextPage()
          "
          :class="[
            'btn',
            'secondary',
            'small',
            { 'is-disabled': currentPage >= totalPages || isLoading },
          ]"
        >
          下一页
          <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z"
            />
          </svg>
        </div>
      </div>

      <div class="toolbar-right">
        <span class="toolbar-text">文件夹:</span>
        <select
          v-model="selectedFolder"
          @change="loadEmails"
          class="toolbar-select"
        >
          <option value="inbox">收件箱</option>
          <option value="junk">垃圾邮件</option>
        </select>
        <span class="toolbar-text" v-if="emails.length > 0"
          >共 {{ totalEmails }} 封邮件</span
        >
        <span class="toolbar-text" v-else-if="!isLoading">暂无邮件</span>
      </div>
    </div>
  </div>

  <!-- 邮件详情查看器 -->
  <ModalContainer
    :visible="showEmailDetails"
    title="邮件详情"
    size="large"
    @close="showEmailDetails = false"
  >
    <EmailDetails
      v-if="showEmailDetails"
      :email="email"
      :message-id="selectedEmailForDetails"
      @show-status="$emit('show-status', $event)"
    />
  </ModalContainer>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";
import EmailDetails from "./EmailDetails.vue";

const props = defineProps({
  email: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(["show-status"]);

// 响应式数据
const emails = ref([]);
const selectedFolder = ref("inbox");
const currentPage = ref(1);
const pageSize = ref(20);
const totalEmails = ref(0);
const isLoading = ref(false);
const isExtractingCode = ref(false);
const selectedEmailId = ref("");
const showEmailDetails = ref(false);
const selectedEmailForDetails = ref("");

// 计算属性
const totalPages = computed(() => {
  return Math.ceil(totalEmails.value / pageSize.value);
});

// 方法
const loadEmails = async () => {
  isLoading.value = true;
  try {
    const response = await invoke("outlook_get_emails", {
      email: props.email,
      folder: selectedFolder.value,
      page: currentPage.value,
      pageSize: pageSize.value,
    });

    emails.value = response.emails;
    totalEmails.value = response.total_emails;

    if (emails.value.length === 0 && currentPage.value > 1) {
      // 如果当前页没有邮件且不是第一页，回到第一页
      currentPage.value = 1;
      await loadEmails();
    }
  } catch (error) {
    emit("show-status", `加载邮件失败: ${error}`, "error");
    emails.value = [];
    totalEmails.value = 0;
  } finally {
    isLoading.value = false;
  }
};

const refreshEmails = async () => {
  currentPage.value = 1;
  await loadEmails();
  emit("show-status", "邮件列表已重新加载", "success");
};

const previousPage = async () => {
  if (currentPage.value > 1) {
    currentPage.value--;
    await loadEmails();
  }
};

const nextPage = async () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
    await loadEmails();
  }
};

const selectEmail = (emailItem) => {
  selectedEmailId.value = emailItem.message_id;
};

const viewEmailDetails = (emailItem) => {
  selectedEmailForDetails.value = emailItem.message_id;
  showEmailDetails.value = true;
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
    });
  } catch (error) {
    return dateString;
  }
};

const extractVerificationCode = async (emailItem) => {
  if (isExtractingCode.value) return;

  isExtractingCode.value = true;
  try {
    console.log("=== 开始提取验证码 ===");
    console.log("邮件项目:", emailItem);
    console.log("邮箱:", props.email);
    console.log("消息ID:", emailItem.message_id);

    // 获取邮件详细内容
    const emailDetails = await invoke("outlook_get_email_details", {
      email: props.email,
      messageId: emailItem.message_id,
    });

    console.log("=== 验证码提取 - 邮件详情返回数据 ===");
    console.log("完整返回数据:", emailDetails);
    console.log("数据类型:", typeof emailDetails);
    console.log("数据结构:", JSON.stringify(emailDetails, null, 2));
    console.log("=== 验证码提取 - 邮件详情数据结束 ===");

    // 解析验证码的正则表达式 - 针对你的验证码格式优化
    const codePatterns = [
      // 精确匹配 "Your verification code is: 554775" 格式（允许更多空白字符）
      /Your\s+verification\s+code\s+is:\s*(\d{4,8})/i,
      // 匹配其他常见格式
      /verification\s+code\s+is:\s*(\d{4,8})/i,
      /verification\s+code:\s*(\d{4,8})/i,
      // 更宽松的匹配
      /verification.*?code.*?(\d{4,8})/i,
      /code.*?(\d{4,8})/i,
      // 中文格式
      /验证码[：:]\s*(\d{4,8})/i,
      // 匹配独立的6位数字（最常见的验证码长度）
      /\b(\d{6})\b/g,
      // 匹配4-8位数字作为备选
      /\b(\d{4,8})\b/g,
    ];

    let extractedCode = null;
    // 优先使用后端定义的字段：body_plain / body_html
    let emailContent =
      emailDetails.body_plain ||
      emailDetails.body_html ||
      emailDetails.text_body ||
      emailDetails.body ||
      emailDetails.body_preview ||
      "";

    console.log("=== 邮件内容分析 ===");
    console.log("原始邮件内容长度:", emailContent.length);
    console.log(
      "原始邮件内容预览（前500字符）:",
      emailContent.substring(0, 500)
    );
    console.log("邮件详情可用字段:", Object.keys(emailDetails));
    console.log("=== 邮件内容分析结束 ===");

    // 处理HTML格式的邮件内容 - 去除HTML标签
    const stripHtml = (html) => {
      try {
        // 创建一个临时div元素来解析HTML
        const tempDiv = document.createElement("div");
        tempDiv.innerHTML = html;

        // 获取纯文本内容
        let textContent = tempDiv.textContent || tempDiv.innerText || "";

        // 清理多余的空白字符和换行符
        textContent = textContent
          .replace(/\s+/g, " ") // 多个空白字符合并为一个空格
          .replace(/\n+/g, " ") // 换行符替换为空格
          .trim();

        return textContent;
      } catch (error) {
        // 如果HTML解析失败，使用正则表达式作为备选方案
        return html
          .replace(/<[^>]*>/g, " ") // 移除HTML标签
          .replace(/&[^;]+;/g, " ") // 移除HTML实体
          .replace(/\s+/g, " ")
          .trim();
      }
    };

    // 如果内容包含HTML标签，先去除HTML
    if (emailContent.includes("<") && emailContent.includes(">")) {
      const originalContent = emailContent;
      emailContent = stripHtml(emailContent);

      // 调试信息：显示HTML处理前后的内容
      console.log("=== 邮件内容调试 ===");
      console.log("原始HTML内容长度:", originalContent.length);
      console.log("原始HTML内容预览:", originalContent.substring(0, 300));
      console.log("处理后纯文本长度:", emailContent.length);
      console.log("处理后纯文本内容:", emailContent.substring(0, 300));
      console.log("==================");
    } else {
      console.log("=== 邮件内容调试 ===");
      console.log("邮件内容（非HTML）:", emailContent.substring(0, 300));
      console.log("==================");
    }

    // 尝试匹配验证码 - 优先匹配精确格式
    console.log("开始验证码匹配，使用", codePatterns.length, "个模式");

    for (let i = 0; i < codePatterns.length; i++) {
      const pattern = codePatterns[i];
      console.log(`尝试模式 ${i + 1}:`, pattern.toString());

      const match = emailContent.match(pattern);
      if (match) {
        console.log(`模式 ${i + 1} 匹配成功:`, match);
        extractedCode = match[1] || match[0];
        console.log(`提取的验证码候选:`, extractedCode);

        // 对于通用数字匹配，进行额外验证
        if (i >= 5) {
          // 通用数字模式的索引
          console.log("进行通用数字模式验证...");
          // 确保是合理的验证码长度
          if (extractedCode.length >= 4 && extractedCode.length <= 8) {
            // 检查是否在验证码相关的上下文中
            const contextPattern = /(?:verification|code|verify|auth)/i;
            const surroundingText = emailContent.substring(
              Math.max(0, match.index - 50),
              Math.min(emailContent.length, match.index + 50)
            );
            console.log("上下文文本:", surroundingText);
            if (contextPattern.test(surroundingText)) {
              console.log("通用模式验证通过");
              break;
            } else {
              console.log("通用模式验证失败，跳过");
            }
          } else {
            console.log("验证码长度不符合要求，跳过");
          }
          continue; // 跳过不符合条件的通用匹配
        }
        console.log("精确模式匹配，直接使用");
        break; // 精确匹配直接使用
      } else {
        console.log(`模式 ${i + 1} 无匹配`);
      }
    }

    if (extractedCode) {
      // 自动复制验证码
      await copyVerificationCode(extractedCode);

      // 将验证码写回当前列表项，便于列表展示
      try {
        emailItem.verification_code = extractedCode;
        const idx = emails.value.findIndex(
          (e) => e.message_id === emailItem.message_id
        );
        if (idx !== -1) {
          emails.value[idx] = {
            ...emails.value[idx],
            verification_code: extractedCode,
          };
        }
      } catch (e) {
        console.warn("写回验证码到列表项失败（不影响复制）:", e);
      }

      emit("show-status", `验证码已提取并复制: ${extractedCode}`, "success");
      console.log(`=== 验证码提取成功 ===`);
      console.log(`提取的验证码: ${extractedCode}`);
      console.log(`=== 验证码提取成功结束 ===`);
    } else {
      emit("show-status", "未在邮件中找到验证码", "warning");
      console.log("=== 验证码提取失败 ===");
      console.log("完整邮件内容:", emailContent);
      console.log("邮件内容长度:", emailContent.length);
      console.log("邮件内容预览（前500字符）:", emailContent.substring(0, 500));
      console.log(
        "邮件内容预览（后500字符）:",
        emailContent.substring(Math.max(0, emailContent.length - 500))
      );
      console.log("=== 验证码提取失败结束 ===");
    }
  } catch (error) {
    console.error("=== 验证码提取异常 ===");
    console.error("错误信息:", error);
    console.error("错误类型:", typeof error);
    console.error("=== 验证码提取异常结束 ===");
    emit("show-status", `提取验证码失败: ${error}`, "error");
  } finally {
    isExtractingCode.value = false;
  }
};

const copyVerificationCode = async (code) => {
  try {
    await navigator.clipboard.writeText(code);
    emit("show-status", `验证码已复制: ${code}`, "success");
  } catch (error) {
    // 降级方案
    try {
      const textArea = document.createElement("textarea");
      textArea.value = code;
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand("copy");
      document.body.removeChild(textArea);
      emit("show-status", `验证码已复制: ${code}`, "success");
    } catch (fallbackError) {
      emit("show-status", "复制失败，请手动复制", "error");
    }
  }
};

// 生命周期
onMounted(() => {
  loadEmails();
});
</script>

<style scoped>
/* 完全对齐 TokenForm 的结构与样式 */
.token-form-content {
  display: flex;
  flex-direction: column;
  max-height: 58vh;
}

.form-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 0 20px 0;
}

/* 固定底部工具栏 */
.form-footer {
  display: flex;
  gap: 12px;
  padding: 20px 0 0 0;
  border-top: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
  margin: 0 -32px -32px -32px;
  padding: 20px 32px;
  flex-shrink: 0;
  align-items: center;
  justify-content: space-between;
}

.toolbar-left,
.toolbar-center,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.toolbar-center {
  flex: 1;
  justify-content: center;
}

.toolbar-right {
  justify-content: flex-end;
}

.toolbar-text {
  font-size: 14px;
  color: #64748b;
  white-space: nowrap;
}

.toolbar-select {
  padding: 6px 12px;
  border: 1px solid rgba(226, 232, 240, 0.5);
  border-radius: 6px;
  background: white;
  font-size: 14px;
  color: #1e293b;
}

.toolbar-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.page-info {
  font-size: 13px;
  color: #6b7280;
  white-space: nowrap;
  font-weight: 500;
}

/* 手绘按钮样式 */
.btn {
  padding: 10px 16px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 40px;
  position: relative;
  overflow: hidden;
  text-decoration: none;
  user-select: none;
}

.btn.small {
  padding: 8px 12px;
  font-size: 12px;
  min-height: 32px;
  gap: 6px;
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(59, 130, 246, 0.25);
  border: none;
}

.btn.primary:hover:not(.is-disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
  transform: translateY(-1px);
  box-shadow: 0 8px 20px rgba(59, 130, 246, 0.35);
}

.btn.secondary {
  background: rgba(241, 245, 249, 0.8);
  color: #475569;
  border: 1px solid rgba(226, 232, 240, 0.6);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.btn.secondary:hover:not(.is-disabled) {
  background: rgba(226, 232, 240, 0.9);
  border-color: rgba(203, 213, 225, 0.8);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.btn.success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(16, 185, 129, 0.25);
  border: none;
}

.btn.success:hover:not(.is-disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 8px 20px rgba(16, 185, 129, 0.35);
}

/* 禁用状态 */
.is-disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
  pointer-events: none;
}

/* 邮件列表样式 */
.emails-list {
  margin-top: 1px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.email-item {
  display: flex;
  align-items: center;
  padding: 16px;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.5);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(5px);
}

.email-item:hover {
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.1);
  transform: translateY(-1px);
}

.email-sender {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 200px;
}

.sender-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 16px;
}

.sender-info {
  flex: 1;
}

.sender-name {
  font-weight: 600;
  color: #1e293b;
  font-size: 14px;
}

.email-date {
  font-size: 12px;
  color: #64748b;
  margin-top: 2px;
}

.email-content {
  flex: 1;
  margin: 0 16px;
}

.email-subject {
  font-weight: 600;
  color: #1e293b;
  font-size: 15px;
  margin-bottom: 4px;
}

.email-meta-row {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: #64748b;
}

.email-folder {
  padding: 2px 8px;
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  border-radius: 4px;
  font-weight: 500;
}

.verification-code {
  display: flex;
  align-items: center;
  gap: 4px;
}

.code-value {
  font-family: monospace;
  background: rgba(16, 185, 129, 0.1);
  color: #059669;
  padding: 2px 6px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
}

.code-value:hover {
  background: rgba(16, 185, 129, 0.2);
}

.email-actions {
  display: flex;
  gap: 8px;
}

.email-status {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 12px;
}

.unread-indicator {
  color: #3b82f6;
  font-size: 12px;
}

.attachment-indicator {
  font-size: 14px;
}

.verification-indicator {
  color: #10b981;
  font-size: 12px;
  filter: drop-shadow(0 1px 2px rgba(16, 185, 129, 0.3));
}

/* 加载和空状态 */
.loading-state,
.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #6b7280;
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid rgba(59, 130, 246, 0.2);
  border-top: 3px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-hint {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 4px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .form-footer {
    padding: 16px 24px;
    margin: 0 -24px -24px -24px;
    flex-direction: column;
    gap: 16px;
  }

  .toolbar-left,
  .toolbar-center,
  .toolbar-right {
    justify-content: center;
    width: 100%;
  }

  .toolbar-center {
    order: 1;
  }

  .toolbar-left {
    order: 2;
  }

  .toolbar-right {
    order: 3;
  }

  .email-item {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .email-sender {
    min-width: auto;
  }

  .email-content {
    margin: 0;
  }

  .email-actions {
    justify-content: center;
  }

  .email-status {
    margin-left: 0;
    justify-content: center;
  }
}
</style>
