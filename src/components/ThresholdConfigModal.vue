<template>
  <ModalContainer
    :visible="visible"
    title="账号状态阈值配置"
    size="medium"
    @close="handleClose"
  >
    <div class="threshold-config-container">
      <!-- 时间阈值配置 -->
      <div class="config-section">
        <div class="section-header">
          <h4 class="section-title">时间阈值（天）</h4>
        </div>
        <div class="input-container">
          <!-- 颜色区域规则说明 -->
          <div class="color-rules">
            <span class="rule-item red">
              0 &lt; <span class="color-dot"></span> ≤
              {{ localTimeThresholds.warning }} </span
            >&nbsp;&nbsp;&nbsp;&nbsp;
            <span class="rule-item yellow">
              {{ localTimeThresholds.warning }} &lt;
              <span class="color-dot"></span> ≤
              {{ localTimeThresholds.safe }} </span
            >&nbsp;&nbsp;&nbsp;&nbsp;
            <span class="rule-item green">
              {{ localTimeThresholds.safe }} &lt;
              <span class="color-dot"></span> ≤ {{ localTimeMax }}
            </span>
          </div>

          <!-- 输入框区域 -->
          <div class="threshold-inputs">
            <div class="input-group">
              <div>
                <label>红色上限:</label>
                <input
                  type="number"
                  v-model.number="localTimeThresholds.warning"
                  @blur="handleTimeWarningBlur"
                  @input="clearError('timeWarning')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.timeWarning },
                  ]"
                  placeholder="1-365"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.timeWarning" class="error-message">{{
                errors.timeWarning
              }}</span>
            </div>
            <div class="input-group">
              <div>
                <label>黄色上限:</label>
                <input
                  type="number"
                  v-model.number="localTimeThresholds.safe"
                  @blur="handleTimeSafeBlur"
                  @input="clearError('timeSafe')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.timeSafe },
                  ]"
                  placeholder="1-365"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.timeSafe" class="error-message">{{
                errors.timeSafe
              }}</span>
            </div>
            <div class="input-group">
              <div>
                <label>绿色上限:</label>
                <input
                  type="number"
                  v-model.number="localTimeMax"
                  @blur="handleTimeMaxBlur"
                  @input="clearError('timeMax')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.timeMax },
                  ]"
                  placeholder="3-365"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.timeMax" class="error-message">{{
                errors.timeMax
              }}</span>
            </div>
          </div>
        </div>
      </div>
      <!-- 额度阈值配置 -->
      <div class="config-section">
        <div class="section-header">
          <h4 class="section-title">额度阈值</h4>
        </div>
        <div class="input-container">
          <!-- 颜色区域规则说明 -->
          <div class="color-rules">
            <span class="rule-item red">
              0 &lt; <span class="color-dot"></span> ≤
              {{ localBalanceThresholds.warning }} </span
            >&nbsp;&nbsp;&nbsp;&nbsp;
            <span class="rule-item yellow">
              {{ localBalanceThresholds.warning }} &lt;
              <span class="color-dot"></span> ≤
              {{ localBalanceThresholds.safe }} </span
            >&nbsp;&nbsp;&nbsp;&nbsp;
            <span class="rule-item green">
              {{ localBalanceThresholds.safe }} &lt;
              <span class="color-dot"></span> ≤ {{ localBalanceMax }}
            </span>
          </div>

          <!-- 输入框区域 -->
          <div class="threshold-inputs">
            <div class="input-group">
              <div>
                <label>红色上限:</label>
                <input
                  type="number"
                  v-model.number="localBalanceThresholds.warning"
                  @blur="handleBalanceWarningBlur"
                  @input="clearError('balanceWarning')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.balanceWarning },
                  ]"
                  placeholder="1-1000000"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.balanceWarning" class="error-message">{{
                errors.balanceWarning
              }}</span>
            </div>
            <div class="input-group">
              <div>
                <label>黄色上限:</label>
                <input
                  type="number"
                  v-model.number="localBalanceThresholds.safe"
                  @blur="handleBalanceSafeBlur"
                  @input="clearError('balanceSafe')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.balanceSafe },
                  ]"
                  placeholder="1-1000000"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.balanceSafe" class="error-message">{{
                errors.balanceSafe
              }}</span>
            </div>
            <div class="input-group">
              <div>
                <label>绿色上限:</label>
                <input
                  type="number"
                  v-model.number="localBalanceMax"
                  @blur="handleBalanceMaxBlur"
                  @input="clearError('balanceMax')"
                  :class="[
                    'input-threshold',
                    { 'input-error': errors.balanceMax },
                  ]"
                  placeholder="3-1000000"
                />
                <span class="unit"></span>
              </div>
              <span v-if="errors.balanceMax" class="error-message">{{
                errors.balanceMax
              }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <template #footer>
      <div class="modal-actions">
        <button @click="handleReset" class="btn-reset">恢复默认</button>
        <div class="action-buttons">
          <button @click="handleClose" class="btn-cancel">取消</button>
          <button @click="handleSave" class="btn-save">保存</button>
        </div>
      </div>
    </template>
  </ModalContainer>
</template>
<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalContainer from "./ModalContainer.vue";
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  timeThresholds: {
    type: Object,
    required: true,
  },
  balanceThresholds: {
    type: Object,
    required: true,
  },
  timeMax: {
    type: Number,
    required: true,
  },
  balanceMax: {
    type: Number,
    required: true,
  },
});
const emit = defineEmits(["close", "save", "reset"]);
// 本地状态
const localTimeThresholds = ref({ ...props.timeThresholds });
const localBalanceThresholds = ref({ ...props.balanceThresholds });
const localTimeMax = ref(props.timeMax);
const localBalanceMax = ref(props.balanceMax);

// 错误状态
const errors = ref({
  timeWarning: "",
  timeSafe: "",
  timeMax: "",
  balanceWarning: "",
  balanceSafe: "",
  balanceMax: "",
});
// 监听 props 变化
watch(
  () => props.timeThresholds,
  (newVal) => {
    localTimeThresholds.value = { ...newVal };
  },
  { deep: true }
);
watch(
  () => props.balanceThresholds,
  (newVal) => {
    localBalanceThresholds.value = { ...newVal };
  },
  { deep: true }
);
// 监听弹窗打开，重置所有状态
watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      // 弹窗打开时，恢复为上次保存的配置
      localTimeThresholds.value = { ...props.timeThresholds };
      localBalanceThresholds.value = { ...props.balanceThresholds };
      // 恢复绿色上限为上次保存的配置
      localTimeMax.value = props.timeMax;
      localBalanceMax.value = props.balanceMax;
    }
  }
);
// 清除错误
const clearError = (field) => {
  errors.value[field] = "";
};

// 清除所有错误
const clearAllErrors = () => {
  Object.keys(errors.value).forEach((key) => {
    errors.value[key] = "";
  });
};

// 验证单个数字输入
const validateNumber = (value, min, max, fieldName) => {
  const num = parseInt(value);

  if (value === "" || value === null || value === undefined) {
    return { valid: false, error: `请输入${fieldName}` };
  }

  if (isNaN(num)) {
    return { valid: false, error: `${fieldName}必须是数字` };
  }

  if (num < min || num > max) {
    return { valid: false, error: `${fieldName}必须在 ${min}-${max} 之间` };
  }

  return { valid: true, value: num };
};

// 验证时间阈值的关系
const validateTimeThresholdsRelation = () => {
  const warning = parseInt(localTimeThresholds.value.warning);
  const safe = parseInt(localTimeThresholds.value.safe);
  const max = parseInt(localTimeMax.value);

  if (isNaN(warning) || isNaN(safe) || isNaN(max)) {
    return true; // 如果有无效值，由单独的验证处理
  }

  let hasError = false;

  if (warning >= safe) {
    errors.value.timeSafe = "黄色上限必须大于红色上限";
    hasError = true;
  } else if (
    !errors.value.timeSafe ||
    errors.value.timeSafe === "黄色上限必须大于红色上限"
  ) {
    errors.value.timeSafe = "";
  }

  if (safe >= max) {
    errors.value.timeMax = "绿色上限必须大于黄色上限";
    hasError = true;
  } else if (
    !errors.value.timeMax ||
    errors.value.timeMax === "绿色上限必须大于黄色上限"
  ) {
    errors.value.timeMax = "";
  }

  return !hasError;
};

// 验证额度阈值的关系
const validateBalanceThresholdsRelation = () => {
  const warning = parseInt(localBalanceThresholds.value.warning);
  const safe = parseInt(localBalanceThresholds.value.safe);
  const max = parseInt(localBalanceMax.value);

  if (isNaN(warning) || isNaN(safe) || isNaN(max)) {
    return true; // 如果有无效值，由单独的验证处理
  }

  let hasError = false;

  if (warning >= safe) {
    errors.value.balanceSafe = "黄色上限必须大于红色上限";
    hasError = true;
  } else if (
    !errors.value.balanceSafe ||
    errors.value.balanceSafe === "黄色上限必须大于红色上限"
  ) {
    errors.value.balanceSafe = "";
  }

  if (safe >= max) {
    errors.value.balanceMax = "绿色上限必须大于黄色上限";
    hasError = true;
  } else if (
    !errors.value.balanceMax ||
    errors.value.balanceMax === "绿色上限必须大于黄色上限"
  ) {
    errors.value.balanceMax = "";
  }

  return !hasError;
};

// 处理时间阈值输入框失去焦点
const handleTimeWarningBlur = () => {
  const result = validateNumber(
    localTimeThresholds.value.warning,
    1,
    365,
    "红色上限"
  );
  if (!result.valid) {
    errors.value.timeWarning = result.error;
  } else {
    errors.value.timeWarning = "";
    validateTimeThresholdsRelation();
  }
};

const handleTimeSafeBlur = () => {
  const result = validateNumber(
    localTimeThresholds.value.safe,
    1,
    365,
    "黄色上限"
  );
  if (!result.valid) {
    errors.value.timeSafe = result.error;
  } else {
    errors.value.timeSafe = "";
    validateTimeThresholdsRelation();
  }
};

const handleTimeMaxBlur = () => {
  const result = validateNumber(localTimeMax.value, 3, 365, "绿色上限");
  if (!result.valid) {
    errors.value.timeMax = result.error;
  } else {
    errors.value.timeMax = "";
    validateTimeThresholdsRelation();
  }
};

// 处理额度阈值输入框失去焦点
const handleBalanceWarningBlur = () => {
  const result = validateNumber(
    localBalanceThresholds.value.warning,
    1,
    1000000,
    "红色上限"
  );
  if (!result.valid) {
    errors.value.balanceWarning = result.error;
  } else {
    errors.value.balanceWarning = "";
    validateBalanceThresholdsRelation();
  }
};

const handleBalanceSafeBlur = () => {
  const result = validateNumber(
    localBalanceThresholds.value.safe,
    1,
    1000000,
    "黄色上限"
  );
  if (!result.valid) {
    errors.value.balanceSafe = result.error;
  } else {
    errors.value.balanceSafe = "";
    validateBalanceThresholdsRelation();
  }
};

const handleBalanceMaxBlur = () => {
  const result = validateNumber(localBalanceMax.value, 3, 1000000, "绿色上限");
  if (!result.valid) {
    errors.value.balanceMax = result.error;
  } else {
    errors.value.balanceMax = "";
    validateBalanceThresholdsRelation();
  }
};
// 验证所有输入
const validateAll = () => {
  clearAllErrors();
  let hasError = false;

  // 验证时间阈值
  const timeWarningResult = validateNumber(
    localTimeThresholds.value.warning,
    1,
    365,
    "时间红色上限"
  );
  if (!timeWarningResult.valid) {
    errors.value.timeWarning = timeWarningResult.error;
    hasError = true;
  }

  const timeSafeResult = validateNumber(
    localTimeThresholds.value.safe,
    1,
    365,
    "时间黄色上限"
  );
  if (!timeSafeResult.valid) {
    errors.value.timeSafe = timeSafeResult.error;
    hasError = true;
  }

  const timeMaxResult = validateNumber(
    localTimeMax.value,
    3,
    365,
    "时间绿色上限"
  );
  if (!timeMaxResult.valid) {
    errors.value.timeMax = timeMaxResult.error;
    hasError = true;
  }

  // 验证额度阈值
  const balanceWarningResult = validateNumber(
    localBalanceThresholds.value.warning,
    1,
    1000000,
    "额度红色上限"
  );
  if (!balanceWarningResult.valid) {
    errors.value.balanceWarning = balanceWarningResult.error;
    hasError = true;
  }

  const balanceSafeResult = validateNumber(
    localBalanceThresholds.value.safe,
    1,
    1000000,
    "额度黄色上限"
  );
  if (!balanceSafeResult.valid) {
    errors.value.balanceSafe = balanceSafeResult.error;
    hasError = true;
  }

  const balanceMaxResult = validateNumber(
    localBalanceMax.value,
    3,
    1000000,
    "额度绿色上限"
  );
  if (!balanceMaxResult.valid) {
    errors.value.balanceMax = balanceMaxResult.error;
    hasError = true;
  }

  // 如果基本验证通过，验证阈值关系
  if (!hasError) {
    if (!validateTimeThresholdsRelation()) {
      hasError = true;
    }
    if (!validateBalanceThresholdsRelation()) {
      hasError = true;
    }
  }

  return !hasError;
};

// 处理保存
const handleSave = () => {
  // 验证所有输入
  if (!validateAll()) {
    return; // 如果验证失败，阻止保存
  }

  const saveData = {
    timeThresholds: {
      warning: localTimeThresholds.value.warning,
      safe: localTimeThresholds.value.safe,
    },
    balanceThresholds: {
      warning: localBalanceThresholds.value.warning,
      safe: localBalanceThresholds.value.safe,
    },
    timeMax: localTimeMax.value,
    balanceMax: localBalanceMax.value,
  };

  // 所有验证通过，保存数据（v-model.number 已经是数字类型）
  emit("save", saveData);
  emit("close");
};
// 处理关闭
const handleClose = () => {
  // 恢复原始值（恢复为上次保存的配置）
  localTimeThresholds.value = { ...props.timeThresholds };
  localBalanceThresholds.value = { ...props.balanceThresholds };
  // 恢复最大值为上次保存的配置
  localTimeMax.value = props.timeMax;
  localBalanceMax.value = props.balanceMax;
  emit("close");
};
// 恢复默认值（从后端获取）
const handleReset = async () => {
  try {
    const result = await invoke("load_status_thresholds");
    if (result) {
      localTimeThresholds.value = { ...result.time };
      localBalanceThresholds.value = { ...result.balance };
      localTimeMax.value = result.timeMax;
      localBalanceMax.value = result.balanceMax;
      emit("reset");
    }
  } catch (error) {
    console.error("从后端获取默认阈值失败:", error);
  }
};
</script>
<style scoped>
.threshold-config-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-bottom: 30px;
}
.config-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.section-header {
  display: flex;
  flex-direction: column;
  /* gap: 4px; */
}
.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}
.section-description {
  margin: 0;
  font-size: 13px;
  color: #64748b;
}
.input-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.color-rules {
  display: flex;
  flex-direction: row;
  gap: 16px;
  padding: 12px 16px;
  background: rgba(248, 250, 252, 0.8);
  border-radius: 8px;
  flex-wrap: wrap;
}
.rule-item {
  font-size: 13px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
  color: #475569;
}
.color-dot {
  width: 12px;
  height: 12px;
  border-radius: 3px;
  flex-shrink: 0;
}
.rule-item.red .color-dot {
  background: #b91c1c;
}
.rule-item.yellow .color-dot {
  background: #a16207;
}
.rule-item.green .color-dot {
  background: #15803d;
}
.threshold-inputs {
  display: flex;
  flex-direction: row;
  gap: 16px;
  flex-wrap: wrap;
}
.input-group {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
  flex: 1;
  min-width: 150px;
  position: relative;
}

.input-group > div {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}
.input-group label {
  font-size: 13px;
  font-weight: 500;
  color: #64748b;
  white-space: nowrap;
}
.input-threshold {
  flex: 1;
  min-width: 90px;
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  transition: border-color 0.15s ease;
}

/* 隐藏数字输入框的上下箭头 */
.input-threshold::-webkit-outer-spin-button,
.input-threshold::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.input-threshold[type="number"] {
  -moz-appearance: textfield;
  appearance: textfield;
}

.input-threshold:focus {
  outline: none;
  border-color: #6366f1;
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.1);
}

.input-threshold.input-error {
  border-color: #dc2626;
  background: rgba(220, 38, 38, 0.05);
}

.input-threshold.input-error:focus {
  border-color: #dc2626;
  box-shadow: 0 0 0 2px rgba(220, 38, 38, 0.1);
}

.error-message {
  font-size: 12px;
  color: #dc2626;
  margin-top: 2px;
  display: block;
  animation: slideDown 0.2s ease;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
.input-group .unit {
  font-size: 13px;
  color: #94a3b8;
  white-space: nowrap;
}
.modal-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.action-buttons {
  display: flex;
  gap: 12px;
}
.btn-reset,
.btn-cancel,
.btn-save {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  border: none;
  position: relative;
  overflow: hidden;
}
.btn-reset::before,
.btn-cancel::before,
.btn-save::before {
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
.btn-reset:hover::before,
.btn-cancel:hover::before,
.btn-save:hover::before {
  left: 100%;
}
.btn-reset {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border: 1px solid #e2e8f0;
}
.btn-reset:hover {
  background: #f1f5f9;
  color: #475569;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(100, 116, 139, 0.15);
}
.btn-cancel {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border: 1px solid #e2e8f0;
}
.btn-cancel:hover {
  background: #f1f5f9;
  color: #475569;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(100, 116, 139, 0.15);
}
.btn-save {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
}
.btn-save:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}
</style>
