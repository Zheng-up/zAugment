<template>
  <div class="token-card">
    <!-- 状态指示器 -->
    <div
      v-if="(token.portal_url && portalInfo.data) || token.ban_status"
      class="status-indicator"
    >
      <!-- 综合状态显示：结合API状态和Portal信息 -->
      <span
        :class="[
          'status-badge',
          getStatusClass(
            getComprehensiveStatus(token.ban_status, portalInfo.data)
          ),
        ]"
      >
        {{
          getStatusText(
            getComprehensiveStatus(token.ban_status, portalInfo.data)
          )
        }}
      </span>
    </div>

    <div class="card-main">
      <div class="token-info">
        <h3 class="tenant-name">{{ displayUrl }}</h3>
        <div class="token-meta">
          <!-- 第一行：创建日期和邮箱 -->
          <div class="meta-row">
            <span class="created-date">{{ formatDate(token.created_at) }}</span>
            <div v-if="token.email_note" class="email-note-container">
              <div
                class="email-note-wrapper clickable"
                @click="copyEmailNote"
                title="点击复制邮箱"
              >
                <span
                  class="email-note"
                  ref="emailNoteRef"
                  @mouseenter="showTooltip"
                  @mouseleave="hideTooltip"
                >
                  <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="email-icon"
                  >
                    <path
                      d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.89 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
                    />
                  </svg>
                  {{ maskedEmail }}
                </span>
              </div>
            </div>
          </div>
          <!-- 第二行：Portal信息 -->
          <template v-if="token.portal_url">
            <div class="meta-row portal-row">
              <!-- 优先显示Portal数据，无论是来自本地缓存还是网络请求 -->
              <template v-if="portalInfo.data">
                <span
                  v-if="portalInfo.data.expiry_date"
                  class="portal-meta expiry"
                  :style="getExpiryStyle(portalInfo.data.expiry_date)"
                  >剩余：{{
                    formatExpiryDate(portalInfo.data.expiry_date)
                  }}</span
                >
                <span
                  class="portal-meta balance"
                  :style="getBalanceStyle(portalInfo.data.credits_balance)"
                  >额度：{{ portalInfo.data.credits_balance }}次</span
                >
              </template>
              <!-- 如果没有数据且正在加载，显示加载状态 -->
              <span v-else-if="isLoadingPortalInfo" class="portal-meta loading"
                >加载中...</span
              >
              <!-- 不显示错误信息，静默处理所有错误 -->
            </div>
          </template>
        </div>
      </div>

      <div class="actions">
        <button
          @click="openEditorModal"
          class="btn-action vscode"
          title="选择编辑器登录"
        >
          <img
            :src="editorIcons.vscode"
            alt="选择编辑器登录"
            width="18"
            height="18"
          />
        </button>
        <button @click="copyToken" class="btn-action copy" title="复制Token">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
            />
          </svg>
        </button>
        <button
          @click="copyTenantUrl"
          class="btn-action link"
          title="复制租户URL"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"
            />
          </svg>
        </button>
        <button
          @click="checkAccountStatus"
          :class="['btn-action', 'status-check', { loading: isCheckingStatus }]"
          :disabled="isCheckingStatus"
          title="刷新账号数据"
        >
          <svg
            v-if="!isCheckingStatus"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path
              d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
            />
          </svg>
          <div v-else class="loading-spinner"></div>
        </button>
        <button
          v-if="token.portal_url"
          @click="$emit('open-portal', token)"
          class="btn-action portal"
          title="打开View usage"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"
            />
          </svg>
        </button>
        <button
          @click="$emit('edit', token)"
          class="btn-action edit"
          title="编辑账号"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"
            />
          </svg>
        </button>
        <button @click="deleteToken" class="btn-action delete" title="删除账号">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>

  <!-- 编辑器选择模态框 -->
  <ModalContainer
    :visible="showEditorModal"
    title="选择编辑器登录"
    size="large"
    @close="showEditorModal = false"
  >
    <div class="editor-modal-content">
      <p class="modal-description">
        选择您使用的编辑器，我们将为您自动打开并登录账号：
      </p>

      <div class="editor-grid">
        <!-- VSCode 系列 -->
        <div class="editor-category">
          <h4 class="category-title">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M23.15 2.587L18.21.21a1.494 1.494 0 0 0-1.705.29l-9.46 8.63-4.12-3.128a.999.999 0 0 0-1.276.057L.327 7.261A1 1 0 0 0 .326 8.74L3.899 12 .326 15.26a1 1 0 0 0 .001 1.479L1.65 17.94a.999.999 0 0 0 1.276.057l4.12-3.128 9.46 8.63a1.492 1.492 0 0 0 1.704.29l4.942-2.377A1.5 1.5 0 0 0 24 20.06V3.939a1.5 1.5 0 0 0-.85-1.352zm-5.146 14.861L10.826 12l7.178-5.448v10.896z"
              />
            </svg>
            VSCode 系列
          </h4>
          <div class="vscode-grid">
            <button
              @click="handleEditorClick('vscode')"
              class="editor-option vscode-option"
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
              @click="handleEditorClick('cursor')"
              class="editor-option cursor-option"
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
              @click="handleEditorClick('kiro')"
              class="editor-option kiro-option"
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
              @click="handleEditorClick('trae')"
              class="editor-option trae-option"
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
              @click="handleEditorClick('windsurf')"
              class="editor-option windsurf-option"
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
              @click="handleEditorClick('qoder')"
              class="editor-option qoder-option"
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
              @click="handleEditorClick('vscodium')"
              class="editor-option vscodium-option"
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
              @click="handleEditorClick('codebuddy')"
              class="editor-option codebuddy-option"
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

        <!-- JetBrains 系编辑器区域 -->
        <div class="editor-category">
          <h4 class="category-title">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M0 0h24v24H0V0z" fill="none" />
              <path
                d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0L19.2 12l-4.6-4.6L16 6l6 6-6 6-1.4-1.4z"
              />
            </svg>
            JetBrains 系列
          </h4>
          <div class="jetbrains-grid">
            <button
              @click="handleEditorClick('idea')"
              class="editor-option idea-option"
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
              @click="handleEditorClick('pycharm')"
              class="editor-option pycharm-option"
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
              @click="handleEditorClick('goland')"
              class="editor-option goland-option"
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
              @click="handleEditorClick('rustrover')"
              class="editor-option rustrover-option"
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
              @click="handleEditorClick('webstorm')"
              class="editor-option webstorm-option"
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
              @click="handleEditorClick('phpstorm')"
              class="editor-option phpstorm-option"
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
              @click="handleEditorClick('androidstudio')"
              class="editor-option androidstudio-option"
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
              @click="handleEditorClick('clion')"
              class="editor-option clion-option"
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
              @click="handleEditorClick('datagrip')"
              class="editor-option datagrip-option"
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
              @click="handleEditorClick('rider')"
              class="editor-option rider-option"
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
              @click="handleEditorClick('rubymine')"
              class="editor-option rubymine-option"
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
              @click="handleEditorClick('aqua')"
              class="editor-option aqua-option"
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
  </ModalContainer>

  <!-- Email Tooltip - 使用 Teleport 渲染到 body -->
  <Teleport to="body">
    <div
      v-if="showEmailTooltip"
      class="email-tooltip"
      :style="tooltipStyle"
      @mouseenter="onTooltipMouseEnter"
      @mouseleave="onTooltipMouseLeave"
    >
      {{ token.email_note }}
      <div class="tooltip-arrow"></div>
    </div>
  </Teleport>
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";

// 防抖函数
function debounce(func, wait) {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

// Props
const props = defineProps({
  token: {
    type: Object,
    required: true,
  },
});

// Emits
const emit = defineEmits([
  "delete",
  "copy-success",
  "open-portal",
  "edit",
  "token-updated",
]);

// Reactive data
const isLoadingPortalInfo = ref(false);
const portalInfo = ref({ data: null, error: null });
const isCheckingStatus = ref(false);
const showEmailTooltip = ref(false);
const tooltipStyle = ref({});
const emailNoteRef = ref(null);
const showEditorModal = ref(false);
const isModalClosing = ref(false);
const tooltipHideTimer = ref(null);

// 图标映射
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

// Computed properties
const displayUrl = computed(() => {
  try {
    const url = new URL(props.token.tenant_url);
    return url.hostname;
  } catch {
    return props.token.tenant_url;
  }
});

const maskedToken = computed(() => {
  const token = props.token.access_token;
  if (token.length <= 20) return token;
  return token.substring(0, 10) + "..." + token.substring(token.length - 10);
});

const maskedEmail = computed(() => {
  const email = props.token.email_note;
  if (!email || !email.includes("@")) return email;

  const [username, domain] = email.split("@");

  // 如果用户名太短，直接返回原邮箱
  if (username.length <= 3) {
    return email;
  }

  let maskedUsername;
  if (username.length <= 6) {
    // 短邮箱：保留前1-2个字符，其余用星号替换
    const keepChars = username.length <= 4 ? 1 : 2;
    const hiddenCount = username.length - keepChars;
    maskedUsername = username.substring(0, keepChars) + "*".repeat(hiddenCount);
  } else {
    // 长邮箱：保留前后各2-3个字符，中间用4个星号替换
    const frontKeep = username.length >= 8 ? 3 : 2;
    const backKeep = 2;
    maskedUsername =
      username.substring(0, frontKeep) +
      "****" +
      username.substring(username.length - backKeep);
  }

  return maskedUsername + "@" + domain;
});

// Methods
const formatDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
};

const deleteToken = () => {
  // 直接发出删除事件，让父组件处理确认逻辑
  emit("delete", props.token.id);
};

// 复制到剪贴板的通用方法
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch (error) {
    // 备用方案
    const textArea = document.createElement("textarea");
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.select();
    document.execCommand("copy");
    document.body.removeChild(textArea);
    return true;
  }
};

// 复制Token
const copyToken = async () => {
  const success = await copyToClipboard(props.token.access_token);
  if (success) {
    emit("copy-success", "Token已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制Token失败", "error");
  }
};

// 复制租户URL
const copyTenantUrl = async () => {
  const success = await copyToClipboard(props.token.tenant_url);
  if (success) {
    emit("copy-success", "租户URL已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制租户URL失败", "error");
  }
};

// 复制邮箱备注
const copyEmailNote = async () => {
  const success = await copyToClipboard(props.token.email_note);
  if (success) {
    emit("copy-success", "邮箱备注已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制邮箱备注失败", "error");
  }
};

// 键盘事件处理
const handleKeydown = (event) => {
  if (event.key === "Escape" && showEditorModal.value) {
    showEditorModal.value = false;
  }
};

// 打开编辑器模态框
const openEditorModal = () => {
  showEditorModal.value = true;
};

// 关闭模态框
const closeModal = (event) => {
  if (isModalClosing.value) return;

  // 如果事件来自模态框内部，不关闭
  if (event && event.target.closest(".editor-modal")) {
    return;
  }

  showEditorModal.value = false;
  isModalClosing.value = false;
};

// 生成 Cursor 协议 URL
const getCursorProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");

    const augmentUrl = `cursor://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `cursor://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate Cursor protocol URL:", error);
    return "#";
  }
};

// 生成 VS Code 协议 URL
const getVSCodeProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `vscode://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `vscode://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate VS Code protocol URL:", error);
    return "#";
  }
};

// 生成 Kiro 协议 URL
const getKiroProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `kiro://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `kiro://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate Kiro protocol URL:", error);
    return "#";
  }
};

// 生成 Trae 协议 URL
const getTraeProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `trae://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `trae://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate Trae protocol URL:", error);
    return "#";
  }
};

// 生成 Windsurf 协议 URL
const getWindsurfProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `windsurf://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `windsurf://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate Windsurf protocol URL:", error);
    return "#";
  }
};

// 生成 Qoder 协议 URL
const getQoderProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `qoder://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `qoder://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate Qoder protocol URL:", error);
    return "#";
  }
};

// 生成 VSCodium 协议 URL
const getVSCodiumProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `vscodium://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `vscodium://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate VSCodium protocol URL:", error);
    return "#";
  }
};

// 生成 CodeBuddy 协议 URL
const getCodeBuddyProtocolUrl = () => {
  try {
    const token = encodeURIComponent(props.token.access_token);
    const url = encodeURIComponent(props.token.tenant_url);
    const portalUrl = encodeURIComponent(props.token.portal_url || "");
    const augmentUrl = `codebuddy://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`;
    const balanceUrl = `codebuddy://zheng.Zaugment-balance/autoConfig?portal=${portalUrl}`;

    return { augmentUrl, balanceUrl };
  } catch (error) {
    console.error("Failed to generate CodeBuddy protocol URL:", error);
    return "#";
  }
};

// 为 JetBrains 编辑器创建 JSON 文件
const createJetBrainsTokenFile = async (editorType) => {
  try {
    // 创建 JSON 数据
    const tokenData = {
      url: props.token.tenant_url,
      token: props.token.access_token,
      timestamp: Date.now(),
      ide: editorType,
    };

    // 调用 Tauri 后端命令创建文件
    const result = await invoke("create_jetbrains_token_file", {
      editorType,
      tokenData: JSON.stringify(tokenData, null, 2),
    });

    return { success: true, filePath: result };
  } catch (error) {
    console.error(`Failed to create ${editorType} token file:`, error);
    return { success: false, error: error.toString() };
  }
};

// 处理编辑器链接点击事件（修改后）
const handleEditorClick = async (editorType) => {
  try {
    // 关闭模态框
    showEditorModal.value = false;
    isModalClosing.value = false;

    // 定义 JetBrains 系编辑器列表
    const jetbrainsEditors = [
      "idea",
      "pycharm",
      "goland",
      "rustrover",
      "webstorm",
      "phpstorm",
      "androidstudio",
      "clion",
      "datagrip",
      "rider",
      "rubymine",
      "aqua",
    ];

    // 获取编辑器名称
    const getEditorName = (type) => {
      const editorNames = {
        cursor: "Cursor",
        vscode: "VS Code",
        kiro: "Kiro",
        trae: "Trae",
        windsurf: "Windsurf",
        qoder: "Qoder",
        vscodium: "VSCodium",
        codebuddy: "CodeBuddy",
        idea: "IntelliJ IDEA",
        pycharm: "PyCharm",
        goland: "GoLand",
        rustrover: "RustRover",
        webstorm: "WebStorm",
        phpstorm: "PhpStorm",
        androidstudio: "Android Studio",
        clion: "CLion",
        datagrip: "DataGrip",
        rider: "Rider",
        rubymine: "RubyMine",
        aqua: "Aqua",
      };
      return editorNames[type] || type;
    };

    const editorName = getEditorName(editorType);

    // 检查是否为 JetBrains 系编辑器
    if (jetbrainsEditors.includes(editorType)) {
      // JetBrains 编辑器保持原有逻辑
      const result = await createJetBrainsTokenFile(editorType);
      if (result.success) {
        emit("copy-success", `${editorName} Token 文件已创建`, "success");
      } else {
        emit(
          "copy-success",
          `创建 ${editorName} Token 文件失败: ${result.error}`,
          "error"
        );
      }
    } else {
      // VSCode 系编辑器使用双协议 URL 方式
      let protocolUrls;

      switch (editorType) {
        case "cursor":
          protocolUrls = getCursorProtocolUrl();
          break;
        case "vscode":
          protocolUrls = getVSCodeProtocolUrl();
          break;
        case "kiro":
          protocolUrls = getKiroProtocolUrl();
          break;
        case "trae":
          protocolUrls = getTraeProtocolUrl();
          break;
        case "windsurf":
          protocolUrls = getWindsurfProtocolUrl();
          break;
        case "qoder":
          protocolUrls = getQoderProtocolUrl();
          break;
        case "vscodium":
          protocolUrls = getVSCodiumProtocolUrl();
          break;
        case "codebuddy":
          protocolUrls = getCodeBuddyProtocolUrl();
          break;
        default:
          throw new Error(`Unknown VSCode editor type: ${editorType}`);
      }

      // 先调用 Augment 主插件
      await invoke("open_editor_with_protocol", {
        protocolUrl: protocolUrls.augmentUrl,
      });

      // 延迟调用 Balance 插件，确保主插件先完成配置
      // setTimeout(async () => {
      //   try {
      //     await invoke("open_editor_with_protocol", {
      //       protocolUrl: protocolUrls.balanceUrl,
      //     });
      //     console.log(`Balance plugin called for ${editorName}`);
      //   } catch (error) {
      //     console.error(
      //       `Failed to call Balance plugin for ${editorName}:`,
      //       error
      //     );
      //   }
      // }, 1000);

      emit("copy-success", `正在打开 ${editorName}...`, "success");
    }
  } catch (error) {
    console.error("Failed to handle editor click:", error);
    emit("copy-success", "操作失败", "error");
    showEditorModal.value = false;
    isModalClosing.value = false;
  }
};

// Tooltip 处理函数
const showTooltip = () => {
  // 清除任何待执行的隐藏定时器
  if (tooltipHideTimer.value) {
    clearTimeout(tooltipHideTimer.value);
    tooltipHideTimer.value = null;
  }

  if (!emailNoteRef.value) return;

  const rect = emailNoteRef.value.getBoundingClientRect();
  const tooltipWidth = 200; // 预估tooltip宽度

  // 计算tooltip位置
  let left = rect.left + rect.width / 2 - tooltipWidth / 2;
  const top = rect.top - 40; // 在元素上方显示

  // 确保tooltip不会超出屏幕边界
  if (left < 10) {
    left = 10;
  } else if (left + tooltipWidth > window.innerWidth - 10) {
    left = window.innerWidth - tooltipWidth - 10;
  }

  tooltipStyle.value = {
    position: "fixed",
    left: `${left}px`,
    top: `${top}px`,
    zIndex: 10000,
  };

  showEmailTooltip.value = true;
};

const hideTooltip = () => {
  // 延迟隐藏，给用户时间移动到tooltip上
  tooltipHideTimer.value = setTimeout(() => {
    showEmailTooltip.value = false;
    tooltipHideTimer.value = null;
  }, 100); // 100ms延迟
};

// Tooltip鼠标事件处理
const onTooltipMouseEnter = () => {
  // 鼠标进入tooltip，取消隐藏
  if (tooltipHideTimer.value) {
    clearTimeout(tooltipHideTimer.value);
    tooltipHideTimer.value = null;
  }
};

const onTooltipMouseLeave = () => {
  // 鼠标离开tooltip，立即隐藏
  showEmailTooltip.value = false;
  if (tooltipHideTimer.value) {
    clearTimeout(tooltipHideTimer.value);
    tooltipHideTimer.value = null;
  }
};

const extractTokenFromPortalUrl = (portalUrl) => {
  try {
    const url = new URL(portalUrl);
    return url.searchParams.get("token");
  } catch {
    return null;
  }
};

const loadPortalInfo = async (forceRefresh = false) => {
  console.log("loadPortalInfo called with forceRefresh:", forceRefresh);
  console.log("token.portal_url:", props.token.portal_url);
  console.log("token.portal_info:", props.token.portal_info);

  if (!props.token.portal_url) {
    console.log("No portal_url, returning");
    return;
  }

  const token = extractTokenFromPortalUrl(props.token.portal_url);
  console.log("Extracted token:", token ? "found" : "not found");
  if (!token) return;

  // 处理数据显示逻辑
  if (forceRefresh) {
    // 强制刷新时：立即清空数据，显示加载状态
    console.log("Force refresh: clearing portal data to show loading state");
    portalInfo.value = { data: null, error: null };
  } else if (props.token.portal_info) {
    // 非强制刷新且有缓存数据：显示缓存数据
    console.log("Using cached portal info");
    portalInfo.value = {
      data: {
        credits_balance: props.token.portal_info.credits_balance,
        expiry_date: props.token.portal_info.expiry_date,
        is_active: props.token.portal_info.is_active,
      },
      error: null,
    };
  } else {
    // 没有缓存数据：清空状态
    console.log("No cached data, clearing error state");
    portalInfo.value = { data: null, error: null };
  }

  // 在后台获取最新信息
  console.log("Starting background fetch");
  // 只有在未设置加载状态时才设置（避免重复设置）
  if (!isLoadingPortalInfo.value) {
    isLoadingPortalInfo.value = true;
  }

  try {
    // 首先获取customer信息
    console.log("Calling get_customer_info...");
    const customerResponse = await invoke("get_customer_info", { token });
    console.log("Customer response received:", customerResponse);
    const customerData = JSON.parse(customerResponse);
    console.log("Customer data parsed:", customerData);

    if (
      customerData.customer &&
      customerData.customer.ledger_pricing_units &&
      customerData.customer.ledger_pricing_units.length > 0
    ) {
      const customerId = customerData.customer.id;
      const pricingUnitId = customerData.customer.ledger_pricing_units[0].id;
      console.log(
        "Customer ID:",
        customerId,
        "Pricing Unit ID:",
        pricingUnitId
      );

      // 获取ledger summary
      console.log("Calling get_ledger_summary...");
      const ledgerResponse = await invoke("get_ledger_summary", {
        customerId,
        pricingUnitId,
        token,
      });
      console.log("Ledger response received:", ledgerResponse);
      const ledgerData = JSON.parse(ledgerResponse);
      console.log("Ledger data parsed:", ledgerData);

      // 处理credits_balance数据，无论credit_blocks是否为空
      if (ledgerData.credits_balance !== undefined) {
        console.log("Credits balance found:", ledgerData.credits_balance);

        // 构建Portal数据对象
        const newPortalData = {
          credits_balance: parseInt(ledgerData.credits_balance) || 0,
        };

        // 如果有credit_blocks数据，添加过期时间和状态信息
        if (ledgerData.credit_blocks && ledgerData.credit_blocks.length > 0) {
          console.log("Credit blocks found:", ledgerData.credit_blocks.length);
          newPortalData.expiry_date = ledgerData.credit_blocks[0].expiry_date;
          newPortalData.is_active = ledgerData.credit_blocks[0].is_active;
        } else {
          console.log("No credit blocks, but credits_balance available");
          // 当没有credit_blocks时，设置默认值
          newPortalData.expiry_date = null;
          newPortalData.is_active = false;
        }

        console.log("New portal data:", newPortalData);

        // 先更新本地token对象（避免数据闪烁）
        const updatedPortalInfo = {
          credits_balance: newPortalData.credits_balance,
          expiry_date: newPortalData.expiry_date,
          is_active: newPortalData.is_active,
        };

        // 原子性更新：同时更新token和UI显示
        props.token.portal_info = updatedPortalInfo;
        portalInfo.value = {
          data: newPortalData,
          error: null,
        };

        console.log("Updated token portal_info and UI:", updatedPortalInfo);
      } else {
        // 如果没有credits_balance数据且没有本地数据，静默处理
        if (!props.token.portal_info) {
          portalInfo.value = { data: null, error: null };
        }
      }
    } else {
      // 如果没有本地数据，静默处理，不显示错误信息
      if (!props.token.portal_info) {
        portalInfo.value = { data: null, error: null };
      }
    }
  } catch (error) {
    console.error("Failed to load portal info:", error);

    // 错误处理：根据是否为强制刷新决定如何处理
    if (forceRefresh) {
      // 强制刷新失败：清空数据，不显示错误信息
      portalInfo.value = { data: null, error: null };
      // 抛出错误以便上层处理
      throw error;
    } else {
      // 非强制刷新失败：保持现有数据不变，静默处理
      if (!props.token.portal_info) {
        portalInfo.value = { data: null, error: null };
      }
      // 不抛出错误，静默处理
    }
  } finally {
    isLoadingPortalInfo.value = false;
  }
};

// 格式化过期时间为"剩余：N天NN时NN分"格式
const formatExpiryDate = (dateString) => {
  try {
    const now = new Date();
    const expiry = new Date(dateString);
    const diffMs = expiry.getTime() - now.getTime();

    if (diffMs <= 0) {
      return "已过期";
    }

    const days = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    const hours = Math.floor(
      (diffMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)
    );
    const minutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));

    return `${days}天${hours.toString().padStart(2, "0")}时${minutes
      .toString()
      .padStart(2, "0")}分`;
  } catch {
    return dateString;
  }
};

// 过期时间样式：根据剩余时间显示不同颜色（浅色背景 + 深色文字）
const getExpiryStyle = (dateString) => {
  if (!dateString) return {};

  const now = new Date();
  const expiry = new Date(dateString);
  const diffMs = expiry.getTime() - now.getTime();
  const days = diffMs / (1000 * 60 * 60 * 24);

  let background, borderColor, color;

  if (days > 3) {
    // 剩余时间 > 3天：绿色系（浅绿背景 + 深绿文字）
    background = "rgba(34, 197, 94, 0.15)"; // 浅绿背景，半透明
    borderColor = "rgba(34, 197, 94, 0.3)"; // 绿色边框
    color = "#15803d"; // 深绿文字
  } else if (days > 1) {
    // 剩余时间 > 1天 且 ≤ 3天：黄色系（浅黄背景 + 深黄文字）
    background = "rgba(245, 158, 11, 0.15)"; // 浅黄背景，半透明
    borderColor = "rgba(245, 158, 11, 0.3)"; // 黄色边框
    color = "#a16207"; // 深黄文字
  } else {
    // 剩余时间 ≤ 1天：红色系（浅红背景 + 深红文字）
    background = "rgba(239, 68, 68, 0.15)"; // 浅红背景，半透明
    borderColor = "rgba(239, 68, 68, 0.3)"; // 红色边框
    color = "#b91c1c"; // 深红文字
  }

  return {
    background,
    borderColor,
    color,
  };
};

// 剩余额度样式：根据额度数量显示不同颜色（浅色背景 + 深色文字）
const getBalanceStyle = (balance) => {
  let background, borderColor, color;

  if (balance > 30) {
    // 额度 > 30：绿色系（浅绿背景 + 深绿文字）
    background = "rgba(34, 197, 94, 0.15)"; // 浅绿背景，半透明
    borderColor = "rgba(34, 197, 94, 0.3)"; // 绿色边框
    color = "#15803d"; // 深绿文字
  } else if (balance > 10) {
    // 额度 > 10 且 ≤ 30：黄色系（浅黄背景 + 深黄文字）
    background = "rgba(245, 158, 11, 0.15)"; // 浅黄背景，半透明
    borderColor = "rgba(245, 158, 11, 0.3)"; // 黄色边框
    color = "#a16207"; // 深黄文字
  } else {
    // 额度 ≤ 10：红色系（浅红背景 + 深红文字）
    background = "rgba(239, 68, 68, 0.15)"; // 浅红背景，半透明
    borderColor = "rgba(239, 68, 68, 0.3)"; // 红色边框
    color = "#b91c1c"; // 深红文字
  }

  return {
    background,
    borderColor,
    color,
  };
};

// 检测账号状态

// 综合状态判断：结合API状态和Portal信息
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
    // 只有在明确的过期条件下才判断为过期

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

// 状态样式与文案映射
const getStatusClass = (status) => {
  switch (status) {
    case "SUSPENDED":
      return "banned";
    case "INVALID_TOKEN":
      return "invalid";
    case "EXPIRED":
      return "expired";
    case "ACTIVE":
      return "active";
    default:
      return "active";
  }
};

const getStatusText = (status) => {
  switch (status) {
    case "SUSPENDED":
      return "已封禁";
    case "INVALID_TOKEN":
      return "Token失效";
    case "EXPIRED":
      return "已过期";
    case "ACTIVE":
      return "正常";
    default:
      return "未知";
  }
};

const checkAccountStatus = async () => {
  console.log("checkAccountStatus called");
  if (isCheckingStatus.value) return;

  isCheckingStatus.value = true;

  try {
    // 并行执行两个操作：账号状态检测和Portal信息获取
    const promises = [];

    // 1. 账号状态检测
    console.log("Adding account status check promise");
    const statusCheckPromise = invoke("check_account_status", {
      token: props.token.access_token,
      tenantUrl: props.token.tenant_url,
    });
    promises.push(statusCheckPromise);

    // 2. Portal信息获取（如果有portal_url）
    let portalInfoPromise = null;
    if (props.token.portal_url) {
      console.log("Adding portal info promise");
      portalInfoPromise = loadPortalInfo(true); // 强制刷新
      promises.push(portalInfoPromise);
    } else {
      console.log("No portal_url, skipping portal info fetch");
    }

    // 等待所有操作完成
    const results = await Promise.allSettled(promises);

    // 处理账号状态检测结果
    const statusResult = results[0];
    let statusMessage = "";
    let statusType = "info";

    let apiStatus = null;
    if (statusResult.status === "fulfilled") {
      const result = statusResult.value;
      apiStatus = result.status || (result.is_banned ? "SUSPENDED" : "ACTIVE");
    } else {
      console.error("Account status check failed:", statusResult.reason);
      statusMessage = `刷新失败: ${statusResult.reason}`;
      statusType = "error";
    }

    // 处理Portal信息获取结果（静默更新，不在通知中显示）
    if (portalInfoPromise && results.length > 1) {
      const portalResult = results[1];
      if (portalResult.status === "rejected") {
        console.error("Portal info fetch failed:", portalResult.reason);
        // 如果有本地数据，继续显示本地数据，不显示错误
      }
      // loadPortalInfo方法已经处理了成功和失败的情况
    }

    // 应用综合状态判断
    if (apiStatus) {
      const comprehensiveStatus = getComprehensiveStatus(
        apiStatus,
        portalInfo.value
      );

      // 更新本地token对象
      props.token.ban_status = comprehensiveStatus;
      props.token.updated_at = new Date().toISOString();

      // 通知父组件保存更新后的token数据
      emit("token-updated", props.token);

      statusMessage =
        comprehensiveStatus === "SUSPENDED"
          ? "账号已封禁"
          : comprehensiveStatus === "INVALID_TOKEN"
          ? "Token失效"
          : comprehensiveStatus === "EXPIRED"
          ? "账号已过期"
          : "账号状态正常";
      statusType =
        comprehensiveStatus === "SUSPENDED" ||
        comprehensiveStatus === "INVALID_TOKEN" ||
        comprehensiveStatus === "EXPIRED"
          ? "error"
          : "success";
    }

    // 发送账号状态消息（不包含次数信息）
    const finalMessage = `刷新完成：${statusMessage}`;
    emit("copy-success", finalMessage, statusType);
  } catch (error) {
    console.error("Account status check failed:", error);
    emit("copy-success", `刷新失败: ${error}`, "error");
  } finally {
    isCheckingStatus.value = false;
    isLoadingPortalInfo.value = false;
  }
};

// 移除了防抖，直接调用状态检测方法

// 暴露刷新Portal信息的方法
const refreshPortalInfo = async () => {
  if (props.token.portal_url) {
    // 立即设置加载状态，提供即时视觉反馈
    isLoadingPortalInfo.value = true;
    // 立即清空当前显示的数据，显示加载状态
    portalInfo.value = { data: null, error: null };

    try {
      return await loadPortalInfo(true); // 强制刷新
    } catch (error) {
      // 确保即使出错也重置加载状态
      isLoadingPortalInfo.value = false;
      throw error;
    }
  }
  return Promise.resolve();
};

// 组件挂载时加载Portal信息
onMounted(() => {
  if (props.token.portal_url) {
    // 如果有本地数据，立即显示
    if (props.token.portal_info) {
      portalInfo.value = {
        data: {
          credits_balance: props.token.portal_info.credits_balance,
          expiry_date: props.token.portal_info.expiry_date,
          is_active: props.token.portal_info.is_active,
        },
        error: null,
      };
    }
    // 然后在后台刷新数据
    loadPortalInfo(false);
  }

  // 添加键盘事件监听器
  document.addEventListener("keydown", handleKeydown);
});

// 组件卸载时清理事件监听器和定时器
onUnmounted(() => {
  document.removeEventListener("keydown", handleKeydown);
  if (tooltipHideTimer.value) {
    clearTimeout(tooltipHideTimer.value);
    tooltipHideTimer.value = null;
  }
});

// 暴露检查账号状态的方法
const refreshAccountStatus = async () => {
  return await checkAccountStatus();
};

// 暴露方法给父组件
defineExpose({
  refreshPortalInfo,
  refreshAccountStatus,
});
</script>

<style scoped>
.token-card {
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 12px;
  padding: 10px;
  margin: 0px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 0 0 1px rgba(255, 255, 255, 0.05);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  height: fit-content;
  min-height: 100px;
  position: relative;
  backdrop-filter: blur(20px);
  overflow: hidden;
}

.status-indicator {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 10;
}

.status-badge {
  font-size: 10px;
  font-weight: 700;
  padding: 4px 10px;
  border-radius: 20px;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  border: none;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.status-badge.active {
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
}

.status-badge.inactive {
  background: linear-gradient(135deg, #f59e0b, #d97706);
  color: white;
}

.status-badge.banned {
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
}

.token-card:hover {
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.12), 0 0 0 1px rgba(255, 255, 255, 0.1);
  /* transform: translateY(-3px); */
  background: rgba(255, 255, 255, 0.98);
}

.status-badge.invalid {
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: white;
}

.status-badge.expired {
  background: linear-gradient(135deg, #f97316, #ea580c);
  color: white;
}

.card-main {
  display: flex;
  flex-direction: column;
  gap: 12px;
  height: 100%;
}

.token-info {
  flex: 1;
  min-width: 0;
}

.tenant-name {
  margin: 0 0 6px 0;
  font-size: 15px;
  font-weight: 700;
  background: linear-gradient(135deg, #1e293b 0%, #475569 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  word-break: break-all;
  line-height: 1.3;
  letter-spacing: -0.025em;
}

.token-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.meta-row {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 8px;
  flex-wrap: nowrap;
  min-height: 30px;
}

.portal-row {
  margin-top: 2px;
}

.created-date {
  font-size: 12px;
  color: #64748b;
  font-weight: 500;
  background: rgba(248, 250, 252, 0.6);
  padding: 3px 4px;
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.3);
}

.email-note-container {
  display: flex;
  align-items: center;
  gap: 6px;
  min-height: 20px;
}

.email-note-wrapper {
  position: relative;
  display: inline-block;
}

.email-note {
  font-size: 12px;
  color: #4f46e5;
  display: flex;
  align-items: center;
  gap: 4px;
  background: linear-gradient(
    135deg,
    rgba(79, 70, 229, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  padding: 4px 8px;
  border-radius: 8px;
  border: 1px solid rgba(79, 70, 229, 0.2);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  user-select: none;
  font-weight: 500;
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 3px rgba(79, 70, 229, 0.1);
}

.email-note:hover {
  background: linear-gradient(
    135deg,
    rgba(79, 70, 229, 0.12),
    rgba(139, 92, 246, 0.06)
  );
  border-color: rgba(79, 70, 229, 0.3);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(79, 70, 229, 0.15);
  color: #3730a3;
}

.email-icon {
  flex-shrink: 0;
  opacity: 0.7;
}

.email-note-wrapper.clickable {
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.email-note-wrapper.clickable:hover .email-note {
  color: #4f46e5;
}

.email-note-wrapper.clickable:hover .email-icon {
  opacity: 1;
}

.email-note-wrapper.clickable:active {
  transform: scale(0.98);
}

/* Email Tooltip 样式 */
.email-tooltip {
  background: rgba(15, 23, 42, 0.95);
  color: white;
  padding: 8px 12px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  animation: tooltipFadeIn 0.2s ease-out;
  max-width: 300px;
  word-break: break-all;
  white-space: normal;
}

.tooltip-arrow {
  position: absolute;
  bottom: -5px;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 5px solid rgba(15, 23, 42, 0.95);
}

@keyframes tooltipFadeIn {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.portal-meta {
  font-size: 12px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 8px;

  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  /* 移除 backdrop-filter，使用纯色背景 */
}

.portal-meta.loading {
  color: #64748b;
  font-style: italic;
  background: rgba(248, 250, 252, 0.6);
  border-color: rgba(226, 232, 240, 0.3);
}

.portal-meta.error {
  color: #dc2626;
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.08),
    rgba(220, 38, 38, 0.04)
  );
  border-color: rgba(239, 68, 68, 0.2);
}

.portal-meta.expiry {
  /* 样式由 getExpiryStyle 函数动态设置 */
  font-weight: 600;
}

.portal-meta.balance {
  /* 样式由 getBalanceStyle 函数动态设置 */
  font-weight: 700;
}

.actions {
  display: flex;
  flex-direction: row;
  gap: 6px;
  justify-content: flex-end;
  margin-top: auto;
  flex-wrap: wrap;
  padding-top: 2px;
}

.btn-action {
  background: rgba(248, 250, 252, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  padding: 8px;
  cursor: pointer;
  color: #64748b;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  min-height: 32px;
  flex-shrink: 0;
  font-weight: 500;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.btn-action::before {
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
  transition: left 0.5s;
}

.btn-action:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border-color: rgba(203, 213, 225, 0.8);
  background: rgba(255, 255, 255, 0.9);
}

.btn-action:hover::before {
  left: 100%;
}

.btn-action:active {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.btn-action.copy {
  color: #6366f1;
}

.btn-action.copy:hover {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1),
    rgba(139, 92, 246, 0.05)
  );
  border-color: rgba(99, 102, 241, 0.3);
  color: #4f46e5;
}

.btn-action.link {
  color: #0ea5e9;
}

.btn-action.link:hover {
  background: linear-gradient(
    135deg,
    rgba(14, 165, 233, 0.1),
    rgba(59, 130, 246, 0.05)
  );
  border-color: rgba(14, 165, 233, 0.3);
  color: #0284c7;
}

.btn-action.delete {
  color: #ef4444;
}

.btn-action.delete:hover {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.1),
    rgba(220, 38, 38, 0.05)
  );
  border-color: rgba(239, 68, 68, 0.3);
  color: #dc2626;
}

.btn-action.portal {
  color: #8b5cf6;
}

.btn-action.portal:hover {
  background: linear-gradient(
    135deg,
    rgba(139, 92, 246, 0.1),
    rgba(168, 85, 247, 0.05)
  );
  border-color: rgba(139, 92, 246, 0.3);
  color: #7c3aed;
}

.btn-action.edit {
  color: #10b981;
}

.btn-action.edit:hover {
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.1),
    rgba(5, 150, 105, 0.05)
  );
  border-color: rgba(16, 185, 129, 0.3);
  color: #059669;
}

.btn-action.vscode {
  color: #f59e0b;
}

.btn-action.vscode:hover {
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.1),
    rgba(217, 119, 6, 0.05)
  );
  border-color: rgba(245, 158, 11, 0.3);
  color: #d97706;
}

.btn-action.status-check {
  color: #f59e0b;
}

.btn-action.status-check:hover {
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.1),
    rgba(217, 119, 6, 0.05)
  );
  border-color: rgba(245, 158, 11, 0.3);
  color: #d97706;
}

.btn-action.status-check.loading {
  opacity: 0.7;
  cursor: not-allowed;
  pointer-events: none;
}

.btn-action.status-check.loading:hover {
  transform: none;
  background: rgba(248, 250, 252, 0.8);
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

/* 编辑器选择模态框内容样式 */
.editor-modal-content {
  max-height: 60vh;
  overflow-y: auto;
}

.modal-description {
  margin-bottom: 24px;
  color: #666;
  text-align: center;
  line-height: 1.5;
}

/* 编辑器网格布局 */
.editor-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.editor-category {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.category-title {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
  padding-bottom: 8px;
  border-bottom: 2px solid rgba(59, 130, 246, 0.1);
}

.category-title svg {
  color: #3b82f6;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0px);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(12px);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 32px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
}

.modal-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
}

.modal-close {
  background: none;
  border: none;
  color: #64748b;
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.modal-close:hover {
  background: rgba(226, 232, 240, 0.5);
  color: #374151;
}

.modal-content {
  padding: 32px;
}

.editor-section {
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid #e1e5e9;
}

.editor-section:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.vscode-grid,
.jetbrains-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.editor-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border: 2px solid rgba(226, 232, 240, 0.4);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  text-align: left;
  width: 100%;
  position: relative;
  font-family: inherit;
  font-size: inherit;
  text-decoration: none;
  color: inherit;
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

/* 确保链接在所有状态下都保持正确的样式 */
.editor-option:visited {
  color: inherit;
  text-decoration: none;
}

.editor-option:focus {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

.editor-icon {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.9) 0%,
    rgba(241, 245, 249, 0.7) 100%
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
  transform: scale(1.05);
}

.editor-icon img {
  width: 24px;
  height: 24px;
  object-fit: contain;
  transition: transform 0.3s ease;
}

.editor-option:hover .editor-icon img {
  transform: scale(1.1);
}

.cursor-option .editor-icon {
  background: #f0f9ff;
  border-color: #e0f2fe;
}

.vscode-option .editor-icon {
  background: #f0f9ff;
  border-color: #e0f2fe;
}

.kiro-option .editor-icon,
.trae-option .editor-icon,
.windsurf-option .editor-icon,
.qoder-option .editor-icon,
.vscodium-option .editor-icon,
.codebuddy-option .editor-icon {
  background: #f0f9ff;
  border-color: #e0f2fe;
}

.idea-option .editor-icon,
.pycharm-option .editor-icon,
.goland-option .editor-icon,
.rustrover-option .editor-icon,
.webstorm-option .editor-icon,
.phpstorm-option .editor-icon,
.androidstudio-option .editor-icon,
.clion-option .editor-icon,
.datagrip-option .editor-icon,
.rider-option .editor-icon,
.rubymine-option .editor-icon,
.aqua-option .editor-icon {
  background: #f0f9ff;
  border-color: #e0f2fe;
}

.editor-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.editor-name {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
  transition: color 0.3s ease;
}

.editor-option:hover .editor-name {
  color: #3b82f6;
}

/* 响应式处理 */
@media (max-width: 768px) {
  .token-card {
    padding: 6px;
    min-height: 90px;
    margin: 4px 4px 2px;
  }

  .card-main {
    gap: 10px;
  }

  .tenant-name {
    font-size: 14px;
  }

  .actions {
    gap: 4px;
  }

  .btn-action {
    padding: 6px;
    min-width: 28px;
    min-height: 28px;
  }

  .btn-action svg,
  .btn-action img {
    width: 14px;
    height: 14px;
  }
}

@media (max-width: 480px) {
  .token-card {
    padding: 6px;
    min-height: 85px;
    margin: 4px 4px 2px;
  }

  .actions {
    gap: 3px;
    justify-content: center;
  }

  .btn-action {
    padding: 6px;
    min-width: 26px;
    min-height: 26px;
  }

  .btn-action svg,
  .btn-action img {
    width: 12px;
    height: 12px;
  }

  .token-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .meta-row {
    gap: 6px;
  }

  .created-date {
    font-size: 11px;
    padding: 2px 4px;
  }

  .email-note {
    font-size: 11px;
    padding: 3px 6px;
  }

  .tenant-name {
    font-size: 13px;
  }

  /* 模态框响应式样式 */
  .editor-modal {
    width: 95%;
    margin: 16px;
    max-height: 90vh;
  }

  .modal-header {
    padding: 16px 20px 12px;
  }

  .modal-header h3 {
    font-size: 16px;
  }

  .modal-content {
    padding: 16px 20px 20px;
  }

  .editor-section {
    margin-bottom: 20px;
    padding-bottom: 20px;
  }

  .jetbrains-grid {
    grid-template-columns: 1fr;
  }

  .editor-option {
    padding: 12px;
    gap: 12px;
  }

  .editor-icon {
    width: 40px;
    height: 40px;
  }

  .editor-icon img {
    width: 28px;
    height: 28px;
  }

  .editor-name {
    font-size: 15px;
  }
}
</style>
