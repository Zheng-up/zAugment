<template>
  <ModalContainer
    :visible="visible"
    title="重置 Augment 配置"
    size="large"
    @close="handleClose"
  >
    <div class="editor-reset-content">
      <div class="warning-section">
        <div class="warning-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M12 2L1 21H23L12 2M12 6L19.53 19H4.47L12 6M11 10V14H13V10H11M11 16V18H13V16H11Z"
            />
          </svg>
        </div>
        <div class="warning-text">
          <h4>重要提醒 此操作将会：</h4>

          <ul>
            <li>关闭选中的编辑器进程</li>
            <li>清理 Augment 相关的数据</li>
            <li>重置编辑器的遥测标识</li>
          </ul>
        </div>
      </div>

      <div class="editor-selection">
        <h4>选择要重置的编辑器：</h4>
        <div class="editor-grid">
          <!-- VSCode 系列 -->
          <div class="editor-category">
            <h4 class="category-title">
              <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M23.15 2.587L18.21.21a1.494 1.494 0 0 0-1.705.29l-9.46 8.63-4.12-3.128a.999.999 0 0 0-1.276.057L.327 7.261A1 1 0 0 0 .326 8.74L3.899 12 .326 15.26a1 1 0 0 0 .001 1.479L1.65 17.94a.999.999 0 0 0 1.276.057l4.12-3.128 9.46 8.63a1.492 1.492 0 0 0 1.704.29l4.942-2.377A1.5 1.5 0 0 0 24 20.06V3.939a1.5 1.5 0 0 0-.85-1.352zm-5.146 14.861L10.826 12l7.178-5.448v10.896z"
                />
              </svg>
              VSCode 系列
            </h4>
            <div class="editor-options vscode-grid">
              <button
                @click="selectEditor('vscode')"
                class="editor-option vscode-option"
                :class="{ active: selectedEditor === 'vscode' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.vscode"
                    alt="VS Code"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">VS Code</span>
                </div>
              </button>
              <button
                @click="selectEditor('cursor')"
                class="editor-option cursor-option"
                :class="{ active: selectedEditor === 'cursor' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.cursor"
                    alt="Cursor"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Cursor</span>
                </div>
              </button>
              <button
                @click="selectEditor('kiro')"
                class="editor-option kiro-option"
                :class="{ active: selectedEditor === 'kiro' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.kiro"
                    alt="Kiro"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Kiro</span>
                </div>
              </button>
              <button
                @click="selectEditor('trae')"
                class="editor-option trae-option"
                :class="{ active: selectedEditor === 'trae' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.trae"
                    alt="Trae"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Trae</span>
                </div>
              </button>
              <button
                @click="selectEditor('windsurf')"
                class="editor-option windsurf-option"
                :class="{ active: selectedEditor === 'windsurf' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.windsurf"
                    alt="Windsurf"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Windsurf</span>
                </div>
              </button>
              <button
                @click="selectEditor('qoder')"
                class="editor-option qoder-option"
                :class="{ active: selectedEditor === 'qoder' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.qoder"
                    alt="Qoder"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Qoder</span>
                </div>
              </button>
              <button
                @click="selectEditor('vscodium')"
                class="editor-option vscodium-option"
                :class="{ active: selectedEditor === 'vscodium' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.vscodium"
                    alt="VSCodium"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">VSCodium</span>
                </div>
              </button>
              <button
                @click="selectEditor('codebuddy')"
                class="editor-option codebuddy-option"
                :class="{ active: selectedEditor === 'codebuddy' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.codebuddy"
                    alt="CodeBuddy"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">CodeBuddy</span>
                </div>
              </button>
            </div>
          </div>

          <!-- JetBrains 系列 -->
          <div class="editor-category">
            <h4 class="category-title">
              <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M0 0h24v24H0V0z" fill="none" />
                <path
                  d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0L19.2 12l-4.6-4.6L16 6l6 6-6 6-1.4-1.4z"
                />
              </svg>
              JetBrains 系列
            </h4>
            <div class="editor-options jetbrains-grid">
              <button
                @click="selectEditor('idea')"
                class="editor-option idea-option"
                :class="{ active: selectedEditor === 'idea' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.idea"
                    alt="IntelliJ IDEA"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">IntelliJ IDEA</span>
                </div>
              </button>
              <button
                @click="selectEditor('pycharm')"
                class="editor-option pycharm-option"
                :class="{ active: selectedEditor === 'pycharm' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.pycharm"
                    alt="PyCharm"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">PyCharm</span>
                </div>
              </button>
              <button
                @click="selectEditor('goland')"
                class="editor-option goland-option"
                :class="{ active: selectedEditor === 'goland' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.goland"
                    alt="GoLand"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">GoLand</span>
                </div>
              </button>
              <button
                @click="selectEditor('rustrover')"
                class="editor-option rustrover-option"
                :class="{ active: selectedEditor === 'rustrover' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.rustrover"
                    alt="RustRover"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">RustRover</span>
                </div>
              </button>
              <button
                @click="selectEditor('webstorm')"
                class="editor-option webstorm-option"
                :class="{ active: selectedEditor === 'webstorm' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.webstorm"
                    alt="WebStorm"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">WebStorm</span>
                </div>
              </button>
              <button
                @click="selectEditor('phpstorm')"
                class="editor-option phpstorm-option"
                :class="{ active: selectedEditor === 'phpstorm' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.phpstorm"
                    alt="PhpStorm"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">PhpStorm</span>
                </div>
              </button>
              <button
                @click="selectEditor('androidstudio')"
                class="editor-option androidstudio-option"
                :class="{ active: selectedEditor === 'androidstudio' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.androidstudio"
                    alt="Android Studio"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Android Studio</span>
                </div>
              </button>
              <button
                @click="selectEditor('clion')"
                class="editor-option clion-option"
                :class="{ active: selectedEditor === 'clion' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.clion"
                    alt="CLion"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">CLion</span>
                </div>
              </button>
              <button
                @click="selectEditor('datagrip')"
                class="editor-option datagrip-option"
                :class="{ active: selectedEditor === 'datagrip' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.datagrip"
                    alt="DataGrip"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">DataGrip</span>
                </div>
              </button>
              <button
                @click="selectEditor('rider')"
                class="editor-option rider-option"
                :class="{ active: selectedEditor === 'rider' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.rider"
                    alt="Rider"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Rider</span>
                </div>
              </button>
              <button
                @click="selectEditor('rubymine')"
                class="editor-option rubymine-option"
                :class="{ active: selectedEditor === 'rubymine' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.rubymine"
                    alt="RubyMine"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">RubyMine</span>
                </div>
              </button>
              <button
                @click="selectEditor('aqua')"
                class="editor-option aqua-option"
                :class="{ active: selectedEditor === 'aqua' }"
              >
                <div class="editor-icon">
                  <img
                    :src="editorIcons.aqua"
                    alt="Aqua"
                    width="32"
                    height="32"
                  />
                </div>
                <div class="editor-info">
                  <span class="editor-name">Aqua</span>
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="isProcessing" class="processing-status">
        <div class="processing-spinner"></div>
        <div class="processing-text">
          <h4>正在处理...</h4>
          <p>{{ currentStep }}</p>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-footer-buttons">
        <button
          @click="handleClose"
          class="btn secondary"
          :disabled="isProcessing"
        >
          {{ processResult ? "关闭" : "取消" }}
        </button>
        <button
          v-if="!processResult"
          @click="startReset"
          class="btn danger"
          :disabled="!selectedEditor || isProcessing"
        >
          <span v-if="isProcessing" class="loading-spinner"></span>
          {{ isProcessing ? "处理中..." : "开始重置" }}
        </button>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
});

// Emits
const emit = defineEmits(["close", "reset-complete"]);

// Reactive data
const selectedEditor = ref("");
const isProcessing = ref(false);
const currentStep = ref("");
const processResult = ref(null);

// 图标映射 - 与TokenCard保持一致
const editorIcons = {
  vscode: "/icons/vscode.svg",
  cursor: "/icons/cursor.svg",
  kiro: "/icons/kiro.svg",
  trae: "/icons/trae.svg",
  windsurf: "/icons/windsurf.svg",
  qoder: "/icons/qoder.svg",
  vscodium: "/icons/vscodium.svg",
  codebuddy: "/icons/codebuddy.svg",
  idea: "/icons/idea.svg",
  pycharm: "/icons/pycharm.svg",
  goland: "/icons/goland.svg",
  rustrover: "/icons/rustrover.svg",
  webstorm: "/icons/webstorm.svg",
  phpstorm: "/icons/phpstorm.svg",
  clion: "/icons/clion.svg",
  datagrip: "/icons/datagrip.svg",
  rider: "/icons/rider.svg",
  rubymine: "/icons/rubymine.svg",
  aqua: "/icons/aqua.svg",
  androidstudio: "/icons/androidstudio.svg",
};

// Methods
const selectEditor = (editorType) => {
  if (!isProcessing.value) {
    selectedEditor.value = editorType;
  }
};

const handleClose = () => {
  if (!isProcessing.value) {
    resetState();
    emit("close");
  }
};

const resetState = () => {
  selectedEditor.value = "";
  isProcessing.value = false;
  currentStep.value = "";
  processResult.value = null;
};

const startReset = async () => {
  if (!selectedEditor.value) return;

  isProcessing.value = true;
  processResult.value = null;

  // 用于收集所有步骤的详细结果
  const stepResults = [];

  try {
    // 步骤1: 关闭编辑器进程
    console.log("=== 开始步骤1: 关闭编辑器进程 ===");
    currentStep.value = "正在关闭编辑器进程...";
    const closeResult = await invoke("close_editor_processes", {
      editorType: selectedEditor.value,
    });
    console.log("关闭进程结果:", closeResult);
    stepResults.push(`✅ 步骤1: ${closeResult}`);

    // 等待一段时间确保进程完全关闭
    console.log("等待2秒确保进程完全关闭...");
    currentStep.value = "等待进程完全关闭...";
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // 步骤2: 清理数据库
    console.log("=== 开始步骤2: 清理数据库 ===");
    currentStep.value = "正在清理 Augment 数据...";
    const cleanResult = await invoke("clean_editor_database", {
      editorType: selectedEditor.value,
      keyword: "augment",
    });
    console.log("清理数据库结果:", cleanResult);
    stepResults.push(`✅ 步骤2: ${cleanResult}`);

    // 步骤3: 修改遥测ID
    console.log("=== 开始步骤3: 重置遥测ID ===");
    currentStep.value = "正在重置遥测标识...";
    const resetResult = await invoke("reset_editor_telemetry", {
      editorType: selectedEditor.value,
    });
    console.log("重置遥测结果:", resetResult);
    stepResults.push(`✅ 步骤3: ${resetResult}`);

    // 发送重置完成事件
    emit("reset-complete", {
      editor: selectedEditor.value,
      success: true,
      details: stepResults, // 传递详细步骤结果
    });

    // 等待1秒后自动关闭弹窗
    setTimeout(() => {
      emit("close");
    }, 1000);
  } catch (error) {
    console.error("重置失败:", error);
    processResult.value = {
      success: false,
      message: `重置失败: ${error.message || error}`,
    };

    emit("reset-complete", {
      editor: selectedEditor.value,
      success: false,
      error: error.message || error,
    });
  } finally {
    isProcessing.value = false;
    currentStep.value = "";
  }
};
</script>

<style scoped>
.editor-reset-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 8px 0;
}

.warning-section {
  display: flex;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.05),
    rgba(245, 158, 11, 0.05)
  );
  border: 1px solid rgba(239, 68, 68, 0.1);
  border-radius: 12px;
}

.warning-icon {
  color: #f59e0b;
  flex-shrink: 0;
}

.warning-text h4 {
  margin: 0 0 12px 0;
  color: #dc2626;
  font-size: 16px;
  font-weight: 600;
}

.warning-text p {
  margin: 0 0 8px 0;
  color: #666;
  line-height: 1.5;
}

.warning-text ul {
  margin: 8px 0;
  padding-left: 20px;
  color: #666;
}

.warning-text li {
  margin-bottom: 4px;
}

.warning-note {
  font-weight: 500;
  color: #dc2626 !important;
  margin-top: 12px !important;
}

.editor-selection h4 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

/* 编辑器网格布局 - 与TokenCard保持一致 */
.editor-grid {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.editor-category {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.category-title {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 18px;
  font-weight: 700;
  color: #1e293b;
  margin: 0;
  padding: 16px 0 8px 0;
  border-bottom: 2px solid rgba(226, 232, 240, 0.6);
}

.category-title svg {
  color: #3b82f6;
}

.editor-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.vscode-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.jetbrains-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.editor-option {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  border: 2px solid rgba(226, 232, 240, 0.4);
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  text-decoration: none;
  color: inherit;
  width: 100%;
  text-align: left;
  font-family: inherit;
  font-size: inherit;
  box-sizing: border-box;
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.editor-option:hover {
  border-color: rgba(59, 130, 246, 0.4);
  background: rgba(248, 250, 252, 0.95);
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.08),
    0 0 0 1px rgba(59, 130, 246, 0.05);
  transform: translateY(-2px);
}

.editor-option:active {
  background: #f1f5f9;
  box-shadow: 0 1px 4px rgba(59, 130, 246, 0.08);
}

.editor-option.active {
  border-color: #3b82f6;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1),
    rgba(99, 102, 241, 0.05)
  );
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.15);
}

.editor-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(
    135deg,
    rgba(255, 255, 255, 0.9) 0%,
    rgba(248, 250, 252, 0.8) 100%
  );
  border: 1px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(8px);
  transition: all 0.3s ease;
}

.editor-option:hover .editor-icon {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1) 0%,
    rgba(147, 197, 253, 0.05) 100%
  );
  border-color: rgba(59, 130, 246, 0.2);
}

.editor-icon img {
  width: 36px;
  height: 36px;
  object-fit: contain;
  transition: transform 0.3s ease;
}

.editor-option:hover .editor-icon img {
  transform: scale(1.1);
}

.editor-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.editor-name {
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
  transition: color 0.3s ease;
}

.editor-option:hover .editor-name {
  color: #3b82f6;
}

.processing-status {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.05),
    rgba(99, 102, 241, 0.05)
  );
  border: 1px solid rgba(59, 130, 246, 0.1);
  border-radius: 12px;
}

.processing-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #e5e7eb;
  border-top: 3px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.processing-text h4 {
  margin: 0 0 4px 0;
  color: #3b82f6;
  font-size: 16px;
  font-weight: 600;
}

.processing-text p {
  margin: 0;
  color: #666;
  font-size: 14px;
}

.result-section {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  border-radius: 12px;
}

.result-section .success {
  background: linear-gradient(
    135deg,
    rgba(34, 197, 94, 0.05),
    rgba(16, 185, 129, 0.05)
  );
  border: 1px solid rgba(34, 197, 94, 0.1);
}

.result-section .error {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.05),
    rgba(220, 38, 38, 0.05)
  );
  border: 1px solid rgba(239, 68, 68, 0.1);
}

.result-icon.success {
  color: #22c55e;
}

.result-icon.error {
  color: #ef4444;
}

.result-text h4 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
}

.result-text p {
  margin: 0;
  color: #666;
  font-size: 14px;
  white-space: pre-line;
  line-height: 1.5;
}

.modal-footer-buttons {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn.secondary {
  background: #f3f4f6;
  color: #374151;
}

.btn.secondary:hover:not(:disabled) {
  background: #e5e7eb;
}

.btn.danger {
  background: #ef4444;
  color: white;
}

.btn.danger:hover:not(:disabled) {
  background: #dc2626;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top: 2px solid white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
