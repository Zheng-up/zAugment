<template>
  <ModalContainer
    :visible="visible"
    title="标签"
    size="small"
    :show-close-button="true"
    @close="handleCancel"
  >
    <div class="tag-editor-content">
      <!-- 标签文字输入 -->
      <div class="form-group">
        <label for="tagText">标签文字</label>
        <input
          id="tagText"
          v-model="tagText"
          type="text"
          placeholder="请输入标签文字（最多6个字符）"
          maxlength="6"
          autocomplete="off"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
          @input="validateTagText"
        />
        <div class="char-count">{{ tagText.length }}/6</div>
        <div v-if="errors.tagText" class="error-message">
          {{ errors.tagText }}
        </div>
      </div>

      <!-- 颜色选择器 -->
      <div class="form-group">
        <label>标签颜色</label>
        <div class="color-palette">
          <div
            v-for="color in colorOptions"
            :key="color.value"
            :class="[
              'color-option',
              { selected: selectedColor === color.value },
            ]"
            :style="{ backgroundColor: color.hex }"
            :title="color.name"
            @click="selectColor(color.value)"
          >
            <svg
              v-if="selectedColor === color.value"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="white"
            >
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
            </svg>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部按钮 -->
    <template #footer>
      <div class="modal-footer">
        <button @click.stop="handleCancel" class="btn-secondary">取消</button>
        <button
          @click.stop="handleConfirm"
          class="btn-primary"
          :disabled="!isValid"
        >
          确认
        </button>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import ModalContainer from "./ModalContainer.vue";

// Props
const props = defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  initialTagText: {
    type: String,
    default: "",
  },
  initialTagColor: {
    type: String,
    default: null,
  },
});

// Emits
const emit = defineEmits(["close", "confirm"]);

// 颜色选项（黑色排在第一个）
const colorOptions = [
  { name: "黑色", value: "black", hex: "#1f2937" },
  { name: "红色", value: "red", hex: "#b91c1c" },
  { name: "绿色", value: "green", hex: "#15803d" },
  { name: "黄色", value: "yellow", hex: "#a16207" },
  { name: "蓝色", value: "blue", hex: "#3b82f6" },
];

// 颜色映射
const colorMap = {
  red: "#b91c1c",
  green: "#15803d",
  yellow: "#a16207",
  blue: "#3b82f6",
  black: "#1f2937",
};

// Reactive data
const tagText = ref("");
const selectedColor = ref("black"); // 默认为黑色
const errors = ref({
  tagText: "",
});

// Computed
const isValid = computed(() => {
  // 标签文字为可选，只要不超过6个字符即可
  return tagText.value.length <= 6;
});

// Methods
const validateTagText = () => {
  if (tagText.value.length > 6) {
    errors.value.tagText = "标签文字不能超过6个字符";
  } else {
    errors.value.tagText = "";
  }
};

const selectColor = (color) => {
  selectedColor.value = color;
};

const handleCancel = () => {
  emit("close");
};

const handleConfirm = () => {
  if (!isValid.value) {
    return;
  }

  // 如果标签文字为空，则传递空字符串
  const trimmedText = tagText.value.trim();

  emit("confirm", {
    tagText: trimmedText,
    tagColor: selectedColor.value,
  });
};

// Watch for visibility changes to reset form
watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      // 弹窗打开时，初始化表单数据
      tagText.value = props.initialTagText || "";
      selectedColor.value = props.initialTagColor || "black"; // 默认为黑色
      errors.value.tagText = "";
    }
  }
);

// 导出颜色映射供外部使用
defineExpose({
  colorMap,
});
</script>

<style scoped>
.tag-editor-content {
  padding: 20px 0;
}

.form-group {
  margin-bottom: 24px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #1e293b;
  font-size: 14px;
}

.form-group input {
  width: 100%;
  padding: 10px 12px;
  border: 2px solid rgba(226, 232, 240, 0.8);
  border-radius: 8px;
  font-size: 14px;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.form-group input:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.5);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.char-count {
  margin-top: 4px;
  font-size: 12px;
  color: #64748b;
  text-align: right;
}

.error-message {
  margin-top: 4px;
  font-size: 12px;
  color: #ef4444;
}

.color-palette {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.color-option {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  border: 3px solid transparent;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.color-option:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.color-option.selected {
  border-color: #3b82f6;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 20px;
}

.btn-primary,
.btn-secondary {
  padding: 10px 24px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.btn-primary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: white;
  color: #64748b;
  border: 2px solid rgba(226, 232, 240, 0.8);
}

.btn-secondary:hover {
  background: rgba(248, 250, 252, 0.8);
  border-color: rgba(203, 213, 225, 0.8);
}
</style>
