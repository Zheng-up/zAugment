<template>
  <div
    class="custom-title-bar"
    @mousedown="handleMouseDown"
    @contextmenu.prevent
  >
    <!-- 左侧：应用标题 悬浮提示 -->
    <div class="title-bar-left">
      <div
        class="app-title"
        title="点击切换窗口尺寸"
        @click="handleTitleDoubleClick"
      >
        ZAugment
      </div>
    </div>

    <!-- 中间：空白区域 -->
    <div class="title-bar-center">
      <!-- 可以在这里添加其他内容，比如当前窗口尺寸指示器 -->
      <div class="window-size-indicator">
        <span class="size-text">{{ getSizeDisplayText() }}</span>
      </div>
    </div>

    <!-- 右侧：窗口控制按钮 -->
    <div class="title-bar-right">
      <div class="window-controls" @mousedown.stop @dblclick.stop>
        <!-- 最小化按钮 -->
        <button
          @click="minimizeWindow"
          class="control-btn minimize-btn"
          title="最小化"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 12h12v2H6z" />
          </svg>
        </button>

        <!-- 窗口尺寸切换按钮 -->
        <button
          @click="cycleWindowSize"
          class="control-btn maximize-btn"
          :title="getNextSizeTitle()"
        >
          <svg
            v-if="currentSize === 'small'"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <rect x="6" y="6" width="12" height="12" rx="2" />
          </svg>
          <svg
            v-else-if="currentSize === 'medium'"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <rect x="4" y="4" width="16" height="16" rx="2" />
          </svg>
          <svg
            v-else
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <rect x="2" y="2" width="20" height="20" rx="2" />
          </svg>
        </button>

        <!-- 关闭按钮 -->
        <button @click="closeWindow" class="control-btn close-btn" title="关闭">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M18.3 5.71a.996.996 0 0 0-1.41 0L12 10.59 7.11 5.7A.996.996 0 1 0 5.7 7.11L10.59 12 5.7 16.89a.996.996 0 1 0 1.41 1.41L12 13.41l4.89 4.89a.996.996 0 1 0 1.41-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 响应式状态
const isMaximized = ref(false);
const currentSize = ref("medium"); // 'small', 'medium', 'fullscreen'

// 拖拽相关状态
const isDragging = ref(false);

// 窗口控制函数
const minimizeWindow = async () => {
  try {
    await invoke("minimize_window");
  } catch (error) {
    console.error("最小化窗口失败:", error);
  }
};

// 循环切换窗口尺寸（优化：从全屏直接跳到小窗口）
const cycleWindowSize = async () => {
  try {
    let nextSize;

    // 优化切换逻辑：从全屏直接跳到小窗口
    if (currentSize.value === "fullscreen") {
      nextSize = "small";
    } else if (currentSize.value === "small") {
      nextSize = "medium";
    } else if (currentSize.value === "medium") {
      nextSize = "fullscreen";
    } else {
      // 如果当前尺寸不在预定义列表中，从小窗口开始
      nextSize = "small";
    }

    console.log(`循环切换: ${currentSize.value} -> ${nextSize}`);

    await setWindowSize(nextSize);
  } catch (error) {
    console.error("循环切换窗口尺寸失败:", error);
  }
};

// 获取下一个尺寸的提示文本（匹配新的切换逻辑）
const getNextSizeTitle = () => {
  let nextSize;

  if (currentSize.value === "fullscreen") {
    nextSize = "small";
  } else if (currentSize.value === "small") {
    nextSize = "medium";
  } else if (currentSize.value === "medium") {
    nextSize = "fullscreen";
  } else {
    nextSize = "small";
  }

  const sizeNames = {
    small: "小窗口 (760×700)",
    medium: "中等窗口 (1050×700)",
    fullscreen: "全屏模式",
  };

  return `切换到${sizeNames[nextSize]}`;
};

// 获取当前尺寸的显示文本
const getSizeDisplayText = () => {
  const sizeNames = {
    small: "小",
    medium: "中",
    fullscreen: "大",
  };

  return sizeNames[currentSize.value] || "未知";
};

const closeWindow = async () => {
  try {
    await invoke("close_app_window");
  } catch (error) {
    console.error("关闭窗口失败:", error);
  }
};

// 窗口拖拽功能（无延迟，立即启动）
const handleMouseDown = async (event) => {
  // 只处理左键点击
  if (event.button !== 0) return;

  // 防止在标题文字区域触发拖拽（因为标题有双击功能）
  if (event.target.closest(".app-title")) {
    return;
  }

  // 防止在按钮区域触发拖拽
  if (event.target.closest(".window-controls")) {
    return;
  }

  try {
    isDragging.value = true;
    await invoke("start_drag");
  } catch (error) {
    console.error("开始拖拽失败:", error);
  } finally {
    isDragging.value = false;
  }
};

// 标题双击切换窗口尺寸
const handleTitleDoubleClick = async () => {
  console.log("标题双击事件触发，切换窗口尺寸");

  try {
    await cycleWindowSize();
  } catch (error) {
    console.error("标题双击切换窗口尺寸失败:", error);
  }
};

// 窗口尺寸切换函数（优化版本）
const setWindowSize = async (size) => {
  try {
    // 如果点击的是当前已激活的尺寸，则不执行操作
    if (currentSize.value === size) return;

    console.log(`切换窗口尺寸: ${currentSize.value} -> ${size}`);

    // 处理全屏模式
    if (size === "fullscreen") {
      // 如果当前不是全屏，切换到全屏
      if (currentSize.value !== "fullscreen") {
        await invoke("set_fullscreen", { fullscreen: true });
        currentSize.value = size;
        console.log("已切换到全屏模式");
      }
      return;
    }

    // 设置窗口尺寸
    let width, height;
    switch (size) {
      case "small":
        width = 760;
        height = 700;
        break;
      case "medium":
        width = 1020;
        height = 700;
        break;
      default:
        console.error("未知的窗口尺寸:", size);
        return;
    }

    // 如果当前是全屏，先退出全屏并同时设置尺寸
    if (currentSize.value === "fullscreen") {
      await invoke("set_fullscreen", { fullscreen: false });
      // 立即设置尺寸，不等待
      await invoke("set_window_size", { width, height });
    } else {
      // 直接设置尺寸
      await invoke("set_window_size", { width, height });
    }

    currentSize.value = size;
    console.log(`窗口尺寸已设置为: ${width}x${height}`);
  } catch (error) {
    console.error("设置窗口尺寸失败:", error);
  }
};

// 初始化
onMounted(async () => {
  try {
    // 获取当前窗口状态
    const windowState = await invoke("get_window_state");
    isMaximized.value = windowState.isMaximized;

    // 优先检查是否为全屏模式
    if (windowState.isFullscreen) {
      currentSize.value = "fullscreen";
      console.log("初始化: 当前为全屏模式");
      return;
    }

    // 根据当前窗口尺寸判断当前模式
    const size = await invoke("get_window_size");
    console.log(`初始化: 当前窗口尺寸 ${size.width}x${size.height}`);

    if (size.width === 760 && size.height === 700) {
      currentSize.value = "small";
      console.log("初始化: 识别为小窗口模式");
    } else if (size.width === 1020 && size.height === 700) {
      currentSize.value = "medium";
      console.log("初始化: 识别为中等窗口模式");
    } else {
      // 如果尺寸不匹配预定义的尺寸，默认设置为中等窗口
      currentSize.value = "medium";
      console.log("初始化: 尺寸不匹配，默认设置为中等窗口模式");
    }
  } catch (error) {
    console.error("获取窗口状态失败:", error);
    // 如果获取状态失败，默认设置为中等窗口
    currentSize.value = "medium";
  }
});
</script>

<style scoped>
.custom-title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 40px;
  background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  padding: 0 16px;
  user-select: none;
  position: relative;
  z-index: 10001;
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.title-bar-left {
  display: flex;
  align-items: center;
  flex: 1;
}

.app-title {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
  margin-left: 8px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.app-title:hover {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.title-bar-center {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.window-size-indicator {
  display: flex;
  align-items: center;
  padding: 4px 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
}

.size-text {
  font-size: 11px;
  color: #64748b;
  font-weight: 500;
  letter-spacing: 0.5px;
}

.title-bar-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex: 1;
}

.window-controls {
  display: flex;
  gap: 6px;
}

.control-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.8);
  color: #64748b;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(226, 232, 240, 0.4);
  overflow: hidden;
}

.control-btn::before {
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
  transition: left 0.5s;
}

.control-btn:hover::before {
  left: 100%;
}

.control-btn:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.minimize-btn:hover {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.15) 0%,
    rgba(99, 102, 241, 0.1) 100%
  );
  color: #3b82f6;
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 4px 14px rgba(59, 130, 246, 0.25);
}

.maximize-btn:hover {
  background: linear-gradient(
    135deg,
    rgba(16, 185, 129, 0.15) 0%,
    rgba(34, 197, 94, 0.1) 100%
  );
  color: #10b981;
  border-color: rgba(16, 185, 129, 0.3);
  box-shadow: 0 4px 14px rgba(16, 185, 129, 0.25);
}

.close-btn:hover {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.15) 0%,
    rgba(248, 113, 113, 0.1) 100%
  );
  color: #ef4444;
  border-color: rgba(239, 68, 68, 0.3);
  box-shadow: 0 4px 14px rgba(239, 68, 68, 0.25);
}

.control-btn:active {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .custom-title-bar {
    height: 36px;
    padding: 0 12px;
  }

  .app-title {
    font-size: 13px;
  }

  .window-size-controls {
    gap: 2px;
    padding: 2px;
  }

  .size-btn {
    width: 24px;
    height: 24px;
  }

  .control-btn {
    width: 28px;
    height: 28px;
  }
}
</style>
