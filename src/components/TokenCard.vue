<template>
  <div
    :class="[
      'token-card',
      {
        'menu-open': showCopyMenu,
        highlighted: isHighlighted,
      },
    ]"
    @click="handleClickOutside"
  >
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
        <div class="tenant-name-container">
          <span
            class="tenant-name"
            :style="tenantNameStyle"
            @click.stop="openTagEditor"
            title="标签"
          >
            {{ displayUrl }}
          </span>
          <span v-if="token.tag_text" class="tag-text" :style="tagTextStyle">
            {{ token.tag_text }}
          </span>
        </div>
        <div class="token-meta">
          <!-- 第一行：创建日期和邮箱 -->
          <div class="meta-row">
            <span class="created-date">{{ formatDate(token.created_at) }}</span>
            <span
              v-if="token.email_note"
              class="email-note"
              @click.stop="copyEmailNote"
              @mouseenter="showFullEmail = true"
              @mouseleave="showFullEmail = false"
              title="点击复制"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="email-icon"
              >
                <path
                  d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.89 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
                />
              </svg>
              <span class="email-text">{{ displayEmail }}</span>
            </span>
          </div>
          <!-- 第二行：Portal信息 -->
          <div class="meta-row portal-row">
            <!-- 如果没有数据且正在加载，显示加载状态 -->
            <template
              v-if="
                isLoadingPortalInfo ||
                isCheckingStatus ||
                (isBatchChecking && !token.skip_check)
              "
            >
              <span class="portal-meta loading">加载中...</span>
            </template>
            <!-- 如果账号已封禁，固定显示未知 -->
            <template
              v-else-if="
                token.ban_status === 'SUSPENDED' ||
                token.ban_status === 'EXPIRED'
              "
            >
              <span class="portal-meta unknown">
                <span class="portal-info-item"
                  ><span class="portal-value">时间：未知</span></span
                >
                <span class="portal-separator">|</span>
                <span class="portal-info-item"
                  ><span class="portal-value">额度：未知</span></span
                >
              </span>
            </template>
            <!-- 如果有portal_url且有数据，显示Portal数据 -->
            <template v-else-if="token.portal_url && portalInfo.data">
              <span class="portal-meta">
                <span class="portal-info-item expiry">
                  时间：
                  <span
                    class="portal-value"
                    :style="getExpiryStyle(portalInfo.data.expiry_date)"
                    >{{ formatExpiryDate(portalInfo.data.expiry_date) }}</span
                  >
                </span>
                <span class="portal-separator">|</span>
                <span class="portal-info-item balance">
                  额度：
                  <span
                    class="portal-value"
                    :style="getBalanceStyle(portalInfo.data.credits_balance)"
                    >{{ portalInfo.data.credits_balance }}
                  </span>
                </span>
              </span>
            </template>
            <!-- 其他情况：没有portal_url或获取失败，显示未知信息 -->
            <template v-else>
              <span class="portal-meta unknown">
                <span class="portal-info-item"
                  ><span class="portal-value">时间：未知</span></span
                >
                <span class="portal-separator">|</span>
                <span class="portal-info-item"
                  ><span class="portal-value">额度：未知</span></span
                >
              </span>
            </template>
          </div>
        </div>
      </div>

      <div class="actions">
        <button
          @click.stop="openEditorModal"
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

        <div class="check-menu-wrapper">
          <button
            @click.stop="checkAccountStatus"
            @mousedown.stop="handleCheckButtonMouseDown"
            @mouseup.stop="handleCheckButtonMouseUp"
            @mouseleave="handleCheckButtonMouseLeave"
            :class="[
              'btn-action',
              'status-check',
              {
                loading:
                  isCheckingStatus || (isBatchChecking && !token.skip_check),
                disabled: token.skip_check,
                'long-pressing': isLongPressing,
              },
            ]"
            :disabled="
              isCheckingStatus || (isBatchChecking && !token.skip_check)
            "
            :title="
              token.skip_check
                ? '检测已禁用（长按启用）'
                : '刷新账号数据（长按禁用）'
            "
          >
            <!-- 长按进度圆环 -->
            <svg
              v-if="isLongPressing"
              class="long-press-progress"
              width="32"
              height="32"
              viewBox="0 0 32 32"
            >
              <circle
                class="progress-ring"
                cx="16"
                cy="16"
                r="14"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              />
            </svg>
            <svg
              v-if="
                !isCheckingStatus &&
                !(isBatchChecking && !token.skip_check) &&
                !token.skip_check
              "
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <path
                d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
              />
            </svg>
            <!-- 禁用检测时显示暂停图标 -->
            <svg
              v-if="token.skip_check"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="currentColor"
              opacity="0.5"
            >
              <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
            </svg>
            <div
              v-else-if="
                isCheckingStatus || (isBatchChecking && !token.skip_check)
              "
              class="loading-spinner"
            ></div>
          </button>
        </div>
        <div
          class="copy-button-container"
          @mouseenter="startCopyMenuTimer"
          @mouseleave="handleCopyMenuMouseLeave"
        >
          <button
            @click.stop="showCopyMenu = !showCopyMenu"
            class="btn-action copy"
            title="复制账号信息"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
              />
            </svg>
          </button>

          <!-- 复制菜单 -->
          <div
            v-if="showCopyMenu"
            class="copy-menu"
            @mouseenter="clearCopyMenuTimer"
            @mouseleave="handleCopyMenuMouseLeave"
          >
            <button @click.stop="copyAccountInfoAsJson" class="copy-menu-item">
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M7 5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h1v-2H7V7h1V5H7zm10 0v2h1v10h-1v2h1c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-1z"
                />
              </svg>
              复制 JSON
            </button>
            <button
              v-if="token.auth_session"
              @click.stop="copySessionValue"
              class="copy-menu-item"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"
                />
              </svg>
              复制 Session
            </button>
          </div>
        </div>

        <button
          @click.stop="$emit('edit', token)"
          class="btn-action edit"
          title="编辑账号"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"
            />
          </svg>
        </button>
        <button
          @click.stop="deleteToken"
          class="btn-action delete"
          title="删除账号"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>

  <!-- 标签编辑弹窗 -->
  <TagEditorModal
    :visible="showTagEditor"
    :initial-tag-text="token.tag_text || ''"
    :initial-tag-color="token.tag_color || null"
    @close="showTagEditor = false"
    @confirm="handleTagConfirm"
  />

  <!-- 编辑器选择模态框 -->
  <ModalContainer
    :visible="showEditorModal"
    title="选择编辑器登录"
    size="medium"
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
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";
import TagEditorModal from "./TagEditorModal.vue";

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
  statusThresholds: {
    type: Object,
    default: () => ({
      time: { warning: 3, safe: 5 },
      balance: { warning: 10, safe: 30 },
      timeMax: 365,
      balanceMax: 100000,
    }),
  },
  isBatchChecking: {
    type: Boolean,
    default: false,
  },
  isHighlighted: {
    type: Boolean,
    default: false,
  },
});

// Emits
const emit = defineEmits(["delete", "copy-success", "edit", "token-updated"]);

// Reactive data
const isLoadingPortalInfo = ref(false);
const portalInfo = ref({ data: null, error: null });
const isCheckingStatus = ref(false);
const showEditorModal = ref(false);
const isModalClosing = ref(false);
const showTagEditor = ref(false);
const showFullEmail = ref(false);
const showCopyMenu = ref(false);
const isLongPressing = ref(false);

// 复制菜单计时器
let copyMenuTimer = null;

// 长按计时器
let longPressTimer = null;
let isLongPress = false;

// 颜色映射
const colorMap = {
  red: "#b91c1c",
  green: "#15803d",
  yellow: "#a16207",
  blue: "#3b82f6",
  black: "#1f2937",
};

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

const maskedEmail = computed(() => {
  const email = props.token.email_note;
  if (!email || !email.includes(".")) return email;

  const [username, domain] = email.split(".");

  // 如果用户名太短，直接返回原邮箱
  if (username.length <= 3) {
    return email;
  }

  let maskedUsername;
  if (username.length <= 10) {
    // 短邮箱：保留前1-2个字符，其余用星号替换
    const keepChars = username.length <= 4 ? 1 : 2;
    const hiddenCount = username.length - keepChars;
    maskedUsername = username.substring(0, keepChars) + "*".repeat(hiddenCount);
  } else {
    // 长邮箱：保留前后各2-3个字符，中间用4个星号替换
    const frontKeep = username.length >= 8 ? 4 : 3;
    const backKeep = 2;
    maskedUsername =
      username.substring(0, frontKeep) +
      "****" +
      username.substring(username.length - backKeep);
  }

  return maskedUsername + "." + domain;
});

// 显示邮箱（根据悬浮状态决定显示完整或省略）
const displayEmail = computed(() => {
  return showFullEmail.value ? props.token.email_note : maskedEmail.value;
});

// 标签相关的计算属性
const tenantNameStyle = computed(() => {
  if (props.token.tag_color && colorMap[props.token.tag_color]) {
    return {
      color: colorMap[props.token.tag_color] + " !important",
    };
  }
  return {};
});

const tagTextStyle = computed(() => {
  if (props.token.tag_color && colorMap[props.token.tag_color]) {
    const color = colorMap[props.token.tag_color];
    return {
      color: color,
      backgroundColor: `${color}15`, // 15% 透明度
      borderColor: `${color}40`, // 40% 透明度
    };
  }
  return {};
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

// 复制账号信息为 JSON
const copyAccountInfoAsJson = async () => {
  const accountInfo = {
    tenant_url: props.token.tenant_url,
    access_token: props.token.access_token,
    portal_url: props.token.portal_url || "",
    email_note: props.token.email_note || "",
  };

  const jsonString = JSON.stringify(accountInfo, null, 2);
  const success = await copyToClipboard(jsonString);

  if (success) {
    emit("copy-success", "JSON信息已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制JSON信息到剪贴板失败", "error");
  }

  showCopyMenu.value = false;
};

// 复制 Session 值
const copySessionValue = async () => {
  console.log("Token object:", props.token);
  console.log("Auth session value:", props.token.auth_session);

  if (!props.token.auth_session) {
    emit("copy-success", "该账号没有 Session 信息", "error");
    showCopyMenu.value = false;
    return;
  }

  const success = await copyToClipboard(props.token.auth_session);

  if (success) {
    emit("copy-success", "Session信息已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制 Session信息到剪贴板失败", "error");
  }

  showCopyMenu.value = false;
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

// 复制邮箱备注
const copyEmailNote = async () => {
  const success = await copyToClipboard(props.token.email_note);
  if (success) {
    emit("copy-success", "邮箱备注已复制到剪贴板!", "success");
  } else {
    emit("copy-success", "复制邮箱备注失败", "error");
  }
};

// 打开标签编辑器
const openTagEditor = () => {
  showTagEditor.value = true;
};

// 处理标签确认
const handleTagConfirm = async ({ tagText, tagColor }) => {
  // 直接更新 props.token 对象
  props.token.tag_text = tagText;
  props.token.tag_color = tagColor;
  props.token.updated_at = new Date().toISOString();

  // 发出更新事件，传递完整的 token 对象让父组件处理保存
  emit("token-updated", props.token);

  // 关闭弹窗
  showTagEditor.value = false;

  // 显示成功提示
  emit("copy-success", "标签已保存", "success");
};

// 键盘事件处理
const handleKeydown = (event) => {
  if (event.key === "Escape" && showEditorModal.value) {
    showEditorModal.value = false;
  }
  if (event.key === "Escape" && showTagEditor.value) {
    showTagEditor.value = false;
  }
  if (event.key === "Escape" && showCopyMenu.value) {
    showCopyMenu.value = false;
  }
};

// 点击外部关闭菜单（已由 handleCardMouseLeave 处理）
const handleClickOutside = () => {
  // 菜单关闭由 handleCardMouseLeave 处理，这里保留以兼容其他事件
};

// 切换跳过检测状态
const toggleSkipCheck = () => {
  // 切换 skip_check 状态
  props.token.skip_check = !props.token.skip_check;
  props.token.updated_at = new Date().toISOString();

  // 通知父组件有更新
  emit("token-updated", props.token);

  // 显示提示
  const message = props.token.skip_check ? "已禁用检测" : "已启用检测";
  emit("copy-success", message, "info");
};

// 启动复制菜单计时器（悬浮0.3s后显示）
const startCopyMenuTimer = () => {
  clearCopyMenuTimer();
  copyMenuTimer = setTimeout(() => {
    showCopyMenu.value = true;
  }, 300);
};

// 清除复制菜单计时器
const clearCopyMenuTimer = () => {
  if (copyMenuTimer) {
    clearTimeout(copyMenuTimer);
    copyMenuTimer = null;
  }
};

// 处理复制菜单鼠标离开事件
const handleCopyMenuMouseLeave = () => {
  clearCopyMenuTimer();
  // 设置延迟隐藏菜单，避免鼠标在按钮和菜单之间移动时闪烁
  copyMenuTimer = setTimeout(() => {
    showCopyMenu.value = false;
  }, 100);
};

// 长按刷新按钮处理
const handleCheckButtonMouseDown = () => {
  // 如果正在加载，不处理
  if (
    isCheckingStatus.value ||
    (props.isBatchChecking && !props.token.skip_check)
  ) {
    return;
  }

  isLongPress = false;
  isLongPressing.value = true; // 开始长按动画

  // 设置长按计时器（800ms）
  longPressTimer = setTimeout(() => {
    isLongPress = true;
    isLongPressing.value = false; // 结束长按动画
    // 长按触发：切换禁用/启用检测
    toggleSkipCheck();
  }, 800);
};

const handleCheckButtonMouseUp = () => {
  // 清除长按计时器
  if (longPressTimer) {
    clearTimeout(longPressTimer);
    longPressTimer = null;
  }
  isLongPressing.value = false; // 取消长按动画
};

const handleCheckButtonMouseLeave = () => {
  // 鼠标离开时清除长按计时器
  if (longPressTimer) {
    clearTimeout(longPressTimer);
    longPressTimer = null;
  }
  isLongPress = false;
  isLongPressing.value = false; // 取消长按动画
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

// 格式化过期时间为"X天XX时XX分"格式
const formatExpiryDate = (dateString) => {
  if (!dateString) return "未知";

  try {
    // 直接使用 Date 对象解析，让浏览器自动处理时区
    const now = new Date();
    const expiry = new Date(dateString);

    // 计算时间差（毫秒）
    const diffMs = expiry.getTime() - now.getTime();

    if (diffMs <= 0) {
      return "已过期";
    }

    // 计算天、时、分
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

// 过期时间样式：根据剩余时间显示不同颜色（只改变文字颜色）
const getExpiryStyle = (dateString) => {
  if (!dateString)
    return {
      color: "#b91c1c",
    };

  try {
    // 直接使用 Date 对象解析，与 formatExpiryDate 保持一致
    const now = new Date();
    const expiry = new Date(dateString);

    // 计算剩余天数
    const diffMs = expiry.getTime() - now.getTime();
    const days = diffMs / (1000 * 60 * 60 * 24);

    // 使用配置的阈值
    const safeThreshold = props.statusThresholds.time.safe;
    const warningThreshold = props.statusThresholds.time.warning;

    let color;

    // 颜色区域规则：
    // 红色：0 < 值 ≤ warning
    // 黄色：warning < 值 ≤ safe
    // 绿色：safe < 值 ≤ max
    if (days > safeThreshold) {
      // 剩余时间 > 安全阈值：深绿文字
      color = "#15803d";
    } else if (days > warningThreshold) {
      // 剩余时间 > 警告阈值 且 ≤ 安全阈值：深黄文字
      color = "#a16207";
    } else {
      // 剩余时间 ≤ 警告阈值：深红文字
      color = "#b91c1c";
    }

    return {
      color,
    };
  } catch {
    return {
      color: "#b91c1c",
    };
  }
};

// 剩余额度样式：根据额度数量显示不同颜色（只改变文字颜色）
const getBalanceStyle = (balance) => {
  // 使用配置的阈值
  const safeThreshold = props.statusThresholds.balance.safe;
  const warningThreshold = props.statusThresholds.balance.warning;

  let color;

  // 颜色区域规则：
  // 红色：0 < 值 ≤ warning
  // 黄色：warning < 值 ≤ safe
  // 绿色：safe < 值 ≤ max
  if (balance > safeThreshold) {
    // 额度 > 安全阈值：深绿文字
    color = "#15803d";
  } else if (balance > warningThreshold) {
    // 额度 > 警告阈值 且 ≤ 安全阈值：深黄文字
    color = "#a16207";
  } else {
    // 额度 ≤ 警告阈值：深红文字
    color = "#b91c1c";
  }

  return {
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

      if (expiry.getTime() <= now.getTime()) {
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

const checkAccountStatus = async (showNotification = true) => {
  // 如果是长按操作，不执行刷新
  if (isLongPress) {
    isLongPress = false;
    return;
  }

  // 如果禁用了检测，静默返回
  if (props.token.skip_check) {
    return;
  }

  // 如果正在检测中，或者批量检测中（且未禁用），则返回
  if (
    isCheckingStatus.value ||
    (props.isBatchChecking && !props.token.skip_check)
  )
    return;

  isCheckingStatus.value = true;

  // 清空Portal数据以显示加载状态
  if (props.token.portal_url) {
    portalInfo.value = { data: null, error: null };
  }

  try {
    // 单次API调用同时获取账号状态和Portal信息
    const batchResults = await invoke("batch_check_tokens_status", {
      tokens: [
        {
          id: props.token.id,
          access_token: props.token.access_token,
          tenant_url: props.token.tenant_url,
          portal_url: props.token.portal_url || null,
          auth_session: props.token.auth_session || null,
        },
      ],
    });

    // 处理结果
    let statusMessage = "";
    let statusType = "info";

    if (batchResults && batchResults.length > 0) {
      const result = batchResults[0]; // 取第一个结果对象
      const statusResult = result.status_result; // 账号状态结果

      // 使用后端返回的具体状态
      const banStatus = statusResult.status;

      // 始终更新 access_token、tenant_url 和 portal_url (如果 token 被刷新,这里会是新值)
      props.token.access_token = result.access_token;
      props.token.tenant_url = result.tenant_url;

      // 如果后端返回了新的 portal_url，更新它
      if (result.portal_url) {
        props.token.portal_url = result.portal_url;
        console.log(
          `Updated token ${props.token.id} portal_url:`,
          result.portal_url
        );
      }

      // 更新本地token对象 - 账号状态
      props.token.ban_status = banStatus;

      // 记录是否自动禁用了检测
      let wasAutoDisabled = false;

      // 自动禁用封禁或过期的账号检测
      if (
        (banStatus === "SUSPENDED" || banStatus === "EXPIRED") &&
        !props.token.skip_check
      ) {
        props.token.skip_check = true;
        wasAutoDisabled = true;
      }

      // 更新 suspensions 信息（如果有）
      if (result.suspensions) {
        props.token.suspensions = result.suspensions;
        console.log(
          `Updated suspensions for token ${props.token.id}:`,
          result.suspensions
        );
      }

      // 更新Portal信息（如果有）
      if (result.portal_info) {
        props.token.portal_info = {
          credits_balance: result.portal_info.credits_balance,
          expiry_date: result.portal_info.expiry_date,
          can_still_use: result.portal_info.can_still_use,
        };

        // 更新UI显示
        portalInfo.value = {
          data: props.token.portal_info,
          error: null,
        };
      } else if (result.portal_error) {
        portalInfo.value = {
          data: null,
          error: result.portal_error,
        };
      }

      // 更新时间戳以确保双向同步时选择正确版本
      props.token.updated_at = new Date().toISOString();

      // 通知父组件保存更新后的数据
      emit("token-updated", props.token);

      // 根据具体状态设置消息
      switch (banStatus) {
        case "SUSPENDED":
          statusMessage = "账号已封禁";
          statusType = "error";
          // 如果刚刚自动禁用了检测，添加提示
          if (wasAutoDisabled) {
            statusMessage += "，已自动禁用检测";
          }
          break;
        case "EXPIRED":
          statusMessage = "账号已过期";
          statusType = "warning";
          // 如果刚刚自动禁用了检测，添加提示
          if (wasAutoDisabled) {
            statusMessage += "，已自动禁用检测";
          }
          break;
        case "INVALID_TOKEN":
          statusMessage = "Token失效";
          statusType = "warning";
          break;
        case "ACTIVE":
          statusMessage = "账号状态正常";
          statusType = "success";
          break;
        case "ERROR":
          statusMessage = `状态检查失败: ${
            statusResult.error_message || "Unknown error"
          }`;
          statusType = "error";
          break;
        default:
          statusMessage = `账号状态: ${banStatus}`;
          statusType = "info";
      }
    } else {
      statusMessage = "状态检查失败: No results returned";
      statusType = "error";
    }

    // Portal信息现在已经包含在批量API结果中，无需单独处理

    // 发送账号状态消息（不包含次数信息）
    if (showNotification) {
      const finalMessage = `刷新完成：${statusMessage}`;
      emit("copy-success", finalMessage, statusType);
    }
  } catch (error) {
    console.error("Account status check failed:", error);
    if (showNotification) {
      emit("copy-success", `刷新失败: ${error}`, "error");
    }
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

// 监听 token.portal_info 的变化（批量刷新时更新）
watch(
  () => props.token.portal_info,
  (newPortalInfo) => {
    if (newPortalInfo && props.token.portal_url) {
      portalInfo.value = {
        data: {
          credits_balance: newPortalInfo.credits_balance,
          expiry_date: newPortalInfo.expiry_date,
          is_active: newPortalInfo.is_active,
        },
        error: null,
      };
    }
  },
  { deep: true }
);

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
  // 添加点击外部关闭菜单的监听器
  document.addEventListener("click", handleClickOutside);
});

// 组件卸载时清理事件监听器和定时器
onUnmounted(() => {
  document.removeEventListener("keydown", handleKeydown);
  document.removeEventListener("click", handleClickOutside);
  clearCopyMenuTimer();

  // 清理长按计时器
  if (longPressTimer) {
    clearTimeout(longPressTimer);
    longPressTimer = null;
  }
});

// 暴露检查账号状态的方法
const refreshAccountStatus = async () => {
  return await checkAccountStatus();
};

// 关闭所有弹窗
const closeAllModals = () => {
  showTagEditor.value = false;
  showEditorModal.value = false;
  showCopyMenu.value = false;
};

// 暴露方法给父组件
defineExpose({
  refreshPortalInfo,
  refreshAccountStatus,
  closeAllModals,
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
  padding: 4px 8px;
  border-radius: 8px;
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

.tenant-name-container {
  margin: 0 0 6px 0;
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.tenant-name {
  font-size: 16px;
  font-weight: 700;
  word-break: break-all;
  line-height: 1.3;
  letter-spacing: -0.025em;
  color: #1e293b;
  cursor: pointer;
  transition: opacity 0.2s ease;
  opacity: 0.75;
}

.tenant-name:hover {
  opacity: 1;
}

.tag-text {
  font-size: 10px;
  font-weight: 600;
  padding: 2px 4px;
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.06);
  white-space: nowrap;
  letter-spacing: 0.02em;
  line-height: 1.2;
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
  gap: 4px;
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
  background: rgba(247, 246, 246, 0.7);
  border: 1px solid rgba(210, 215, 223, 0.4);
  padding: 3px 4px;
  border-radius: 8px;
}

.email-note {
  font-size: 12px;
  color: #64748b;
  font-weight: 500;
  background: rgba(247, 246, 246, 0.7);
  border: 1px solid rgba(210, 215, 223, 0.4);
  padding: 3px 4px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.email-note:hover {
  color: #1e293b;
  background: rgba(232, 234, 235, 0.8);
  border-color: rgba(203, 213, 225, 0.5);
  box-shadow: 0 1px 3px rgba(100, 116, 139, 0.08);
}

.email-icon {
  flex-shrink: 0;
  opacity: 0.7;
  transition: opacity 0.3s ease;
}

.email-note:hover .email-icon {
  opacity: 1;
}

.email-text {
  display: inline-block;
  white-space: nowrap;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.portal-meta {
  font-size: 12px;
  font-weight: 500;
  background: rgba(247, 246, 246, 0.7);
  border: 1px solid rgba(210, 215, 223, 0.4);
  padding: 3px 8px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.3s ease;
}

.portal-meta.loading {
  color: #64748b;
  font-style: italic;
}

.portal-meta.unknown {
  color: #64748b;
}

.portal-separator {
  color: rgba(100, 116, 139, 0.3);
  font-weight: 400;
}

.portal-info-item {
  white-space: nowrap;
  color: #64748b; /* 和时间、邮箱的默认颜色一致 */
}

.portal-value {
  /* 值的颜色由 getExpiryStyle 和 getBalanceStyle 动态设置 */
  display: inline;
}

.portal-info-item.expiry .portal-value {
  font-weight: 600;
}

.portal-info-item.balance .portal-value {
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

/* 复制按钮容器 */
.copy-button-container {
  position: relative;
  display: flex;
  align-items: center;
}

/* 检测菜单容器 */
.check-menu-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

/* 检测菜单包装器 */
.check-menu-wrapper {
  position: relative;
}

/* 复制菜单 */
.copy-menu {
  position: absolute;
  bottom: 100%;
  right: 0;
  background: rgba(255, 255, 255, 0.98);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: 140px;
  margin-bottom: 8px;
  pointer-events: auto;
  animation: slideUp 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 菜单项 */
.copy-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  background: none;
  border: none;
  color: #64748b;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s ease;
  text-align: left;
}

.copy-menu-item:first-child {
  border-radius: 8px 8px 0 0;
}

.copy-menu-item:last-child {
  border-radius: 0 0 8px 8px;
}

.copy-menu-item:not(:last-child) {
  border-bottom: 1px solid rgba(226, 232, 240, 0.4);
}

.copy-menu-item:hover {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  color: #4f46e5;
}

.copy-menu-item:active {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.12),
    rgba(139, 92, 246, 0.08)
  );
}

.copy-menu-item svg {
  flex-shrink: 0;
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.copy-menu-item:hover svg {
  opacity: 1;
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

/* 禁用检测状态样式 */
.btn-action.status-check.disabled {
  color: #64748b;
  opacity: 0.85;
}

.btn-action.status-check.disabled:hover {
  background: linear-gradient(
    135deg,
    rgba(100, 116, 139, 0.15),
    rgba(71, 85, 105, 0.08)
  );
  border-color: rgba(100, 116, 139, 0.3);
  color: #475569;
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

/* 长按进度动画 */
.btn-action.status-check {
  position: relative;
}

.long-press-progress {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 10;
  opacity: 0.6;
}

.progress-ring {
  stroke-dasharray: 87.96; /* 2 * π * 14 */
  stroke-dashoffset: 87.96;
  transform-origin: center;
  transform: rotate(-90deg);
  animation: longPressProgress 0.8s linear forwards;
  stroke-linecap: round;
}

@keyframes longPressProgress {
  from {
    stroke-dashoffset: 87.96;
  }
  to {
    stroke-dashoffset: 0;
  }
}

/* 编辑器选择模态框内容样式 */
.editor-modal-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
  max-height: 50vh;

  padding: 8px 0;
}

.modal-description {
  color: #666;
  text-align: center;
  margin-top: -20px;
  /* line-height: 1.5; */
}

/* 编辑器网格布局 */
.editor-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding-bottom: 20px;
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
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
  margin: 0;
  padding: 16px 0 8px 0;
  border-bottom: 2px solid rgba(226, 232, 240, 0.6);
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
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
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
    padding: 8px;
    min-height: 90px;
    margin: 4px 4px 2px;
  }

  .card-main {
    gap: 8px;
  }

  .token-info {
    min-width: 0;
  }

  .tenant-name-container {
    margin: 0 0 4px 0;
    gap: 4px;
  }

  .tenant-name {
    font-size: 14px;
    line-height: 1.2;
  }

  .tag-text {
    font-size: 9px;
    padding: 1px 3px;
  }

  .token-meta {
    gap: 3px;
  }

  .meta-row {
    gap: 6px;
    min-height: 24px;
  }

  .created-date {
    font-size: 11px;
  }

  .email-note {
    font-size: 11px;
    padding: 2px 6px;
  }

  .email-icon {
    width: 10px;
    height: 10px;
  }

  .portal-meta {
    font-size: 11px;
    padding: 2px 6px;
  }

  .status-indicator {
    top: 8px;
    right: 8px;
  }

  .status-badge {
    font-size: 10px;
    padding: 2px 6px;
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

  .loading-spinner {
    width: 16px;
    height: 16px;
    border-width: 2px;
  }
}

/* Dropdown 过渡动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.dropdown-enter-from {
  opacity: 0;
  transform: translateY(8px);
}

.dropdown-leave-to {
  opacity: 0;
  transform: translateY(8px);
}
</style>
