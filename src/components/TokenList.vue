<template>
  <div class="token-list-container">
    <!-- 统一的账号管理卡片 -->
    <div class="unified-account-card">
     

      <!-- 分割线 -->
      <div v-if="tokens.length > 0" class="card-divider"></div>

      <!-- 内容区域 -->
      <div class="card-content" :class="windowSizeClass">
      

    

        <!-- 搜索和排序工具栏 -->
        <div class="list-toolbar">
          <!-- 搜索框 -->
          <div class="search-box">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="search-icon">
              <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" />
            </svg>
            <input
              type="text"
              v-model="searchQuery"
              placeholder="邮箱 标签 Token Sessin）"
              class="search-input"
              name="token-search"
              autocomplete="new-password"
              autocorrect="off"
              autocapitalize="off"
              spellcheck="false"
              :disabled="tokens.length === 0"
            />
            <button v-if="searchQuery.trim()" @click="searchQuery = ''" class="clear-search-btn" title="清空搜索">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>

          <!-- 排序按钮 -->
          <div class="sort-dropdown" @click.stop @mouseenter="tokens.length > 0 ? showSortMenuOnHover() : null" @mouseleave="hideSortMenuOnLeave">
            <button class="sort-btn" @click="tokens.length > 0 ? toggleSortMenu() : null" :disabled="tokens.length === 0" title="排序方式">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z"/>
              </svg>
              <span>{{ sortType === 'time' ? '时间' : '额度' }}</span>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" :class="['arrow-icon', { rotated: showSortMenu }]">
                <path d="M7 10l5 5 5-5z"/>
              </svg>
            </button>

            <!-- 下拉菜单 -->
            <div v-if="showSortMenu" class="sort-menu" @click.stop @mouseenter="clearSortMenuTimer" @mouseleave="hideSortMenuOnLeave">
              <button :class="['sort-option', { active: sortType === 'time' && sortOrder === 'asc' }]" @click="setSortType('time', 'asc')">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>
                </svg>
                <span>时间正序</span>
                
                <svg v-if="sortType === 'time' && sortOrder === 'asc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                </svg>
              </button>

              <button :class="['sort-option', { active: sortType === 'time' && sortOrder === 'desc' }]" @click="setSortType('time', 'desc')">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>
                </svg>
                <span>时间倒序</span>
                
                <svg v-if="sortType === 'time' && sortOrder === 'desc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                </svg>
              </button>

             

              <button :class="['sort-option', { active: sortType === 'balance' && sortOrder === 'asc' }]" @click="setSortType('balance', 'asc')">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                </svg>
                <span>额度正序</span>
                
                <svg v-if="sortType === 'balance' && sortOrder === 'asc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                </svg>
              </button>

              <button :class="['sort-option', { active: sortType === 'balance' && sortOrder === 'desc' }]" @click="setSortType('balance', 'desc')">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                </svg>
                <span>额度倒序</span>
              
                <svg v-if="sortType === 'balance' && sortOrder === 'desc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                </svg>
              </button>
            </div>
          </div>

          <!-- 导入导出按钮 -->
          <button
            class="import-export-btn"
            @click="handleImportExport"
            title="批量导入/导出"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M9 3L5 6.99h3V14h2V6.99h3L9 3zm7 14.01V10h-2v7.01h-3L15 21l4-3.99h-3z"/>
            </svg>
            <span>批量</span>
          </button>

          <!-- 批量删除按钮 -->
          <button
            class="batch-delete-btn"
            @click="handleBatchDelete"
            :disabled="tokens.length === 0"
            :title="tokens.length > 0 ? '批量删除账号' : '暂无账号'"
          >
            <span class="btn-inner">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
              </svg>
            </span>
            <span v-if="deletableTokensCount > 0" class="badge">{{ deletableTokensCount }}</span>
          </button>

          <!-- 分页信息 -->
          <div v-if="tokens.length > 0" class="pagination-info">
            显示{{ paginationInfo.start }}-{{ paginationInfo.end }} 共{{ paginationInfo.total }}条
          </div>
        </div>
  <!-- Empty State -->
  <div v-if="tokens.length === 0" class="empty-state">
          <div class="empty-state-content">
            <div class="empty-icon">
              <svg
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
                />
              </svg>
            </div>
            <h4>还没有账号</h4>
            <p>
              点击右上角"新增账号"按钮添加您的第一个Augment账号，或在注册页面获取新账号。
            </p>
            <div class="empty-actions">
              <button @click="$emit('add-new-token')" class="btn-empty primary">
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                </svg>
                立即新增
              </button>
            </div>
          </div>
        </div>
        <!-- Token List -->
        <div v-if="tokens.length > 0" class="token-list">
          <div class="token-grid">
            <TokenCard
              v-for="token in paginatedTokens"
              :key="token.id"
              :ref="(el) => setTokenCardRef(el, token.id)"
              :token="token"
              :is-batch-checking="isRefreshing"
              :statusThresholds="statusThresholds"
              @delete="handleDeleteToken"
              @copy-success="handleCopySuccess"
              @edit="handleEditToken"
              @token-updated="handleTokenUpdated"
            />
          </div>

          
        </div>
        <!-- 分页控件 -->
        <div v-if="tokens.length > 0" class="pagination-controls">
            <!-- 中间：翻页控件 -->
            <div class="pagination-center">
              <button @click="prevPage" :disabled="currentPage === 1 || totalPages <= 1" class="pagination-btn">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>
                </svg>
                上一页
              </button>

              <!-- 页码信息 -->
              <div class="page-info">
                <span class="current-page">{{ currentPage }}</span>
                <span class="page-separator">/</span>
                <span class="total-pages">{{ totalPages }}</span>
              </div>

              <button @click="nextPage" :disabled="currentPage === totalPages || totalPages <= 1" class="pagination-btn">
                下一页
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                </svg>
              </button>
            </div>

            <!-- 右侧：每页数量选择器 -->
            <div class="page-size-dropdown" @click.stop @mouseenter="showPageSizeMenuOnHover" @mouseleave="hidePageSizeMenuOnLeave">
              <button
                class="page-size-btn"
                @click="togglePageSizeMenu"
                title="每页数量"
              >
                <span>{{ pageSize }} 条/页</span>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" :class="['arrow-icon', { rotated: showPageSizeMenu }]">
                  <path d="M7 10l5 5 5-5z"/>
                </svg>
              </button>

              <!-- 下拉菜单 -->
              <div v-if="showPageSizeMenu" class="page-size-menu" @click.stop @mouseenter="clearPageSizeMenuTimer" @mouseleave="hidePageSizeMenuOnLeave">
                <button
                  v-for="size in pageSizeOptions"
                  :key="size"
                  :class="['page-size-option', { active: pageSize === size }]"
                  @click="selectPageSize(size)"
                >

                  <span>{{ size }} 条/页</span>
                  <svg v-if="pageSize === size" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                    <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>
      </div>
    </div>
  </div>

  <!-- AccountManagerModal (导入/导出) -->
  <AccountManagerModal
    :visible="showAccountManagerModal"
    :tokens="sortedTokens"
    :initial-tab="accountManagerTab"
    @close="showAccountManagerModal = false"
    @import="handleAccountManagerImport"
    @import-error="handleAccountManagerImportError"
    @export-clipboard="handleAccountManagerExportClipboard"
    @export-session="handleAccountManagerExportSession"
  />

  <!-- TagEditorModal (导入导出时创建标签) -->
  <TagEditorModal
    v-if="showTagEditorForImportExport"
    :visible="showTagEditorForImportExport"
    :initial-tag-text="''"
    :initial-tag-color="''"
    @close="handleTagEditorForImportExportClose"
    @confirm="handleTagEditorForImportExportConfirm"
  />

  <!-- TokenForm Modal -->
  <TokenForm
    v-if="showTokenFormModal"
    :token="editingToken"
    @close="closeTokenForm"
    @success="handleTokenFormSuccess"
    @update-token="handleUpdateToken"
    @add-token="handleAddTokenFromForm"
    @show-status="handleCopySuccess"
  />

  <!-- 单个删除确认对话框 -->
  <ModalContainer
    :visible="showDeleteConfirm"
    title="确认删除账号"
    size="small"
    @close="cancelDelete"
  >
    <div class="delete-confirm-content">
      <div class="confirm-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M12,2C17.53,2 22,6.47 22,12C22,17.53 17.53,22 12,22C6.47,22 2,17.53 2,12C2,6.47 6.47,2 12,2M15.59,7L12,10.59L8.41,7L7,8.41L10.59,12L7,15.59L8.41,17L12,13.41L15.59,17L17,15.59L13.41,12L17,8.41L15.59,7Z"
          />
        </svg>
      </div>
      <h4>确认删除账号</h4>
      <p>
        确定要删除 <strong>{{ tokenToDeleteName }}</strong> 吗？
      </p>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button @click="cancelDelete" class="btn secondary">取消</button>
        <button @click="confirmDelete" class="btn danger">确认删除</button>
      </div>
    </template>
  </ModalContainer>

  <!-- 批量删除选项对话框 -->
  <ModalContainer
    :visible="showBatchDeleteOptions"
    title="批量删除"
    size="medium"
    @close="showBatchDeleteOptions = false"
  >
    <div class="batch-delete-options-content">
      <p class="options-description">请选择删除条件：</p>

      <!-- 删除选项 -->
      <div class="delete-options">
        <!-- 按状态删除 -->
        <div
          :class="['delete-option', { active: batchDeleteType === 'status', disabled: deletableTokensCount === 0 }]"
          @click="deletableTokensCount > 0 ? selectDeleteType('status') : null"
        >
          <div class="option-radio">
            <div v-if="batchDeleteType === 'status'" class="radio-dot"></div>
          </div>
          <div class="option-content">
            <div class="option-title">按状态删除</div>
            <div class="option-description">
              删除已封禁/过期的账号
              <span v-if="deletableTokensCount > 0" class="option-count">（{{ deletableTokensCount }} 个）</span>
              <span v-else class="option-count disabled">（暂无）</span>
            </div>
          </div>
        </div>

        <!-- 按标签删除 -->
        <div
          :class="['delete-option', { active: batchDeleteType === 'tag', disabled: availableTags.length === 0 }]"
          @click="availableTags.length > 0 ? selectDeleteType('tag') : null"
        >
          <div class="option-radio">
            <div v-if="batchDeleteType === 'tag'" class="radio-dot"></div>
          </div>
          <div class="option-content">
            <div class="option-title">按标签删除</div>
            <div class="option-description">
              删除指定标签的账号
              <span v-if="availableTags.length > 0" class="option-count">（{{ availableTags.length }} 个标签）</span>
              <span v-else class="option-count disabled">（暂无标签）</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 标签选择器 -->
      <div v-if="batchDeleteType === 'tag'" class="tag-selector-container">
        <label class="tag-selector-label">选择要删除的标签：</label>
        <div class="tags-grid">
          <div
            v-for="tag in availableTags"
            :key="tag.name"
            :class="['tag-item', { selected: selectedTagForDelete === tag.name }]"
            @click="selectedTagForDelete = tag.name"
          >
            <span class="tag-badge" :style="getTagStyle(tag.color)">
              {{ tag.name }}
            </span>
            <span class="tag-count-badge">{{ tag.count }}</span>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button @click="showBatchDeleteOptions = false" class="btn secondary">
          取消
        </button>
        <button
          @click="confirmBatchDeleteOptions"
          class="btn danger"
          :disabled="!canProceedToConfirm"
        >
          下一步
        </button>
      </div>
    </template>
  </ModalContainer>

  <!-- 批量删除确认对话框 -->
  <ModalContainer
    :visible="showBatchDeleteConfirm"
    title="确认批量删除"
    size="small"
    @close="showBatchDeleteConfirm = false"
  >
    <div class="batch-delete-content">
      <div class="batch-delete-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
        </svg>
      </div>
      <h4>批量删除账号</h4>
      <p class="delete-message">
        <template v-if="batchDeleteType === 'status'">
          即将删除 <strong class="delete-count">{{ deletableTokensCount }}</strong> 个账号
          <span class="delete-breakdown">（已封禁 {{ bannedTokensCount }} 个，已过期 {{ expiredTokensCount }} 个）</span>
        </template>
        <template v-else-if="batchDeleteType === 'tag'">
          即将删除 <strong class="delete-count">{{ tokensToDeleteByTag.length }}</strong> 个账号
          <span class="delete-breakdown">（标签为 "{{ selectedTagForDelete }}"）</span>
        </template>
      </p>
    </div>
    <template #footer>
      <div class="modal-actions">
        <button @click="showBatchDeleteConfirm = false" class="btn secondary" :disabled="isDeleting">
          取消
        </button>
        <button @click="executeBatchDelete" class="btn danger" :disabled="isDeleting">
          <svg v-if="!isDeleting" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
          </svg>
          <span>{{ isDeleting ? '删除中...' : '确认删除' }}</span>
        </button>
      </div>
    </template>
  </ModalContainer>
</template>

<script setup>
import { ref, computed, watch, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TokenCard from "./TokenCard.vue";
import ModalContainer from "./ModalContainer.vue";
import TokenForm from "./TokenForm.vue";
import AccountManagerModal from "./AccountManagerModal.vue";
import TagEditorModal from "./TagEditorModal.vue";

// Props - 移除 tokens，改为内部管理
const props = defineProps({
  statusThresholds: {
    type: Object,
    default: () => ({
      time: { warning: 10, safe: 20 },
      balance: { warning: 1000, safe: 2000 },
    }),
  },
  createTagOnImportExport: {
    type: Boolean,
    default: false,
  },
});

// 内部状态管理 - TokenList 直接管理 tokens
const tokens = ref([]);
const hasUnsavedChanges = ref(false);

// 排序状态管理
const sortType = ref('time'); // 'time' = 按时间排序, 'balance' = 按余额排序
const sortOrder = ref('asc'); // 'asc' = 最旧优先/余额从少到多, 'desc' = 最新优先/余额从多到少
const showSortMenu = ref(false); // 排序下拉菜单显示状态
let sortMenuTimer = null; // 排序菜单延迟隐藏计时器

// 搜索状态管理
const searchQuery = ref('');

// 批量删除状态管理
const showBatchDeleteOptions = ref(false); // 批量删除选项对话框
const showBatchDeleteConfirm = ref(false); // 批量删除确认对话框
const isDeleting = ref(false);
const batchDeleteType = ref('status'); // 'status' 或 'tag'
const selectedTagForDelete = ref(''); // 选中的要删除的标签

// 单个删除状态管理
const showDeleteConfirm = ref(false);
const tokenToDelete = ref(null);
const tokenToDeleteName = ref('此账号');

// TokenForm 状态管理
const showTokenFormModal = ref(false);
const editingToken = ref(null);

// AccountManagerModal 状态管理
const showAccountManagerModal = ref(false);
const accountManagerTab = ref('import'); // 'import' 或 'export'

// TagEditorModal 状态管理（用于导入导出时创建标签）
const showTagEditorForImportExport = ref(false);
const pendingImportTokens = ref(null); // 待导入的账号数据
const pendingExportData = ref(null); // 待导出的数据
const pendingExportType = ref(null); // 'json' 或 'session'
const pendingExportCount = ref(0); // 待导出的账号数量
const pendingExportTokenIds = ref([]); // 待导出的账号ID列表

// 从 props 获取 createTagOnImportExport 配置
const createTagOnImportExport = computed(() => props.createTagOnImportExport || false);

// 分页状态管理
const currentPage = ref(1);
const pageSize = ref(10); // 默认每页10条
const pageSizeOptions = [10, 20, 50, 100, 200];
const showPageSizeMenu = ref(false); // 每页数量下拉菜单显示状态
let pageSizeMenuTimer = null; // 每页数量菜单延迟隐藏计时器

// 窗口尺寸状态管理
const windowWidth = ref(window.innerWidth);

// 根据窗口宽度计算尺寸类
const windowSizeClass = computed(() => {
  if (windowWidth.value <= 760) {
    return 'size-small'; // 小窗口
  } else if (windowWidth.value <= 1050) {
    return 'size-medium'; // 中等窗口
  } else {
    return 'size-large'; // 大窗口/全屏
  }
});

// localStorage 配置键名
const STORAGE_KEY_PAGE_SIZE = 'zaugment-page-size';

// 从 localStorage 加载分页配置
const loadPageSize = () => {
  try {
    const stored = localStorage.getItem(STORAGE_KEY_PAGE_SIZE);
    if (stored) {
      const size = parseInt(stored, 10);
      if (pageSizeOptions.includes(size)) {
        return size;
      }
    }
  } catch (error) {
    console.warn('Failed to load page size from localStorage:', error);
  }
  return 20; // 默认值
};

// 保存分页配置到 localStorage
const savePageSize = (size) => {
  try {
    localStorage.setItem(STORAGE_KEY_PAGE_SIZE, size.toString());
  } catch (error) {
    console.warn('Failed to save page size to localStorage:', error);
  }
};

// Emits
const emit = defineEmits([
  "save-changes",
  "refresh",
  "copy-success",
  "token-updated",
  "add-new-token",
  "auto-check-completed",
  "check-all-status",
]);

// 排序后的tokens计算属性
const sortedTokens = computed(() => {
  if (tokens.value.length === 0) return [];

  return [...tokens.value].sort((a, b) => {
    if (sortType.value === 'time') {
      // 按时间排序
      const dateA = new Date(a.created_at);
      const dateB = new Date(b.created_at);

      if (sortOrder.value === 'desc') {
        return dateB - dateA; // 最新优先
      } else {
        return dateA - dateB; // 最旧优先
      }
    } else if (sortType.value === 'balance') {
      // 按余额排序
      const balanceA = a.portal_info?.credits_balance;
      const balanceB = b.portal_info?.credits_balance;

      // 处理没有余额信息的情况
      const hasBalanceA = balanceA !== undefined && balanceA !== null;
      const hasBalanceB = balanceB !== undefined && balanceB !== null;

      // 没有余额信息的排在最后
      if (!hasBalanceA && !hasBalanceB) return 0;
      if (!hasBalanceA) return 1;
      if (!hasBalanceB) return -1;

      // 都有余额信息,按数值排序
      if (sortOrder.value === 'desc') {
        return balanceB - balanceA; // 余额从多到少
      } else {
        return balanceA - balanceB; // 余额从少到多
      }
    }
    return 0;
  });
});

// 过滤后的tokens计算属性（搜索 + 排序）
const filteredTokens = computed(() => {
  if (!searchQuery.value.trim()) {
    return sortedTokens.value;
  }

  const query = searchQuery.value.toLowerCase().trim();
  return sortedTokens.value.filter(token => {
    return (
      token.access_token?.toLowerCase().includes(query) ||
      token.email_note?.toLowerCase().includes(query) ||
      token.auth_session?.toLowerCase().includes(query) ||
      token.tag_name?.toLowerCase().includes(query)
    );
  });
});

// 总页数
const totalPages = computed(() => {
  return Math.ceil(filteredTokens.value.length / pageSize.value);
});

// 当前页的 tokens
const paginatedTokens = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return filteredTokens.value.slice(start, end);
});

// 分页信息
const paginationInfo = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value + 1;
  const end = Math.min(start + pageSize.value - 1, filteredTokens.value.length);
  return {
    start,
    end,
    current: currentPage.value,
    total: filteredTokens.value.length
  };
});

// 计算可删除的 token 数量（已封禁或过期）
const deletableTokensCount = computed(() => {
  return tokens.value.filter(token =>
    token.ban_status === 'SUSPENDED' || token.ban_status === 'EXPIRED'
  ).length;
});

// 计算已封禁的 token 数量
const bannedTokensCount = computed(() => {
  return tokens.value.filter(token => token.ban_status === 'SUSPENDED').length;
});

// 计算已过期的 token 数量
const expiredTokensCount = computed(() => {
  return tokens.value.filter(token => token.ban_status === 'EXPIRED').length;
});

// 获取所有可用的标签（去重并统计数量）
const availableTags = computed(() => {
  const tagMap = new Map();

  tokens.value.forEach(token => {
    if (token.tag_name && token.tag_name.trim()) {
      const tagName = token.tag_name.trim();
      const tagColor = token.tag_color || '#3b82f6'; // 默认蓝色

      if (tagMap.has(tagName)) {
        const existing = tagMap.get(tagName);
        tagMap.set(tagName, { ...existing, count: existing.count + 1 });
      } else {
        tagMap.set(tagName, { name: tagName, color: tagColor, count: 1 });
      }
    }
  });

  // 转换为数组并按名称排序
  return Array.from(tagMap.values())
    .sort((a, b) => a.name.localeCompare(b.name));
});

// 计算按标签删除时要删除的 tokens
const tokensToDeleteByTag = computed(() => {
  if (!selectedTagForDelete.value) return [];
  return tokens.value.filter(token =>
    token.tag_name && token.tag_name.trim() === selectedTagForDelete.value
  );
});

// 判断是否可以进入确认步骤
const canProceedToConfirm = computed(() => {
  if (batchDeleteType.value === 'status') {
    return deletableTokensCount.value > 0;
  } else if (batchDeleteType.value === 'tag') {
    return selectedTagForDelete.value && tokensToDeleteByTag.value.length > 0;
  }
  return false;
});

// 将十六进制颜色转换为 rgba
const hexToRgba = (hex, alpha) => {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
};

// 获取标签样式（与 TokenCard 中的样式一致）
const getTagStyle = (color) => {
  if (!color) return {};
  return {
    color: color,
    backgroundColor: hexToRgba(color, 0.15), // 15% 透明度
  };
};

// 计算所有状态的统计数据
const getStatusCounts = () => {
  let active = 0;
  let banned = 0;
  let invalid = 0;
  let expired = 0;
  let unknown = 0;

  for (const t of tokens.value) {
    if (t.ban_status === "ACTIVE") active++;
    else if (t.ban_status === "SUSPENDED") banned++;
    else if (t.ban_status === "INVALID") invalid++;
    else if (t.ban_status === "EXPIRED") expired++;
    else unknown++;
  }

  return { active, banned, invalid, expired, unknown };
};

// Token card refs for portal refresh
const tokenCardRefs = ref({});
const isRefreshing = ref(false);

const setTokenCardRef = (el, tokenId) => {
  if (el) {
    tokenCardRefs.value[tokenId] = el;
  } else {
    delete tokenCardRefs.value[tokenId];
  }
};

const handleTokenUpdated = (updatedToken) => {
  emit("token-updated", updatedToken);
};

const handleCopySuccess = (message, type) => {
  emit("copy-success", message, type);
};

// 深度比对两个对象是否相等
const isEqual = (obj1, obj2) => {
  if (obj1 === obj2) return true;
  if (obj1 == null || obj2 == null) return false;
  if (typeof obj1 !== 'object' || typeof obj2 !== 'object') return obj1 === obj2;

  const keys1 = Object.keys(obj1);
  const keys2 = Object.keys(obj2);

  if (keys1.length !== keys2.length) return false;

  for (const key of keys1) {
    if (!keys2.includes(key)) return false;
    if (!isEqual(obj1[key], obj2[key])) return false;
  }

  return true;
};

// 检查所有Token的账号状态
const checkAllAccountStatus = async () => {
  const tokensToCheck = tokens.value.filter(token => !token.skip_check);

  if (tokensToCheck.length === 0) {
    return { hasChanges: false, stats: { total: 0, success: 0, failed: 0, retried: 0 } };
  }

  isRefreshing.value = true;

  try {
    const tokenInfos = tokensToCheck.map(token => ({
      id: token.id,
      access_token: token.access_token,
      tenant_url: token.tenant_url,
      portal_url: token.portal_url || null,
      auth_session: token.auth_session || null,
      email_note: token.email_note || null
    }));

    const results = await invoke('batch_check_tokens_status', {
      tokens: tokenInfos
    });

    let hasChanges = false;

    results.forEach(result => {
      const token = tokens.value.find(t => t.id === result.token_id);
      if (token) {
        const statusResult = result.status_result;
        let tokenHasChanges = false;

        if (token.access_token !== result.access_token) {
          token.access_token = result.access_token;
          tokenHasChanges = true;
        }

        if (token.tenant_url !== result.tenant_url) {
          token.tenant_url = result.tenant_url;
          tokenHasChanges = true;
        }

        if (token.ban_status !== statusResult.status) {
          token.ban_status = statusResult.status;
          tokenHasChanges = true;
        }

        if ((statusResult.status === 'SUSPENDED' || statusResult.status === 'EXPIRED') && !token.skip_check) {
          token.skip_check = true;
          tokenHasChanges = true;
        }

        if (result.suspensions) {
          if (!isEqual(token.suspensions, result.suspensions)) {
            token.suspensions = result.suspensions;
            tokenHasChanges = true;
          }
        }

        if (result.portal_info) {
          const newPortalInfo = {
            credits_balance: result.portal_info.credits_balance,
            expiry_date: result.portal_info.expiry_date
          };

          if (!isEqual(token.portal_info, newPortalInfo)) {
            token.portal_info = newPortalInfo;
            tokenHasChanges = true;
          }
        }

        if (result.email_note && token.email_note !== result.email_note) {
          token.email_note = result.email_note;
          tokenHasChanges = true;
        }

        if (tokenHasChanges) {
          token.updated_at = new Date().toISOString();
          hasChanges = true;
        }
      }
    });

    if (hasChanges) {
      await saveTokens(false);
    }

    let successCount = 0;
    let failedCount = 0;
    let suspendedCount = 0;
    let expiredCount = 0;

    results.forEach(result => {
      if (result.status_result && result.status_result.status === 'ERROR') {
        failedCount++;
      } else if (result.status_result && result.status_result.status === 'SUSPENDED') {
        suspendedCount++;
        successCount++;
      } else if (result.status_result && result.status_result.status === 'EXPIRED') {
        expiredCount++;
        successCount++;
      } else {
        successCount++;
      }
    });

    return {
      hasChanges,
      stats: {
        total: tokensToCheck.length,
        success: successCount,
        failed: failedCount,
        suspended: suspendedCount,
        expired: expiredCount,
        retried: 0
      }
    };
  } catch (error) {
    console.error('Batch check all error:', error);
    throw error;
  } finally {
    isRefreshing.value = false;
  }
};

// 关闭所有 TokenCard 的弹窗
const closeAllTokenCardModals = () => {
  Object.values(tokenCardRefs.value).forEach((cardRef) => {
    if (cardRef && cardRef.closeAllModals) {
      cardRef.closeAllModals();
    }
  });
};

// 关闭 TokenList 自己的所有弹窗
const closeAllModals = () => {
  showTokenFormModal.value = false;
  showDeleteConfirm.value = false;
  showAccountManagerModal.value = false;
  // 同时关闭所有 TokenCard 的弹窗
  closeAllTokenCardModals();
};

// 加载 tokens 从后端
const loadTokens = async (showSuccessMessage = false) => {
  try {
    const jsonString = await invoke('load_tokens_json');
    const parsedTokens = JSON.parse(jsonString);

    // 确保是数组
    if (Array.isArray(parsedTokens)) {
      // 使用展开运算符创建新数组，确保触发响应式更新
      tokens.value = [...parsedTokens];
    } else {
      tokens.value = [];
    }

    if (showSuccessMessage) {
      emit("copy-success", "账号加载成功", "success");
    }
  } catch (error) {
    emit("copy-success", `账号加载失败: ${error}`, "error");
    tokens.value = [];
  }
};

// 保存 tokens 到后端
const saveTokens = async (showSuccessMessage = false) => {
  try {
    const jsonString = JSON.stringify(tokens.value, null, 2);
    await invoke('save_tokens_json', { jsonString });
    if (showSuccessMessage) {
      emit("copy-success", "账号保存成功", "success");
    }
  } catch (error) {
    emit("copy-success", `账号保存失败: ${error}`, "error");
    throw error;
  }
};

// 添加 token 的统一函数（处理所有添加场景：OAuth、Session、手动）
const addToken = (tokenData) => {
  // 1. 如果有邮箱，优先检查邮箱重复
  if (tokenData.emailNote && tokenData.emailNote.trim()) {
    const emailToCheck = tokenData.emailNote.trim().toLowerCase();
    const duplicate = tokens.value.find(
      (token) =>
        token.email_note &&
        token.email_note.trim().toLowerCase() === emailToCheck
    );

    if (duplicate) {
      emit(
        "copy-success",
        `邮箱 ${tokenData.emailNote} 已存在，跳过添加`,
        "warning"
      );
      return { success: false, duplicateId: duplicate.id }; // 返回重复的 token ID
    }
  }

  // 2. 检查 access_token 重复
  if (tokenData.accessToken && tokenData.accessToken.trim()) {
    const tokenToCheck = tokenData.accessToken.trim();
    const duplicate = tokens.value.find(
      (token) => token.access_token === tokenToCheck
    );

    if (duplicate) {
      const displayName = duplicate.email_note || duplicate.tenant_url || 'Unknown';
      emit(
        "copy-success",
        `Access Token 已存在（${displayName}），跳过添加`,
        "warning"
      );
      return { success: false, duplicateId: duplicate.id };
    }
  }

  // 3. 检查 tenant_url 重复（可选，根据需求决定是否启用）
  // if (tokenData.tenantUrl && tokenData.tenantUrl.trim()) {
  //   const urlToCheck = tokenData.tenantUrl.trim();
  //   const duplicate = tokens.value.find(
  //     (token) => token.tenant_url === urlToCheck
  //   );
  //
  //   if (duplicate) {
  //     const displayName = duplicate.email_note || duplicate.tenant_url || 'Unknown';
  //     emit(
  //       "copy-success",
  //       `Tenant URL 已存在（${displayName}），跳过添加`,
  //       "warning"
  //     );
  //     return { success: false, duplicateId: duplicate.id };
  //   }
  // }

  const now = new Date().toISOString();
  const tagName = tokenData.tagName ? tokenData.tagName.trim() : "";
  const tagColor = tokenData.tagColor || "#3b82f6"; // 默认蓝色

  // 构建 portal_info (如果有 creditsBalance 或 expiryDate)
  let portalInfo = null;
  if (
    (tokenData.creditsBalance !== undefined && tokenData.creditsBalance !== null) ||
    (tokenData.expiryDate !== undefined && tokenData.expiryDate !== null)
  ) {
    portalInfo = {
      credits_balance: tokenData.creditsBalance !== undefined ? tokenData.creditsBalance : null, // 驼峰转蛇形
      expiry_date: tokenData.expiryDate || null, // 驼峰转蛇形
    };
  }

  const newToken = {
    id: crypto.randomUUID(),
    tenant_url: tokenData.tenantUrl, // 驼峰转蛇形
    access_token: tokenData.accessToken, // 驼峰转蛇形
    created_at: now, // 驼峰转蛇形
    updated_at: now, // 驼峰转蛇形
    portal_url: tokenData.portalUrl || null, // 驼峰转蛇形
    ban_status: tokenData.banStatus || null, // 驼峰转蛇形，Session 导入时为 'ACTIVE'
    portal_info: portalInfo, // 驼峰转蛇形
    email_note: tokenData.emailNote || null, // 驼峰转蛇形
    tag_name: tagName || null, // 驼峰转蛇形
    tag_color: tagName ? tagColor : null, // 驼峰转蛇形
    auth_session: tokenData.authSession || null, // 驼峰转蛇形
    suspensions: tokenData.suspensions || null,
    skip_check: false, // 驼峰转蛇形，默认不跳过检测
  };

  // 按创建时间排序插入（最新的在最前面）
  const newCreatedTime = new Date(newToken.created_at);
  let insertIndex = tokens.value.length;

  for (let i = 0; i < tokens.value.length; i++) {
    const token = tokens.value[i];
    const tokenCreatedTime = new Date(token.created_at || 0);

    // 如果新token的创建时间更晚，应该插在当前位置之前
    if (newCreatedTime > tokenCreatedTime) {
      insertIndex = i;
      break;
    }

    // 如果创建时间相同，按ID排序
    if (newCreatedTime.getTime() === tokenCreatedTime.getTime()) {
      if (newToken.id.localeCompare(token.id) > 0) {
        insertIndex = i;
        break;
      }
    }
  }

  // 直接插入到 tokens 数组
  tokens.value.splice(insertIndex, 0, newToken);

  // 保存到文件
  saveTokens(false);

  emit("copy-success", "账号添加成功", "success");
  return { success: true, token: newToken };
};

// 高亮并滚动到指定 token
const highlightAndScrollTo = (tokenId) => {
  // 高亮逻辑
  const tokenCard = tokenCardRefs.value[tokenId];
  if (tokenCard && tokenCard.$el) {
    tokenCard.$el.scrollIntoView({ behavior: "smooth", block: "center" });
    // 添加高亮效果
    tokenCard.$el.classList.add("highlight-duplicate");
    setTimeout(() => {
      tokenCard.$el.classList.remove("highlight-duplicate");
    }, 3000);
  }
};

// 刷新所有 Portal 信息的辅助函数
const refreshAllPortalInfo = async () => {
  // 刷新所有有portal_url的账号的额度信息
  const refreshOperations = [];

  tokens.value.forEach((token) => {
    if (token.portal_url && tokenCardRefs.value[token.id]) {
      const cardRef = tokenCardRefs.value[token.id];
      if (cardRef && cardRef.refreshPortalInfo) {
        // 立即调用刷新方法，不等待结果
        const refreshPromise = cardRef
          .refreshPortalInfo()
          .then(() => {
            return { success: true, tokenId: token.id };
          })
          .catch((error) => {
            console.warn(`刷新账号 ${token.id} 额度信息失败:`, error);
            return { success: false, tokenId: token.id, error };
          });

        refreshOperations.push(refreshPromise);
      }
    }
  });

  // 等待所有刷新操作完成并统计结果
  if (refreshOperations.length > 0) {
    const results = await Promise.allSettled(refreshOperations);

    // 统计成功的刷新数量
    let refreshedCount = 0;
    let failedCount = 0;

    results.forEach((result) => {
      if (result.status === "fulfilled" && result.value.success) {
        refreshedCount++;
      } else {
        failedCount++;
      }
    });

    return {
      success: true,
      message: `已刷新 ${refreshedCount} 个账号的额度信息${
        failedCount > 0 ? `，${failedCount} 个失败` : ""
      }`,
      refreshedCount,
      failedCount,
    };
  } else {
    return {
      success: true,
      message: "没有需要刷新额度的账号",
      refreshedCount: 0,
      failedCount: 0,
    };
  }
};

// 关闭批量删除对话框的辅助函数
const closeBatchDeleteDialog = () => {
  showBatchDeleteOptions.value = false;
  showBatchDeleteConfirm.value = false;
};

// 切换排序方式
const toggleSort = (type = 'time') => {
  // 如果切换到不同的排序类型,重置为降序
  if (sortType.value !== type) {
    sortType.value = type;
    sortOrder.value = 'desc';
  } else {
    // 同一类型,切换升序/降序
    sortOrder.value = sortOrder.value === 'desc' ? 'asc' : 'desc';
  }
};

// 切换排序菜单显示
const toggleSortMenu = () => {
  showSortMenu.value = !showSortMenu.value;
};

// 悬浮显示排序菜单
const showSortMenuOnHover = () => {
  clearSortMenuTimer();
  showSortMenu.value = true;
};

// 清除排序菜单计时器
const clearSortMenuTimer = () => {
  if (sortMenuTimer) {
    clearTimeout(sortMenuTimer);
    sortMenuTimer = null;
  }
};

// 延迟隐藏排序菜单
const hideSortMenuOnLeave = () => {
  clearSortMenuTimer();
  sortMenuTimer = setTimeout(() => {
    showSortMenu.value = false;
  }, 200);
};

// 设置排序类型和顺序
const setSortType = (type, order) => {
  sortType.value = type;
  sortOrder.value = order;
  showSortMenu.value = false;
};

// 处理导入导出按钮点击
const handleImportExport = () => {
  showAccountManagerModal.value = true;
  accountManagerTab.value = 'import'; // 默认显示导入tab
};

// AccountManagerModal 事件处理
const handleAccountManagerImport = async (importedTokens) => {
  // 如果开启了导入导出时创建标签功能，先打开标签编辑器
  if (createTagOnImportExport.value) {
    pendingImportTokens.value = importedTokens;
    showAccountManagerModal.value = false; // 关闭导入导出弹框
    showTagEditorForImportExport.value = true; // 打开标签编辑器
  } else {
    // 直接导入
    await finalizeImport(importedTokens, '', '');
  }
};

const handleAccountManagerImportError = (errorMessage) => {
  emit('copy-success', errorMessage, 'error');
};

const handleAccountManagerExportClipboard = async ({ data, count, tokenIds }) => {
  // 如果开启了导入导出时创建标签功能，先打开标签编辑器
  if (createTagOnImportExport.value) {
    pendingExportData.value = data;
    pendingExportType.value = 'json';
    pendingExportCount.value = count;
    pendingExportTokenIds.value = tokenIds || [];
    showAccountManagerModal.value = false; // 关闭导入导出弹框
    showTagEditorForImportExport.value = true; // 打开标签编辑器
  } else {
    // 直接导出
    try {
      await navigator.clipboard.writeText(data);
      emit('copy-success', `已将 ${count} 个账号数据复制到剪贴板`, 'success');
      showAccountManagerModal.value = false;
    } catch (error) {
      emit('copy-success', '复制到剪贴板失败', 'error');
    }
  }
};

const handleAccountManagerExportSession = async ({ data, count, tokenIds }) => {
  // 如果开启了导入导出时创建标签功能，先打开标签编辑器
  if (createTagOnImportExport.value) {
    pendingExportData.value = data;
    pendingExportType.value = 'session';
    pendingExportCount.value = count;
    pendingExportTokenIds.value = tokenIds || [];
    showAccountManagerModal.value = false; // 关闭导入导出弹框
    showTagEditorForImportExport.value = true; // 打开标签编辑器
  } else {
    // 直接导出
    try {
      await navigator.clipboard.writeText(data);
      emit('copy-success', `已将 ${count} 个账号数据复制到剪贴板`, 'success');
      showAccountManagerModal.value = false;
    } catch (error) {
      emit('copy-success', '复制到剪贴板失败', 'error');
    }
  }
};

// 最终导入处理（带标签）
const finalizeImport = async (importedTokens, tagText, tagColor) => {
  let successCount = 0;
  let duplicateCount = 0;
  let taggedDuplicateCount = 0;

  for (const tokenData of importedTokens) {
    // 转换蛇形命名为驼峰命名（AccountManagerModal 返回的是蛇形命名）
    const tokenWithTag = {
      tenantUrl: tokenData.tenant_url,
      accessToken: tokenData.access_token,
      portalUrl: tokenData.portal_url || null,
      emailNote: tokenData.email_note || null,
      authSession: tokenData.auth_session || null,
      banStatus: tokenData.ban_status || null,
      suspensions: tokenData.suspensions || null,
      creditsBalance: tokenData.portal_info?.credits_balance || null,
      expiryDate: tokenData.portal_info?.expiry_date || null,
      tagName: tagText,
      tagColor: tagColor,
    };

    const result = addToken(tokenWithTag);

    if (result.success) {
      successCount++;
    } else if (result.duplicateId) {
      duplicateCount++;

      // 如果有标签颜色，给重复的账号也打上标签（文字可以为空）
      if (tagColor) {
        const duplicateToken = tokens.value.find(t => t.id === result.duplicateId);
        if (duplicateToken) {
          duplicateToken.tag_name = tagText || '';
          duplicateToken.tag_color = tagColor;
          duplicateToken.updated_at = new Date().toISOString();
          taggedDuplicateCount++;
        }
      }
    }
  }

  // 保存到文件
  await saveTokens();

  // 显示结果
  if (duplicateCount > 0) {
    let message = `导入完成！成功: ${successCount}, 跳过(重复): ${duplicateCount}`;
    if (taggedDuplicateCount > 0) {
      message += `，已为 ${taggedDuplicateCount} 个重复账号添加标签`;
    }
    emit('copy-success', message, 'success');
  } else {
    emit('copy-success', `成功导入 ${successCount} 个账号`, 'success');
  }
};

// 最终导出处理（带标签）
const finalizeExport = async (data, count, tokenIds, tagText, tagColor) => {
  // 如果有标签，给这些账号打上标签
  if (tagText && tokenIds && tokenIds.length > 0) {
    for (const tokenId of tokenIds) {
      const token = tokens.value.find(t => t.id === tokenId);
      if (token) {
        token.tag_name = tagText;
        token.tag_color = tagColor;
      }
    }
    await saveTokens();
  }

  // 复制到剪贴板
  try {
    await navigator.clipboard.writeText(data);
    emit('copy-success', `已将 ${count} 个账号数据复制到剪贴板`, 'success');
  } catch (error) {
    emit('copy-success', '复制到剪贴板失败', 'error');
  }
};

// TagEditorModal 事件处理
const handleTagEditorForImportExportClose = async () => {
  showTagEditorForImportExport.value = false;

  // 如果是导出操作，点击"取消"应该继续导出，只是不修改标签
  if (pendingExportData.value) {
    await finalizeExport(
      pendingExportData.value,
      pendingExportCount.value,
      pendingExportTokenIds.value,
      '', // 空标签文字
      ''  // 空标签颜色
    );
    pendingExportData.value = null;
    pendingExportType.value = null;
    pendingExportCount.value = 0;
    pendingExportTokenIds.value = [];
  }
  // 如果是导入操作，点击"取消"应该取消整个导入
  else if (pendingImportTokens.value) {
    pendingImportTokens.value = null;
  }
};

const handleTagEditorForImportExportConfirm = async (tagData) => {
  showTagEditorForImportExport.value = false;

  const tagText = tagData.tagText || '';
  const tagColor = tagData.tagColor || '';

  // 处理导入
  if (pendingImportTokens.value) {
    await finalizeImport(pendingImportTokens.value, tagText, tagColor);
    pendingImportTokens.value = null;
  }
  // 处理导出
  else if (pendingExportData.value) {
    await finalizeExport(
      pendingExportData.value,
      pendingExportCount.value,
      pendingExportTokenIds.value,
      tagText,
      tagColor
    );
    pendingExportData.value = null;
    pendingExportType.value = null;
    pendingExportCount.value = 0;
    pendingExportTokenIds.value = [];
  }
};

// 处理批量删除按钮点击
const handleBatchDelete = () => {
  if (tokens.value.length === 0) {
    return;
  }

  // 重置状态
  batchDeleteType.value = 'status';
  selectedTagForDelete.value = '';

  // 如果没有可删除的状态账号，默认选择标签删除
  if (deletableTokensCount.value === 0 && availableTags.value.length > 0) {
    batchDeleteType.value = 'tag';
  }

  showBatchDeleteOptions.value = true;
};

// 选择删除类型
const selectDeleteType = (type) => {
  batchDeleteType.value = type;
  if (type === 'tag') {
    selectedTagForDelete.value = ''; // 重置标签选择
  }
};

// 确认批量删除选项，进入确认对话框
const confirmBatchDeleteOptions = () => {
  if (!canProceedToConfirm.value) return;

  showBatchDeleteOptions.value = false;
  showBatchDeleteConfirm.value = true;
};

// TokenForm 事件处理
const handleEditToken = (token) => {
  editingToken.value = token;
  showTokenFormModal.value = true;
};

const closeTokenForm = () => {
  showTokenFormModal.value = false;
  editingToken.value = null;
};

// 用于标记最后一次添加是否成功
const lastAddTokenSuccess = ref(true);

const handleTokenFormSuccess = () => {
  // 编辑模式总是关闭
  if (editingToken.value) {
    closeTokenForm();
    emit('copy-success', '账号已更新', 'success');
  } else {
    // 添加模式：只有成功时才关闭和提示
    if (lastAddTokenSuccess.value) {
      closeTokenForm();
      emit('copy-success', '账号已保存', 'success');
    }
    // 如果失败（重复），不关闭对话框，已经显示了警告并高亮了重复的 token
  }
};

const handleUpdateToken = async (updatedTokenData) => {
  const index = tokens.value.findIndex(token => token.id === updatedTokenData.id);
  if (index !== -1) {
    const tagName = updatedTokenData.tagName ? updatedTokenData.tagName.trim() : '';
    const tagColor = updatedTokenData.tagColor || '';

    tokens.value[index] = {
      ...tokens.value[index],
      tenant_url: updatedTokenData.tenantUrl,
      access_token: updatedTokenData.accessToken,
      portal_url: updatedTokenData.portalUrl,
      email_note: updatedTokenData.emailNote,
      tag_name: tagName,
      tag_color: tagColor,
      updated_at: new Date().toISOString(),
    };

    // 保存到文件
    await saveTokens();
  }
};

const handleAddTokenFromForm = async (tokenData) => {
  const result = addToken(tokenData);
  lastAddTokenSuccess.value = result.success;

  if (!result.success && result.duplicateId) {
    // 重复邮箱，高亮并滚动到重复的 token
    highlightAndScrollTo(result.duplicateId);
    return;
  }

  // 添加成功，保存到文件
  if (result.success && result.token) {
    await saveTokens();
  }
};

// 处理单个删除
const handleDeleteToken = (tokenId) => {
  // 查找 token
  const token = tokens.value.find(t => t.id === tokenId);
  if (token) {
    tokenToDelete.value = tokenId;
    tokenToDeleteName.value = token.email_note || token.tenant_url || '此账号';
    showDeleteConfirm.value = true;
  }
};

// 确认删除
const confirmDelete = async () => {
  if (!tokenToDelete.value) return;

  try {
    // 从内存中删除
    tokens.value = tokens.value.filter(token => token.id !== tokenToDelete.value);

    // 调整当前页码（如果当前页超过了总页数）
    nextTick(() => {
      if (currentPage.value > totalPages.value && totalPages.value > 0) {
        currentPage.value = totalPages.value;
      } else if (totalPages.value === 0) {
        currentPage.value = 1;
      }
    });

    // 保存到文件
    await saveTokens();

    emit('copy-success', '账号已成功删除!', 'success');
  } catch (error) {
    emit('copy-success', `删除Token失败: ${error}`, 'error');
  }

  showDeleteConfirm.value = false;
  tokenToDelete.value = null;
  tokenToDeleteName.value = '此账号';
};

// 取消删除
const cancelDelete = () => {
  showDeleteConfirm.value = false;
  tokenToDelete.value = null;
  tokenToDeleteName.value = '此账号';
};

// 执行批量删除
const executeBatchDelete = async () => {
  isDeleting.value = true;

  try {
    let tokenIdsToDelete = [];
    let deleteMessage = '';

    // 根据删除类型获取要删除的 token IDs
    if (batchDeleteType.value === 'status') {
      tokenIdsToDelete = tokens.value
        .filter(token => token.ban_status === 'SUSPENDED' || token.ban_status === 'EXPIRED')
        .map(token => token.id);
      deleteMessage = `成功删除 ${tokenIdsToDelete.length} 个已封禁/过期账号`;
    } else if (batchDeleteType.value === 'tag') {
      tokenIdsToDelete = tokensToDeleteByTag.value.map(token => token.id);
      deleteMessage = `成功删除 ${tokenIdsToDelete.length} 个标签为"${selectedTagForDelete.value}"的账号`;
    }

    if (tokenIdsToDelete.length === 0) {
      emit('copy-success', '没有符合条件的账号需要删除', 'warning');
      showBatchDeleteConfirm.value = false;
      isDeleting.value = false;
      return;
    }

    // 从内存中删除
    tokens.value = tokens.value.filter(token => !tokenIdsToDelete.includes(token.id));

    // 关闭对话框
    showBatchDeleteConfirm.value = false;

    // 调整当前页码（如果当前页超过了总页数）
    nextTick(() => {
      if (currentPage.value > totalPages.value && totalPages.value > 0) {
        currentPage.value = totalPages.value;
      } else if (totalPages.value === 0) {
        currentPage.value = 1;
      }
    });

    // 保存到文件
    await saveTokens();

    // 显示提示消息
    emit('copy-success', deleteMessage, 'success');
  } catch (error) {
    console.error('Batch delete failed:', error);
    emit('copy-success', '批量删除失败', 'error');
  } finally {
    isDeleting.value = false;
  }
};

// 切换每页数量菜单显示
const togglePageSizeMenu = () => {
  showPageSizeMenu.value = !showPageSizeMenu.value;
};

// 悬浮显示每页数量菜单
const showPageSizeMenuOnHover = () => {
  clearPageSizeMenuTimer();
  showPageSizeMenu.value = true;
};

// 清除每页数量菜单计时器
const clearPageSizeMenuTimer = () => {
  if (pageSizeMenuTimer) {
    clearTimeout(pageSizeMenuTimer);
    pageSizeMenuTimer = null;
  }
};

// 延迟隐藏每页数量菜单
const hidePageSizeMenuOnLeave = () => {
  clearPageSizeMenuTimer();
  pageSizeMenuTimer = setTimeout(() => {
    showPageSizeMenu.value = false;
  }, 200);
};

// 选择每页数量
const selectPageSize = (size) => {
  changePageSize(size);
  showPageSizeMenu.value = false;
};

// 分页方法
const goToPage = (page) => {
  if (page < 1 || page > totalPages.value) return;
  currentPage.value = page;
  // 滚动到顶部
  nextTick(() => {
    const container = document.querySelector('.token-list');
    if (container) {
      container.scrollTop = 0;
    }
  });
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    goToPage(currentPage.value + 1);
  }
};

const prevPage = () => {
  if (currentPage.value > 1) {
    goToPage(currentPage.value - 1);
  }
};

const changePageSize = (newSize) => {
  // 保存到 localStorage
  savePageSize(newSize);
  // 重新计算当前页(保持当前第一条 token 可见)
  const firstTokenIndex = (currentPage.value - 1) * pageSize.value;
  pageSize.value = newSize;
  currentPage.value = Math.floor(firstTokenIndex / newSize) + 1;
};

// 搜索时重置到第一页
watch(searchQuery, () => {
  currentPage.value = 1;
});

// 排序变化时重置到第一页
watch([sortType, sortOrder], () => {
  currentPage.value = 1;
});

// 生命周期
onMounted(async () => {
  // 加载分页配置
  pageSize.value = loadPageSize();

  // 加载 tokens
  await loadTokens(false);

  // 监听窗口尺寸变化
  const handleResize = () => {
    windowWidth.value = window.innerWidth;
  };
  window.addEventListener('resize', handleResize);

  // 组件卸载时移除监听
  return () => {
    window.removeEventListener('resize', handleResize);
  };
});

// 打开 TokenForm（用于新增）
const openTokenForm = () => {
  editingToken.value = null; // 清空编辑的 token，表示新增
  showTokenFormModal.value = true;
};

// 暴露给父组件的方法和数据
defineExpose({
  tokens,
  loadTokens,
  saveTokens,
  checkAllAccountStatus,
  addToken,
  highlightAndScrollTo,
  closeAllTokenCardModals,
  closeAllModals, // 暴露关闭所有弹窗的方法
  closeBatchDeleteDialog,
  refreshAllPortalInfo,
  openTokenForm, // 暴露打开 TokenForm 的方法
  getStatusCounts, // 暴露状态统计方法
  get isRefreshing() {
    return isRefreshing.value;
  }
});
</script>

<style scoped>
.token-list-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.list-content {
  flex: 1;
  overflow: visible;
  min-height: 0;
  /* 性能优化 */
  transform: translateZ(0);
  backface-visibility: hidden;
  perspective: 1000px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  flex: 1; /* 占据剩余空间 */
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-state-content {
  max-width: 400px;
  margin: 0 auto;
}

.empty-icon {
  color: rgba(59, 130, 246, 0.6);
  margin-bottom: 24px;
  filter: drop-shadow(0 2px 4px rgba(59, 130, 246, 0.1));
}

.empty-state h4 {
  color: #1e293b;
  margin: 0 0 12px 0;
  font-size: 1.25rem;
  font-weight: 600;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.empty-state p {
  color: #64748b;
  margin: 0 0 24px 0;
  font-size: 1rem;
  line-height: 1.6;
}

.empty-actions {
  margin-top: 24px;
}

.btn-empty {
  padding: 12px 24px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
  box-shadow: 0 2px 12px rgba(79, 70, 229, 0.25);
}

.btn-empty::before {
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

.btn-empty:hover::before {
  left: 100%;
}

.btn-empty.primary {
  background: #4f46e5;
  color: white;
}

.btn-empty.primary:hover {
  background: #4338ca;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(79, 70, 229, 0.35);
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.token-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: 10px;
  padding: 10px 0 0;
  /* 性能优化 */
  will-change: scroll-position;
  transform: translateZ(0);
  -webkit-overflow-scrolling: touch;
}

/* Token列表整体布局优化 */
.token-list {
  animation: fadeIn 0.3s ease-out;
  margin: 0 10px;
  flex: 1; /* 占据剩余空间 */
  overflow-y: auto; /* 允许垂直滚动 */
  overflow-x: hidden;
  min-height: 0; /* 允许 flex 子元素缩小 */
  /* 性能优化 */
  contain: layout style;
  transform: translateZ(0);
}

@keyframes fadeIn {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 响应式布局优化 */
@media (max-width: 1200px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 10px;
  }
}

@media (max-width: 1200px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 10px;
  }
}

@media (max-width: 768px) {
  .token-grid {
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 4px;
  }

}

/* 搜索和排序工具栏 */
.list-toolbar {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: linear-gradient(
    135deg,
    rgba(249, 250, 251, 0.98) 0%,
    rgba(243, 244, 246, 0.95) 100%
  );
  border-bottom: 1px solid rgba(226, 232, 240, 0.4);
  flex-shrink: 0; /* 不允许缩小 */
  /* backdrop-filter: blur(10px); */ /* 移除以提升滚动性能 */
}

/* 搜索框样式 */
.search-box {
  position: relative;
  flex: 1;
  max-width: 230px;
}

.search-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: #94a3b8;
  pointer-events: none;
  transition: color 0.2s;
}

.search-input {
  width: 100%;
  padding: 8px 32px 8px 32px;
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  background: rgba(255, 255, 255, 0.95);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.search-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: rgba(248, 250, 252, 0.95);
}

.search-input:focus {
  outline: none;
  border-color: #4f46e5;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1), 0 2px 8px rgba(99, 102, 241, 0.15);
  background: white;
}

.search-input:focus + .search-icon {
  color: #4f46e5;
}

.clear-search-btn {
  position: absolute;
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
  padding: 4px;
  background: none;
  border: none;
  color: #94a3b8;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.clear-search-btn:hover {
  background: rgba(148, 163, 184, 0.15);
  color: #64748b;
  transform: translateY(-50%) scale(1.1);
}

/* 排序下拉菜单 */
.sort-dropdown {
  position: relative;
}

.sort-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
  overflow: hidden;
}

.sort-btn::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(59, 130, 246, 0.1),
    transparent
  );
  transition: left 0.5s;
}

.sort-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: rgba(248, 250, 252, 0.95);
}

.sort-btn:disabled::before {
  display: none;
}

.sort-btn:hover:not(:disabled) {
  border-color: #4f46e5;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(139, 92, 246, 0.04));
  color: #4f46e5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
}

.sort-btn:hover:not(:disabled)::before {
  left: 100%;
}

.sort-btn .arrow-icon {
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.sort-btn .arrow-icon.rotated {
  transform: rotate(180deg);
}

.sort-menu {
  position: absolute;
  top: 100%;
  right: 0;
  background: rgba(255, 255, 255, 0.98);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  z-index: 99999;
  min-width: 130px;
  margin-top: 8px;
  pointer-events: auto;
  animation: slideDown 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.sort-option {
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

.sort-option:first-child {
  border-radius: 8px 8px 0 0;
}

.sort-option:last-child {
  border-radius: 0 0 8px 8px;
}

.sort-option:not(:last-child) {
  border-bottom: 1px solid rgba(226, 232, 240, 0.4);
}

.sort-option:hover {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  color: #4f46e5;
}

.sort-option:active {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.12),
    rgba(139, 92, 246, 0.08)
  );
}

.sort-option.active {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  color: #4f46e5;
}

.sort-option svg {
  flex-shrink: 0;
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.sort-option:hover svg {
  opacity: 1;
}

.sort-option .check-icon {

  color: #4f46e5;
  opacity: 1;
}


/* 分页信息 */
.pagination-info {
  font-size: 14px;
  font-weight: 500;
  color: #64748b;
  margin-left: auto;
  padding: 8px 10px;
  background: rgba(255, 255, 255, 0.6);
  border-radius: 8px;
}

.pagination-info.disabled {
  opacity: 0.5;
}

/* 分页控件 */
.pagination-controls {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 12px 16px;
  border-top: 1px solid rgba(226, 232, 240, 0.4);
  background: linear-gradient(
    135deg,
    rgba(249, 250, 251, 0.98) 0%,
    rgba(243, 244, 246, 0.95) 100%
  );
  /* backdrop-filter: blur(10px); */ /* 移除以提升滚动性能 */
  margin-top: auto; /* 推到底部 */
  flex-shrink: 0; /* 不允许缩小 */
  position: relative;
  gap: 12px;
  margin-top: 10px;
}

.pagination-controls.disabled {
  opacity: 0.5;
}

.pagination-center {
  display: flex;
  align-items: center;
  gap: 8px;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.page-size-dropdown {
  position: relative;
}

.pagination-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 7px 14px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
  overflow: hidden;
}

.pagination-btn::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(59, 130, 246, 0.15),
    transparent
  );
  transition: left 0.5s;
}

.pagination-btn:hover:not(:disabled)::before {
  left: 100%;
}

.pagination-btn:hover:not(:disabled) {
  border-color: #4f46e5;
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08) 0%,
    rgba(139, 92, 246, 0.04) 100%
  );
  color: #4f46e5;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.2);
}

.pagination-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  background: rgba(241, 245, 249, 0.5);
}

/* 页码信息 */
.page-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 16px;
  font-size: 14px;
  font-weight: 500;
  color: #475569;
  user-select: none;
  min-width: 80px;
  justify-content: center;
}

.current-page {
  font-size: 16px;
  font-weight: 600;
  color: #4f46e5;
  min-width: 24px;
  text-align: right;
}

.page-separator {
  color: #94a3b8;
  font-weight: 400;
}

.total-pages {
  color: #64748b;
  min-width: 24px;
  text-align: left;
}

/* 每页数量下拉菜单 */
.page-size-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 7px 12px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
  overflow: hidden;
}

.page-size-btn::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(59, 130, 246, 0.1),
    transparent
  );
  transition: left 0.5s;
}

.page-size-btn:hover::before {
  left: 100%;
}

.page-size-btn:hover {
  border-color: #4f46e5;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(139, 92, 246, 0.04));
  color: #4f46e5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
}

.page-size-btn .arrow-icon {
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-size-btn .arrow-icon.rotated {
  transform: rotate(180deg);
}

.page-size-menu {
  position: absolute;
  bottom: 100%;
  right: 0;
  background: rgba(255, 255, 255, 0.98);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  z-index: 99999;
  min-width: 110px;
  margin-bottom: 8px;
  pointer-events: auto;
  animation: slideUpMenu 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideUpMenu {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.page-size-option {
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

.page-size-option:first-child {
  border-radius: 8px 8px 0 0;
}

.page-size-option:last-child {
  border-radius: 0 0 8px 8px;
}

.page-size-option:not(:last-child) {
  border-bottom: 1px solid rgba(226, 232, 240, 0.4);
}

.page-size-option:hover {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  color: #4f46e5;
}

.page-size-option:active {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.12),
    rgba(139, 92, 246, 0.08)
  );
}

.page-size-option.active {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.08),
    rgba(139, 92, 246, 0.04)
  );
  color: #4f46e5;
}

.page-size-option svg {
  flex-shrink: 0;
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.page-size-option:hover svg {
  opacity: 1;
}

.page-size-option .check-icon {
  margin-left: auto;
  color: #4f46e5;
  opacity: 1;
}

/* 统一账号管理卡片样式 */
.unified-account-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.4);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  /* backdrop-filter: blur(20px); */ /* 移除以提升滚动性能 */
}

.unified-account-card:hover {
  /* transform: translateY(-2px); */
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  border-color: rgba(226, 232, 240, 0.6);
}

/* 卡片头部样式 */
.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 28px 32px;
  background: linear-gradient(
    135deg,
    rgba(249, 250, 251, 0.8) 0%,
    rgba(243, 244, 246, 0.5) 100%
  );
}

.header-main {
  display: flex;
  align-items: flex-start;
  gap: 20px;
  flex: 1;
}

.section-icon {
  width: 56px;
  height: 56px;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1) 0%,
    rgba(139, 92, 246, 0.1) 100%
  );
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid rgba(59, 130, 246, 0.2);
  flex-shrink: 0;
}

.section-icon svg {
  color: #3b82f6;
}

.header-text h3 {
  margin: 0 0 8px 0;
  font-size: 22px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.025em;
}

.header-description {
  color: #64748b;
  font-size: 15px;
  line-height: 1.6;
  margin: 0;
  font-weight: 400;
}

.header-stats {
  display: flex;
  gap: 16px;
  align-items: center;
  margin-top: 8px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  background: rgba(59, 130, 246, 0.1);
  border-radius: 24px;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.stat-label {
  font-size: 13px;
  color: #64748b;
  font-weight: 500;
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  color: #3b82f6;
}

/* 分割线 */
.card-divider {
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(226, 232, 240, 0.5) 50%,
    transparent 100%
  );
  margin: 0 32px;
}

/* 内容区域 */
.card-content {
  display: flex;
  flex-direction: column;
}

/* 根据窗口尺寸动态调整高度 */
.card-content.size-small {
  min-height: 76vh; /* 小窗口 (760×700) */
}

.card-content.size-medium {
  min-height: 81vh; /* 中等窗口 (1050×700) */
}

.card-content.size-large {
  min-height: 87.5vh; /* 大窗口/全屏 */
}

/* Button styles */
.btn {
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn.small {
  padding: 6px 10px;
  font-size: 11px;
}

.btn.success {
  background: #22c55e;
  color: white;
}

.btn.success:hover:not(:disabled) {
  background: #16a34a;
}

.btn.secondary {
  background: #64748b;
  color: white;
}

.btn.secondary:hover {
  background: #475569;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 导入导出按钮 */
.import-export-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
  overflow: hidden;
  white-space: nowrap;
}

.import-export-btn::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(99, 102, 241, 0.2), transparent);
  transition: left 0.5s;
  z-index: 0;
}

.import-export-btn:hover:not(:disabled)::before {
  left: 100%;
}

.import-export-btn:hover:not(:disabled) {
  border-color: #4f46e5;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(79, 70, 229, 0.04));
  color: #4f46e5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
}

.import-export-btn svg {
  flex-shrink: 0;
  position: relative;
}

.import-export-btn span {
  position: relative;
  line-height: 1;
}

/* 批量删除按钮 */
.batch-delete-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
  /* 不设置 overflow，让角标可以溢出 */
}

/* 按钮内部包装层 - 用于裁剪光栅效果 */
.batch-delete-btn .btn-inner {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  position: relative;
  overflow: hidden; /* 裁剪光栅效果在按钮内部 */
  border-radius: 8px;
}

.batch-delete-btn .btn-inner::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(239, 68, 68, 0.2), transparent);
  transition: left 0.5s;
  z-index: 0;
}

.batch-delete-btn:hover:not(:disabled) .btn-inner::before {
  left: 100%;
}

.batch-delete-btn .btn-inner svg {
  position: relative;
}

.batch-delete-btn:hover:not(:disabled) {
  border-color: #ef4444;
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.08), rgba(220, 38, 38, 0.04));
  color: #ef4444;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(239, 68, 68, 0.15);
}

.batch-delete-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.batch-delete-btn .badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 16px;
  height: 16px;
  padding: 0 5px;
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  border-radius: 9px;
  font-size: 11px;
  font-weight: 600;
  line-height: 1;
  position: absolute;
  top: -8px;
  right: -8px;
  z-index: 2; /* 确保角标在最上层 */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15), 0 0 0 2px rgba(255, 255, 255, 0.9);
  white-space: nowrap;
}

/* 删除确认对话框内容 */
.delete-confirm-content {
  text-align: center;
  padding: 10px 0;
}

.confirm-icon {
  color: #dc2626;
  margin: 0 auto 16px;
  filter: drop-shadow(0 2px 4px rgba(220, 38, 38, 0.1));
  animation: iconPulse 2s ease-in-out infinite;
}

.delete-confirm-content h4 {
  margin: 0 0 12px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.delete-confirm-content p {
  margin: 0;
  font-size: 14px;
  color: #64748b;
  line-height: 1.6;
}

.delete-confirm-content strong {
  color: #dc2626;
  font-weight: 600;
}

/* 批量删除对话框内容 */
.batch-delete-content {
  text-align: center;
  padding: 10px 0;
}

.batch-delete-icon {
  color: #dc2626;
  margin: 0 auto 16px;
  filter: drop-shadow(0 2px 4px rgba(220, 38, 38, 0.1));
  animation: iconPulse 2s ease-in-out infinite;
}

@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

.batch-delete-content h4 {
  margin: 0 0 12px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.delete-message {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: #64748b;
  font-weight: 400;
}

.delete-stats {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 16px;
  text-align: left;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  background: linear-gradient(135deg, rgba(248, 250, 252, 0.8), rgba(241, 245, 249, 0.4));
  border-radius: 8px;
  border: 1px solid rgba(226, 232, 240, 0.6);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.stat-item:hover {
  transform: translateX(4px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.stat-item.suspended {
  border-left: 3px solid #ef4444;
}

.stat-item.expired {
  border-left: 3px solid #f59e0b;
}

.stat-item.total {
  border-left: 3px solid #4f46e5;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(79, 70, 229, 0.04));
}

.stat-icon {
  font-size: 22px;
  line-height: 1;
}

.stat-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.stat-label {
  font-size: 12px;
  color: #64748b;
  font-weight: 500;
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  color: #1e293b;
}

.delete-warning {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 14px;
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.08), rgba(220, 38, 38, 0.04));
  border-left: 3px solid #ef4444;
  border-radius: 8px;
  color: #dc2626;
  font-size: 13px;
  font-weight: 500;
}

.delete-warning svg {
  flex-shrink: 0;
}

.modal-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}

.modal-actions .btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  border: none;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.modal-actions .btn.secondary {
  background: rgba(248, 250, 252, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.6);
  color: #64748b;
}

.modal-actions .btn.secondary:hover:not(:disabled) {
  background: rgba(226, 232, 240, 0.4);
  border-color: #cbd5e1;
  color: #475569;
  transform: translateY(-1px);
}

.modal-actions .btn.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(239, 68, 68, 0.3);
  position: relative;
  overflow: hidden;
}

.modal-actions .btn.danger::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.5s;
}

.modal-actions .btn.danger:hover:not(:disabled)::before {
  left: 100%;
}

.modal-actions .btn.danger:hover:not(:disabled) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
  transform: translateY(-2px);
}

.modal-actions .btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 动画 */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Modal 过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .batch-delete-dialog,
.modal-leave-active .batch-delete-dialog {
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-enter-from .batch-delete-dialog,
.modal-leave-to .batch-delete-dialog {
  transform: translateY(20px);
}
</style>

<style>
/* 批量删除对话框样式 - 非 scoped，用于 ModalContainer */
.batch-delete-content {
  text-align: center;
  padding: 10px 0;
}

.batch-delete-icon {
  color: #dc2626;
  margin: 0 auto 16px;
  filter: drop-shadow(0 2px 4px rgba(220, 38, 38, 0.1));
  animation: batchDeleteIconPulse 2s ease-in-out infinite;
}

@keyframes batchDeleteIconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

.batch-delete-content h4 {
  margin: 0 0 12px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.batch-delete-content .delete-message {
  margin: 0 0 16px 0;
  font-size: 15px;
  color: #475569;
  font-weight: 400;
  line-height: 1.6;
}

.batch-delete-content .delete-count {
  color: #dc2626;
  font-weight: 700;
  font-size: 18px;
}

.batch-delete-content .delete-breakdown {
  display: block;
  margin-top: 8px;
  font-size: 13px;
  color: #94a3b8;
  font-weight: 400;
}

.batch-delete-content .delete-warning {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 14px;
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.08), rgba(220, 38, 38, 0.04));
  border-left: 3px solid #ef4444;
  border-radius: 8px;
  color: #dc2626;
  font-size: 13px;
  font-weight: 500;
}

.batch-delete-content .delete-warning svg {
  flex-shrink: 0;
}

/* 批量删除选项对话框样式 */
.batch-delete-options-content {
  padding: 10px 0;
  max-height: 60vh;
}

.options-description {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: #64748b;
  font-weight: 500;
}

.delete-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.delete-option {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 14px;
  border: 2px solid rgba(226, 232, 240, 0.6);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  background: rgba(255, 255, 255, 0.5);
}

.delete-option:hover:not(.disabled) {
  border-color: rgba(59, 130, 246, 0.5);
  background: rgba(59, 130, 246, 0.04);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
}

.delete-option.active {
  border-color: #b5d1ffbe;
  background: rgba(59, 130, 246, 0.1);
  box-shadow: 0 0 0 1px rgba(102, 158, 249, 0.2);
}

.delete-option.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: rgba(241, 245, 249, 0.5);
}

.option-radio {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
  border: 2px solid #cbd5e1;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  margin-top: 2px;
}

.delete-option.active .option-radio {
  border-color: #3b82f6;
  background: #3b82f6;
}

.radio-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: white;
  animation: radioDotAppear 0.2s ease-out;
}

@keyframes radioDotAppear {
  0% {
    transform: scale(0);
  }
  100% {
    transform: scale(1);
  }
}

.option-content {
  flex: 1;
}

.option-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin-bottom: 4px;
}

.option-description {
  font-size: 13px;
  color: #64748b;
  line-height: 1.5;
}

.option-count {
  font-weight: 600;
  color: #3b82f6;
}

.option-count.disabled {
  color: #94a3b8;
}

/* 标签选择器容器 */
.tag-selector-container {
  padding: 16px;
  background: linear-gradient(135deg, rgba(249, 250, 251, 0.9), rgba(243, 244, 246, 0.7));
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 10px;
  animation: slideDown 0.3s ease-out;
}

@keyframes slideDown {
  0% {
    opacity: 0;
    transform: translateY(-10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.tag-selector-label {
  display: block;
  margin-bottom: 12px;
  font-size: 13px;
  font-weight: 600;
  color: #475569;
}

/* 标签网格 */
.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 10px;
}

.tag-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 10px 12px;
  background: white;
  border: 2px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.tag-item:hover {
  border-color: rgba(59, 130, 246, 0.5);
  background: rgba(59, 130, 246, 0.04);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
}

.tag-item.selected {
  border-color: #b5d1ffbe;
  background: rgba(59, 130, 246, 0.1);
  box-shadow: 0 0 0 1px rgba(102, 158, 249, 0.2);
  transform: translateY(-2px);
}

.tag-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
  letter-spacing: 0.02em;
  line-height: 1.2;
  /* 颜色和背景由内联样式控制 */
}

.tag-count-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 6px;
  background: linear-gradient(135deg, #3b82f6, #60a5fa);
  color: white;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 700;
  box-shadow: 0 2px 6px rgba(59, 130, 246, 0.3);
  flex-shrink: 0;
}
</style>
