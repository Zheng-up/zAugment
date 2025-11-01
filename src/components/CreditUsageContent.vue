<template>
  <div class="credit-usage-content">
    <!-- 加载状态 - 骨架屏 -->
    <div v-if="loading" class="content-wrapper">
      <div class="chart-card">
        <!-- 统计数据骨架 -->
        <div class="stats-row">
          <div class="stat-item skeleton">
            <div class="stat-icon-skeleton skeleton-shimmer"></div>
            <div class="stat-text-skeleton">
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 56px; height: 14px"
              ></div>
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 45px; height: 20px; margin-top: 4px"
              ></div>
            </div>
          </div>

          <div class="stat-item skeleton">
            <div class="stat-icon-skeleton skeleton-shimmer"></div>
            <div class="stat-text-skeleton">
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 42px; height: 14px"
              ></div>
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 50px; height: 20px; margin-top: 4px"
              ></div>
            </div>
          </div>

          <div class="stat-item skeleton">
            <div class="stat-icon-skeleton skeleton-shimmer"></div>
            <div class="stat-text-skeleton">
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 56px; height: 14px"
              ></div>
              <div
                class="skeleton-line skeleton-shimmer"
                style="width: 60px; height: 20px; margin-top: 4px"
              ></div>
            </div>
          </div>
        </div>

        <!-- 柱状图骨架 -->
        <div class="chart-skeleton">
          <div class="skeleton-chart-container">
            <div class="skeleton-bars">
              <div
                class="skeleton-bar skeleton-shimmer"
                style="height: 45%"
              ></div>
              <div
                class="skeleton-bar skeleton-shimmer"
                style="height: 75%"
              ></div>
              <div
                class="skeleton-bar skeleton-shimmer"
                style="height: 60%"
              ></div>
              <div
                class="skeleton-bar skeleton-shimmer"
                style="height: 90%"
              ></div>
              <div
                class="skeleton-bar skeleton-shimmer"
                style="height: 55%"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <div class="error-icon">
        <svg
          width="56"
          height="56"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="8" x2="12" y2="12" />
          <line x1="12" y1="16" x2="12.01" y2="16" />
        </svg>
      </div>
      <div class="error-content">
        <h3 class="error-title">加载失败</h3>
        <p class="error-message">{{ error }}</p>
      </div>
    </div>

    <!-- 数据展示 -->
    <div v-else class="content-wrapper">
      <!-- 卡片 -->
      <div class="chart-card">
        <!-- 统计数据行 -->
        <div class="stats-row">
          <div class="stat-item">
            <div class="stat-icon today">
              <svg
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"
                />
              </svg>
            </div>
            <div class="stat-text">
              <span class="stat-label">今日消耗</span>
              <span class="stat-value">{{ todayCreditsDisplay }}</span>
            </div>
          </div>

          <div class="stat-item">
            <div class="stat-icon total">
              <svg
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"
                />
              </svg>
            </div>
            <div class="stat-text">
              <span class="stat-label">总消耗</span>
              <span class="stat-value">{{ totalCreditsDisplay }}</span>
            </div>
          </div>

          <div class="stat-item">
            <div class="stat-icon remaining">
              <svg
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M21 18v1c0 1.1-.9 2-2 2H5c-1.11 0-2-.9-2-2V5c0-1.1.89-2 2-2h14c1.1 0 2 .9 2 2v1h-9c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h9zm-9-2h10V8H12v8zm4-2.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"
                />
              </svg>
            </div>
            <div class="stat-text">
              <span class="stat-label">剩余额度</span>
              <span class="stat-value">{{ remainingCreditsDisplay }}</span>
            </div>
          </div>
        </div>

        <!-- 图表内容 -->
        <div
          v-if="
            !chartData ||
            !chartData.data_points ||
            chartData.data_points.length === 0
          "
          class="no-data"
        >
          <svg
            width="48"
            height="48"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
          >
            <rect x="3" y="3" width="7" height="18" rx="1" />
            <rect x="14" y="8" width="7" height="13" rx="1" />
            <line x1="6.5" y1="1" x2="6.5" y2="3" />
            <line x1="17.5" y1="6" x2="17.5" y2="8" />
          </svg>
          <p>暂无数据</p>
        </div>

        <div v-else class="chart-wrapper">
          <Bar :data="barChartData" :options="barChartOptions" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Bar } from "vue-chartjs";
import {
  Chart as ChartJS,
  BarElement,
  CategoryScale,
  LinearScale,
  Tooltip,
  Legend,
} from "chart.js";

// 注册 Chart.js 组件
ChartJS.register(BarElement, CategoryScale, LinearScale, Tooltip, Legend);

const props = defineProps({
  authSession: {
    type: String,
    required: true,
  },
  creditsBalance: {
    type: [Number, String],
    default: null,
  },
  hasPortalUrl: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update-portal-url']);

const loading = ref(false);
const error = ref(null);
const statsData = ref(null);
const chartData = ref(null);

// 统计卡片数据
const todayCredits = computed(() => {
  if (!statsData.value?.data_points?.length) return "0";
  // 获取最新的一天数据
  const latest =
    statsData.value.data_points[statsData.value.data_points.length - 1];
  return latest?.credits_consumed || "0";
});

const totalCredits = computed(() => {
  if (!statsData.value?.data_points?.length) return "0";
  // 计算所有数据点的总消耗
  const total = statsData.value.data_points.reduce((sum, point) => {
    return sum + (parseInt(point.credits_consumed) || 0);
  }, 0);
  return total.toString();
});

const formatCredits = (value) => {
  if (value === null || value === undefined || value === "") {
    return "--";
  }
  const numeric = Number(value);
  if (Number.isNaN(numeric)) {
    return `${value}`;
  }
  return numeric.toLocaleString();
};

const todayCreditsDisplay = computed(() => formatCredits(todayCredits.value));
const totalCreditsDisplay = computed(() => formatCredits(totalCredits.value));
const remainingCreditsDisplay = computed(() =>
  formatCredits(props.creditsBalance)
);

// 柱状图数据
const barChartData = computed(() => {
  if (!chartData.value?.data_points?.length) {
    return { labels: [], datasets: [] };
  }

  const labels = chartData.value.data_points.map(
    (p) => p.group_key || "未知模型"
  );
  const data = chartData.value.data_points.map(
    (p) => parseInt(p.credits_consumed) || 0
  );

  return {
    labels,
    datasets: [
      {
        label: "Credit 消耗",
        data,
        backgroundColor: "#6366f1",
        borderRadius: 8,
        barThickness: 40,
        maxBarThickness: 60,
      },
    ],
  };
});

// 柱状图配置
const barChartOptions = computed(() => {
  return {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      tooltip: {
        backgroundColor: "#ffffff",
        titleColor: "#1f2937",
        bodyColor: "#374151",
        borderColor: "#e5e7eb",
        borderWidth: 1,
        padding: 12,
        boxPadding: 6,
        callbacks: {
          label: (context) => {
            return `${context.parsed.y.toLocaleString()} credits`;
          },
        },
      },
    },
    scales: {
      x: {
        grid: {
          display: false,
        },
        ticks: {
          color: "#6b7280",
          font: {
            size: 11,
          },
        },
      },
      y: {
        beginAtZero: true,
        grid: {
          color: "#e5e7eb",
          drawBorder: false,
        },
        ticks: {
          color: "#6b7280",
          font: {
            size: 11,
          },
          callback: (value) => {
            return value.toLocaleString();
          },
        },
      },
    },
  };
});

// 获取数据
const fetchData = async () => {
  loading.value = true;
  error.value = null;

  try {
    // 如果已有 portal_url，则不需要获取
    const fetchPortalUrl = !props.hasPortalUrl;
    console.log('[CreditUsageContent] hasPortalUrl =', props.hasPortalUrl, ', fetchPortalUrl =', fetchPortalUrl);

    const result = await invoke("fetch_batch_credit_consumption", {
      authSession: props.authSession,
      fetchPortalUrl: fetchPortalUrl,  // 只有在没有 portal_url 时才获取
    });

    console.log('[CreditUsageContent] received portal_url =', result.portal_url);

    statsData.value = result.stats_data;
    chartData.value = result.chart_data;

    // 如果获取到 portal_url，通知父组件更新
    if (result.portal_url) {
      console.log('[CreditUsageContent] emitting update-portal-url event');
      emit('update-portal-url', result.portal_url);
    }
  } catch (e) {
    error.value = e.toString();
    console.error("Failed to fetch credit data:", e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchData();
});
</script>

<style scoped>
.credit-usage-content {
  display: flex;
  flex-direction: column;
  height: 56vh;
}

.content-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* 骨架屏 */
.content-wrapper .stat-item.skeleton {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(241, 245, 249, 0.6) 100%
  );
  pointer-events: none;
}

.content-wrapper .stat-item.skeleton:hover {
  transform: none;
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(241, 245, 249, 0.6) 100%
  );
}

.stat-icon-skeleton {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: #e5e7eb;
  flex-shrink: 0;
}

.stat-text-skeleton {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.skeleton-line {
  border-radius: 4px;
  background: #e5e7eb;
}

.skeleton-shimmer {
  background: linear-gradient(90deg, #e5e7eb 0%, #f3f4f6 50%, #e5e7eb 100%);
  background-size: 200% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
}

@keyframes shimmer {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}

.chart-skeleton {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 0 20px;
  min-height: 0;
}

.skeleton-chart-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  min-height: 0;
}

.skeleton-bars {
  display: flex;
  align-items: flex-end;
  justify-content: space-around;
  gap: 16px;
  width: 100%;
  height: 280px;
  margin-bottom: 8px;
}

.skeleton-bar {
  flex: 1;
  background: #e5e7eb;
  border-radius: 6px 6px 0 0;
  min-height: 40px;
  max-width: 60px;
}

/* 错误状态 */
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 24px;
  padding: 80px 20px;
  min-height: 400px;
}

.error-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #ef4444;
  animation: errorPulse 2s ease-in-out infinite;
}

@keyframes errorPulse {
  0%,
  100% {
    transform: scale(1);
    box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.4);
  }
  50% {
    transform: scale(1.05);
    box-shadow: 0 0 0 10px rgba(239, 68, 68, 0);
  }
}

.error-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  max-width: 400px;
  text-align: center;
}

.error-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.error-message {
  margin: 0;
  font-size: 14px;
  color: #6b7280;
  line-height: 1.6;
  word-break: break-word;
}

/* 柱状图卡片 */
.chart-card {
  padding: 24px 20px 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* 统计数据行 */
.stats-row {
  display: flex;

  gap: 24px;
  padding-bottom: 24px;
  margin-bottom: 14px;
}

.stat-item {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(241, 245, 249, 0.6) 100%
  );
  border-radius: 10px;
  transition: all 0.2s ease;
}

.stat-item:hover {
  background: linear-gradient(
    135deg,
    rgba(241, 245, 249, 1) 0%,
    rgba(226, 232, 240, 0.8) 100%
  );
  transform: translateY(-1px);
}

.stat-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-icon.today {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.25);
}

.stat-icon.total {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.25);
}

.stat-icon.remaining {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(245, 158, 11, 0.25);
}

.stat-text {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.stat-label {
  font-size: 12px;
  font-weight: 500;
  color: #64748b;
  line-height: 1.2;
}

.stat-value {
  font-size: 18px;
  font-weight: 700;
  color: #1e293b;
  line-height: 1.2;
}

/* 图表内容 */
.chart-wrapper {
  flex: 1;
  min-height: 0;
  position: relative;
}

.no-data {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: #9ca3af;
  padding: 60px 20px;
}

.no-data svg {
  opacity: 0.3;
}

.no-data p {
  margin: 0;
  font-size: 14px;
}
</style>
