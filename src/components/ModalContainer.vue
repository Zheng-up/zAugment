<template>
  <Teleport to="body">
    <Transition name="modal" appear>
      <div
        v-if="visible"
        class="modal-container-overlay"
        @click="handleOverlayClick"
      >
        <div
          class="modal-container-content"
          :class="[size, { 'full-screen': fullScreen }]"
          @click.stop
        >
          <!-- 头部 -->
          <div
            v-if="showHeader"
            class="modal-header"
            :class="{ 'has-tabs': hasTabs }"
          >
            <!-- 单标题模式 -->
            <h3 v-if="!hasTabs" class="modal-title">{{ title }}</h3>

            <!-- 多标题（Tab）模式 -->
            <div v-if="hasTabs" class="modal-tabs">
              <button
                v-for="tab in tabs"
                :key="tab.key"
                @click="handleTabChange(tab.key)"
                class="modal-tab-button"
                :class="{ active: activeTab === tab.key }"
              >
                {{ tab.title }}
              </button>
            </div>

            <!-- 标题右侧自定义内容 slot -->
            <div v-if="$slots['header-actions']" class="modal-header-actions">
              <slot name="header-actions"></slot>
            </div>

            <button
              v-if="showCloseButton"
              class="modal-close-btn"
              @click="$emit('close')"
              type="button"
            >
              <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
                />
              </svg>
            </button>
          </div>

          <!-- 内容区域 -->
          <div
            class="modal-body"
            :class="{ 'no-header': !showHeader, 'has-footer': $slots.footer }"
          >
            <slot></slot>
          </div>

          <!-- 页脚区域 -->
          <div v-if="$slots.footer" class="modal-footer">
            <slot name="footer"></slot>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { computed, watch } from "vue";

const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: "",
  },
  // 新增：多标题支持
  tabs: {
    type: Array,
    default: () => [],
    // 格式: [{ key: 'tab1', title: '标题1' }, { key: 'tab2', title: '标题2' }]
  },
  activeTab: {
    type: String,
    default: "",
  },
  size: {
    type: String,
    default: "medium", // small, medium, large, auto
    validator: (value) => ["small", "medium", "large", "auto"].includes(value),
  },
  showHeader: {
    type: Boolean,
    default: true,
  },
  showCloseButton: {
    type: Boolean,
    default: true,
  },
  closeOnOverlay: {
    type: Boolean,
    default: true,
  },
  fullScreen: {
    type: Boolean,
    default: false,
  },
  zIndex: {
    type: Number,
    default: 1001,
  },
});

const emit = defineEmits(["close", "overlay-click", "tab-change"]);

// 计算属性
const hasTabs = computed(() => props.tabs && props.tabs.length > 0);
const currentTitle = computed(() => {
  if (hasTabs.value && props.activeTab) {
    const activeTabObj = props.tabs.find((tab) => tab.key === props.activeTab);
    return activeTabObj ? activeTabObj.title : props.title;
  }
  return props.title;
});

// 处理tab切换
const handleTabChange = (tabKey) => {
  emit("tab-change", tabKey);
};

// 处理遮罩层点击
const handleOverlayClick = () => {
  emit("overlay-click");
  if (props.closeOnOverlay) {
    emit("close");
  }
};

// 监听 ESC 键关闭弹窗
watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      document.addEventListener("keydown", handleEscKey);
      document.body.style.overflow = "hidden";
    } else {
      document.removeEventListener("keydown", handleEscKey);
      document.body.style.overflow = "";
    }
  }
);

const handleEscKey = (e) => {
  if (e.key === "Escape") {
    emit("close");
  }
};
</script>

<style scoped>
/* 弹窗容器遮罩层 */
.modal-container-overlay {
  position: fixed;
  top: 0;
  left: 200px; /* 左侧边栏宽度 */
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: v-bind(zIndex);
  padding: 20px;
}

/* 弹窗内容容器 */
.modal-container-content {
  background: linear-gradient(145deg, #ffffff 0%, #fafbfc 100%);
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.25),
    0 8px 16px -8px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  max-height: calc(100vh - 40px);
  display: flex;
  flex-direction: column;
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

/* 尺寸变体 */
.modal-container-content.small {
  width: 90%;
  max-width: 400px;
}

.modal-container-content.medium {
  width: 90%;
  max-width: 600px;
}

.modal-container-content.large {
  width: 95%;
  max-width: 800px;
}

.modal-container-content.auto {
  width: 90%;
  max-width: 600px;
  min-width: 300px;
}

.modal-container-content.full-screen {
  width: calc(100vw - 320px); /* 减去侧边栏宽度和边距 */
  height: calc(100vh - 40px);
  max-width: none;
  max-height: none;
}

/* 弹窗头部 */
.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 32px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
  flex-shrink: 0;
}

.modal-header.has-tabs {
  padding: 16px 32px 0 32px;
  border-bottom: none;
  background: transparent;
}

.modal-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
}

.modal-header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 12px;
  margin-right: auto;
}

/* Tab 样式 - 简化版 */
.modal-tabs {
  display: flex;
  background: rgba(248, 250, 252, 0.6);
  border-radius: 8px;
  padding: 2px;
  gap: 2px;
  width: auto;
  margin-right: 16px;
}

.modal-tab-button {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  background: transparent;
  color: #64748b;
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.modal-tab-button::before {
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

.modal-tab-button:hover::before {
  left: 100%;
}

.modal-tab-button:hover:not(.active) {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1) 0%,
    rgba(79, 70, 229, 0.05) 100%
  );
  color: #4f46e5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
}

.modal-tab-button.active {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  transform: translateY(-1px);
}

.modal-close-btn {
  background: none;
  border: none;
  color: #64748b;
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close-btn:hover {
  background: rgba(226, 232, 240, 0.5);
  color: #374151;
}

/* 弹窗内容区域 */
.modal-body {
  padding: 14px 32px;
  overflow-y: auto;
  flex: 1;
  box-sizing: border-box;
}

.modal-body.no-header {
  padding-top: 32px;
}

.modal-body.has-footer {
  padding-bottom: 0;
}

/* 有tabs时的内容区域样式调整 */
.modal-header.has-tabs + .modal-body {
  padding-top: 24px;
  border-top: 1px solid rgba(226, 232, 240, 0.3);
}

/* 弹窗页脚 */
.modal-footer {
  padding: 20px 32px;
  border-top: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
  flex-shrink: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .modal-container-overlay {
    left: 0;
    padding: 12px;
  }

  .modal-container-content {
    width: calc(100% - 24px);
    max-width: 600px;
    max-height: 90vh;
    border-radius: 16px;
  }

  .modal-container-content.small {
    max-width: 400px;
  }

  .modal-container-content.medium {
    max-width: 600px;
  }

  .modal-container-content.large {
    max-width: 700px;
  }

  .modal-container-content.full-screen {
    width: calc(100% - 24px) !important;
    height: calc(100vh - 24px) !important;
    border-radius: 0;
    max-height: none;
    max-width: none;
  }

  .modal-header {
    padding: 20px 24px;
  }

  .modal-header.has-tabs {
    padding: 12px 24px 0 24px;
  }

  .modal-title {
    font-size: 18px;
  }

  .modal-tabs {
    margin-right: 12px;
  }

  .modal-tab-button {
    padding: 8px 12px;
    font-size: 13px;
  }

  .modal-body {
    padding: 14px 24px;
  }

  .modal-header.has-tabs + .modal-body {
    padding-top: 14px;
    margin-top: 12px;
  }
}

/* 过渡动画 - 优化流畅度 */
.modal-enter-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.modal-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
  backdrop-filter: blur(0px);
}

.modal-enter-to,
.modal-leave-from {
  opacity: 1;
  backdrop-filter: blur(4px);
}

.modal-enter-from .modal-container-content {
  transform: translateY(-30px) scale(0.9);
  opacity: 0;
}

.modal-leave-to .modal-container-content {
  transform: translateY(20px) scale(0.95);
  opacity: 0;
}

.modal-enter-to .modal-container-content,
.modal-leave-from .modal-container-content {
  transform: translateY(0) scale(1);
  opacity: 1;
}
</style>
