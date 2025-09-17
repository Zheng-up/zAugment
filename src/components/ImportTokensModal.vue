<template>
  <ModalContainer
    :visible="visible"
    title="导入账号"
    size="large"
    @close="handleClose"
  >
    <div class="import-modal-content">
      <!-- 文本域输入 -->
      <div class="text-input-section">
        <div class="section-header">
          <h4>粘贴JSON数据或导入</h4>
          <div class="header-buttons">
            <button
              @click="handleFileImport"
              class="action-btn file-import"
              :disabled="isImporting"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"
                />
              </svg>
              从本地文件导入
            </button>
            <button
              @click="fillExample"
              class="action-btn example"
              type="button"
              :disabled="isImporting"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M19,3H5C3.9,3 3,3.9 3,5V19C3.9,19 5,19 5,19H19C20.1,19 21,18.1 21,17V5C21,3.9 20.1,3 19,3M19,17H5V5H19V17Z"
                />
              </svg>
              填入示例数据
            </button>
          </div>
        </div>

        <textarea
          v-model="jsonText"
          placeholder="请粘贴JSON格式的账号数据..."
          class="json-textarea"
          :disabled="isImporting"
          rows="5"
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

    <template #footer>
      <div class="import-footer">
        <button
          @click="handleClose"
          class="btn secondary"
          :disabled="isImporting"
        >
          取消
        </button>
        <button
          @click="handleImportFromText"
          class="btn primary"
          :disabled="isImporting || !jsonText.trim()"
        >
          <span v-if="isImporting" class="loading-spinner"></span>
          {{ isImporting ? "导入中..." : "开始导入" }}
        </button>
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
  />
</template>

<script setup>
import { ref } from "vue";
import ModalContainer from "./ModalContainer.vue";

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
});

// Emits
const emit = defineEmits(["close", "import", "import-error"]);

// Refs
const fileInput = ref(null);
const jsonText = ref("");
const isImporting = ref(false);

// 示例数据（使用对象序列格式）
const exampleData = `{
  "tenant_url": "租户URL",
  "access_token": "Token",
  "portal_url": "View usage URL"
},
{
  "tenant_url": "租户URL",
  "access_token": "Token",
  "portal_url": "View usage URL"
}`;

// Methods
const handleClose = () => {
  if (!isImporting.value) {
    jsonText.value = "";
    emit("close");
  }
};

const fillExample = () => {
  jsonText.value = exampleData;
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
    // 清空文件输入
    if (fileInput.value) {
      fileInput.value.value = "";
    }
  }
};

const handleImportFromText = async () => {
  if (!jsonText.value.trim()) return;

  isImporting.value = true;
  try {
    let importedData;
    const trimmedText = jsonText.value.trim();

    console.log("原始文本长度:", trimmedText.length);
    console.log("文本开始:", trimmedText.substring(0, 100));
    console.log("文本结束:", trimmedText.substring(trimmedText.length - 100));
    console.log("完整文本:", trimmedText);

    // 更健壮的解析逻辑
    try {
      if (trimmedText.startsWith("[") && trimmedText.endsWith("]")) {
        // 标准JSON数组格式
        console.log("尝试解析标准JSON数组");
        importedData = JSON.parse(trimmedText);
      } else if (trimmedText.startsWith("{")) {
        // 检查是否以 } 结尾，或者以 } 后跟逗号和换行结尾
        const endsWithObject =
          trimmedText.endsWith("}") || /}\s*,?\s*$/.test(trimmedText);

        if (endsWithObject) {
          // 对象或对象序列格式
          if (trimmedText.includes("},{") || /}\s*,\s*{/.test(trimmedText)) {
            // 多个对象，需要包装成数组
            console.log("尝试解析对象序列，包装为数组");
            // 清理末尾可能的逗号
            const cleanedForArray = trimmedText.replace(/,\s*$/, "");
            const wrappedText = `[${cleanedForArray}]`;
            console.log("包装后的文本:", wrappedText.substring(0, 200) + "...");
            importedData = JSON.parse(wrappedText);
          } else {
            // 单个对象
            console.log("尝试解析单个对象");
            // 清理末尾可能的逗号
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
      console.error("JSON解析详细错误:", parseError);

      // 尝试清理可能的隐藏字符
      const cleanedText = trimmedText
        .replace(/[\u200B-\u200D\uFEFF]/g, "") // 移除零宽字符
        .replace(/[""]/g, '"') // 替换中文引号
        .replace(/['']/g, "'"); // 替换中文单引号

      if (cleanedText !== trimmedText) {
        console.log("检测到特殊字符，尝试清理后重新解析");
        try {
          if (
            cleanedText.startsWith("{") &&
            cleanedText.endsWith("}") &&
            cleanedText.includes("},{")
          ) {
            const wrappedCleanText = `[${cleanedText}]`;
            importedData = JSON.parse(wrappedCleanText);
          } else if (cleanedText.startsWith("{") && cleanedText.endsWith("}")) {
            // 单个对象，包装成数组
            const singleObject = JSON.parse(cleanedText);
            importedData = [singleObject];
          } else if (cleanedText.startsWith("[") && cleanedText.endsWith("]")) {
            // 数组格式
            importedData = JSON.parse(cleanedText);
          } else {
            importedData = JSON.parse(cleanedText);
          }
        } catch (secondError) {
          throw parseError; // 如果清理后仍然失败，抛出原始错误
        }
      } else {
        throw parseError;
      }
    }

    // 确保结果是数组
    if (!Array.isArray(importedData)) {
      importedData = [importedData];
    }

    console.log("解析成功，对象数量:", importedData.length);

    // 验证数组中的每个元素都有必需字段
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
    console.error("JSON解析失败:", error);
    emit("import-error", `JSON格式错误：${error.message}`);
  } finally {
    isImporting.value = false;
  }
};
</script>

<style scoped>
.import-modal-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 8px 0;
}

.import-methods {
  display: flex;
  justify-content: center;
  margin-bottom: 8px;
}

.import-method-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 24px;
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 12px;
  background: rgba(248, 250, 252, 0.8);
  color: #374151;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.import-method-btn:hover:not(:disabled) {
  border-color: rgba(59, 130, 246, 0.5);
  background: rgba(239, 246, 255, 0.9);
  color: #1d4ed8;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
}

.import-method-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

.text-input-section {
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
  letter-spacing: -0.01em;
}

.header-buttons {
  display: flex;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 8px;
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(5px);
}

.action-btn:hover:not(:disabled) {
  background: rgba(241, 245, 249, 0.9);
  border-color: rgba(203, 213, 225, 0.7);
  color: #475569;
  transform: translateY(-1px);
}

.action-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

.json-textarea {
  width: 100%;
  min-height: 80px;
  padding: 14px 16px;
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 12px;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: vertical;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  background: rgba(248, 250, 252, 0.5);
  backdrop-filter: blur(5px);
  box-sizing: border-box;
}

.json-textarea:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.5);
  box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.1), 0 1px 3px rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
}

.json-textarea:disabled {
  background: rgba(241, 245, 249, 0.7);
  color: #64748b;
  cursor: not-allowed;
  border-color: rgba(203, 213, 225, 0.5);
  opacity: 0.6;
}

.format-info {
  background: rgba(248, 250, 252, 0.8);
  border: 2px solid rgba(226, 232, 240, 0.3);
  border-radius: 12px;
  padding: 20px;
  backdrop-filter: blur(10px);
}

.format-info h5 {
  margin: 0 0 16px 0;
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  letter-spacing: -0.01em;
}

.format-info h6 {
  margin: 0 0 8px 0;
  font-size: 13px;
  font-weight: 600;
  color: #475569;
  letter-spacing: -0.01em;
}

.format-example {
  margin-bottom: 20px;
}

.format-example:last-of-type {
  margin-bottom: 16px;
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
}

.format-notes p {
  margin: 0;
  font-size: 13px;
  color: #64748b;
  line-height: 1.4;
}

.import-footer {
  display: flex;
  gap: 12px;
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 24px;
  border: none;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  flex: 1;
  justify-content: center;
  min-height: 48px;
  letter-spacing: -0.01em;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

.btn.secondary {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border: 2px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(10px);
}

.btn.secondary:hover:not(:disabled) {
  background: rgba(241, 245, 249, 0.9);
  border-color: rgba(203, 213, 225, 0.7);
  color: #475569;
  transform: translateY(-1px);
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  border: 2px solid transparent;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
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
</style>
