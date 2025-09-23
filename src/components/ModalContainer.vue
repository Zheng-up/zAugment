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
          <div v-if="showHeader" class="modal-header">
            <h3 class="modal-title">{{ title }}</h3>
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

const emit = defineEmits(["close", "overlay-click"]);

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
  max-height: 82vh;
  display: flex;
  flex-direction: column;
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

/* 尺寸变体 */
.modal-container-content.small {
  width: 400px;
  max-width: 90vw;
}

.modal-container-content.medium {
  width: 600px;
  max-width: 90vw;
}

.modal-container-content.large {
  width: 800px;
  max-width: 95vw;
}

.modal-container-content.auto {
  width: auto;
  max-width: 90vw;
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

.modal-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
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
  padding: 32px;
  overflow-y: auto;
  flex: 1;
}

.modal-body.no-header {
  padding-top: 32px;
}

.modal-body.has-footer {
  padding-bottom: 0;
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
    padding: 16px;
  }

  .modal-container-content {
    width: 100% !important;
    max-width: none !important;
    max-height: 90vh;
    border-radius: 16px;
  }

  .modal-container-content.full-screen {
    width: 100% !important;
    height: 100vh !important;
    border-radius: 0;
    max-height: none;
  }

  .modal-header {
    padding: 20px 24px;
  }

  .modal-title {
    font-size: 18px;
  }

  .modal-body {
    padding: 24px;
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
