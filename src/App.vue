<template>
  <div class="app">
    <!-- 自定义标题栏 -->
    <CustomTitleBar />

    <!-- 应用主体内容 -->
    <div class="app-content">
      <!-- Sidebar -->
      <aside class="sidebar">
        <div class="sidebar-header">
          <!-- ZAugment Logo - 现代精致设计 -->
          <div class="app-logo">
            <h1 class="logo-text">ZAugment</h1>
          </div>
        </div>

        <nav class="sidebar-nav">
          <button
            @click="handleNavClick('token-generator')"
            :class="['nav-item', { active: currentView === 'token-generator' }]"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
              />
            </svg>
            <span>新增流程</span>
          </button>
          <button
            @click="handleNavClick('token-list')"
            :class="['nav-item', { active: currentView === 'token-list' }]"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"
              />
            </svg>
            <span>账号管理</span>
          </button>

          <!-- 邮箱管理功能 -->
          <button
            @click="handleNavClick('email-manager')"
            :class="['nav-item', { active: currentView === 'email-manager' }]"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
              />
            </svg>
            <span>邮箱管理</span>
          </button>

         

          <button
            @click="handleNavClick('settings')"
            :class="['nav-item', { active: currentView === 'settings' }]"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"
              />
            </svg>
            <span>功能设置</span>
          </button>
        </nav>
      </aside>

      <!-- 小窗口时的顶部标签页导航 -->
      <nav class="top-tabs-nav">
        <button
          @click="handleNavClick('token-generator')"
          :class="['top-tab-item', { active: currentView === 'token-generator' }]"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
            />
          </svg>
          <span>新增流程</span>
        </button>
        <button
          @click="handleNavClick('token-list')"
          :class="['top-tab-item', { active: currentView === 'token-list' }]"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"
            />
          </svg>
          <span>账号管理</span>
        </button>
        <button
          @click="handleNavClick('email-manager')"
          :class="['top-tab-item', { active: currentView === 'email-manager' }]"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
            />
          </svg>
          <span>邮箱管理</span>
        </button>
        
        <button
          @click="handleNavClick('settings')"
          :class="['top-tab-item', { active: currentView === 'settings' }]"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"
            />
          </svg>
          <span>功能设置</span>
        </button>
      </nav>

      <!-- Main Content -->
      <main class="main-content">
        <!-- Token Generator View -->
        <div
          v-show="currentView === 'token-generator'"
          class="token-generator-view"
        >
          <!-- 标题栏 -->
          <div class="token-generator-header">
            <div class="header-left">
              <h2>新增流程</h2>
            </div>
            <div class="header-right">
              <button
                @click="openPurchaseLink"
                class="btn-header-unified purchase"
                title="购买账号"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H5.21l-.94-2H1zm16 16c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z"
                  />
                </svg>
                购买账号
              </button>
              <!-- 逻辑未完善 暂时隐藏 -->
              <!-- <button
                @click="showEditorResetModal = true"
                class="btn-header-unified danger"
                title="重置编辑器中的Augment配置"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
                  />
                </svg>
                重置Augment
              </button> -->
              <button
                @click="showPluginModal = true"
                class="btn-header-unified special"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" />
                </svg>
                下载防封插件
              </button>
            </div>
          </div>

          <!-- 简化的卡片结构 -->
          <div class="token-generator-card">
            <!-- 统一的步骤卡片 -->
            <div class="unified-steps-card">
              <!-- Tab Navigation -->
              <div class="steps-tab-navigation">
                <button
                  :class="[
                    'steps-tab-btn',
                    { active: currentTokenTab === 'steps' },
                  ]"
                  @click="currentTokenTab = 'steps'"
                >
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path
                      d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
                    />
                  </svg>
                  OAuth 流程
                </button>
                <button
                  :class="[
                    'steps-tab-btn',
                    { active: currentTokenTab === 'session' },
                  ]"
                  @click="currentTokenTab = 'session'"
                >
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path
                      d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"
                    />
                  </svg>
                  Session 导入
                </button>
              </div>

              <!-- OAuth Steps Content -->
              <div v-if="currentTokenTab === 'steps'" class="steps-content">
                <!-- Step 1: Generate Authorization URL -->
                <div class="step-item">
                  <div class="step-header">
                    <div class="step-number">1</div>
                    <h3>生成授权URL</h3>
                    <button
                      v-if="authUrl"
                      @click="resetAuthUrl"
                      class="reset-btn"
                      title="重置"
                    >
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
                        />
                      </svg>
                    </button>
                  </div>

                  <!-- 生成前的状态 -->
                  <div v-if="!authUrl" class="generate-section">
                    <div class="input-with-integrated-button">
                      <input
                        type="text"
                        v-model="authUrl"
                        placeholder="点击生成按钮生成授权URL进行注册，或直接输入URL"
                        class="generate-input"
                        autocomplete="off"
                        autocapitalize="off"
                        autocorrect="off"
                        spellcheck="false"
                      />
                      <button
                        @click="generateAuthUrl"
                        :class="[
                          'integrated-btn',
                          'primary',
                          { loading: isGenerating },
                        ]"
                        :disabled="isGenerating"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                        </svg>
                        生成
                      </button>
                    </div>
                    <div class="info-hint">
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"
                        />
                      </svg>
                      点击按钮生成新的授权URL，进入指纹浏览器进行注册
                    </div>
                  </div>

                  <!-- 生成后的状态 -->
                  <div v-if="authUrl" class="generated-section">
                    <div class="input-with-integrated-button">
                      <input
                        type="text"
                        v-model="authUrl"
                        ref="authUrlInput"
                        autocomplete="off"
                        autocapitalize="off"
                        autocorrect="off"
                        spellcheck="false"
                      />
                      <button
                        @click="copyAuthUrl"
                        class="integrated-btn primary"
                        title="复制URL"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
                          />
                        </svg>
                        复制
                      </button>
                    </div>
                    <div class="info-hint">
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"
                        />
                      </svg>
                      点击按钮生成新的授权URL，进入指纹浏览器进行注册
                    </div>
                  </div>
                </div>
                <!-- Step 2: Enter Authorization Code -->
                <div class="step-item">
                  <div class="step-header">
                    <div class="step-number">2</div>
                    <h3>通过授权URL获取需要的JSON数据</h3>
                  </div>
                  <!-- <div class="field-container">
                      <label>Portal URL:</label>
                      <input
                        type="text"
                        v-model="portalUrl"
                        placeholder="粘贴  Payment history 页面地址栏 URL"
                        class="field-input"
                      />
                    </div> -->
                  <input
                    v-model="authCode"
                    placeholder="按照流程登录完成后，点击 Copy 按钮，复制需要的JSON数据，把获取到的JSON粘贴到这里"
                    class="field-input"
                  />

                  <!-- 隐藏获取访问令牌按钮 -->
                  <div class="button-container" style="display: none">
                    <button
                      @click="getAccessToken"
                      :class="['btn', 'primary', { loading: isGettingToken }]"
                      :disabled="!canGetToken || isGettingToken"
                    >
                      获取访问令牌
                    </button>
                  </div>
                </div>

                <!-- Step 3: Access Token -->
                <div class="step-item">
                  <div class="step-header">
                    <div class="step-number">3</div>
                    <h3>获取 Portal URL</h3>
                    <button
                      @click="copyAccountUrl"
                      class="integrated-btn primary "
                      title="复制后台URL"
                    >
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
                        />
                      </svg>
                      复制后台URL
                    </button>
                  </div>
                  <div class="token-section">
                    <!-- 隐藏访问令牌部分 -->
                    <div class="result-container" style="display: none">
                      <label>访问令牌:</label>
                      <div class="token-container">
                        <input
                          type="text"
                          :value="tokenResult?.access_token || ''"
                          :placeholder="
                            !tokenResult ? '请先完成步骤2获取访问令牌' : ''
                          "
                          readonly
                          :disabled="!tokenResult"
                          ref="accessTokenInput"
                          autocomplete="off"
                          autocapitalize="off"
                          autocorrect="off"
                          spellcheck="false"
                        />
                        <button
                          @click="copyAccessToken"
                          class="btn secondary"
                          :disabled="!tokenResult"
                        >
                          复制
                        </button>
                      </div>
                    </div>
                    <!-- 隐藏租户URL部分 -->
                    <div class="result-container" style="display: none">
                      <label>租户URL:</label>
                      <div class="token-container">
                        <input
                          type="text"
                          :value="tokenResult?.tenant_url || ''"
                          :placeholder="
                            !tokenResult ? '请先完成步骤2获取访问令牌' : ''
                          "
                          readonly
                          :disabled="!tokenResult"
                          ref="tenantUrlInput"
                        />
                        <button
                          @click="copyTenantUrl"
                          class="btn secondary"
                          :disabled="!tokenResult"
                        >
                          复制
                        </button>
                      </div>
                    </div>

                    <!-- Additional Fields -->
                    <div class="additional-fields">
                      <div class="field-container">
                        <label>Portal URL (可选):</label>
                        <input
                          type="text"
                          v-model="portalUrl"
                          placeholder="可选，主要用于编辑器额度插件"
                          class="field-input"
                        />
                        <div class="info-hint">
                          <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                          >
                            <path
                              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"
                            />
                          </svg>
                          额度和邮箱会自动获取，无需填写 Portal URL ，主要用于编辑器额度插件
                        </div>
                      </div>
                      <div class="field-container">
                        <label>邮箱账号 (可选):</label>
                        <input
                          type="text"
                          v-model="emailNote"
                          placeholder="可选，会自动获取邮箱账号"
                          class="field-input"
                        />
                      </div>
                    </div>

                    <div class="button-container">
                      <button
                        @click="saveTokenWithAutoFetch"
                        :class="['btn', 'primary', { loading: isGettingToken }]"
                        :disabled="!canGetToken || isGettingToken"
                      >
                        {{ isGettingToken ? "正在获取令牌..." : "保存账号" }}
                      </button>
                    </div>
                    <!-- button-container -->
                  </div>
                  <!-- save-section -->
                </div>
                <!-- step-item -->
              </div>
              <!-- steps-content -->

              <!-- Session Import Content -->
              <div
                v-if="currentTokenTab === 'session'"
                class="session-import-content"
              >
                <div class="session-section">
                  <div class="session-header">
                    <div class="step-number">1</div>
                    <h3>通过 Session 导入账号</h3>
                    <button
                      @click="showSessionHelpModal = true"
                      class="help-btn"
                      title="查看使用说明"
                    >
                      <svg
                        width="18"
                        height="18"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                      >
                        <path
                          d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"
                        />
                      </svg>
                    </button>
                  </div>

                  <textarea
                    v-model="sessionInput"
                    placeholder="在此粘贴 auth session..."
                    rows="6"
                    :disabled="isImportingSession"
                    class="field-input"
                  ></textarea>

                  <div class="button-container">
                    <button
                      @click="importFromSession"
                      class="btn primary"
                      :disabled="!sessionInput.trim() || isImportingSession"
                    >
                      {{
                        isImportingSession
                          ? sessionImportProgress
                          : "导入 Session"
                      }}
                    </button>
                  </div>
                </div>
              </div>
              <!-- session-import-content -->

              <!-- unified-steps-card -->
            </div>
            <!-- token-generator-card -->
          </div>
          <!-- token-generator-view -->
        </div>

        <!-- Token List View -->
        <div v-show="currentView === 'token-list'" class="token-list-view">
          <!-- 标题栏 -->
          <div class="view-header-bar">
            <div class="header-left">
              <div class="header-left-title">
                <h2>账号管理</h2>

                <!-- 悬浮状态统计 - 可点击筛选 - 一行显示 - 全部/正常/封禁/其他 -->
                <div class="floating-status-stats">
                  <!-- 全部 -->
                  <div
                    :class="['status-item', 'all', { 'selected': selectedStatusFilter === null }]"
                    @click="selectStatusFilter(null)"
                    title="点击显示全部账号"
                  >
                    <span class="status-label">全部</span>
                    <span class="status-count">{{ totalTokensCount || 0 }}</span>
                  </div>
                  <!-- 正常 -->
                  <div
                    :class="['status-item', 'saved', { 'selected': selectedStatusFilter === 'ACTIVE' }]"
                    @click="selectStatusFilter('ACTIVE')"
                    title="点击筛选正常账号"
                  >
                    <span class="status-label">正常</span>
                    <span class="status-count">{{ savedTokensCount || 0 }}</span>
                  </div>
                  <!-- 封禁 -->
                  <div
                    :class="['status-item', 'banned', { 'selected': selectedStatusFilter === 'SUSPENDED' }]"
                    @click="selectStatusFilter('SUSPENDED')"
                    title="点击筛选已封禁账号"
                  >
                    <span class="status-label">封禁</span>
                    <span class="status-count">{{ bannedTokensCount || 0 }}</span>
                  </div>
                  <!-- 其他（过期+Token失效+未知） -->
                  <div
                    :class="['status-item', 'other', { 'selected': selectedStatusFilter === 'OTHER' }]"
                    @click="selectStatusFilter('OTHER')"
                    title="点击筛选其他状态账号（过期、Token失效、未知）"
                  >
                    <span class="status-label">其他</span>
                    <span class="status-count">{{ otherTokensCount || 0 }}</span>
                  </div>
                </div>
              </div>
            </div>
            <div class="header-right">
              <button
                @click="smartRefresh"
                :class="[
                  'btn-header-unified',
                  'secondary',
                ]"
                :disabled="tokenListRef?.isRefreshing"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
                  />
                </svg>
                {{ tokenListRef?.isRefreshing ? "刷新中" : "刷新" }}
              </button>
              <button
                @click="handleAddNewToken"
                class="btn-header-unified primary"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                </svg>
                新增
              </button>
            </div>
          </div>

          <!-- 内容卡片 -->
          <div class="view-content-card">
            <div class="account-management-container">
              <TokenList
                ref="tokenListRef"
                :statusThresholds="statusThresholds"
                :createTagOnImportExport="createTagOnImportExport"
                @add-new-token="handleAddNewToken"
                @copy-success="showStatus"
                @token-updated="handleTokenUpdated"
              />
            </div>
          </div>
        </div>

        <!-- Email Manager View -->
        <div
          v-show="currentView === 'email-manager'"
          class="email-manager-view"
        >
          <!-- 标题栏 -->
          <div class="view-header-bar">
            <div class="header-left">
              <h2>邮箱管理</h2>
              <div
                class="account-count-badge"
                v-if="outlookAccountsCount > 0"
                :class="`status-${primaryAccountStatus.type}`"
              >
                <span>{{ primaryAccountStatus.count }}</span>
                <span>{{ primaryAccountStatus.label }}</span>
              </div>
            </div>
            <div class="header-right">
              <div class="outlook-manager-actions">
                <button
                  @click="handleRefreshAllStatuses"
                  :disabled="isRefreshingStatuses"
                  class="btn-header-unified secondary"
                >
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path
                      d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z"
                    />
                  </svg>
                  {{ isRefreshingStatuses ? "刷新中..." : "刷新" }}
                </button>
                <button
                  @click="handleAddAccount"
                  class="btn-header-unified primary"
                >
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                  </svg>
                  新增
                </button>
              </div>
            </div>
          </div>

          <!-- 内容卡片 -->
          <div class="view-content-card">
            <div class="account-management-container">
              <OutlookManager
                ref="outlookManagerRef"
                @show-status="showStatus"
                @accounts-count-changed="updateOutlookAccountsCount"
                @accounts-status-changed="updateOutlookAccountsStatus"
                :import-format="outlookImportFormat"
              />
            </div>
          </div>
        </div>

        <!-- Settings View -->
        <div v-show="currentView === 'settings'" class="settings-view">
          <!-- 标题栏 -->
          <div class="view-header-bar">
            <div class="header-left">
              <h2>应用设置</h2>
            </div>
            <div class="header-right">
              <button
                @click="showCommunityModal = true"
                class="btn-header-unified community"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"
                  />
                </svg>
                加入交流群
              </button>
              <button
                @click="showAppreciationModal = true"
                class="btn-header-unified appreciation"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                  <path
                    d="M12 21.35L10.55 20.03C5.4 15.36 2 12.27 2 8.5 2 5.41 4.42 3 7.5 3C9.24 3 10.91 3.81 12 5.08C13.09 3.81 14.76 3 16.5 3C19.58 3 22 5.41 22 8.5C22 12.27 18.6 15.36 13.45 20.03L12 21.35Z"
                  />
                </svg>
                赞赏
              </button>
            </div>
          </div>

          <!-- 内容卡片 -->
          <div class="view-content-card">
            <div class="unified-steps-card">
              <!-- 基础设置 -->
              <div class="settings-section">
                <div class="section-header">
                  <div class="section-icon">
                    <svg
                      width="20"
                      height="20"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M12,15.5A3.5,3.5 0 0,1 8.5,12A3.5,3.5 0 0,1 12,8.5A3.5,3.5 0 0,1 15.5,12A3.5,3.5 0 0,1 12,15.5M19.43,12.97C19.47,12.65 19.5,12.33 19.5,12C19.5,11.67 19.47,11.34 19.43,11L21.54,9.37C21.73,9.22 21.78,8.95 21.66,8.73L19.66,5.27C19.54,5.05 19.27,4.96 19.05,5.05L16.56,6.05C16.04,5.66 15.5,5.32 14.87,5.07L14.5,2.42C14.46,2.18 14.25,2 14,2H10C9.75,2 9.54,2.18 9.5,2.42L9.13,5.07C8.5,5.32 7.96,5.66 7.44,6.05L4.95,5.05C4.73,4.96 4.46,5.05 4.34,5.27L2.34,8.73C2.21,8.95 2.27,9.22 2.46,9.37L4.57,11C4.53,11.34 4.5,11.67 4.5,12C4.5,12.33 4.53,12.65 4.57,12.97L2.46,14.63C2.27,14.78 2.21,15.05 2.34,15.27L4.34,18.73C4.46,18.95 4.73,19.03 4.95,18.95L7.44,17.94C7.96,18.34 8.5,18.68 9.13,18.93L9.5,21.58C9.54,21.82 9.75,22 10,22H14C14.25,22 14.46,21.82 14.5,21.58L14.87,18.93C15.5,18.67 16.04,18.34 16.56,17.94L19.05,18.95C19.27,19.03 19.54,18.95 19.66,18.73L21.66,15.27C21.78,15.05 21.73,14.78 21.54,14.63L19.43,12.97Z"
                      />
                    </svg>
                  </div>
                  <h3>基础设置</h3>
                </div>
                <p class="section-description">
                  配置应用的基本功能选项，包括数据存储和导入导出等。
                </p>

                <!-- 数据存储目录 -->
                <div class="setting-item">
                  <div class="setting-label">
                    <label>数据存储目录</label>
                    <span class="setting-help">
                      请选择不包含中文字符的文件夹路径，如：D:\TokenData
                      ,重启后生效
                    </span>
                  </div>
                  <div class="setting-control">
                    <div class="path-input-group">
                      <input
                        type="text"
                        :value="dataDirectory"
                        readonly
                        class="path-input"
                        :class="{ loading: dataDirectory === '正在获取...' }"
                        placeholder="选择数据存储目录"
                      />
                      <button
                        @click="selectDataDirectory"
                        class="btn-select-path"
                        :disabled="isSelectingPath"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M10,4H4C2.89,4 2,4.89 2,6V18A2,2 0 0,0 4,20H20A2,2 0 0,0 22,18V8C22,6.89 21.1,6 20,6H12L10,4Z"
                          />
                        </svg>
                        {{ isSelectingPath ? "选择中..." : "选择目录" }}
                      </button>
                      <button
                        @click="openDataDirectory"
                        class="btn-open-folder"
                        title="在文件管理器中打开数据目录"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"
                          />
                        </svg>
                      </button>
                    </div>
                  </div>
                  <!-- 批量导入导出时创建标签 -->
                  <div class="setting-item-create-tag">
                    <div class="setting-label">
                      <label>批量导入导出时创建标签</label>
                      <span class="setting-help">
                        开启后，批量导入或导出账号时可以为所有账号创建或应用标签
                      </span>
                    </div>
                    <div class="setting-control">
                      <div class="toggle-wrapper">
                        <label class="toggle-label-switch">
                          <input
                            type="checkbox"
                            v-model="createTagOnImportExport"
                            @change="toggleCreateTagOnImportExport"
                            class="toggle-checkbox"
                          />
                          <span class="toggle-slider"></span>
                        </label>
                        <span class="toggle-text">
                          {{ createTagOnImportExport ? "已开启" : "已关闭" }}
                        </span>
                      </div>
                    </div>
                  </div>
 <!-- 账号状态阈值配置 -->
 <div class="setting-item-create-tag">
                    <div class="setting-label">
                      <label>账号阈值配置</label>
                      <span class="setting-help">
                        配置账号时间 和 剩余额度 的颜色显示阈值
                      </span>
                    </div>

                    <div class="setting-control">
                      <div class="threshold-config-wrapper">
                        <button
                          @click="showThresholdConfigModal = true"
                          class="btn-config-threshold"
                        >
                          <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                          >
                            <path
                              d="M12,15.5A3.5,3.5 0 0,1 8.5,12A3.5,3.5 0 0,1 12,8.5A3.5,3.5 0 0,1 15.5,12A3.5,3.5 0 0,1 12,15.5M19.43,12.97C19.47,12.65 19.5,12.33 19.5,12C19.5,11.67 19.47,11.34 19.43,11L21.54,9.37C21.73,9.22 21.78,8.95 21.66,8.73L19.66,5.27C19.54,5.05 19.27,4.96 19.05,5.05L16.56,6.05C16.04,5.66 15.5,5.32 14.87,5.07L14.5,2.42C14.46,2.18 14.25,2 14,2H10C9.75,2 9.54,2.18 9.5,2.42L9.13,5.07C8.5,5.32 7.96,5.66 7.44,6.05L4.95,5.05C4.73,4.96 4.46,5.05 4.34,5.27L2.34,8.73C2.21,8.95 2.27,9.22 2.46,9.37L4.57,11C4.53,11.34 4.5,11.67 4.5,12C4.5,12.33 4.53,12.65 4.57,12.97L2.46,14.63C2.27,14.78 2.21,15.05 2.34,15.27L4.34,18.73C4.46,18.95 4.73,19.03 4.95,18.95L7.44,17.94C7.96,18.34 8.5,18.68 9.13,18.93L9.5,21.58C9.54,21.82 9.75,22 10,22H14C14.25,22 14.46,21.82 14.5,21.58L14.87,18.93C15.5,18.67 16.04,18.34 16.56,17.94L19.05,18.95C19.27,19.03 19.54,18.95 19.66,18.73L21.66,15.27C21.78,15.05 21.73,14.78 21.54,14.63L19.43,12.97Z"
                            />
                          </svg>
                          <span>配置阈值</span>
                        </button>

                        <div class="threshold-display">
                          <div class="threshold-rules">
                            <span class="threshold-label">时间:</span>
                            &nbsp;
                            <span class="rule-item red">
                              0 &lt; <span class="color-dot"></span> ≤
                              {{ statusThresholds.time.warning }}
                            </span>
                            &nbsp;&nbsp;&nbsp;&nbsp;
                            <span class="rule-item yellow">
                              {{ statusThresholds.time.warning }} &lt;
                              <span class="color-dot"></span> ≤
                              {{ statusThresholds.time.safe }} </span
                            >&nbsp;&nbsp;&nbsp;&nbsp;
                            <span class="rule-item green">
                              {{ statusThresholds.time.safe }} &lt;
                              <span class="color-dot"></span> ≤
                              {{ statusThresholds.timeMax }}
                            </span>
                          </div>

                          <div class="threshold-rules">
                            <span class="threshold-label">额度:</span>
                            &nbsp;
                            <span class="rule-item red">
                              0 &lt; <span class="color-dot"></span> ≤
                              {{ statusThresholds.balance.warning }} </span
                            >&nbsp;&nbsp;&nbsp;&nbsp;
                            <span class="rule-item yellow">
                              {{ statusThresholds.balance.warning }} &lt;
                              <span class="color-dot"></span> ≤
                              {{ statusThresholds.balance.safe }} </span
                            >&nbsp;&nbsp;&nbsp;&nbsp;
                            <span class="rule-item green">
                              {{ statusThresholds.balance.safe }} &lt;
                              <span class="color-dot"></span> ≤
                              {{ statusThresholds.balanceMax }}
                            </span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  <!-- API 服务器管理 -->
                  <div class="setting-item-create-tag">
                    <div class="setting-label">
                      <label>API 服务器</label>
                      <span class="setting-help">
                        管理内置 HTTP API 服务器，支持外部程序自动化导入账号
                      </span>
                    </div>

                    <div class="setting-control">
                      <div class="threshold-config-wrapper">
                        <button
                          @click="showApiServerModal = true"
                          class="btn-config-threshold"
                        >
                          <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                          >
                            <rect x="2" y="2" width="20" height="8" rx="2" ry="2" />
                            <rect x="2" y="14" width="20" height="8" rx="2" ry="2" />
                            <line x1="6" y1="6" x2="6.01" y2="6" />
                            <line x1="6" y1="18" x2="6.01" y2="18" />
                          </svg>
                          管理 API 服务器
                        </button>
                        <button
                          @click="showImportListModal = true"
                          class="btn-config-threshold secondary"
                        >
                          <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                          >
                            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
                          </svg>
                          导入列表
                        </button>
                      </div>
                    </div>
                  </div>

                 
                </div>
              </div>

              <!-- 云同步设置 -->
              <div class="settings-section">
                <div class="section-header">
                  <div class="section-icon">
                    <svg
                      width="20"
                      height="20"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M19.35,10.04C18.67,6.59 15.64,4 12,4C9.11,4 6.6,5.64 5.35,8.04C2.34,8.36 0,10.91 0,14A6,6 0 0,0 6,20H19A5,5 0 0,0 24,15C24,12.36 21.95,10.22 19.35,10.04Z"
                      />
                    </svg>
                  </div>
                  <h3>云同步设置</h3>
                </div>
                <p class="section-description">
                  配置WebDAV信息，实现多设备间的数据同步备份。
                </p>

                <!-- WebDAV配置 -->
                <div class="setting-item">
                  <div class="setting-label">
                    <div class="webdav-config-header">
                      <label>WebDAV配置</label>
                      <div
                        class="connection-status-badge"
                        :class="getConnectionStatusClass"
                      >
                        {{ getConnectionStatusText }}
                      </div>
                    </div>
                    <span class="setting-help">
                      选择云存储平台或填入WebDAV信息，实现数据云端同步
                    </span>
                  </div>
                  <div class="setting-control">
                    <div class="webdav-config-group">
                      <!-- 平台选择和服务器地址在同一行 -->
                      <div class="input-row">
                        <div class="input-group platform-group">
                          <label class="input-label">云存储平台</label>
                          <button
                            @click="showPlatformSelector = true"
                            class="platform-selector-btn-compact"
                          >
                            <span class="selected-platform-icon-compact">{{
                              getCurrentPreset.icon
                            }}</span>
                            <span class="selected-platform-name-compact">
                              {{ getCurrentPreset.name }}
                            </span>
                            <svg
                              width="16"
                              height="16"
                              viewBox="0 0 24 24"
                              fill="currentColor"
                              class="dropdown-icon-compact"
                            >
                              <path d="M7 10L12 15L17 10H7Z" />
                            </svg>
                          </button>
                        </div>
                        <div class="input-group server-group">
                          <label class="input-label">服务器地址</label>
                          <input
                            type="url"
                            v-model="webdavConfigs[selectedPreset].serverUrl"
                            class="webdav-input"
                            :placeholder="
                              getCurrentPreset.serverUrl ||
                              'https://example.com/dav/'
                            "
                            :readonly="selectedPreset !== 'custom'"
                          />
                        </div>
                      </div>
                      <div class="input-row">
                        <div class="input-group">
                          <label class="input-label">账户</label>
                          <input
                            type="email"
                            v-model="webdavConfigs[selectedPreset].username"
                            class="webdav-input"
                            :placeholder="
                              getCurrentPreset.usernamePlaceholder ||
                              'your@email.com'
                            "
                          />
                        </div>
                        <div class="input-group">
                          <label class="input-label">密码</label>
                          <div class="password-input-wrapper">
                            <input
                              :type="showWebdavPassword ? 'text' : 'password'"
                              v-model="webdavConfigs[selectedPreset].password"
                              class="webdav-input password-input"
                              :placeholder="
                                getCurrentPreset.passwordPlaceholder ||
                                '输入密码'
                              "
                            />
                            <button
                              type="button"
                              @click="showWebdavPassword = !showWebdavPassword"
                              class="password-toggle-btn"
                              :title="
                                showWebdavPassword ? '隐藏密码' : '显示密码'
                              "
                            >
                              <svg
                                width="18"
                                height="18"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                              >
                                <path
                                  v-if="!showWebdavPassword"
                                  d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"
                                />
                                <path
                                  v-else
                                  d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z"
                                />
                              </svg>
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                    <!-- 实时同步状态指示器 -->
                    <!-- <div
                    v-if="currentSyncStatus.isActive"
                    class="sync-status-indicator"
                  >
                    <div class="status-icon">
                      <svg
                        v-if="currentSyncStatus.operation === 'uploading'"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="animate-spin"
                      >
                        <path
                          d="M12,4V2A10,10 0 0,1 22,12H20A8,8 0 0,0 12,4Z"
                        />
                      </svg>
                      <svg
                        v-else-if="
                          currentSyncStatus.operation === 'downloading'
                        "
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="animate-spin"
                      >
                        <path
                          d="M12,20V22A10,10 0 0,1 2,12H4A8,8 0 0,0 12,20Z"
                        />
                      </svg>
                      <svg
                        v-else-if="currentSyncStatus.operation === 'connecting'"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="animate-spin"
                      >
                        <path
                          d="M12,4V2A10,10 0 0,1 22,12H20A8,8 0 0,0 12,4Z"
                        />
                      </svg>
                      <svg
                        v-else
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="animate-spin"
                      >
                        <path
                          d="M12,4V2A10,10 0 0,1 22,12H20A8,8 0 0,0 12,4Z"
                        />
                      </svg>
                    </div>
                    <div class="status-content">
                      <span class="status-message">{{
                        currentSyncStatus.message
                      }}</span>
                      <span
                        class="status-duration"
                        v-if="currentSyncStatus.startTime"
                      >
                        {{
                          Math.floor(
                            (Date.now() - currentSyncStatus.startTime) / 1000
                          )
                        }}s
                      </span>
                    </div>
                  </div> -->

                    <!-- 统一按钮行 -->
                    <div class="webdav-unified-actions">
                      <button
                        @click="testWebDAVConnection"
                        class="btn-test-connection"
                        :disabled="
                          !canTestConnection ||
                          isTestingConnection ||
                          currentSyncStatus.isActive
                        "
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20A8,8 0 0,0 20,12A8,8 0 0,0 12,4M11,16.5L6.5,12L7.91,10.59L11,13.67L16.59,8.09L18,9.5L11,16.5Z"
                          />
                        </svg>
                        {{
                          isTestingConnection ||
                          (currentSyncStatus.isActive &&
                            currentSyncStatus.operation === "connecting")
                            ? "测试中..."
                            : "测试连接"
                        }}
                      </button>

                      <button
                        @click="saveWebDAVConfig"
                        class="btn-save-config"
                        :disabled="
                          !canSaveConfig ||
                          isSavingConfig ||
                          currentSyncStatus.isActive
                        "
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z"
                          />
                        </svg>
                        {{ isSavingConfig ? "保存中..." : "保存配置" }}
                      </button>

                      <div class="divider"></div>

                      <button
                        @click="showForceUploadConfirm"
                        class="btn-upload"
                        :disabled="
                          !isWebDAVConfigured ||
                          isSyncing ||
                          currentSyncStatus.isActive
                        "
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5Z"
                          />
                        </svg>
                        {{
                          isSyncing ||
                          (currentSyncStatus.isActive &&
                            currentSyncStatus.operation === "uploading")
                            ? "上传中..."
                            : "强制上传"
                        }}
                      </button>

                      <button
                        @click="showForceDownloadConfirm"
                        class="btn-download"
                        :disabled="
                          !isWebDAVConfigured ||
                          isSyncing ||
                          currentSyncStatus.isActive
                        "
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path d="M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z" />
                        </svg>
                        {{
                          isSyncing ||
                          (currentSyncStatus.isActive &&
                            currentSyncStatus.operation === "downloading")
                            ? "下载中..."
                            : "强制下载"
                        }}
                      </button>

                      <button
                        @click="handleShowSyncHistory"
                        class="btn-history"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M13.5,8H12V13L16.28,15.54L17,14.33L13.5,12.25V8M13,3A9,9 0 0,0 4,12H1L4.96,16.03L9,12H6A7,7 0 0,1 13,5A7,7 0 0,1 20,12A7,7 0 0,1 13,19C11.07,19 9.32,18.21 8.06,16.94L6.64,18.36C8.27,19.99 10.51,21 13,21A9,9 0 0,0 22,12A9,9 0 0,0 13,3"
                          />
                        </svg>
                        同步历史
                      </button>
                    </div>

                    <!-- 自动上传和自动下载开关 -->
                    <div class="auto-sync-toggle" v-if="isWebDAVConfigured">
                      <div class="auto-sync-row">
                        <!-- 自动上传开关 -->
                        <div class="auto-sync-item">
                          <div class="toggle-wrapper">
                            <label class="toggle-label-switch">
                              <input
                                type="checkbox"
                                v-model="autoUploadEnabled"
                                @change="toggleAutoUpload"
                                class="toggle-checkbox"
                              />
                              <span class="toggle-slider"></span>
                            </label>
                            <span class="toggle-text">
                              {{
                                autoUploadEnabled
                                  ? "自动上传已开启"
                                  : "自动上传已关闭"
                              }}
                            </span>
                          </div>
                        </div>

                        <!-- 自动下载开关 -->
                        <div class="auto-sync-item">
                          <div class="toggle-wrapper">
                            <label class="toggle-label-switch">
                              <input
                                type="checkbox"
                                v-model="autoDownloadEnabled"
                                @change="toggleAutoDownload"
                                class="toggle-checkbox"
                              />
                              <span class="toggle-slider"></span>
                            </label>
                            <span class="toggle-text">
                              {{
                                autoDownloadEnabled
                                  ? "自动下载已开启"
                                  : "自动下载已关闭"
                              }}
                            </span>
                          </div>
                        </div>
                      </div>
                      <div class="toggle-help">
                        开启后，每3分钟自动上传到云端或下载至本地，关闭软件前也会自动上传
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 应用信息 -->
              <div class="settings-section">
                <div class="section-header">
                  <div class="section-icon">
                    <svg
                      width="20"
                      height="20"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                    >
                      <path
                        d="M13,9H11V7H13M13,17H11V11H13M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2Z"
                      />
                    </svg>
                  </div>
                  <h3>应用信息</h3>
                </div>

                <div class="app-info">
                  <div class="info-item">
                    <span class="info-label">版本号</span>
                    <span class="info-value">{{ currentVersion }}</span>
                  </div>
                  <div class="info-item">
                    <span class="info-label">检查更新</span>
                    <div class="info-value-with-action">
                      <button
                        @click="checkForUpdates"
                        class="info-action-btn primary"
                        :disabled="isCheckingUpdates"
                      >
                        <svg
                          v-if="!isCheckingUpdates"
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
                          />
                        </svg>
                        <svg
                          v-else
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                          class="loading-icon"
                        >
                          <path
                            d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
                          />
                        </svg>
                        {{ isCheckingUpdates ? "检查中..." : "检查更新" }}
                      </button>
                    </div>
                  </div>
                  <div class="info-item">
                    <span class="info-label">GitHub</span>
                    <div class="info-value-with-action">
                      <div class="github-content">
                        <div class="github-tip-line">
                          <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="github-icon"
                          >
                            <path
                              d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            />
                          </svg>
                          <span class="github-tip">
                            欢迎前往 GitHub 点亮小星星
                            ⭐，支持作者持续更新优化！
                          </span>
                        </div>
                      </div>
                      <button
                        @click="openGitHubRepo"
                        class="info-action-btn"
                        title="在浏览器中打开 GitHub 仓库"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                        >
                          <path
                            d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"
                          />
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

      </main>

      <!-- 更新检测组件 -->
      <UpdateChecker ref="updateChecker" @show-status="showStatus" />

      <!-- 改进的多条状态提示系统提示 -->
      <div class="status-messages-container">
        <div
          v-for="(statusItem, index) in statusMessages"
          :key="statusItem.id"
          :class="[
            'status-toast',
            statusItem.type,
            { removing: statusItem.isRemoving },
          ]"
          :style="{
            bottom: `${20 + (statusMessages.length - 1 - index) * 60}px`,
          }"
        >
          {{ statusItem.message }}
        </div>
      </div>

      <!-- 防封插件选择对话框 -->
      <ModalContainer
        :visible="showPluginHomeDialog"
        title="选择插件版本"
        size="small"
        @close="showPluginHomeDialog = false"
      >
        <div class="dialog-buttons">
          <button @click="openVSCodePlugin" class="dialog-btn vscode">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M23.15 2.587L18.21.21a1.494 1.494 0 0 0-1.705.29l-9.46 8.63-4.12-3.128a.999.999 0 0 0-1.276.057L.327 7.261A1 1 0 0 0 .326 8.74L3.899 12 .326 15.26a1 1 0 0 0 .001 1.479L1.65 17.94a.999.999 0 0 0 1.276.057l4.12-3.128 9.46 8.63a1.492 1.492 0 0 0 1.704.29l4.942-2.377A1.5 1.5 0 0 0 24 20.06V3.939a1.5 1.5 0 0 0-.85-1.352zm-5.146 14.861L10.826 12l7.178-5.448v10.896z"
              />
            </svg>
            VSCode 系列
          </button>
          <button @click="openJetBrainsPlugin" class="dialog-btn jetbrains">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0L19.2 12l-4.6-4.6L16 6l6 6-6 6-1.4-1.4z"
              />
            </svg>
            JetBrains 系列
          </button>
          <button
            @click="showPluginHomeDialog = false"
            class="dialog-btn cancel"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
              />
            </svg>
            取消
          </button>
        </div>
      </ModalContainer>

      <!-- 插件下载弹窗 -->
      <ModalContainer
        :visible="showPluginModal"
        title="下载防封插件"
        size="medium"
        @close="showPluginModal = false"
      >
        <p class="plugin-modal-description">请选择适合您的开发环境的插件：</p>

        <div class="plugin-options-grid">
          <div class="plugin-option-card">
            <div class="plugin-icon vscode">
              <svg
                width="32"
                height="32"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M23.15 2.587L18.21.21a1.494 1.494 0 0 0-1.705.29l-9.46 8.63-4.12-3.128a.999.999 0 0 0-1.276.057L.327 7.261A1 1 0 0 0 .326 8.74L3.899 12 .326 15.26a1 1 0 0 0 .001 1.479L1.65 17.94a.999.999 0 0 0 1.276.057l4.12-3.128 9.46 8.63a1.492 1.492 0 0 0 1.704.29l4.942-2.377A1.5 1.5 0 0 0 24 20.06V3.939a1.5 1.5 0 0 0-.85-1.352zm-5.146 14.861L10.826 12l7.178-5.448v10.896z"
                />
              </svg>
            </div>
            <h4>VSCode 系列</h4>
            <p>适用于 Visual Studio Code、cursor、Trae 等</p>
            <button @click="openVSCodePlugin" class="plugin-btn vscode-btn">
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" />
              </svg>
              获取 VSCode 插件
            </button>
          </div>

          <div class="plugin-option-card">
            <div class="plugin-icon jetbrains">
              <svg
                width="32"
                height="32"
                viewBox="0 0 32 32"
                fill="currentColor"
              >
                <!-- JetBrains 标志性方形logo -->
                <rect
                  x="0"
                  y="0"
                  width="32"
                  height="32"
                  rx="2"
                  fill="currentColor"
                />
                <rect x="2" y="2" width="28" height="28" rx="1" fill="#000" />

                <!-- JB 字母 -->
                <g fill="#fff">
                  <!-- J -->
                  <rect x="6" y="6" width="6" height="2" />
                  <rect x="9" y="8" width="2" height="6" />
                  <rect x="6" y="14" width="5" height="2" />
                  <rect x="6" y="12" width="2" height="2" />

                  <!-- B -->
                  <rect x="14" y="6" width="2" height="10" />
                  <rect x="16" y="6" width="4" height="2" />
                  <rect x="16" y="10" width="3" height="2" />
                  <rect x="16" y="14" width="4" height="2" />
                  <rect x="20" y="8" width="2" height="2" />
                  <rect x="19" y="12" width="2" height="2" />
                </g>

                <!-- 底部标识线 -->
                <rect x="6" y="20" width="12" height="1" fill="#fff" />
              </svg>
            </div>
            <h4>JetBrains 系列</h4>
            <p>适用于 IntelliJ IDEA、WebStorm 等（暂无相应防封插件）</p>
            <button
              @click="openJetBrainsPlugin"
              class="plugin-btn jetbrains-btn"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" />
              </svg>
              获取 JetBrains 插件
            </button>
          </div>
        </div>
      </ModalContainer>

      <!-- 账号状态阈值配置弹窗 -->
      <ThresholdConfigModal
        :visible="showThresholdConfigModal"
        :timeThresholds="statusThresholds.time"
        :balanceThresholds="statusThresholds.balance"
        :timeMax="statusThresholds.timeMax"
        :balanceMax="statusThresholds.balanceMax"
        @close="showThresholdConfigModal = false"
        @save="handleSaveThresholds"
        @reset="handleResetThresholds"
      />

      <!-- API 服务器管理弹窗 -->
      <ApiServerStatus
        :visible="showApiServerModal"
        @close="showApiServerModal = false"
        @show-status="showStatus"
      />

      <!-- 导入列表弹窗 -->
      <ImportListModal
        :visible="showImportListModal"
        @close="showImportListModal = false"
        @show-status="showStatus"
      />
    </div>

    <!-- 编辑器重置弹窗 -->
    <EditorResetModal
      :visible="showEditorResetModal"
      @close="handleEditorResetModalClose"
      @reset-complete="handleEditorResetComplete"
    />

    <!-- Session 导入帮助弹窗 -->
    <ModalContainer
      :visible="showSessionHelpModal"
      title="Session 导入使用说明"
      size="medium"
      @close="showSessionHelpModal = false"
    >
      <div class="session-help-content">
        <div class="help-step">
          <div class="step-title">步骤 1: 登录 Augment</div>
          <p>
            在浏览器中登录你的 Augment 账户：
            <code
              class="copyable-code"
              @click="copyLoginUrl"
              title="点击复制"
            >
              <b>https://login.augmentcode.com</b>
            </code>
          </p>
          <p>
            然后访问：
            <code
              class="copyable-code"
              @click="copyAuthPageUrl"
              title="点击复制"
            >
              <b>https://auth.augmentcode.com</b>
            </code>
          </p>
        </div>

        <div class="help-step">
          <div class="step-title">步骤 2: 打开浏览器开发者工具</div>
          <p>
            按
            <code> <b>F12</b> </code> 键或右键点击页面选择"检查"打开开发者工具
          </p>
        </div>

        <div class="help-step">
          <div class="step-title">步骤 3: 找到 Cookie</div>
          <p>
            在开发者工具中，点击
            <code> <b>Application (应用)</b> </code> 标签页，然后在左侧展开
            <code> <b>Cookies</b> </code>，选择
            <code> <b>https://auth.augmentcode.com</b> </code>
          </p>
        </div>

        <div class="help-step">
          <div class="step-title">步骤 4: 复制 session 值</div>
          <p>
            在 Cookie 列表中找到名为 <code> <b>session</b> </code> 的项，双击其
            <code> <b>Value (值)</b> </code> 列，复制完整的 session 值
          </p>
        </div>

        <div class="help-step">
          <div class="step-title">步骤 5: 粘贴到输入框</div>
          <p>
            将复制的 session 值粘贴到上方的输入框中，然后点击
            <code> <b>导入 Session</b> </code> 按钮
          </p>
        </div>
      </div>
    </ModalContainer>

    <!-- 赞赏弹窗 -->
    <ModalContainer
      :visible="showAppreciationModal"
      title="💖 支持开发者"
      size="small"
      @close="showAppreciationModal = false"
    >
      <div class="appreciation-content">
        <p
          style="
            margin-bottom: 24px;
            color: #666;
            text-align: center;
            line-height: 1.5;
          "
        >
          如果这个项目对您有帮助，欢迎通过微信扫码支持开发者继续改进和维护：
        </p>
        <div class="appreciation-qr-container">
          <img :src="appreciationQR" alt="赞赏码" class="appreciation-qr" />
        </div>
        <div class="appreciation-text">
          <p>感谢每一位用户的理解与支持 🙏</p>
        </div>
      </div>
    </ModalContainer>

    <!-- 交流群弹窗 -->
    <ModalContainer
      :visible="showCommunityModal"
      title="💬 加入交流群"
      size="small"
      @close="showCommunityModal = false"
    >
      <div class="appreciation-content">
        <p
          style="
            margin-bottom: 24px;
            color: #666;
            text-align: center;
            line-height: 1.5;
          "
        >
          群内不定时发放 augment、cursor、codex 等AI账号福利
        </p>
        <div class="appreciation-qr-container">
          <img :src="communityQR" alt="交流群二维码" class="appreciation-qr" />
        </div>
        <div class="appreciation-text">
          <p>期待与您在群里交流 🎉</p>
        </div>
      </div>
    </ModalContainer>

    <!-- 平台选择弹窗 -->
    <ModalContainer
      :visible="showPlatformSelector"
      title="选择云存储平台"
      size="medium"
      @close="showPlatformSelector = false"
    >
      <div class="platform-selector-content">
        <p
          style="
            margin-bottom: 24px;
            color: #666;
            text-align: center;
            line-height: 1.5;
          "
        >
          选择您要使用的WebDAV兼容云存储平台，每个平台的账户信息将单独保存。
        </p>
        <div class="platform-modal-grid">
          <div
            v-for="(preset, key) in webdavPresets"
            :key="key"
            @click="selectPresetFromModal(key)"
            class="platform-modal-card"
            :class="{ active: selectedPreset === key }"
          >
            <div class="platform-modal-icon">{{ preset.icon }}</div>
            <div class="platform-modal-info">
              <div class="platform-modal-name">{{ preset.name }}</div>
              <div class="platform-modal-desc">{{ preset.description }}</div>
            </div>
            <div v-if="selectedPreset === key" class="platform-selected-mark">
              <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M9 16.17L4.83 12L3.41 13.41L9 19L21 7L19.59 5.59L9 16.17Z"
                />
              </svg>
            </div>
          </div>
        </div>
      </div>
    </ModalContainer>

    <!-- 强制同步确认弹窗 -->
    <ModalContainer
      :visible="showForceConfirm"
      :title="forceConfirmTitle"
      size="medium"
      @close="cancelForceSync"
    >
      <div class="force-confirm-content">
        <div class="warning-icon-container">
          <div class="warning-icon">
            <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 2L1 21H23L12 2M12 6L19.53 19H4.47L12 6M11 10V14H13V10H11M11 16V18H13V16H11Z"
              />
            </svg>
          </div>
        </div>

        <p
          style="
            margin-bottom: 24px;
            color: #666;
            text-align: center;
            line-height: 1.5;
          "
        >
          {{ forceConfirmMessage }}
        </p>

        <div class="warning-details">
          <div class="warning-item">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M9 12L11 14L15 10M12 2C6.48 2 2 6.48 2 12S6.48 22 12 22 22 17.52 22 12 17.52 2 12 2Z"
              />
            </svg>
            <span>此操作将完全覆盖目标位置的所有数据</span>
          </div>
          <div class="warning-item">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 2C6.48 2 2 6.48 2 12S6.48 22 12 22 22 17.52 22 12 17.52 2 12 2ZM13 17H11V15H13V17ZM13 13H11V7H13V13Z"
              />
            </svg>
            <span>无法撤销，可先备份数据至本地</span>
          </div>
        </div>

        <div class="force-confirm-footer">
          <button @click="cancelForceSync" class="btn-cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z"
              />
            </svg>
            取消
          </button>
          <button @click="confirmForceSync" class="btn-confirm">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M9 16.17L4.83 12L3.41 13.41L9 19L21 7L19.59 5.59L9 16.17Z"
              />
            </svg>
            确认执行
          </button>
        </div>
      </div>
    </ModalContainer>

    <!-- 同步历史弹框模态 -->
    <ModalContainer
      :visible="showSyncHistoryModal"
      title="同步历史记录"
      size="medium"
      @close="showSyncHistoryModal = false"
    >
      <div class="sync-history-content">
        <div class="history-actions" v-if="syncHistory.length > 0">
          <button @click="clearSyncHistory" class="btn-clear-history">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z"
              />
            </svg>
            清空所有记录
          </button>
        </div>

        <div class="history-list-modal" v-if="syncHistory.length > 0">
          <div
            v-for="entry in syncHistory"
            :key="entry.id"
            class="history-item-modal"
            :class="entry.status"
          >
            <div class="history-icon">
              <svg
                v-if="entry.operation === 'upload'"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5Z" />
              </svg>
              <svg
                v-else-if="entry.operation === 'download'"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z" />
              </svg>
              <svg
                v-else-if="entry.operation === 'test'"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20A8,8 0 0,0 20,12A8,8 0 0,0 12,4M11,16.5L6.5,12L7.91,10.59L11,13.67L16.59,8.09L18,9.5L11,16.5Z"
                />
              </svg>
            </div>
            <div class="history-content">
              <div class="history-message">{{ entry.message }}</div>
              <div class="history-meta">
                <span class="history-time">
                  {{ entry.timestamp.toLocaleString("zh-CN") }}
                </span>
                <span v-if="entry.duration" class="history-duration">
                  {{ formatSyncDuration(entry.duration) }}
                </span>
              </div>
            </div>
            <div class="history-status" :class="entry.status">
              <svg
                v-if="entry.status === 'success'"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M11,16.5L6.5,12L7.91,10.59L11,13.67L16.59,8.09L18,9.5L11,16.5Z"
                />
              </svg>
              <svg
                v-else-if="entry.status === 'error'"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M13,13H11V7H13M13,17H11V15H13M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2Z"
                />
              </svg>
              <svg
                v-else
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M13,14H11V10H13M13,18H11V16H13M1,21H23L12,2L1,21Z" />
              </svg>
            </div>
          </div>
        </div>

        <div v-else class="history-empty-modal">
          <svg
            width="64"
            height="64"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="empty-icon"
          >
            <path
              d="M13.5,8H12V13L16.28,15.54L17,14.33L13.5,12.25V8M13,3A9,9 0 0,0 4,12H1L4.96,16.03L9,12H6A7,7 0 0,1 13,5A7,7 0 0,1 20,12A7,7 0 0,1 13,19C11.07,19 9.32,18.21 8.06,16.94L6.64,18.36C8.27,19.99 10.51,21 13,21A9,9 0 0,0 22,12A9,9 0 0,0 13,3"
            />
          </svg>
          <h4>暂无同步历史记录</h4>
          <p>执行WebDAV操作后，历史记录将显示在这里</p>
        </div>
      </div>
    </ModalContainer>

    <!-- AccountManagerModal 和 TagEditorModal 已移至 TokenList 组件内部 -->
  </div>
  <!-- 结束 app -->
</template>

<script setup>
import { ref, onMounted, onUnmounted, onBeforeUnmount, computed, watch, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import TokenList from "./components/TokenList.vue";
import OutlookManager from "./components/OutlookManager.vue";
import ModalContainer from "./components/ModalContainer.vue";
import CustomTitleBar from "./components/CustomTitleBar.vue";
import UpdateChecker from "./components/UpdateChecker.vue";
import EditorResetModal from "./components/EditorResetModal.vue";
import ThresholdConfigModal from "./components/ThresholdConfigModal.vue";
import ApiServerStatus from "./components/ApiServerStatus.vue";
import ImportListModal from "./components/ImportListModal.vue";
import { useImportRecords, ImportStatus } from "./stores/importRecords";
import appreciationQR from "/赞赏.png";
import communityQR from "/交流群.png";

// 导入记录管理
const { addRecord, addRecords, updateRecord } = useImportRecords();

// 简化的状态管理
const currentView = ref("token-generator"); // 当前视图状态

// Token Generator 相关状态
const currentTokenTab = ref("session"); // Token Generator Tab 状态，默认为 session 流程
const sessionInput = ref(""); // Session 输入
const isImportingSession = ref(false); // Session 导入中
const sessionImportProgress = ref(""); // Session 导入进度

// 邮箱管理相关状态
const outlookImportFormat = ref("format2"); // 邮箱账户导入格式
const isRefreshingStatuses = ref(false); // 批量刷新状态
const outlookManagerRef = ref(null); // OutlookManager组件引用
const tokenListRef = ref(null); // TokenList组件引用
const updateChecker = ref(null); // UpdateChecker组件引用
const outlookAccountsCount = ref(0); // 邮箱账户数量
const outlookAccountsStatus = ref({
  total: 0,
  active: 0,
  inactive: 0,
  error: 0,
  unknown: 0,
}); // 邮箱账户状态统计
const hasInitializedEmailManagement = ref(false); // 是否已初始化邮箱管理

// Session 导入事件监听器清理函数
let unlistenSessionProgress = null;

// Deep-Link 导入队列相关状态
const importQueue = ref([]);
const isImportingFromQueue = ref(false);

// 状态筛选管理
const selectedStatusFilter = ref(null); // null表示"全部", 其他值为具体状态: 'ACTIVE', 'SUSPENDED', 'EXPIRED', 'INVALID_TOKEN'

// 注意：视图切换时关闭弹窗的逻辑已移至 handleNavClick 函数中

// 监听WebDAV配置状态变化，自动管理自动同步 - 将在onMounted中启用

// (TokenForm 模态框状态已在后续部分定义)

// 更新现有账号数据的通用函数
const updateExistingToken = (existingToken, newTokenData) => {
  if (!tokenListRef.value || !tokenListRef.value.tokens || !tokenListRef.value.tokens.value) {
    return null;
  }

  const index = tokenListRef.value.tokens.value.findIndex((t) => t.id === existingToken.id);
  if (index !== -1) {
    // 保留原有的创建时间、ID等元数据，只更新业务数据
    tokenListRef.value.tokens.value[index] = {
      ...existingToken,
      tenant_url: newTokenData.tenantUrl,
      access_token: newTokenData.accessToken,
      portal_url: newTokenData.portalUrl,
      email_note: newTokenData.emailNote,
      auth_session: newTokenData.authSession || existingToken.auth_session,
      ban_status: newTokenData.banStatus || null,
      portal_info: newTokenData.portalInfo || null,
      tag_text: newTokenData.tagText || existingToken.tag_text,
      tag_color: newTokenData.tagColor || existingToken.tag_color,
      updated_at: new Date().toISOString(),
    };
    recalcHeaderCounts();
    return tokenListRef.value.tokens.value[index];
  }
  return null;
};

// 按创建时间正确插入新账号的函数
const insertTokenSorted = (newToken) => {
  if (!tokenListRef.value || !tokenListRef.value.tokens || !tokenListRef.value.tokens.value) {
    return;
  }

  const newCreatedTime = new Date(newToken.created_at);
  let insertIndex = tokenListRef.value.tokens.value.length;

  // 找到正确的插入位置
  for (let i = 0; i < tokenListRef.value.tokens.value.length; i++) {
    const token = tokenListRef.value.tokens.value[i];
    const tokenCreatedTime = new Date(token.created_at || 0);

    // 如果新token的创建时间更早，应该插在当前位置
    if (newCreatedTime < tokenCreatedTime) {
      insertIndex = i;
      break;
    }

    // 如果创建时间相同，按ID排序
    if (newCreatedTime.getTime() === tokenCreatedTime.getTime()) {
      if (newToken.id.localeCompare(token.id) < 0) {
        insertIndex = i;
        break;
      }
    }
  }

  // 在正确位置插入新token
  tokenListRef.value.tokens.value.splice(insertIndex, 0, newToken);
};

// 改进的多条状态提示系统
const statusMessages = ref([]);
let statusIdCounter = 0;

// 设置页面相关状态
const dataDirectory = ref("正在获取...");
const currentTokensPath = ref("正在获取...");
const isSelectingPath = ref(false);
const currentVersion = ref("获取中...");

// 更新检查相关状态
const isCheckingUpdates = ref(false);

// WebDAV密码显示状态
const showWebdavPassword = ref(false);

// WebDAV云同步相关状态 - 为每个平台单独存储配置
const webdavConfigs = ref({
  jianguoyun: {
    serverUrl: "https://dav.jianguoyun.com/dav/",
    username: "",
    password: "",
  },
  teracloud: {
    serverUrl: "https://webdav.teracloud.jp/dav/",
    username: "",
    password: "",
  },
  yandex: {
    serverUrl: "https://webdav.yandex.ru/",
    username: "",
    password: "",
  },
  koofr: {
    serverUrl: "https://app.koofr.net/dav/Koofr",
    username: "",
    password: "",
  },
  box: {
    serverUrl: "https://dav.box.com/dav",
    username: "",
    password: "",
  },
  custom: {
    serverUrl: "",
    username: "",
    password: "",
  },
});

// 当前WebDAV配置（指向当前选中平台的配置）
const webdavConfig = computed(() => {
  return (
    webdavConfigs.value[selectedPreset.value] || webdavConfigs.value.custom
  );
});

// WebDAV平台预设配置
const webdavPresets = {
  jianguoyun: {
    name: "坚果云",
    serverUrl: "https://dav.jianguoyun.com/dav/",
    usernamePlaceholder: "your-email@example.com",
    passwordPlaceholder: "应用密码（非登录密码）",
    description: "国内访问稳定，支持应用密码认证",
    icon: "🥜",
  },
  teracloud: {
    name: "TeraCloud",
    serverUrl: "https://webdav.teracloud.jp/dav/",
    usernamePlaceholder: "your-username",
    passwordPlaceholder: "账户密码",
    description: "日本服务器，亚洲地区访问较快",
    icon: "☁️",
  },
  yandex: {
    name: "Yandex Disk",
    serverUrl: "https://webdav.yandex.ru/",
    usernamePlaceholder: "your-yandex-username",
    passwordPlaceholder: "应用密码",
    description: "俄罗斯服务商，免费10GB空间",
    icon: "🔴",
  },
  koofr: {
    name: "Koofr",
    serverUrl: "https://app.koofr.net/dav/Koofr",
    usernamePlaceholder: "your-email@example.com",
    passwordPlaceholder: "应用密码",
    description: "欧洲服务器，注重隐私保护",
    icon: "🟢",
  },
  box: {
    name: "Box (企业版)",
    serverUrl: "https://dav.box.com/dav",
    usernamePlaceholder: "your-box-email@company.com",
    passwordPlaceholder: "Box账户密码",
    description: "企业级服务，仅企业版支持WebDAV",
    icon: "📦",
  },
  custom: {
    name: "自定义",
    serverUrl: "",
    usernamePlaceholder: "用户名或邮箱",
    passwordPlaceholder: "密码或应用密码",
    description: "手动配置其他WebDAV兼容服务",
    icon: "⚙️",
  },
};

const selectedPreset = ref("jianguoyun");
const showPlatformSelector = ref(false);
const isTestingConnection = ref(false);
const isSavingConfig = ref(false);
const isSyncing = ref(false);
const connectionStatus = ref(null);
const syncStatus = ref(null);
const isWebDAVConfigured = ref(false);

// 自动上传和自动下载相关状态
const autoUploadEnabled = ref(false);
const autoDownloadEnabled = ref(false);

// 批量导入导出时创建标签的开关
const createTagOnImportExport = ref(false);

// 账号状态阈值配置
const showThresholdConfigModal = ref(false);
const showApiServerModal = ref(false);
const showImportListModal = ref(false);
const statusThresholds = ref({
  time: {
    warning: 0,
    safe: 0,
  },
  balance: {
    warning: 0,
    safe: 0,
  },
  timeMax: 0,
  balanceMax: 0,
});

// 保存自动上传状态到本地存储
const saveAutoUploadState = () => {
  try {
    localStorage.setItem(
      "auto_upload_enabled",
      JSON.stringify(autoUploadEnabled.value)
    );
    console.log("自动上传状态已保存:", autoUploadEnabled.value);
  } catch (error) {
    console.error("保存自动上传状态失败:", error);
  }
};

// 保存自动下载状态到本地存储
const saveAutoDownloadState = () => {
  try {
    localStorage.setItem(
      "auto_download_enabled",
      JSON.stringify(autoDownloadEnabled.value)
    );
    console.log("自动下载状态已保存:", autoDownloadEnabled.value);
  } catch (error) {
    console.error("保存自动下载状态失败:", error);
  }
};

// 从本地存储加载自动上传状态
const loadAutoUploadState = () => {
  try {
    const saved = localStorage.getItem("auto_upload_enabled");
    if (saved !== null) {
      autoUploadEnabled.value = JSON.parse(saved);
      console.log("自动上传状态已恢复:", autoUploadEnabled.value);
    }
  } catch (error) {
    console.error("加载自动上传状态失败:", error);
    autoUploadEnabled.value = false; // 出错时默认为关闭
  }
};

// 从本地存储加载自动下载状态
const loadAutoDownloadState = () => {
  try {
    const saved = localStorage.getItem("auto_download_enabled");
    if (saved !== null) {
      autoDownloadEnabled.value = JSON.parse(saved);
      console.log("自动下载状态已恢复:", autoDownloadEnabled.value);
    }
  } catch (error) {
    console.error("加载自动下载状态失败:", error);
    autoDownloadEnabled.value = false; // 出错时默认为关闭
  }
};

// 保存导入导出时创建标签的状态到本地存储
const saveCreateTagOnImportExportState = () => {
  try {
    localStorage.setItem(
      "create_tag_on_import_export",
      JSON.stringify(createTagOnImportExport.value)
    );
    console.log("导入导出时创建标签状态已保存:", createTagOnImportExport.value);
  } catch (error) {
    console.error("保存导入导出时创建标签状态失败:", error);
  }
};

// 从本地存储加载导入导出时创建标签的状态
const loadCreateTagOnImportExportState = () => {
  try {
    const saved = localStorage.getItem("create_tag_on_import_export");
    if (saved !== null) {
      createTagOnImportExport.value = JSON.parse(saved);
      console.log(
        "导入导出时创建标签状态已恢复:",
        createTagOnImportExport.value
      );
    }
  } catch (error) {
    console.error("加载导入导出时创建标签状态失败:", error);
    createTagOnImportExport.value = false; // 出错时默认为关闭
  }
};

// 保存账号状态阈值配置到本地存储和云端
const saveStatusThresholds = async () => {
  try {
    // 保存到 localStorage
    localStorage.setItem(
      "status_thresholds",
      JSON.stringify(statusThresholds.value)
    );

    // 保存到后端（config.json）
    await invoke("save_status_thresholds", {
      thresholds: statusThresholds.value,
    });

    // 如果启用了自动上传且WebDAV已配置，同步到云端
    if (autoUploadEnabled.value && isWebDAVConfigured.value) {
      await saveStatusThresholdsToCloud();
    }

    // 显示保存成功提示
    showStatus("账号状态阈值配置已保存", "success");
  } catch (error) {
    console.error("保存账号状态阈值失败:", error);
    showStatus(`保存账号状态阈值失败: ${error}`, "error");
  }
};

// 保存阈值配置到云端
const saveStatusThresholdsToCloud = async () => {
  try {
    const result = await invoke("save_status_thresholds", {
      thresholds: statusThresholds.value,
    });
    console.log("阈值配置已同步到云端:", result);
  } catch (error) {
    console.error("同步阈值配置到云端失败:", error);
  }
};

// 从本地存储加载账号状态阈值配置
const loadStatusThresholds = async () => {
  try {
    // 优先从后端加载（后端会自动处理：云端 -> 本地文件 -> 默认值）
    const result = await invoke("load_status_thresholds");
    if (result) {
      statusThresholds.value = {
        time: result.time,
        balance: result.balance,
        timeMax: result.timeMax,
        balanceMax: result.balanceMax,
      };
      // 同步到 localStorage
      localStorage.setItem(
        "status_thresholds",
        JSON.stringify(statusThresholds.value)
      );
      console.log("账号状态阈值已从后端加载:", statusThresholds.value);
    }
  } catch (error) {
    console.error("从后端加载账号状态阈值失败:", error);
    // 如果后端加载失败，尝试从 localStorage 加载
    const saved = localStorage.getItem("status_thresholds");
    if (saved !== null) {
      const parsed = JSON.parse(saved);
      statusThresholds.value = {
        time: parsed.time,
        balance: parsed.balance,
        timeMax: parsed.timeMax,
        balanceMax: parsed.balanceMax,
      };
      console.log("从 localStorage 恢复账号状态阈值:", statusThresholds.value);
    }
  }
};

// 处理保存阈值配置
const handleSaveThresholds = (config) => {
  statusThresholds.value.time = config.timeThresholds;
  statusThresholds.value.balance = config.balanceThresholds;
  statusThresholds.value.timeMax = config.timeMax;
  statusThresholds.value.balanceMax = config.balanceMax;
  saveStatusThresholds();
  showStatus("阈值配置已保存", "success");
};

// 处理恢复默认阈值配置
const handleResetThresholds = () => {
  showStatus("已恢复默认阈值配置", "success");
};

// 重试进度状态
const retryProgress = ref(null);

// 同步历史记录状态
const syncHistory = ref([]);
const showSyncHistory = ref(false);
const showSyncHistoryModal = ref(false);

// 当前操作状态（覆盖式显示）
const currentOperationStatus = ref({
  visible: false,
  type: "info", // 'success', 'error', 'warning', 'info'
  message: "",
  showProgress: false,
  progress: 0,
  progressText: "",
});

// WebDAV连接状态（持久显示在配置标签旁）
const webdavConnectionState = ref("disconnected"); // 'disconnected', 'connected', 'error'

// 实时同步状态指示器
const currentSyncStatus = ref({
  isActive: false,
  operation: null, // 'uploading', 'downloading', 'connecting'
  progress: 0,
  message: "",
  startTime: null,
});

// 简化的工具函数
const createNewToken = (
  tenantUrl,
  accessToken,
  portalUrl = null,
  emailNote = null,
  authSession = null,
  tagName = null,
  tagColor = null
) => {
  const now = new Date().toISOString();
  return {
    id: "token_" + Date.now() + "_" + Math.random().toString(36).substr(2, 9),
    tenant_url: tenantUrl,
    access_token: accessToken,
    created_at: now,
    updated_at: now,
    portal_url: portalUrl,
    ban_status: null,
    portal_info: null,
    email_note: emailNote,
    auth_session: authSession,
    tag_name: tagName,
    tag_color: tagColor,
  };
};

// 移除了复杂的自动保存、错误处理和回滚机制，保持简单

// Token generator data
const authUrl = ref("");
const authCode = ref("");
const tokenResult = ref(null);
const isGenerating = ref(false);
const isGettingToken = ref(false);
const portalUrl = ref("");
const emailNote = ref("");

// Template refs
const authUrlInput = ref(null);
const accessTokenInput = ref(null);
const tenantUrlInput = ref(null);

// currentTokens 计算属性已移除 - tokens 现在完全由 TokenList 组件内部管理

const showPluginHomeDialog = ref(false);
const showPluginModal = ref(false);

// Editor reset modal
const showEditorResetModal = ref(false);

// Session help modal
const showSessionHelpModal = ref(false);

// Import tokens dialog

// Force sync confirmation dialog
const showForceConfirm = ref(false);
const forceConfirmType = ref(""); // 'upload' or 'download'
const forceConfirmTitle = ref("");
const forceConfirmMessage = ref("");

// Appreciation modal
const showAppreciationModal = ref(false);

// Community modal
const showCommunityModal = ref(false);

// Computed properties

const canGetToken = computed(() => {
  return authUrl.value && authCode.value.trim().length > 0;
});

// 标题状态徽章：统一通过 headerCounts 来驱动，确保任何变更后即时同步
const headerCounts = ref({ active: 0, banned: 0, invalid: 0, expired: 0, unknown: 0 });
const computeHeaderCounts = (arr) => {
  // 防御性检查：确保 arr 是数组
  if (!Array.isArray(arr)) {
    console.warn('[App] computeHeaderCounts: arr 不是数组:', arr);
    return { active: 0, banned: 0, invalid: 0, expired: 0, unknown: 0 };
  }

  let active = 0,
    banned = 0,
    invalid = 0,
    expired = 0,
    unknown = 0;
  for (const t of arr) {
    if (t.ban_status === "ACTIVE") active++;
    else if (t.ban_status === "SUSPENDED") banned++;
    else if (t.ban_status === "INVALID_TOKEN") invalid++;
    else if (t.ban_status === "EXPIRED") expired++;
    else unknown++;
  }
  return { active, banned, invalid, expired, unknown };
};
const recalcHeaderCounts = () => {
  // 调用 TokenList 组件的 getStatusCounts 方法
  if (tokenListRef.value && tokenListRef.value.getStatusCounts) {
    headerCounts.value = tokenListRef.value.getStatusCounts();
  } else {
    // 如果 TokenList 不可用，重置为 0
    headerCounts.value = { active: 0, banned: 0, invalid: 0, expired: 0, unknown: 0 };
  }
};

// 监听 TokenList 的 tokens 变化，自动更新状态统计
watch(
  () => tokenListRef.value?.tokens?.value,
  () => {
    // 直接调用 recalcHeaderCounts 方法，和刷新按钮一样
    recalcHeaderCounts();
  },
  { deep: true, immediate: true }
);

// 提供与模板兼容的计算属性
const savedTokensCount = computed(() => headerCounts.value.active);
const bannedTokensCount = computed(() => headerCounts.value.banned);
const invalidTokensCount = computed(() => headerCounts.value.invalid);
const expiredTokensCount = computed(() => headerCounts.value.expired);
const uncheckedTokensCount = computed(() => headerCounts.value.unknown);
// "其他"状态 = 过期 + Token失效 + 未知
const otherTokensCount = computed(() => {
  return headerCounts.value.expired + headerCounts.value.invalid + headerCounts.value.unknown;
});
// "全部" = 所有状态的总和
const totalTokensCount = computed(() => {
  return headerCounts.value.active + headerCounts.value.banned + headerCounts.value.expired + headerCounts.value.invalid + headerCounts.value.unknown;
});

// 状态筛选方法
const selectStatusFilter = (status) => {
  // 如果点击当前已选中的状态，则取消选择（回到"全部"）
  if (selectedStatusFilter.value === status) {
    selectedStatusFilter.value = null;
  } else {
    selectedStatusFilter.value = status;
  }

  // 将筛选状态传递给 TokenList
  if (tokenListRef.value && tokenListRef.value.setStatusFilter) {
    tokenListRef.value.setStatusFilter(selectedStatusFilter.value);
  }
};

// WebDAV相关计算属性
const canTestConnection = computed(() => {
  return (
    webdavConfig.value.serverUrl.trim() &&
    webdavConfig.value.username.trim() &&
    webdavConfig.value.password.trim()
  );
});

const canSaveConfig = computed(() => {
  return canTestConnection.value;
});

// WebDAV连接状态徽章
const getConnectionStatusClass = computed(() => {
  switch (webdavConnectionState.value) {
    case "connected":
      return "status-connected";
    case "error":
      return "status-error";
    default:
      return "status-disconnected";
  }
});

const getConnectionStatusText = computed(() => {
  switch (webdavConnectionState.value) {
    case "connected":
      return "正常";
    case "error":
      return "异常";
    default:
      return "未连接";
  }
});

// 获取当前选中的平台预设
const getCurrentPreset = computed(() => {
  return webdavPresets[selectedPreset.value] || webdavPresets.custom;
});

// 统一状态指示器相关计算属性
const shouldShowUnifiedStatus = computed(() => {
  return (
    connectionStatus.value ||
    retryProgress.value ||
    syncStatus.value ||
    currentSyncStatus.value.isActive
  );
});

const getUnifiedStatusDotClass = computed(() => {
  // 优先级：当前同步状态 > 重试进度 > 连接状态 > 同步状态
  if (currentSyncStatus.value.isActive) {
    return "active"; // 蓝色，表示进行中
  }
  if (retryProgress.value) {
    return "warning"; // 黄色，表示重试中
  }
  if (connectionStatus.value) {
    return connectionStatus.value.type === "success" ? "success" : "error";
  }
  if (syncStatus.value) {
    return syncStatus.value.type === "success" ? "success" : "error";
  }
  return "idle"; // 默认灰色
});

const getUnifiedStatusMessage = computed(() => {
  // 优先级：当前同步状态 > 重试进度 > 连接状态 > 同步状态
  if (currentSyncStatus.value.isActive) {
    return currentSyncStatus.value.message;
  }
  if (retryProgress.value) {
    return retryProgress.value.statusMessage;
  }
  if (connectionStatus.value) {
    return connectionStatus.value.message;
  }
  if (syncStatus.value) {
    return syncStatus.value.message;
  }
  return "就绪";
});

const shouldShowProgress = computed(() => {
  return retryProgress.value || currentSyncStatus.value.isActive;
});

const getProgressPercentage = computed(() => {
  if (retryProgress.value) {
    return retryProgress.value.progressPercentage || 0;
  }
  if (currentSyncStatus.value.isActive && currentSyncStatus.value.startTime) {
    // 根据时间计算一个伪进度（最多90%）
    const elapsed = Date.now() - currentSyncStatus.value.startTime;
    const progress = Math.min(90, (elapsed / 10000) * 100); // 10秒内达到90%
    return progress;
  }
  return 0;
});

const getProgressText = computed(() => {
  if (retryProgress.value) {
    return `重试中 ${retryProgress.value.attempt}/${retryProgress.value.maxAttempts}`;
  }
  if (currentSyncStatus.value.isActive && currentSyncStatus.value.startTime) {
    const elapsed = Math.floor(
      (Date.now() - currentSyncStatus.value.startTime) / 1000
    );
    return `已用时 ${elapsed}s`;
  }
  return "";
});

// Methods - 改进的多条状态提示系统
// 操作状态显示方法（覆盖式）
const showOperationStatus = (
  message,
  type = "info",
  showProgress = false,
  progress = 0,
  progressText = ""
) => {
  currentOperationStatus.value = {
    visible: true,
    type,
    message,
    showProgress,
    progress,
    progressText,
  };

  // 自动隐藏（除非是正在进行的操作）
  if (type !== "info" || !showProgress) {
    setTimeout(() => {
      if (
        currentOperationStatus.value.type === type &&
        currentOperationStatus.value.message === message
      ) {
        currentOperationStatus.value.visible = false;
      }
    }, 3000);
  }
};

const hideOperationStatus = () => {
  currentOperationStatus.value.visible = false;
};

// Session 导入方法（统一使用 TokenList 的 addToken）
const importFromSession = async () => {
  const trimmedSession = sessionInput.value.trim();

  if (!trimmedSession) {
    showStatus("请输入 Session", "error");
    return;
  }

  isImportingSession.value = true;
  sessionImportProgress.value = "正在提取 access token...";

  try {
    const result = await invoke("add_token_from_session", {
      session: trimmedSession,
    });

    console.log("Session 导入返回结果:", result);

    if (result && result.access_token) {
      sessionImportProgress.value = "正在保存账号...";

      // 创建 tokenData（使用驼峰命名）
      const tokenData = {
        tenantUrl: result.tenant_url,
        accessToken: result.access_token,
        portalUrl: null,  // Session 导入不再获取 portal_url
        emailNote: result.email || null,  // 从 get-models API 获取的邮箱
        authSession: trimmedSession,  // 保存 auth_session
        suspensions: null,  // Session 导入不再获取 suspensions
        creditsBalance: null,  // Session 导入不再获取余额
        expiryDate: null,  // Session 导入不再获取过期时间
        banStatus: 'ACTIVE'  // Session 导入默认设置为 ACTIVE 状态
      };

      console.log("准备添加的 tokenData:", tokenData);

      // 通过 TokenList 添加 token
      if (tokenListRef.value && tokenListRef.value.addToken) {
        const addResult = tokenListRef.value.addToken(tokenData);

        if (addResult.success) {
          // 添加成功，保存到文件
          try {
            await saveTokensToFile(false);
            sessionImportProgress.value = "Session 导入成功！";
            showStatus("Session 导入成功！", "success");
            sessionInput.value = "";
          } catch (error) {
            showStatus("自动保存失败，请手动保存", "warning");
          }
        } else if (addResult.duplicateId) {
          // 添加失败（重复邮箱），高亮并滚动到重复的 token
          sessionImportProgress.value = "";
          tokenListRef.value.highlightAndScrollTo(addResult.duplicateId);
          sessionInput.value = "";
        }
      } else {
        showStatus("TokenList 组件未就绪", "error");
      }
    } else {
      showStatus("导入失败：无效的 Session", "error");
    }
  } catch (error) {
    let errorMessage = error;
    // 映射后端错误标识符到用户友好的提示信息
    if (error.includes("SESSION_ERROR_OR_ACCOUNT_BANNED")) {
      errorMessage = "Session 无效或账号已被封禁";
    }
    showStatus(`导入失败: ${errorMessage}`, "error");
  } finally {
    isImportingSession.value = false;
    sessionImportProgress.value = "";
  }
};

const showStatus = (message, type = "info", duration = 4000) => {
  const id = ++statusIdCounter;
  const statusItem = {
    id,
    message,
    type,
    isVisible: true,
    isRemoving: false,
  };

  // 提示时间和条数 如果已经有5条，立即移除最早的（第一条）
  if (statusMessages.value.length >= 5) {
    const oldestMessage = statusMessages.value[0];
    // 立即开始移除动画，不等待
    removeStatusMessageImmediately(oldestMessage.id);
  }

  // 添加新提示到队列末尾
  statusMessages.value.push(statusItem);

  // 如果 duration > 0，则自动消失；如果 duration = 0，则不自动消失
  if (duration > 0) {
    setTimeout(() => {
      removeStatusMessage(id);
    }, duration);
  }

  // 返回 id，以便手动控制消失
  return id;
};

const removeStatusMessage = (id) => {
  const messageIndex = statusMessages.value.findIndex((msg) => msg.id === id);
  if (messageIndex !== -1) {
    const message = statusMessages.value[messageIndex];
    message.isRemoving = true;

    // 淡出动画完成后从数组中移除
    setTimeout(() => {
      const currentIndex = statusMessages.value.findIndex(
        (msg) => msg.id === id
      );
      if (currentIndex !== -1) {
        statusMessages.value.splice(currentIndex, 1);
      }
    }, 300); // 与CSS动画时长匹配
  }
};

// 立即移除消息（新消息顶掉的情况）
const removeStatusMessageImmediately = (id) => {
  const messageIndex = statusMessages.value.findIndex((msg) => msg.id === id);
  if (messageIndex !== -1) {
    const message = statusMessages.value[messageIndex];
    message.isRemoving = true;

    // 立即开始淡出动画
    setTimeout(() => {
      const currentIndex = statusMessages.value.findIndex(
        (msg) => msg.id === id
      );
      if (currentIndex !== -1) {
        statusMessages.value.splice(currentIndex, 1);
      }
    }, 300); // 与CSS动画时长匹配
  }
};

// 邮箱管理按钮处理方法
const handleAddAccount = () => {
  if (outlookManagerRef.value && outlookManagerRef.value.openAddAccountModal) {
    outlookManagerRef.value.openAddAccountModal();
  }
};

const handleRefreshAllStatuses = async () => {
  if (outlookManagerRef.value && outlookManagerRef.value.refreshAllStatuses) {
    isRefreshingStatuses.value = true;
    try {
      await outlookManagerRef.value.refreshAllStatuses();
    } finally {
      isRefreshingStatuses.value = false;
    }
  }
};

// 更新邮箱账户数量
const updateOutlookAccountsCount = (count) => {
  outlookAccountsCount.value = count;
};

// 更新邮箱账户状态统计
const updateOutlookAccountsStatus = (statusData) => {
  outlookAccountsStatus.value = statusData;
};

// 计算主要显示的状态信息
const primaryAccountStatus = computed(() => {
  const status = outlookAccountsStatus.value;

  if (status.total === 0) {
    return { type: "empty", count: 0, label: "未知" };
  }

  // 优先显示问题状态
  if (status.error > 0) {
    return { type: "error", count: status.error, label: "异常" };
  }

  if (status.inactive > 0) {
    return { type: "inactive", count: status.inactive, label: "失效" };
  }

  if (status.unknown > 0) {
    return { type: "unknown", count: status.unknown, label: "未知" };
  }

  // 最后显示正常状态
  if (status.active > 0) {
    return { type: "active", count: status.active, label: "正常" };
  }

  return { type: "unknown", count: status.total, label: "未检测" };
});

// 智能刷新：只刷新当前页（与 augment-token-mng-1.3.3 保持一致）
const smartRefresh = async () => {
  // 检查 tokens 是否存在且有数据
  if (!tokenListRef.value || !tokenListRef.value.tokens || tokenListRef.value.tokens.length === 0) {
    // 如果没有数据，执行完整加载
    await loadTokens(false);
    return;
  }

  // 显示持久化的加载提示（duration = 0 表示不自动消失）
  const loadingStatusId = showStatus("正在刷新账号状态和Portal信息...", "info", 0);

  try {
    // 先重新加载最新的 tokens
    await loadTokens(false);
    await nextTick();

    // 调用 TokenList 的当前页检查方法
    if (tokenListRef.value && tokenListRef.value.checkPageAccountStatus) {
      const result = await tokenListRef.value.checkPageAccountStatus();

      // 移除加载提示
      removeStatusMessage(loadingStatusId);

      // TokenList 内部已经保存了，这里只需要更新统计
      recalcHeaderCounts();

      // 显示详细统计信息
      if (result && result.stats) {
        const { total, success, failed, suspended, expired } = result.stats;

        // 构建提示消息
        let message = `刷新完成：成功 ${success}/${total}`;
        const details = [];

        if (failed > 0) {
          details.push(`失败 ${failed}`);
        }
        if (suspended > 0) {
          details.push(`已封禁 ${suspended}`);
        }
        if (expired > 0) {
          details.push(`已过期 ${expired}`);
        }

        if (details.length > 0) {
          message += `，${details.join('，')}`;
        }

        // 如果有封禁或过期的账号，添加自动禁用提示
        if (suspended > 0 || expired > 0) {
          message += '（已自动禁用检测）';
        }

        const statusType = failed > 0 ? "warning" : "success";
        showStatus(message, statusType);
      } else {
        showStatus("刷新完成", "success");
      }
    } else {
      // 移除加载提示
      removeStatusMessage(loadingStatusId);
      console.error("TokenList 组件未准备好或没有 checkPageAccountStatus 方法");
      showStatus("刷新失败：组件未准备好", "error");
    }
  } catch (error) {
    // 移除加载提示
    removeStatusMessage(loadingStatusId);
    showStatus("刷新失败", "error");
    console.error("智能刷新错误:", error);
  }
};

// loadTokens 现在由 TokenList 内部管理，这里保留一个代理函数
const loadTokens = async (showSuccessMessage = false) => {
  if (tokenListRef.value && tokenListRef.value.loadTokens) {
    await tokenListRef.value.loadTokens(showSuccessMessage);
    recalcHeaderCounts();
  }
};

// saveTokensToFile 现在由 TokenList 内部管理，这里保留一个代理函数
const saveTokensToFile = async (showSuccessMessage = true) => {
  if (tokenListRef.value && tokenListRef.value.saveTokens) {
    await tokenListRef.value.saveTokens(showSuccessMessage);
    recalcHeaderCounts();
  }
};

// 删除功能已移至 TokenList 组件内部处理

const onTokenSaved = () => {
  loadTokens();
  showStatus("新Token已保存!", "success");
};

// Token generator methods
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch (error) {
    console.error("Failed to copy to clipboard:", error);
    return false;
  }
};

const generateAuthUrl = async () => {
  isGenerating.value = true;

  // 先清空输入框
  authUrl.value = "";

  try {
    const url = await invoke("generate_augment_auth_url");

    // 赋值新的URL
    authUrl.value = url;

    // 自动复制到剪贴板
    const copySuccess = await copyToClipboard(url);
    if (copySuccess) {
      showStatus("授权URL已生成并复制到剪贴板", "success");
    } else {
      showStatus("授权URL生成成功，但复制到剪贴板失败", "warning");
    }
  } catch (error) {
    showStatus(`错误: ${error}`, "error");
  } finally {
    isGenerating.value = false;
  }
};

const copyAuthUrl = async () => {
  const success = await copyToClipboard(authUrl.value);
  showStatus(
    success ? "授权URL已复制到剪贴板!" : "复制URL失败",
    success ? "success" : "error"
  );
};

const resetAuthUrl = () => {
  authUrl.value = "";
  showStatus("已重置授权URL", "info");
};

const getAccessToken = async () => {
  isGettingToken.value = true;

  try {
    const result = await invoke("get_augment_token", { code: authCode.value });
    tokenResult.value = result;
    showStatus("访问令牌获取成功!", "success");
  } catch (error) {
    showStatus(`错误: ${error}`, "error");
  } finally {
    isGettingToken.value = false;
  }
};

const copyAccessToken = async () => {
  const success = await copyToClipboard(tokenResult.value.access_token);
  showStatus(
    success ? "访问令牌已复制到剪贴板!" : "复制访问令牌失败",
    success ? "success" : "error"
  );
};

const copyAccountUrl = async () => {
  const success = await copyToClipboard("https://app.augmentcode.com/account");
  showStatus(
    success ? "后台URL已复制到剪贴板!" : "复制URL失败",
    success ? "success" : "error"
  );
};

const copyTenantUrl = async () => {
  const success = await copyToClipboard(tokenResult.value.tenant_url);
  showStatus(
    success ? "租户URL已复制到剪贴板!" : "复制租户URL失败",
    success ? "success" : "error"
  );
};

// 复制登录URL
const copyLoginUrl = async () => {
  const success = await copyToClipboard('https://login.augmentcode.com');
  showStatus(
    success ? "登录URL已复制到剪贴板!" : "复制URL失败",
    success ? "success" : "error"
  );
};

// 复制认证URL
const copyAuthPageUrl = async () => {
  const success = await copyToClipboard('https://auth.augmentcode.com');
  showStatus(
    success ? "认证URL已复制到剪贴板!" : "复制URL失败",
    success ? "success" : "error"
  );
};

// 从Portal URL中提取token的辅助函数
const extractTokenFromPortalUrl = (portalUrl) => {
  try {
    const url = new URL(portalUrl);
    return url.searchParams.get("token");
  } catch {
    return null;
  }
};

// 新的保存函数：自动获取访问令牌然后保存
const saveTokenWithAutoFetch = async () => {
  try {
    // 首先自动获取访问令牌
    isGettingToken.value = true;

    const result = await invoke("get_augment_token", { code: authCode.value });
    tokenResult.value = result;
    showStatus("访问令牌获取成功!", "success");

    // 尝试自动获取邮箱信息
    let autoEmailNote = emailNote.value.trim() || null;
    if (portalUrl.value.trim()) {
      try {
        const token = extractTokenFromPortalUrl(portalUrl.value.trim());
        if (token) {
          const subscriptionData = await invoke("get_subscriptions_from_link", {
            token,
          });
          const subscriptionInfo = JSON.parse(subscriptionData);

          if (subscriptionInfo.data && subscriptionInfo.data.length > 0) {
            const customerEmail = subscriptionInfo.data[0].customer?.email;
            if (customerEmail) {
              autoEmailNote = customerEmail;
              showStatus(`自动获取到邮箱: ${customerEmail}`, "success");
            }
          }
        }
      } catch (error) {
        console.warn("自动获取邮箱失败:", error);
        showStatus("自动获取邮箱失败，使用手动输入的邮箱", "warning");
      }
    }

    // 准备新账号数据（使用驼峰命名）
    const tokenData = {
      tenantUrl: tokenResult.value.tenant_url,
      accessToken: tokenResult.value.access_token,
      portalUrl: portalUrl.value.trim() || null,
      emailNote: autoEmailNote,
      creditsBalance: tokenResult.value.credits_balance || null,
      expiryDate: tokenResult.value.expiry_date || null,
    };

    // 使用 TokenList 的 addToken 函数
    if (tokenListRef.value && tokenListRef.value.addToken) {
      const result = tokenListRef.value.addToken(tokenData);

      if (!result.success && result.duplicateId) {
        // 重复邮箱，高亮并滚动到重复的 token
        tokenListRef.value.highlightAndScrollTo(result.duplicateId);

        // 重置表单
        authUrl.value = "";
        authCode.value = "";
        tokenResult.value = null;
        portalUrl.value = "";
        emailNote.value = "";
        return;
      }

      // 添加成功，保存到文件
      if (result.success && result.token) {
        try {
          await saveTokensToFile(false);
          showStatus("账号已成功保存!", "success");
        } catch (error) {
          showStatus("自动保存失败，请手动保存", "warning");
        }

        // 重置表单
        authUrl.value = "";
        authCode.value = "";
        tokenResult.value = null;
        portalUrl.value = "";
        emailNote.value = "";
      }
    } else {
      showStatus("TokenList 组件未就绪", "error");
    }
  } catch (error) {
    console.error("保存账号失败:", error);
    showStatus(`保存账号失败: ${error}`, "error");
  } finally {
    isGettingToken.value = false;
  }
};

// Token form methods - 已移至 TokenList 组件内部处理

// 处理token状态更新（如账号封禁状态检测后、标签修改等）
const handleTokenUpdated = async (updatedToken) => {
  // TokenList 内部已经管理 tokens，这里只需要触发保存
  await saveTokensToFile(false);
  recalcHeaderCounts();
};

// 处理侧边栏导航点击
const handleNavClick = (view) => {
  currentView.value = view;

  // 关闭所有弹窗
  showPluginModal.value = false;
  showPluginHomeDialog.value = false;
  showForceConfirm.value = false;
  showThresholdConfigModal.value = false;
  showApiServerModal.value = false;
  showEditorResetModal.value = false;
  showPlatformSelector.value = false;
  showSyncHistoryModal.value = false;
  showAppreciationModal.value = false;
  showCommunityModal.value = false;
  showSessionHelpModal.value = false;
  showImportListModal.value = false;

  // 关闭 TokenList 组件的所有弹窗（包括 TokenForm、AccountManagerModal、单个删除确认对话框、批量删除对话框、TokenCard 弹窗等）
  if (tokenListRef.value && tokenListRef.value.closeAllModals) {
    tokenListRef.value.closeAllModals();
  }

  // 关闭批量删除对话框（兼容旧代码）
  if (tokenListRef.value && tokenListRef.value.closeBatchDeleteDialog) {
    tokenListRef.value.closeBatchDeleteDialog();
  }

  // 关闭 OutlookManager 组件的添加账户弹窗
  if (outlookManagerRef.value && outlookManagerRef.value.closeAddAccountModal) {
    outlookManagerRef.value.closeAddAccountModal();
  }

  // 关闭更新检查弹窗
  if (updateChecker.value) {
    updateChecker.value.closeUpdateModal();
  }
};

// 处理编辑器重置弹窗关闭
const handleEditorResetModalClose = () => {
  showEditorResetModal.value = false;
};

// 打开购买链接
const openPurchaseLink = async () => {
  try {
    await invoke("open_url", { url: "https://pay.ldxp.cn/shop/V6VSA2G8" });
  } catch (error) {
    console.error("打开购买链接失败:", error);
    showStatus("打开购买链接失败", "error");
  }
};

// 处理编辑器重置完成
const handleEditorResetComplete = (result) => {
  if (result.success) {
    // 显示详细的步骤结果
    if (result.details && result.details.length > 0) {
      // 显示每个步骤的结果
      result.details.forEach((detail, index) => {
        setTimeout(() => {
          // 根据步骤结果的类型选择不同的状态
          if (detail.includes("⚠️")) {
            showStatus(detail, "warning");
          } else if (detail.includes("✅")) {
            showStatus(detail, "success");
          } else {
            showStatus(detail, "info");
          }
        }, index * 1200); // 每个步骤间隔1.2秒显示
      });

      // 显示总结消息
      setTimeout(() => {
        if (result.hasWarnings) {
          showStatus(
            `${result.editor} 重置完成，但有部分警告。如果编辑器仍有问题，请手动关闭编辑器后重试。`,
            "warning"
          );
        } else {
          showStatus(`${result.editor} 重置成功完成！`, "success");
        }
      }, result.details.length * 1200 + 500);
    } else {
      // 如果没有详细信息，显示简单成功消息
      if (result.hasWarnings) {
        showStatus(`${result.editor} 重置完成，但有警告`, "warning");
      } else {
        showStatus(`${result.editor} 重置成功！`, "success");
      }
    }
  } else {
    showStatus(`${result.editor} 重置失败: ${result.error}`, "error");

    // 如果有详细信息，也显示出来
    if (result.details && result.details.length > 0) {
      result.details.forEach((detail, index) => {
        setTimeout(() => {
          showStatus(detail, "error");
        }, (index + 1) * 1000);
      });
    }
  }
};

// 检测单个账号状态
const checkSingleAccountStatus = async (tokenId) => {
  if (!tokenListRef.value || !tokenListRef.value.tokens || !tokenListRef.value.tokens.value) {
    console.warn("TokenList 组件未就绪");
    return;
  }

  const token = tokenListRef.value.tokens.value.find((t) => t.id === tokenId);
  if (!token) {
    console.warn(`未找到ID为 ${tokenId} 的账号`);
    return;
  }

  try {
    // 如果没有邮箱信息且有portal_url，先尝试获取邮箱
    let emailUpdated = false;
    if (!token.email_note && token.portal_url) {
      try {
        const portalToken = extractTokenFromPortalUrl(token.portal_url);
        if (portalToken) {
          const subscriptionData = await invoke("get_subscriptions_from_link", {
            token: portalToken,
          });
          const subscriptionInfo = JSON.parse(subscriptionData);

          if (subscriptionInfo.data && subscriptionInfo.data.length > 0) {
            const customerEmail = subscriptionInfo.data[0].customer?.email;
            if (customerEmail) {
              token.email_note = customerEmail;
              emailUpdated = true;
              showStatus(`自动获取到邮箱: ${customerEmail}`, "success");
            }
          }
        }
      } catch (error) {
        console.warn("自动获取邮箱失败:", error);
      }
    }

    const result = await invoke("check_account_status", {
      token: token.access_token,
      tenantUrl: token.tenant_url,
      authSession: token.auth_session || null,
      tokenId: token.id || null,
    });

    const banStatus =
      result.status || (result.is_banned ? "SUSPENDED" : "ACTIVE");

    // 检查是否有任何变化
    const accessTokenChanged = token.access_token !== result.access_token;
    const tenantUrlChanged = token.tenant_url !== result.tenant_url;
    const banStatusChanged = token.ban_status !== banStatus;
    const hasAnyChanges = accessTokenChanged || tenantUrlChanged || banStatusChanged || emailUpdated;

    // 如果有任何变化，更新token
    if (hasAnyChanges) {
      token.access_token = result.access_token;
      token.tenant_url = result.tenant_url;
      token.ban_status = banStatus;
      token.updated_at = new Date().toISOString();

      // 静默保存更新
      const index = tokenListRef.value.tokens.value.findIndex((t) => t.id === token.id);
      if (index !== -1) {
        tokenListRef.value.tokens.value[index] = { ...token };

        // 自动保存到文件
        try {
          await saveTokensToFile(false);
        } catch (error) {
          console.warn("自动保存状态更新失败:", error);
        }

        // 强制更新状态统计（确保及时同步）
        recalcHeaderCounts();
      }
    }

    const statusText =
      banStatus === "SUSPENDED"
        ? "已封禁"
        : banStatus === "INVALID_TOKEN"
        ? "Token失效"
        : banStatus === "EXPIRED"
        ? "已过期"
        : "正常";
    const emailText = emailUpdated ? "，已获取邮箱信息" : "";
    showStatus(
      `账号状态检测完成: ${statusText}${emailText}`,
      banStatus === "SUSPENDED" ||
        banStatus === "INVALID_TOKEN" ||
        banStatus === "EXPIRED"
        ? "error"
        : "success"
    );

    return { success: true, banned: result.is_banned, status: banStatus };
  } catch (error) {
    console.error(`检测账号 ${token.tenant_url} 状态失败:`, error);
    showStatus(`账号状态检测失败: ${error}`, "error");
    // 即使出错也要更新统计
    recalcHeaderCounts();
    return { success: false, error };
  }
};

// 批量检测账号状态（用于导入后的状态检测，统一显示结果）
const checkBatchAccountStatus = async (tokenIds) => {
  if (!tokenIds || tokenIds.length === 0) return;

  let successCount = 0;
  let bannedCount = 0;
  let expiredCount = 0;
  let invalidCount = 0;
  let failedCount = 0;

  // 并行检测所有账号状态
  const promises = tokenIds.map(async (tokenId) => {
    try {
      const result = await checkSingleAccountStatusSilent(tokenId);
      if (result.success) {
        successCount++;
        if (result.status === "SUSPENDED") {
          bannedCount++;
        } else if (result.status === "EXPIRED") {
          expiredCount++;
        } else if (result.status === "INVALID_TOKEN") {
          invalidCount++;
        }
      } else {
        failedCount++;
      }
      return result;
    } catch (error) {
      failedCount++;
      return { success: false, error };
    }
  });

  await Promise.allSettled(promises);

  // 统一显示检测结果
  const totalCount = tokenIds.length;
  const normalCount = successCount - bannedCount - expiredCount - invalidCount;

  let statusParts = [];
  if (normalCount > 0) statusParts.push(`${normalCount} 个正常`);
  if (bannedCount > 0) statusParts.push(`${bannedCount} 个已封禁`);
  if (expiredCount > 0) statusParts.push(`${expiredCount} 个已过期`);
  if (invalidCount > 0) statusParts.push(`${invalidCount} 个Token失效`);
  if (failedCount > 0) statusParts.push(`${failedCount} 个检测失败`);

  const statusText = statusParts.length > 0 ? statusParts.join("，") : "无结果";
  const messageType =
    bannedCount + expiredCount + invalidCount + failedCount > 0
      ? "warning"
      : "success";

  showStatus(`导入账号状态检测完成：${statusText}`, messageType);
};

// 静默检测单个账号状态（不显示提示消息）
const checkSingleAccountStatusSilent = async (tokenId) => {
  if (!tokenListRef.value || !tokenListRef.value.tokens || !tokenListRef.value.tokens.value) {
    console.warn("TokenList 组件未就绪");
    return { success: false, error: "TokenList 组件未就绪" };
  }

  const token = tokenListRef.value.tokens.value.find((t) => t.id === tokenId);
  if (!token) {
    console.warn(`未找到ID为 ${tokenId} 的账号`);
    return { success: false, error: "账号不存在" };
  }

  try {
    // 如果没有邮箱信息且有portal_url，先尝试获取邮箱
    let emailUpdated = false;
    if (!token.email_note && token.portal_url) {
      try {
        const portalToken = extractTokenFromPortalUrl(token.portal_url);
        if (portalToken) {
          const subscriptionData = await invoke("get_subscriptions_from_link", {
            token: portalToken,
          });
          const subscriptionInfo = JSON.parse(subscriptionData);

          if (subscriptionInfo.data && subscriptionInfo.data.length > 0) {
            const customerEmail = subscriptionInfo.data[0].customer?.email;
            if (customerEmail) {
              token.email_note = customerEmail;
              emailUpdated = true;
              console.log(`自动获取到邮箱: ${customerEmail}`);
            }
          }
        }
      } catch (error) {
        console.warn("自动获取邮箱失败:", error);
      }
    }

    const result = await invoke("check_account_status", {
      token: token.access_token,
      tenantUrl: token.tenant_url,
      authSession: token.auth_session || null,
      tokenId: token.id || null,
    });

    const banStatus =
      result.status || (result.is_banned ? "SUSPENDED" : "ACTIVE");

    // 检查是否有任何变化
    const accessTokenChanged = token.access_token !== result.access_token;
    const tenantUrlChanged = token.tenant_url !== result.tenant_url;
    const banStatusChanged = token.ban_status !== banStatus;
    const hasAnyChanges = accessTokenChanged || tenantUrlChanged || banStatusChanged || emailUpdated;

    // 如果有任何变化，更新token
    if (hasAnyChanges) {
      token.access_token = result.access_token;
      token.tenant_url = result.tenant_url;
      token.ban_status = banStatus;
      token.updated_at = new Date().toISOString();

      // 静默保存更新
      const index = tokenListRef.value.tokens.value.findIndex((t) => t.id === token.id);
      if (index !== -1) {
        tokenListRef.value.tokens.value[index] = { ...token };

        // 自动保存到文件
        try {
          await saveTokensToFile(false);
        } catch (error) {
          console.warn("自动保存状态更新失败:", error);
        }

        // 强制更新状态统计（确保及时同步）
        recalcHeaderCounts();
      }
    }

    return {
      success: true,
      banned: result.is_banned,
      status: banStatus,
      emailUpdated,
    };
  } catch (error) {
    console.error(`检测账号 ${token.tenant_url} 状态失败:`, error);
    // 即使出错也要更新统计
    recalcHeaderCounts();
    return { success: false, error };
  }
};



// 键盘事件处理
const handleKeydown = (event) => {
  // 禁用所有 F 类功能键（F1-F12）
  if (event.key && event.key.match(/^F([1-9]|1[0-1])$/)) {
    event.preventDefault();
    event.stopPropagation();
    return false;
  }
};

// 禁用所有输入框自动填充的函数
const disableAutocompleteForAllInputs = () => {
  // 获取所有输入框、文本域和选择框
  const inputs = document.querySelectorAll("input, textarea, select");

  inputs.forEach((input) => {
    // 添加禁用自动填充的属性
    input.setAttribute("autocomplete", "off");
    input.setAttribute("autocapitalize", "off");
    input.setAttribute("autocorrect", "off");
    input.setAttribute("spellcheck", "false");

    // 对于特定类型的输入框，添加额外的属性
    if (input.type === "password") {
      input.setAttribute("autocomplete", "new-password");
    } else if (input.type === "email") {
      input.setAttribute("autocomplete", "off");
    }
  });
};

// 监听键盘事件
onMounted(() => {
  document.addEventListener("keydown", handleKeydown);

  // 屏蔽右键菜单
  document.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    return false;
  });

  // // 禁用F12开发者工具
  // document.addEventListener("keydown", (e) => {
  //   if (
  //     e.key === "F12" ||
  //     (e.ctrlKey && e.shiftKey && e.key === "I") ||
  //     (e.ctrlKey && e.shiftKey && e.key === "C") ||
  //     (e.ctrlKey && e.key === "u")
  //   ) {
  //     e.preventDefault();
  //     return false;
  //   }
  // });

  // 禁用所有输入框的自动填充
  disableAutocompleteForAllInputs();

  // 监听DOM变化，为新添加的输入框也禁用自动填充
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === "childList") {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeType === Node.ELEMENT_NODE) {
            // 检查新添加的节点是否是输入框
            if (node.matches && node.matches("input, textarea, select")) {
              const input = node;
              input.setAttribute("autocomplete", "off");
              input.setAttribute("autocapitalize", "off");
              input.setAttribute("autocorrect", "off");
              input.setAttribute("spellcheck", "false");
            }
            // 检查新添加的节点内部是否包含输入框
            const inputs =
              node.querySelectorAll &&
              node.querySelectorAll("input, textarea, select");
            if (inputs) {
              inputs.forEach((input) => {
                input.setAttribute("autocomplete", "off");
                input.setAttribute("autocapitalize", "off");
                input.setAttribute("autocorrect", "off");
                input.setAttribute("spellcheck", "false");
              });
            }
          }
        });
      }
    });
  });

  // 开始观察DOM变化
  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });
});

onUnmounted(() => {
  // 清理 Session 导入进度事件监听器
  if (unlistenSessionProgress) {
    unlistenSessionProgress();
  }

  // 清理键盘事件监听器
  document.removeEventListener("keydown", handleKeydown);

  // 清理自动上传和自动下载定时器
  stopAutoUpload();
  stopAutoDownload();

  // 清理应用关闭事件监听器
  window.removeEventListener("beforeunload", syncBeforeClose);
});

// External links dialog methods

const openVSCodePlugin = async () => {
  showPluginHomeDialog.value = false;
  const url = "https://github.com/Zheng-up/augment-code-z/releases";

  try {
    await invoke("open_url", { url });
  } catch (error) {
    console.error("Failed to open GitHub repository:", error);
    showStatus("打开 GitHub 仓库失败", "error");
  }
};

const openJetBrainsPlugin = async () => {
  showPluginHomeDialog.value = false;
  showStatus("暂无相应防封插件", "error");
};

const openBalancePluginDownload = async () => {
  const url = "https://www.123684.com/s/f1TeTd-vNvHd?pwd=A6xv";

  try {
    await invoke("open_url", { url });
    showStatus("已在浏览器中打开额度显示插件下载页面", "success");
  } catch (error) {
    console.error("Failed to open Balance Plugin:", error);
    showStatus("打开额度显示插件下载页面失败", "error");
  }
};

// Token 手动管理相关方法 - 调用 TokenList 的方法
const handleAddNewToken = () => {
  // 调用 TokenList 的 openTokenForm 方法
  if (tokenListRef.value && tokenListRef.value.openTokenForm) {
    tokenListRef.value.openTokenForm();
  }
};

// 账号管理相关方法 - 已移至 TokenList 组件内部

// handleAccountManager, handleImportTokens, handleExportTokens 等方法已移至 TokenList 组件内部

// AccountManager 和 TagEditor 相关方法已移至 TokenList 组件内部

const handleTokensImport = async (importedTokens) => {
  let successCount = 0;
  let errorCount = 0;
  let emailFetchCount = 0;
  let duplicateTokens = [];
  let newTokenIds = []; // 记录新添加的账号ID

  showStatus(`开始导入 ${importedTokens.length} 个账号...`, "info");

  // 第一阶段：处理所有账号，收集重复的账号
  for (const tokenData of importedTokens) {
    try {
      // 验证必需字段
      if (!tokenData.tenant_url || !tokenData.access_token) {
        errorCount++;
        continue;
      }

      // 尝试自动获取邮箱信息（如果有portal_url且没有email_note）
      let autoEmailNote = tokenData.email_note || null;
      if (tokenData.portal_url && !autoEmailNote) {
        try {
          const token = extractTokenFromPortalUrl(tokenData.portal_url);
          if (token) {
            const subscriptionData = await invoke(
              "get_subscriptions_from_link",
              {
                token,
              }
            );
            const subscriptionInfo = JSON.parse(subscriptionData);

            if (subscriptionInfo.data && subscriptionInfo.data.length > 0) {
              const customerEmail = subscriptionInfo.data[0].customer?.email;
              if (customerEmail) {
                autoEmailNote = customerEmail;
                emailFetchCount++;
                console.log(`自动获取到邮箱: ${customerEmail}`);
              }
            }
          }
        } catch (error) {
          console.warn("自动获取邮箱失败:", error);
          // 继续导入，只是没有邮箱信息
        }
      }

      // 准备新账号数据（使用驼峰命名）
      const tokenDataToAdd = {
        tenantUrl: tokenData.tenant_url,
        accessToken: tokenData.access_token,
        portalUrl: tokenData.portal_url || null,
        emailNote: autoEmailNote,
        authSession: tokenData.auth_session || null,
        tagName: tokenData.tag_text || null,  // 使用 tagName 而不是 tagText
        tagColor: tokenData.tag_color || null,
        suspensions: tokenData.suspensions || null,
        banStatus: tokenData.ban_status || null,
        creditsBalance: tokenData.portal_info?.credits_balance || null,
        expiryDate: tokenData.portal_info?.expiry_date || null,
      };

      // 使用 TokenList 的 addToken 函数
      if (tokenListRef.value && tokenListRef.value.addToken) {
        const addResult = tokenListRef.value.addToken(tokenDataToAdd);

        if (addResult.success) {
          newTokenIds.push(addResult.token.id); // 记录新添加的账号ID
          successCount++;
          console.log(
            `导入账号成功，标签: ${tokenDataToAdd.tagName || "无"}, 颜色: ${
              tokenDataToAdd.tagColor || "无"
            }, Session: ${tokenDataToAdd.authSession ? "有" : "无"}`
          );
        } else if (addResult.duplicateId) {
          // 重复账号，收集起来稍后处理
          duplicateTokens.push({
            duplicateId: addResult.duplicateId,
            newTokenData: tokenDataToAdd,
            originalData: tokenData,
          });
        } else {
          errorCount++;
        }
      } else {
        console.error("TokenList 组件未就绪");
        errorCount++;
      }
    } catch (error) {
      console.error("导入单个token失败:", error);
      errorCount++;
    }
  }

  // 第二阶段：处理重复账号
  if (duplicateTokens.length > 0) {
    // 关闭导入导出弹框
    showAccountManagerModal.value = false;

    // 显示重复账号提示
    const skippedCount = duplicateTokens.length;
    showStatus(
      `导入完成！成功: ${successCount}, 跳过(重复): ${skippedCount}, 失败: ${errorCount}`,
      successCount > 0 ? "success" : "warning"
    );

    // 如果只有一个重复，高亮显示
    if (duplicateTokens.length === 1 && successCount === 0) {
      tokenListRef.value?.highlightAndScrollTo(duplicateTokens[0].duplicateId);
    }
  } else {
    // 没有重复账号，直接完成导入
    await completeImport(
      successCount,
      0, // updateCount = 0
      errorCount,
      emailFetchCount,
      newTokenIds
    );
    // 关闭导入导出弹框
    showAccountManagerModal.value = false;
  }
};

// 注意：handleDuplicateTokensConfirm 函数已删除，现在使用 TokenList 的 addToken 统一处理重复检测

// 完成导入操作的辅助函数
const completeImport = async (
  successCount,
  updateCount,
  errorCount,
  emailFetchCount,
  newTokenIds = []
) => {
  const totalProcessed = successCount + updateCount;

  if (totalProcessed > 0) {
    // 强制更新状态统计（确保导入后及时同步）
    recalcHeaderCounts();

    // 自动保存到文件（带重试机制）
    let saveSuccess = false;
    let retryCount = 0;
    const maxRetries = 3;

    while (!saveSuccess && retryCount < maxRetries) {
      try {
        if (retryCount > 0) {
          console.log(`保存重试 ${retryCount}/${maxRetries}...`);
          // 重试前等待一小段时间
          await new Promise((resolve) => setTimeout(resolve, 200 * retryCount));
        }

        await saveTokensToFile(false);
        saveSuccess = true;

        const emailInfo =
          emailFetchCount > 0 ? `，自动获取到 ${emailFetchCount} 个邮箱` : "";
        const updateInfo =
          updateCount > 0 ? `，检测到重复账号已更新 ${updateCount} 个` : "";
        showStatus(
          `成功导入 ${successCount} 个账号${updateInfo}${emailInfo}${
            errorCount > 0 ? `，${errorCount} 个失败或跳过` : ""
          }`,
          "success"
        );

        // 导入完成后不自动检测账号状态（避免触发批量刷新）
        // 用户可以手动点击"批量刷新"按钮来检测状态
        console.log(`批量导入完成，成功导入 ${newTokenIds.length} 个账号，不自动触发批量刷新`);
      } catch (error) {
        retryCount++;
        console.error(`保存失败（第 ${retryCount} 次尝试）:`, error);

        if (retryCount >= maxRetries) {
          // 所有重试都失败
          console.error("保存失败，已达到最大重试次数");
          showStatus(
            `导入成功但自动保存失败（已重试${maxRetries}次），请手动保存。成功：${successCount}，更新：${updateCount}，失败：${errorCount}`,
            "warning"
          );
        }
      }
    }
  }

  // 关闭导入弹窗
  showAccountManagerModal.value = false;
};

const handleImportError = (errorMessage) => {
  showStatus(errorMessage, "error");
};

// Initialize
// 设置页面相关方法
const selectDataDirectory = async () => {
  try {
    isSelectingPath.value = true;
    showStatus("正在选择数据存储目录...", "info");

    // 调用Tauri API选择目录
    const selectedPath = await invoke("select_data_directory");
    if (selectedPath) {
      dataDirectory.value = selectedPath;
      showStatus(`数据目录已设置为: ${selectedPath}`, "success");

      // 刷新当前路径显示
      await getCurrentDataPath();
    }
  } catch (error) {
    if (error.toString().includes("用户取消了目录选择")) {
      showStatus("已取消目录选择", "info");
    } else {
      showStatus(`选择目录失败: ${error}`, "error");
    }
  } finally {
    isSelectingPath.value = false;
  }
};

const getCurrentVersion = async () => {
  try {
    // 获取当前应用版本信息
    const versionInfo = await invoke("get_app_version");
    currentVersion.value = `v${versionInfo.current_version}`;
  } catch (error) {
    console.error("获取版本信息失败:", error);
    currentVersion.value = "v0.1.7"; // 降级显示
  }
};

const getCurrentDataPath = async () => {
  try {
    // 获取当前数据路径
    const currentPath = await invoke("get_current_data_path");
    currentTokensPath.value = currentPath;

    // 获取数据目录
    const dataDir = await invoke("get_data_directory");
    dataDirectory.value = dataDir;
  } catch (error) {
    console.error("获取当前数据路径失败:", error);
    currentTokensPath.value = "获取路径失败";
    dataDirectory.value = "未设置";
  }
};

const openDataDirectory = async () => {
  try {
    showStatus("正在打开数据目录...", "info");

    // 调用后端打开目录函数
    await invoke("open_data_directory");

    showStatus("数据目录已在文件管理器中打开", "success");
  } catch (error) {
    showStatus(`打开数据目录失败: ${error}`, "error");
  }
};

// WebDAV相关方法
const testWebDAVConnection = async () => {
  if (!canTestConnection.value) {
    showStatus("请填写完整的WebDAV配置信息", "error");
    return;
  }

  isTestingConnection.value = true;
  // 显示左下角状态提示
  showStatus("正在测试WebDAV连接...", "info");

  const startTime = Date.now();

  try {
    // 先配置WebDAV
    await invoke("configure_webdav", {
      serverUrl: webdavConfig.value.serverUrl,
      username: webdavConfig.value.username,
      password: webdavConfig.value.password,
    });

    // 测试连接
    const isConnected = await invoke("test_webdav_connection");
    const duration = Date.now() - startTime;

    if (isConnected) {
      // 更新持久连接状态
      webdavConnectionState.value = "connected";
      // 显示左下角成功提示
      showStatus("WebDAV连接测试成功", "success");
      isWebDAVConfigured.value = true;
      addSyncHistoryEntry("test", "success", "WebDAV连接测试成功", duration);
    } else {
      // 更新持久连接状态
      webdavConnectionState.value = "error";
      // 显示左下角错误提示
      showStatus("WebDAV连接测试失败，请检查配置信息", "error");
      addSyncHistoryEntry(
        "test",
        "error",
        "连接失败，请检查配置信息",
        duration
      );
    }
  } catch (error) {
    const duration = Date.now() - startTime;
    // 更新持久连接状态
    webdavConnectionState.value = "error";
    // 显示左下角错误提示
    showStatus(`WebDAV连接错误: ${error}`, "error");
    addSyncHistoryEntry("test", "error", `连接错误: ${error}`, duration);
  } finally {
    isTestingConnection.value = false;
  }
};

const saveWebDAVConfig = async () => {
  if (!canSaveConfig.value) {
    showStatus("请填写完整的WebDAV配置信息", "error");
    return;
  }

  console.log("开始保存WebDAV配置...");
  console.log("要保存的配置:", {
    platform: selectedPreset.value,
    serverUrl: webdavConfig.value.serverUrl,
    username: webdavConfig.value.username,
    passwordLength: webdavConfig.value.password.length,
  });

  isSavingConfig.value = true;

  try {
    // 保存当前平台配置到localStorage
    localStorage.setItem("webdav_configs", JSON.stringify(webdavConfigs.value));
    localStorage.setItem("selected_webdav_preset", selectedPreset.value);

    // 同时保存当前配置到后端（为了兼容现有的后端逻辑）
    const result = await invoke("configure_webdav", {
      serverUrl: webdavConfig.value.serverUrl,
      username: webdavConfig.value.username,
      password: webdavConfig.value.password,
    });

    console.log("WebDAV配置保存结果:", result);

    // 验证配置是否完整
    const savedCorrectly =
      webdavConfig.value.serverUrl.trim() !== "" &&
      webdavConfig.value.username.trim() !== "" &&
      webdavConfig.value.password.trim() !== "";

    if (savedCorrectly) {
      isWebDAVConfigured.value = true;
      showStatus(`${getCurrentPreset.value.name} 配置保存成功`, "success");
      console.log("配置保存验证成功");

      // 自动测试连接
      setTimeout(() => {
        testWebDAVConnection();
      }, 500);
    } else {
      showStatus("配置保存验证失败，请重试", "warning");
      console.warn("配置保存验证失败");
    }
  } catch (error) {
    console.error("保存WebDAV配置失败:", error);
    showStatus(`保存WebDAV配置失败: ${error}`, "error");
  } finally {
    isSavingConfig.value = false;
  }
};

const forceUploadToCloud = async () => {
  if (!isWebDAVConfigured.value) {
    showStatus("请先配置并测试WebDAV连接", "error");
    return;
  }

  isSyncing.value = true;
  // 显示左下角状态提示
  showStatus("正在强制上传到云端...", "info");

  const startTime = Date.now();

  try {
    const result = await invoke("force_upload_to_cloud");
    const duration = Date.now() - startTime;

    // 显示左下角成功提示
    showStatus(`强制上传成功: ${result}`, "success");
    addSyncHistoryEntry(
      "upload",
      "success",
      `强制上传成功: ${result}`,
      duration
    );
    updateSyncStatus(false);
  } catch (error) {
    const duration = Date.now() - startTime;
    syncStatus.value = {
      type: "error",
      message: `强制上传失败: ${error}`,
    };
    showStatus(`强制上传失败: ${error}`, "error");
    addSyncHistoryEntry("upload", "error", `强制上传失败: ${error}`, duration);
    updateSyncStatus(false);
  } finally {
    isSyncing.value = false;
  }
};

const forceDownloadFromCloud = async () => {
  if (!isWebDAVConfigured.value) {
    showStatus("请先配置并测试WebDAV连接", "error");
    return;
  }

  isSyncing.value = true;
  // 显示左下角状态提示
  showStatus("正在强制从云端下载...", "info");

  const startTime = Date.now();

  try {
    const result = await invoke("force_download_from_cloud");
    const duration = Date.now() - startTime;

    // 显示左下角成功提示
    showStatus(`强制下载成功: ${result}`, "success");
    addSyncHistoryEntry(
      "download",
      "success",
      `强制下载成功: ${result}`,
      duration
    );
    updateSyncStatus(false);

    // 重新加载所有数据以显示最新内容
    await loadTokens();
    await loadStatusThresholds(); // 重新加载阈值配置
  } catch (error) {
    const duration = Date.now() - startTime;
    syncStatus.value = {
      type: "error",
      message: `强制下载失败: ${error}`,
    };
    showStatus(`强制下载失败: ${error}`, "error");
    addSyncHistoryEntry(
      "download",
      "error",
      `强制下载失败: ${error}`,
      duration
    );
    updateSyncStatus(false);
  } finally {
    isSyncing.value = false;
  }
};

const loadWebDAVConfig = async () => {
  try {
    console.log("开始加载WebDAV配置...");

    // 尝试从localStorage加载多平台配置
    const savedConfigs = localStorage.getItem("webdav_configs");
    if (savedConfigs) {
      console.log("从本地存储加载配置...");
      const parsedConfigs = JSON.parse(savedConfigs);
      webdavConfigs.value = { ...webdavConfigs.value, ...parsedConfigs };

      // 加载选中的平台
      const savedPreset = localStorage.getItem("selected_webdav_preset");
      if (savedPreset && webdavPresets[savedPreset]) {
        selectedPreset.value = savedPreset;
      }

      console.log("多平台配置加载成功:", {
        selectedPreset: selectedPreset.value,
        currentConfig: webdavConfig.value,
      });
    }

    // 检查当前平台配置是否完整
    const currentConfig = webdavConfig.value;
    if (
      currentConfig.serverUrl &&
      currentConfig.username &&
      currentConfig.password
    ) {
      // 配置完整，调用后端配置WebDAV
      try {
        await invoke("configure_webdav", {
          serverUrl: currentConfig.serverUrl.trim(),
          username: currentConfig.username.trim(),
          password: currentConfig.password.trim(),
        });

        isWebDAVConfigured.value = true;
        webdavConnectionState.value = "connected";
        console.log("当前平台配置完整，已调用后端配置并标记为已配置");
      } catch (error) {
        console.error("配置WebDAV后端失败:", error);
        isWebDAVConfigured.value = false;
        webdavConnectionState.value = "error";
      }
    } else {
      isWebDAVConfigured.value = false;
      webdavConnectionState.value = "disconnected";
      console.log("当前平台配置不完整，已标记为未配置");
    }
  } catch (error) {
    console.error("加载WebDAV配置失败:", error);
    showStatus(`加载WebDAV配置失败: ${error}`, "error");

    isWebDAVConfigured.value = false;
    webdavConnectionState.value = "error";
  }
};

// 同步历史记录管理函数
const addSyncHistoryEntry = (operation, status, message, duration = null) => {
  const entry = {
    id: Date.now(),
    timestamp: new Date(),
    operation, // 'upload', 'download', 'test'
    status, // 'success', 'error', 'warning'
    message,
    duration,
  };

  syncHistory.value.unshift(entry);

  // 保持最多50条记录
  if (syncHistory.value.length > 50) {
    syncHistory.value = syncHistory.value.slice(0, 50);
  }

  // 保存到本地存储
  localStorage.setItem(
    "webdav_sync_history",
    JSON.stringify(syncHistory.value)
  );
};

const loadSyncHistory = () => {
  try {
    const saved = localStorage.getItem("webdav_sync_history");
    if (saved) {
      syncHistory.value = JSON.parse(saved).map((entry) => ({
        ...entry,
        timestamp: new Date(entry.timestamp),
      }));
    }
  } catch (error) {
    console.log("加载同步历史失败:", error);
  }
};

const clearSyncHistory = () => {
  syncHistory.value = [];
  localStorage.removeItem("webdav_sync_history");
};

const formatSyncDuration = (duration) => {
  if (!duration) return "";
  if (duration < 1000) return `${duration}ms`;
  return `${(duration / 1000).toFixed(1)}s`;
};

const updateSyncStatus = (
  isActive,
  operation = null,
  message = "",
  progress = 0
) => {
  currentSyncStatus.value.isActive = isActive;
  currentSyncStatus.value.operation = operation;
  currentSyncStatus.value.message = message;
  currentSyncStatus.value.progress = progress;

  if (isActive && !currentSyncStatus.value.startTime) {
    currentSyncStatus.value.startTime = Date.now();
  } else if (!isActive) {
    currentSyncStatus.value.startTime = null;
  }
};

// 自动上传和自动下载相关函数
let autoUploadInterval = null;
let autoDownloadInterval = null;

const startAutoUpload = () => {
  // 如果已经有定时器在运行，先清除
  if (autoUploadInterval) {
    clearInterval(autoUploadInterval);
  }

  // 立即执行一次上传
  performAutoUpload();

  // 每3分钟执行一次自动上传
  autoUploadInterval = setInterval(async () => {
    performAutoUpload();
  }, 3 * 60 * 1000); // 3分钟 = 180000毫秒
};

const stopAutoUpload = () => {
  if (autoUploadInterval) {
    clearInterval(autoUploadInterval);
    autoUploadInterval = null;
  }
};

const performAutoUpload = async () => {
  if (autoUploadEnabled.value && isWebDAVConfigured.value && !isSyncing.value) {
    try {
      await forceUploadToCloud();
      console.log("自动上传完成");
    } catch (error) {
      console.error("自动上传失败:", error);
    }
  }
};

const startAutoDownload = () => {
  // 如果已经有定时器在运行，先清除
  if (autoDownloadInterval) {
    clearInterval(autoDownloadInterval);
  }

  // 立即执行一次下载
  performAutoDownload();

  // 每3分钟执行一次自动下载
  autoDownloadInterval = setInterval(async () => {
    performAutoDownload();
  }, 3 * 60 * 1000); // 3分钟 = 180000毫秒
};

const stopAutoDownload = () => {
  if (autoDownloadInterval) {
    clearInterval(autoDownloadInterval);
    autoDownloadInterval = null;
  }
};

const performAutoDownload = async () => {
  if (
    autoDownloadEnabled.value &&
    isWebDAVConfigured.value &&
    !isSyncing.value
  ) {
    try {
      await forceDownloadFromCloud();
      console.log("自动下载完成");
    } catch (error) {
      console.error("自动下载失败:", error);
    }
  }
};

// 切换自动上传
const toggleAutoUpload = () => {
  // 互斥逻辑：如果自动下载已开启，阻止开启自动上传
  if (autoUploadEnabled.value && autoDownloadEnabled.value) {
    autoUploadEnabled.value = false;
    showStatus("请先关闭自动下载，再开启自动上传", "warning");
    return;
  }

  if (autoUploadEnabled.value) {
    startAutoUpload();
    showStatus("自动上传已开启，每3分钟自动上传到云端", "success");
  } else {
    stopAutoUpload();
    showStatus("自动上传已关闭", "info");
  }

  // 保存状态到本地存储
  saveAutoUploadState();
};

// 切换自动下载
const toggleAutoDownload = () => {
  // 互斥逻辑：如果自动上传已开启，阻止开启自动下载
  if (autoDownloadEnabled.value && autoUploadEnabled.value) {
    autoDownloadEnabled.value = false;
    showStatus("请先关闭自动上传，再开启自动下载", "warning");
    return;
  }

  if (autoDownloadEnabled.value) {
    startAutoDownload();
    showStatus("自动下载已开启，每3分钟自动从云端下载一次", "success");
  } else {
    stopAutoDownload();
    showStatus("自动下载已关闭", "info");
  }

  // 保存状态到本地存储
  saveAutoDownloadState();
};

// 切换导入导出时创建标签
const toggleCreateTagOnImportExport = () => {
  if (createTagOnImportExport.value) {
    showStatus("批量导入导出时创建标签已开启", "success");
  } else {
    showStatus("批量导入导出时创建标签已关闭", "info");
  }

  // 保存状态到本地存储
  saveCreateTagOnImportExportState();
};

// 应用关闭前同步
const syncBeforeClose = async () => {
  // 只有自动上传开启时才在关闭前上传
  if (
    autoUploadEnabled.value &&
    isWebDAVConfigured.value
  ) {
    try {
      await forceUploadToCloud();
      console.log("关闭前上传完成");
    } catch (error) {
      console.error("关闭前上传失败:", error);
    }
  }
};

// 更新检查相关方法
const checkForUpdates = async () => {
  if (updateChecker.value && updateChecker.value.manualCheckUpdate) {
    isCheckingUpdates.value = true;
    try {
      await updateChecker.value.manualCheckUpdate();
    } finally {
      isCheckingUpdates.value = false;
    }
  }
};

const openGitHubRepo = async () => {
  try {
    await invoke("open_url", { url: "https://github.com/Zheng-up/zAugment" });
    showStatus("已在浏览器中打开 GitHub 仓库", "success");
  } catch (error) {
    showStatus("打开 GitHub 仓库失败", "error");
  }
};

const openGitHubReleases = async () => {
  try {
    await invoke("open_url", {
      url: "https://github.com/Zheng-up/zAugment/releases",
    });
    showStatus("已在浏览器中打开发布页面", "success");
  } catch (error) {
    showStatus("打开发布页面失败", "error");
  }
};

// 平台预设选择方法
const selectPresetFromModal = (presetKey) => {
  selectedPreset.value = presetKey;
  const preset = webdavPresets[presetKey];

  if (preset) {
    console.log(`平台选择已切换到: ${preset.name}`);

    // 显示平台选择提示
    showStatus(`已切换到 ${preset.name} 平台`, "success");

    // 关闭弹窗
    showPlatformSelector.value = false;
  }
};

// 强制同步确认弹窗相关方法
const showForceUploadConfirm = () => {
  forceConfirmType.value = "upload";
  forceConfirmTitle.value = "确认强制上传";
  forceConfirmMessage.value =
    "您即将执行强制上传操作，这将把本地数据完全覆盖到云端。";
  showForceConfirm.value = true;
};

const showForceDownloadConfirm = () => {
  forceConfirmType.value = "download";
  forceConfirmTitle.value = "确认强制下载";
  forceConfirmMessage.value =
    "您即将执行强制下载操作，这将把云端数据完全覆盖到本地。";
  showForceConfirm.value = true;
};

const cancelForceSync = () => {
  showForceConfirm.value = false;
  forceConfirmType.value = "";
  forceConfirmTitle.value = "";
  forceConfirmMessage.value = "";
};

// 显示同步历史
const handleShowSyncHistory = () => {
  showSyncHistoryModal.value = true;
};

const confirmForceSync = async () => {
  const operation = forceConfirmType.value;
  showForceConfirm.value = false;

  try {
    if (operation === "upload") {
      await forceUploadToCloud();
    } else if (operation === "download") {
      await forceDownloadFromCloud();
    }
  } catch (error) {
    // 错误处理已在原方法中处理
  }

  // 清理状态
  cancelForceSync();
};

// ============ Deep-Link 导入队列处理 ============
/**
 * 处理导入队列中的 session
 * 确保多个 session 导入时不会冲突
 */
const processImportQueue = async () => {
  while (importQueue.value.length > 0) {
    // 如果正在导入，等待
    if (isImportingFromQueue.value) {
      await new Promise(resolve => setTimeout(resolve, 100));
      continue;
    }

    const session = importQueue.value.shift();
    isImportingFromQueue.value = true;

    // 显示导入中的提示，不自动消失
    const importingStatusId = showStatus("正在导入 Token...", "info", 0);

    try {
      // 打开 TokenList
      currentView.value = "token-list";
      await nextTick();

      // 等待 TokenList 准备好
      if (tokenListRef.value?.waitUntilReady) {
        await tokenListRef.value.waitUntilReady();
      }

      // 设置 session 输入
      sessionInput.value = session;

      // 调用导入方法
      await importFromSession();

      // 移除导入中的提示
      removeStatusMessage(importingStatusId);
    } catch (error) {
      // 移除导入中的提示
      removeStatusMessage(importingStatusId);

      // 显示错误提示
      showStatus(`❌ Token 导入失败: ${error.message}`, "error", 5000);
    } finally {
      isImportingFromQueue.value = false;
    }
  }
};

onMounted(async () => {
  console.log("App.vue onMounted 开始");

  try {
    // 获取当前版本信息
    console.log("开始获取当前版本...");
    await getCurrentVersion();
    console.log("版本信息获取成功");

    // 恢复初始化调用
    console.log("开始获取当前数据路径...");
    await getCurrentDataPath();
    console.log("数据路径获取成功");

    // 注意：账号数据由 TokenList 组件自己在 onMounted 中加载，这里不需要调用
    console.log("账号数据将由 TokenList 组件自动加载");

    console.log("开始加载WebDAV配置...");
    await loadWebDAVConfig();
    console.log("WebDAV配置加载成功");

    console.log("开始加载同步历史...");
    loadSyncHistory();
    console.log("同步历史加载成功");

    // 加载自动上传和自动下载状态
    console.log("开始加载自动上传状态...");
    loadAutoUploadState();
    console.log("自动上传状态加载成功");

    console.log("开始加载自动下载状态...");
    loadAutoDownloadState();
    console.log("自动下载状态加载成功");

    console.log("开始加载导入导出时创建标签状态...");
    loadCreateTagOnImportExportState();
    console.log("导入导出时创建标签状态加载成功");

    console.log("开始加载账号状态阈值配置...");
    loadStatusThresholds();
    console.log("账号状态阈值配置加载成功");

    // 如果自动上传已开启且WebDAV已配置，启动定时器
    if (autoUploadEnabled.value && isWebDAVConfigured.value) {
      startAutoUpload();
      console.log("自动上传定时器已启动");
    }

    // 如果自动下载已开启且WebDAV已配置，启动定时器
    if (autoDownloadEnabled.value && isWebDAVConfigured.value) {
      startAutoDownload();
      console.log("自动下载定时器已启动");
    }

    // 监听应用关闭事件
    window.addEventListener("beforeunload", syncBeforeClose);

    // 启用WebDAV配置状态监听（在所有依赖准备好后）
    watch(isWebDAVConfigured, (newConfigured, oldConfigured) => {
      // 避免初始化时的触发，只在状态真正变化时执行
      if (oldConfigured !== undefined) {
        if (newConfigured) {
          // WebDAV已配置，根据开关状态启动定时器
          if (autoUploadEnabled.value) {
            startAutoUpload();
            console.log("WebDAV配置完成，自动上传定时器已启动");
          }
          if (autoDownloadEnabled.value) {
            startAutoDownload();
            console.log("WebDAV配置完成，自动下载定时器已启动");
          }
        } else {
          // WebDAV未配置，停止所有定时器
          stopAutoUpload();
          stopAutoDownload();
          console.log("WebDAV配置丢失，所有定时器已停止");
        }
      }
    });

    // 移除自动刷新逻辑，与 augment-token-mng-main 保持一致
    // 用户可以手动点击刷新按钮来更新账号状态
    // 这样可以避免启动时干扰用户操作（如翻页等）

    console.log("App.vue onMounted 完成");
  } catch (error) {
    console.error("App.vue onMounted 错误:", error);
  }

  // 监听 Session 导入进度事件
  try {
    unlistenSessionProgress = await listen(
      "session-import-progress",
      (event) => {
        console.log("Progress event received:", event.payload);
        // 后端发送的进度消息
        const progressMap = {
          sessionImportExtractingToken: "正在提取 access token...",
          sessionImportGettingUserInfo: "正在获取用户信息...",
          sessionImportComplete: "导入完成！",
        };
        sessionImportProgress.value =
          progressMap[event.payload] || event.payload;
      }
    );
  } catch (error) {
    console.error("Failed to listen to session-import-progress:", error);
  }

  // 监听导入记录事件
  try {
    // 单个导入开始
    await listen("import-session-started", (event) => {
      const { session } = event.payload;
      addRecord(session, ImportStatus.PENDING);

      // 自动切换到设置页面并打开导入列表
      currentView.value = 'settings';
      showImportListModal.value = true;
    });

    // 单个导入成功
    await listen("import-session-success", (event) => {
      const { session, email } = event.payload;
      const records = useImportRecords().records.value;
      const record = records.find(r => r.session === session);
      if (record) {
        updateRecord(record.id, ImportStatus.SUCCESS, null, email ? `导入成功: ${email}` : '导入成功');
      }
    });

    // 单个导入失败
    await listen("import-session-failed", (event) => {
      const { session, error } = event.payload;
      const records = useImportRecords().records.value;
      const record = records.find(r => r.session === session);
      if (record) {
        updateRecord(record.id, ImportStatus.FAILED, error, null);
      }
    });

    // 批量导入开始
    await listen("import-sessions-started", (event) => {
      const { sessions } = event.payload;
      const newRecords = addRecords(sessions);

      // 保存批量导入的 session 顺序，用于后续匹配
      window.__batchImportSessions = sessions;
      window.__batchImportRecords = newRecords;

      // 自动切换到设置页面并打开导入列表
      currentView.value = 'settings';
      showImportListModal.value = true;
    });

    // 批量导入完成
    await listen("import-sessions-completed", (event) => {
      const { successful, failed, results } = event.payload;
      console.log(`批量导入完成: ${successful} 成功, ${failed} 失败`);

      // 根据返回的 results 更新每条记录的状态
      if (results && Array.isArray(results)) {
        const batchRecords = window.__batchImportRecords || [];

        // 使用索引匹配：results 的顺序应该与 sessions 的顺序一致
        results.forEach((result, index) => {
          const record = batchRecords[index];

          if (record) {
            if (result.success) {
              updateRecord(record.id, ImportStatus.SUCCESS, null, '导入成功');
            } else {
              updateRecord(record.id, ImportStatus.FAILED, result.error || '导入失败', null);
            }
          }
        });

        // 清理临时数据
        delete window.__batchImportSessions;
        delete window.__batchImportRecords;
      }
    });
  } catch (error) {
    console.error("Failed to listen to import events:", error);
  }

  // 监听 Deep-Link Session 接收事件（由后端发送，前端处理导入）
  // 这个事件在用户点击 zaugment://import?session=xxx 链接时触发
  try {
    await listen("deep-link-session-received", async (event) => {
      if (event.payload && event.payload.session) {
        // 将 session 加入队列
        importQueue.value.push(event.payload.session);

        // 显示队列状态提示
        if (isImportingFromQueue.value) {
          // 如果正在导入，提示等待
          showStatus(`已加入导入队列，当前待导入 ${importQueue.value.length} 个`, "info", 3000);
        }

        // 处理队列
        await processImportQueue();
      } else {
        showStatus("❌ Deep-Link 数据无效", "error", 5000);
      }
    });
  } catch (error) {
    console.error("Failed to listen to deep-link-session-received:", error);
  }

  // 监听 API 服务器导入事件（当通过 API 导入 Session 时触发）
  try {
    await listen("tokens-updated", async () => {
      console.log("📥 Tokens updated event received from API server");
      // 重新加载 tokens
      await loadTokens();
      showStatus("账号已通过 API 导入", "success", 3000);
    });
  } catch (error) {
    console.error("Failed to listen to tokens-updated:", error);
  }

  // 应用启动完成后，异步初始化邮箱管理（不阻塞界面渲染）
});
</script>

<style scoped>
.app {
  height: 100vh;
  background: #f8f9fa;
  overflow: hidden;
  display: flex;
  flex-direction: column; /* 改为垂直布局以容纳标题栏 */
}

.app-content {
  display: flex;
  flex-direction: row;
  flex: 1;
  min-height: 0;
}

/* 侧边栏样式 - 高级精致设计 */
.sidebar {
  width: 200px;
  background: linear-gradient(180deg, #0f1419 0%, #141b24 50%, #1a1f2e 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.06);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  position: relative;
  overflow: hidden;

  box-shadow: 2px 0 20px rgba(0, 0, 0, 0.4),
    inset -1px 0 0 rgba(255, 255, 255, 0.02);
}

/* 微妙质感背景 */
.sidebar::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(
    600px circle at 0% 0%,
    rgba(29, 78, 216, 0.05),
    transparent 50%
  );
  z-index: 1;
}

.sidebar-header {
  padding: 28px 20px;

  position: relative;
  z-index: 3;
  background: linear-gradient(
    180deg,
    rgba(255, 255, 255, 0.01) 0%,
    transparent 100%
  );
}

.app-logo {
  display: flex;
  justify-content: center;
  align-items: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  position: relative;
  padding: 12px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.01);
  border: 1px solid rgba(255, 255, 255, 0.02);
}

.app-logo:hover {
  transform: translateY(-2px);
  background: rgba(255, 255, 255, 0.03);
  border-color: rgba(255, 255, 255, 0.05);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

/* logo已改为文字，移除svg样式 */

/* logo已改为文字，移除svg hover样式 */

.logo-text {
  font-family: "Inter", "Segoe UI", -apple-system, sans-serif;
  font-size: 22px;
  font-weight: 700;
  color: #ffffff;
  margin: 0;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  letter-spacing: -0.5px;
}

.sidebar-nav {
  flex: 1;
  padding: 18px 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  position: relative;
  z-index: 3;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  margin-top: 10px;
  border: 1px solid transparent;
  background: transparent;
  color: rgba(255, 255, 255, 0.7);
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 16px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  text-align: left;
  width: 90%;
  position: relative;
  overflow: hidden;
  transform-origin: center;
  white-space: nowrap;
}

/* 悬浮背景效果 */
.nav-item::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.05);
  opacity: 0;
  transition: all 0.2s ease;
  border-radius: 16px;
  z-index: -1;
}

.nav-item:hover::before {
  opacity: 1;
}

.nav-item:hover {
  color: rgba(255, 255, 255, 0.9);
  background: rgba(255, 255, 255, 0.03);
}

.nav-item.active {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.12) 0%,
    rgba(29, 78, 216, 0.08) 100%
  );
  color: #60a5fa;
  box-shadow: 0 1px 3px rgba(59, 130, 246, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.nav-item.active::before {
  opacity: 1;
  background: rgba(255, 255, 255, 0.03);
}

.nav-item.active:hover {
  color: #93c5fd;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.15) 0%,
    rgba(29, 78, 216, 0.1) 100%
  );
  box-shadow: 0 2px 6px rgba(59, 130, 246, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

.nav-item svg {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
  opacity: 0.8;
  transition: all 0.2s ease;
}

.nav-item:hover svg {
  opacity: 1;
}

.nav-item.active svg {
  opacity: 1;
  color: #60a5fa;
}

.nav-item.active:hover svg {
  color: #93c5fd;
}

/* 点击反馈 */
.nav-item:active {
  transform: scale(0.98);
  transition: all 0.1s ease;
}

.nav-item.active:active {
  transform: scale(0.98);
  transition: all 0.1s ease;
}

/* 文字效果 */
.nav-item span {
  transition: all 0.2s ease;
  font-weight: 500;
}

.nav-item:hover span {
  letter-spacing: 0.2px;
}

.nav-item.active span {
  font-weight: 600;
  letter-spacing: 0.1px;
}

.nav-item.active:hover span {
  letter-spacing: 0.3px;
}

/* 隐藏所有滚动条 */
* {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

*::-webkit-scrollbar {
  width: 0px;
  background: transparent;
}

/* 确保body和html不产生滚动条 */
html,
body {
  overflow: hidden;
  height: 100%;
  margin: 0;
  padding: 0;
}

.app-header {
  background: white;
  border-bottom: 1px solid #e1e5e9;
  padding: 12px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  min-height: 60px;
  flex-wrap: wrap;
  gap: 8px;
}

.header-left {
  display: flex;

  gap: 8px;
  flex: 1;
  min-width: 0;
  align-items: center;
}

.app-header h1 {
  margin: 0;
  color: #333;
  font-size: 20px;
  font-weight: 600;
  white-space: nowrap;
}

.header-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.main-content {
  flex: 1;
  overflow: hidden;
  min-height: 0;
  background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
  display: flex;
  flex-direction: column;
}

/* Token生成器视图样式 */
.token-generator-view,
.token-list-view,
.email-manager-view,
.settings-view,
.api-settings-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 0;
}

.token-generator-header,
.view-header-bar {
  display: flex;
  max-height: 70px;
  justify-content: space-between;
  align-items: center;
  padding: 18px 12px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  flex-shrink: 0;
}

.header-left h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-left-title {
  display: flex;
  justify-self: start;
  align-items: center;
  gap: 18px;
  position: relative;
}

/* 悬浮状态统计样式 - 一行横向显示 */
.floating-status-stats {
  position: absolute;
  left: 100%;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  flex-direction: row;
  gap: 8px;
  padding: 8px 12px;
  z-index: 10;
}

.floating-status-stats .status-item {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 13px;
  border: 2px solid transparent;
  white-space: nowrap;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.floating-status-stats .status-item:hover {
  /* transform: translateY(-1px); */
  opacity: 0.85;
}

.floating-status-stats .status-item.selected {
  border-color: currentColor;
  font-weight: 600;
}

/* 全部 */
.floating-status-stats .status-item.all {
  background: #f0f9ff;
  color: #0369a1;
}

/* 正常 */
.floating-status-stats .status-item.saved {
  background: #f0fdf4;
  color: #15803d;
}

/* 封禁 */
.floating-status-stats .status-item.banned {
  background: #fee2e2;
  color: #b91c1c;
}

/* 其他（过期+Token失效+未知） */
.floating-status-stats .status-item.other {
  background: #f3f4f6;
  color: #6b7280;
}

/* 以下样式保留用于兼容（如果有其他地方使用） */
.floating-status-stats .status-item.expired {
  background: #fed7aa;
  color: #c2410c;
  border-color: rgba(194, 65, 12, 0.15);
}

.floating-status-stats .status-item.invalid {
  background: #fef3c7;
  color: #a16207;
  border-color: rgba(161, 98, 7, 0.15);
}

.floating-status-stats .status-label {
  font-size: 12px;
  font-weight: 500;
}

.floating-status-stats .status-count {
  font-size: 13px;
  font-weight: 700;
  min-width: 14px;
  text-align: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.plugin-download-btn-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 8px rgba(139, 92, 246, 0.3);
}

.plugin-download-btn-header:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 16px rgba(139, 92, 246, 0.4);
}

/* 标题栏按钮样式 */
.btn-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  min-height: 40px;
}

.btn-header.success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.3);
}

.btn-header.success:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.4);
}

.btn-header.secondary {
  background: rgba(241, 245, 249, 0.8);
  color: #475569;
  border: 1px solid rgba(226, 232, 240, 0.6);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.btn-header.secondary:hover:not(:disabled) {
  background: rgba(226, 232, 240, 0.9);
  border-color: rgba(203, 213, 225, 0.8);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.btn-header:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-header.loading {
  opacity: 0.8;
  cursor: not-allowed;
  pointer-events: none;
}

.loading-spinner-small {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
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

.btn-header.appreciation {
  background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(236, 72, 153, 0.3);
}

.btn-header.appreciation:hover:not(:disabled) {
  background: linear-gradient(135deg, #db2777 0%, #be185d 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(236, 72, 153, 0.4);
}

/* 格式选择器样式 - 与其他页面标题栏保持一致 */
.format-selector-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.format-label {
  font-weight: 500;
  color: #475569;
  font-size: 14px;
  white-space: nowrap;
}

.format-select-header {
  padding: 8px 12px;
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 8px;
  font-size: 13px;
  background: rgba(255, 255, 255, 0.9);
  color: #374151;
  transition: all 0.2s ease;
  cursor: pointer;
  backdrop-filter: blur(5px);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.format-select-header:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  background: rgba(255, 255, 255, 1);
}

.format-select-header:hover {
  border-color: rgba(203, 213, 225, 0.8);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

/* 状态标志样式 - 用于账号管理页面 */
.status-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 4px 4px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.status-item.saved {
  background: #f0fdf4;
  color: #15803d;
}

.status-item.unsaved {
  background: #fef3c7;
  color: #d97706;
}

.status-item.banned {
  background: #fee2e2;
  color: #b91c1c;
}

.status-item.invalid {
  background: #fef3c7;
  color: #a16207;
}

.status-item.expired {
  background: #fed7aa;
  color: #c2410c;
}

.status-item.unchecked {
  background: #f3f4f6;
  color: #6b7280;
}

.status-label {
  display: flex;

  align-items: center;
  gap: 6px;
  font-size: 10px !important;
}

.status-count {
  font-size: 10px !important;
  font-weight: 600;
}

@keyframes pulse {
  0% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(0.95);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.token-generator-card-container {
  flex: 1;
  padding: 20px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 删除了旧的视图样式，现在使用统一的卡片布局 */

/* 防封插件下载按钮区域 - 顶部 */
.plugin-download-section-top {
  text-align: center;
  margin-bottom: 32px;
}

/* 重置Augment按钮样式 */
.reset-augment-btn-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  border: none;
  border-radius: 10px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
  margin-right: 12px;
}

.reset-augment-btn-header:hover {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  box-shadow: 0 6px 16px rgba(239, 68, 68, 0.4);
  transform: translateY(-2px);
}

.reset-augment-btn-header:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(239, 68, 68, 0.3);
}

/* Token生成器卡片样式 - 现代化设计 */
.token-generator-card,
.view-content-card {
  background: transparent;
  padding: 10px;
  /* padding-right: 4px; */
  position: relative;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
}



/* 卡片容器样式 */
.card-container {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 24px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.4);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(20px);
  height: 100%;
  display: flex;
  flex-direction: column;
}

.card-container:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  border-color: rgba(226, 232, 240, 0.6);
}

/* Steps Tab Navigation - 悬浮在顶部，占满整行 */
.steps-tab-navigation {
  display: flex;
  gap: 0;
  background: linear-gradient(180deg, #ffffff 0%, #fafbfc 100%);
  padding: 8px 16px;
  margin: 0;
  border-radius: 0;
  width: 100%;
  flex-shrink: 0;
  position: sticky;
  top: 0;
  z-index: 100;
  border-bottom: 1px solid rgba(226, 232, 240, 0.5);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.steps-tab-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 24px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #64748b;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
  flex: 1;
  justify-content: center;
  position: relative;
  overflow: hidden;
}

.steps-tab-btn::before {
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

.steps-tab-btn:hover::before {
  left: 100%;
}

.steps-tab-btn:hover:not(.active) {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1) 0%,
    rgba(79, 70, 229, 0.05) 100%
  );
  color: #4f46e5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
}

.steps-tab-btn.active {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  transform: translateY(-1px);
}

/* 统一的步骤卡片设计 */
.unified-steps-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.4);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(20px);
  display: flex;
  flex-direction: column;
  padding: 0;
}

.unified-steps-card:hover {
  /* transform: translateY(-2px); */
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  border-color: rgba(226, 232, 240, 0.6);
}

/* Steps Content Container */
.steps-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px 32px;
}

/* Session Import Content Container */
.session-import-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px 32px;
}

.session-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
  .session-header {
    display: flex;
    align-items: center;
    gap: 16px;
  }
}

.help-btn {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1) 0%,
    rgba(79, 70, 229, 0.05) 100%
  );
  border: 1px solid rgba(99, 102, 241, 0.2);
  color: #6366f1;
  cursor: pointer;
  padding: 6px 10px;
  border-radius: 8px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  position: relative;
  overflow: hidden;
}

.help-btn::before {
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

.help-btn:hover::before {
  left: 100%;
}

.help-btn:hover {
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.2) 0%,
    rgba(79, 70, 229, 0.1) 100%
  );
  border-color: rgba(99, 102, 241, 0.4);
  color: #4f46e5;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.15);
  transform: translateY(-1px);
}

.help-btn:active {
  transform: translateY(0);
}

.session-help-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  max-height: 60vh;
}

.help-step {
  padding: 16px;
  /* margin-bottom: 20px; */
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 0.8) 0%,
    rgba(241, 245, 249, 0.8) 100%
  );
  border-radius: 12px;
  border: 1px solid rgba(226, 232, 240, 0.4);
  transition: all 0.2s ease;
}

.help-step:hover {
  background: linear-gradient(
    135deg,
    rgba(248, 250, 252, 1) 0%,
    rgba(241, 245, 249, 1) 100%
  );
  border-color: rgba(226, 232, 240, 0.6);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.step-title {
  font-weight: 700;
  color: #1e293b;
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.step-title::before {
  content: "";
  display: inline-block;
  width: 4px;
  height: 4px;
  background: #3b82f6;
  border-radius: 50%;
}

.help-step p {
  margin: 0;
  color: #475569;
  font-size: 13px;
  line-height: 1.7;
}

.help-step p + p {
  margin-top: 8px;
}

.help-step a {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.2s ease;
  border-bottom: 1px solid transparent;
}

.help-step a:hover {
  color: #2563eb;
  border-bottom-color: #2563eb;
}

.help-step code {
  background: rgba(59, 130, 246, 0.1);
  padding: 3px 8px;
  border-radius: 4px;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 12px;
  color: #2563eb;
  font-weight: 500;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.copyable-code {
  cursor: pointer !important;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.15) 0%,
    rgba(59, 130, 246, 0.08) 100%
  ) !important;

  border-radius: 6px !important;

  transition: all 0.2s ease !important;
  display: inline-block !important;
  position: relative !important;
}

.copyable-code:hover {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.25) 0%,
    rgba(59, 130, 246, 0.15) 100%
  ) !important;

  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2) !important;
  transform: translateY(-1px) !important;
}

.copyable-code:active {
  transform: translateY(0) !important;
}

.copyable-code b {
  font-weight: 600 !important;
}

.session-section h3 {
  color: #1e293b;
  font-size: 18px;
  font-weight: 600;
  margin: 0;
}

.section-description {
  color: #64748b;
  font-size: 14px;
  margin: 0;
}

.session-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid rgba(226, 232, 240, 0.4);
  border-radius: 8px;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 13px;
  color: #1e293b;
  background: rgba(248, 250, 252, 0.5);
  resize: none;
  transition: all 0.3s ease;
}

.session-textarea:focus {
  outline: none;
  border-color: #3b82f6;
  background: white;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.session-textarea:disabled {
  background: rgba(226, 232, 240, 0.2);
  color: #94a3b8;
  cursor: not-allowed;
}

/* Session Loading State */
.session-loading {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: rgba(59, 130, 246, 0.05);
  border-radius: 8px;
  color: #3b82f6;
  font-size: 14px;
}

.session-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(59, 130, 246, 0.2);
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 单个步骤项 */
.step-item {
  padding: 0 0 24px 0;
  position: relative;
}

.step-item:not(:last-child) {
  border-bottom: 1px solid rgba(226, 232, 240, 0.2);
  padding-bottom: 24px;
}

/* 步骤标题区域 */
.step-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.step-number {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 700;
  box-shadow: 0 4px 16px rgba(99, 102, 241, 0.2);
  flex-shrink: 0;
}

.step-header h3 {
  color: #1e293b;
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  flex: 1;
}

/* 步骤标题中的复制按钮 - 覆盖 integrated-btn 的宽高和圆角 */
.step-header .integrated-btn {
  padding: 8px 16px;
  border-radius: 8px;
  border-left: none;
}

.step-header .integrated-btn svg {
  width: 16px;
  height: 16px;
}

/* 步骤动画 */
@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 防封插件下载按钮区域 */
.plugin-download-section {
  text-align: center;
  margin-top: 24px;
}

.plugin-download-btn {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  color: white;
  border: none;
  padding: 16px 32px;
  border-radius: 16px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 8px 24px rgba(139, 92, 246, 0.3), 0 4px 8px rgba(0, 0, 0, 0.1);
  display: inline-flex;
  align-items: center;
  gap: 12px;
  position: relative;
  overflow: hidden;
  min-height: 56px;
}

.plugin-download-btn::before {
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
  transition: left 0.6s;
}

.plugin-download-btn:hover {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  transform: translateY(-2px);
  box-shadow: 0 12px 32px rgba(139, 92, 246, 0.4),
    0 8px 16px rgba(0, 0, 0, 0.15);
}

.plugin-download-btn:hover::before {
  left: 100%;
}

/* 防封插件弹窗样式 */
.plugin-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.plugin-dialog {
  background: white;
  border-radius: 12px;
  width: 90vw;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

.plugin-dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
  border-radius: 12px 12px 0 0;
}

.plugin-dialog-header h3 {
  margin: 0;
  color: #374151;
  font-size: 18px;
  font-weight: 600;
}

.plugin-dialog-body {
  padding: 24px;
}

.plugin-dialog-body p {
  margin: 0 0 24px 0;
  color: #6b7280;
  line-height: 1.6;
}

.plugin-options {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.plugin-option {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  transition: all 0.2s;
}

.plugin-option:hover {
  border-color: #3b82f6;
  background: #f8fafc;
}

.plugin-option-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.plugin-option-icon.vscode {
  background: linear-gradient(135deg, #007acc 0%, #005a9e 100%);
}

.plugin-option-icon.jetbrains {
  background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
}

.plugin-option-content {
  flex: 1;
}

.plugin-option-content h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
}

.plugin-option-content p {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: #6b7280;
  line-height: 1.5;
}

/* 区块样式 */
.section {
  margin-bottom: 48px;
  padding: 24px 0;
}

.section:last-child {
  margin-bottom: 0;
}

.section h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #374151;
  display: flex;
  align-items: center;
  gap: 8px;
}

.sync-status-container {
  position: absolute;
  top: 0;
  right: -20px;
  display: flex;
  justify-content: flex-end;
  align-items: flex-start;
}

.sync-status-container > * {
  width: 240px;
  max-width: 30%;
}

.generator-header {
  margin-bottom: 32px;
}

.header-content {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  gap: 20px;
}

.title-section {
  text-align: center;
}

.title-section h2 {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.2;
}

.title-section p {
  margin: 0;
  font-size: 16px;
  color: #6b7280;
  line-height: 1.5;
}

.btn {
  padding: 14px 28px;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  font-size: 15px;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  min-height: 48px;
  text-decoration: none;
  user-select: none;
  position: relative;
}

.btn.primary {
  background: #4f46e5;
  color: white;
  border: none;
  box-shadow: 0 4px 14px rgba(79, 70, 229, 0.25);
  font-weight: 600;
  position: relative;
  overflow: hidden;
}

.btn.primary::before {
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

.btn.primary:hover:not(:disabled) {
  background: #4338ca;
  transform: translateY(-1px);
  box-shadow: 0 8px 20px rgba(79, 70, 229, 0.35);
}

.btn.primary:hover:not(:disabled)::before {
  left: 100%;
}

.btn.secondary {
  background: rgba(156, 163, 175, 0.1) !important;
  color: #6b7280 !important;
  border: 1px solid rgba(156, 163, 175, 0.5) !important;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05) !important;
  font-weight: 500;
  position: relative;
  overflow: hidden;
  padding: 12px 20px;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-height: 44px;
  justify-content: center;
}

.btn.secondary::before {
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

.btn.secondary:hover:not(:disabled)::before {
  left: 100%;
}

.btn.secondary:hover:not(:disabled) {
  background: rgba(156, 163, 175, 0.15) !important;
  border-color: rgba(107, 114, 128, 0.6) !important;
  transform: translateY(-2px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1) !important;
}

.btn.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(239, 68, 68, 0.25);
  position: relative;
  overflow: hidden;
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-height: 44px;
  justify-content: center;
}

.btn.danger::before {
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

.btn.danger:hover:not(:disabled)::before {
  left: 100%;
}

.btn.danger:hover:not(:disabled) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.35);
}

.btn.success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  border: none;
  box-shadow: 0 4px 14px rgba(16, 185, 129, 0.25);
  font-weight: 600;
  position: relative;
  overflow: hidden;
}

.btn.success::before {
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

.btn.success:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 8px 20px rgba(16, 185, 129, 0.35);
}

.btn.success:hover:not(:disabled)::before {
  left: 100%;
}

.btn.warning {
  background: #f59e0b;
  color: white;
}

.btn.warning:hover {
  background: #d97706;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(245, 158, 11, 0.3);
}

.btn.info {
  background: #0ea5e9;
  color: white;
}

.btn.info:hover {
  background: #0284c7;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.3);
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.disabled,
.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  pointer-events: none;
  background: linear-gradient(135deg, #e2e8f0 0%, #cbd5e1 100%) !important;
  color: #94a3b8 !important;
  box-shadow: none !important;
  transform: none !important;
}

/* 现代化禁用状态的输入框样式 */
input:disabled {
  background: rgba(248, 250, 252, 0.6) !important;
  color: #94a3b8 !important;
  border-color: rgba(226, 232, 240, 0.5) !important;
  cursor: not-allowed;
  opacity: 0.9;
  box-shadow: none !important;
}

textarea:disabled {
  background: rgba(248, 250, 252, 0.6) !important;
  color: #94a3b8 !important;
  border-color: rgba(226, 232, 240, 0.5) !important;
  cursor: not-allowed;
  opacity: 0.9;
  box-shadow: none !important;
}

.login-buttons {
  display: flex;
  gap: 6px;
  align-items: center;
}

/* 现代化输入框样式 */
input[type="text"] {
  padding: 14px 18px;
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  font-size: 15px;
  font-weight: 400;
  line-height: 1.5;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.8);
  color: #1e293b;
  width: 100%;
  box-sizing: border-box;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

input[type="text"]:focus {
  outline: none;
  border-color: #6366f1;
  background: #ffffff;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

input[type="text"]:hover:not(:disabled):not(:read-only) {
  border-color: rgba(99, 102, 241, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

input[type="text"]:read-only {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border-color: rgba(226, 232, 240, 0.6);
}

input[type="text"]::placeholder {
  color: #94a3b8;
  font-size: 14px;
}

/* 现代化textarea样式 - 与输入框保持一致 */
textarea {
  padding: 14px 18px;
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  font-size: 15px;
  font-weight: 400;
  line-height: 1.6;
  font-family: inherit;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.8);
  color: #1e293b;
  width: 100%;
  box-sizing: border-box;
  resize: vertical;
  min-height: 120px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

/* 类似input的textarea样式 - 固定两行高度 */
.input-like-textarea {
  height: 4rem; /* 约等于两行文字的高度 */
  min-height: 4rem;
  max-height: 4rem;
  line-height: 2rem;
  resize: none;
  overflow-y: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
  line-height: 1.4;
  padding: 12px 16px;
}

textarea:focus {
  outline: none;
  border-color: #6366f1;
  background: #ffffff;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

textarea:hover:not(:disabled) {
  border-color: rgba(99, 102, 241, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

textarea::placeholder {
  color: #94a3b8;
  font-size: 14px;
}

/* 现代化结果容器样式 */
.result-container {
  margin-bottom: 24px;
}

.result-container label {
  display: block;
  margin-bottom: 12px;
  font-weight: 600;
  color: #1e293b;
  font-size: 15px;
  position: relative;
  padding-left: 16px;
}

.result-container label::before {
  content: "";
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 16px;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  border-radius: 2px;
}

/* 统一的表单样式 */
.form-group {
  margin-bottom: 24px;
}

.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: #374151;
  margin-bottom: 8px;
  line-height: 1.4;
}

/* 统一的输入框基础样式 */
.unified-input {
  width: 100%;
  padding: 14px 16px;
  border: 2px solid rgba(226, 232, 240, 0.6);
  border-radius: 12px;
  font-size: 15px;
  font-family: inherit;
  background: #ffffff;
  color: #374151;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.unified-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1), 0 4px 12px rgba(0, 0, 0, 0.1);
  background: #ffffff;
}

.unified-input:disabled {
  background: #f9fafb;
  color: #9ca3af;
  cursor: not-allowed;
  border-color: rgba(226, 232, 240, 0.4);
}

.unified-input::placeholder {
  color: #9ca3af;
  font-size: 14px;
}

/* 文本域样式 */
textarea.unified-input {
  resize: vertical;
  min-height: 100px;
  line-height: 1.5;
}

/* 现代化Token容器样式 - 用于带按钮的输入框组合 */
.token-container {
  display: flex;
  gap: 0;
  align-items: stretch;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.token-container:focus-within {
  border-color: #6366f1;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
  background: #ffffff;
}

.token-container:hover {
  border-color: rgba(99, 102, 241, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.token-container input {
  flex: 1;
  border: none !important;
  background: transparent !important;
  box-shadow: none !important;
  margin: 0;
  padding: 14px 18px !important;
  font-size: 15px;
  color: #1e293b;
  outline: none;
}

.token-container input::placeholder {
  color: #94a3b8;
  font-size: 14px;
}

.token-container input:disabled {
  background: transparent !important;
  color: #94a3b8;
  cursor: not-allowed;
}

.token-container .btn {
  border-radius: 0 !important;
  border-left: 1px solid rgba(226, 232, 240, 0.3);
  min-width: 80px;
  padding: 0 20px;
  font-size: 14px;
  height: 100%;
  margin: 0;
  box-shadow: none !important;
}

.token-container .btn.secondary {
  background: linear-gradient(135deg, #64748b 0%, #475569 100%);
  color: white;
  border: none;
  font-weight: 500;
  transition: all 0.2s ease;
}

.token-container .btn.secondary:hover:not(:disabled) {
  background: linear-gradient(135deg, #475569 0%, #334155 100%);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.1) !important;
}

.additional-fields {
  margin-top: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.button-container {
  margin-top: 24px;
  display: flex;
  gap: 16px;
  justify-content: center;
  padding-top: 20px;
  border-top: 1px solid rgba(226, 232, 240, 0.4);
}

/* 带复制图标的输入框 */
.input-with-copy {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

.input-with-copy input {
  padding-right: 45px;
  flex: 1;
}

.copy-icon-btn {
  position: absolute;
  right: 8px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 6px;
  border-radius: 4px;
  color: #6c757d;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.copy-icon-btn:hover {
  background: #e9ecef;
  color: #495057;
}

.copy-icon-btn:active {
  transform: scale(0.95);
}

/* 响应式设计 */
@media (max-width: 768px) {
  /* 移动端滚动条 */
  .token-generator-card::-webkit-scrollbar,
  .view-content-card::-webkit-scrollbar {
    width: 12px;
  }

  .token-generator-card::-webkit-scrollbar-thumb,
  .view-content-card::-webkit-scrollbar-thumb {
    border: 3px solid transparent;
  }

  .app-header {
    padding: 8px 12px;
    min-height: 56px;
  }

  .app-header h1 {
    font-size: 18px;
  }

  .header-buttons {
    gap: 6px;
  }

  .btn {
    padding: 8px 12px;
    font-size: 13px;
  }

  .header-left-title {
    flex-wrap: wrap;
    gap: 8px;
  }

  .header-left-title h2 {
    width: 100%;

  }

  /* 响应式悬浮状态统计 */
  .floating-status-stats {
    position: absolute;
    left: 100%;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    gap: 8px;
    padding: 8px 12px;
    z-index: 10;
  }

  .floating-status-stats .stats-column {
    gap: 4px;
  }

  .floating-status-stats .status-item {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 4px;
    font-size: 11px;
    white-space: nowrap;
  }

  .floating-status-stats .status-label {
    font-size: 8px;
  }

  .floating-status-stats .status-count {
    font-size: 11px;
    min-width: 10px;
    text-align: center;
  }

  .status-item {
    padding: 4px 4px;
    font-size: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .status-label {
    display: flex;

    align-items: center;
    gap: 6px;
  }
  .btn.small {
    padding: 6px 10px;
    font-size: 12px;
  }
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 12px;
  max-width: 90vw;
  max-height: 95vh;
  overflow-y: auto;
  position: relative;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
  border-radius: 12px 12px 0 0;
}

.modal-header h3 {
  margin: 0;
  color: #374151;
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #6b7280;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #e5e7eb;
  color: #374151;
}

@media (max-width: 480px) {
  .app-header {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
    padding: 8px;
  }

  .header-left {
  }

  .header-buttons {
    justify-content: space-between;
    width: 100%;
  }

  .user-controls {
    margin-left: auto;
    padding-left: 8px;
    border-left: 1px solid #e1e5e9;
  }
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}

.empty-icon {
  margin-bottom: 24px;
  color: #ccc;
}

.empty-state h2 {
  margin: 0 0 12px 0;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.empty-state p {
  margin: 0;
  font-size: 16px;
  line-height: 1.5;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #007bff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.token-list {
  width: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.list-header h2 {
  margin: 0;
  color: #333;
  font-size: 20px;
  font-weight: 600;
}

/* 改进的多条状态提示系统样式 */
.status-messages-container {
  position: fixed;
  left: 20px;
  bottom: 20px;
  z-index: 10000;
  pointer-events: none;
}

.status-toast {
  position: fixed;
  left: 20px;
  width: auto;

  padding: 12px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3), 0 4px 8px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  animation: slideInFromBottom 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-sizing: border-box;
  pointer-events: auto;

  line-height: 1.4;
}

.status-toast.removing {
  animation: fadeOut 0.3s cubic-bezier(0.4, 0, 0.2, 1) forwards;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes slideInFromBottom {
  from {
    transform: translateY(100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes fadeOut {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-10px) scale(0.95);
  }
}

.status-toast.info {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.95) 0%,
    rgba(37, 99, 235, 0.95) 100%
  );
  color: #ffffff;
  border: 1px solid rgba(59, 130, 246, 0.8);
}

.status-toast.success {
  background: linear-gradient(
    135deg,
    rgba(34, 197, 94, 0.95) 0%,
    rgba(22, 163, 74, 0.95) 100%
  );
  color: #ffffff;
  border: 1px solid rgba(34, 197, 94, 0.8);
}

.status-toast.error {
  background: linear-gradient(
    135deg,
    rgba(239, 68, 68, 0.95) 0%,
    rgba(220, 38, 38, 0.95) 100%
  );
  color: #ffffff;
  border: 1px solid rgba(239, 68, 68, 0.8);
}

.status-toast.warning {
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.95) 0%,
    rgba(217, 119, 6, 0.95) 100%
  );
  color: #ffffff;
  border: 1px solid rgba(245, 158, 11, 0.8);
}

/* Portal对话框样式 - 统一设计风格 */
.portal-modal-overlay {
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
  z-index: 1000;
  padding: 20px;
  animation: fadeIn 0.3s ease;
}

.portal-modal {
  background: rgba(255, 255, 255, 0.98);
  border-radius: 24px;
  box-shadow: 0 32px 64px rgba(0, 0, 0, 0.12),
    0 0 0 1px rgba(255, 255, 255, 0.05);
  max-width: 500px;
  width: 90%;
  max-height: 95vh;
  overflow: hidden;
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  position: relative;
  pointer-events: auto;
  margin: auto;
  border: 1px solid rgba(226, 232, 240, 0.3);
  backdrop-filter: blur(20px);
}

.portal-modal .modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 28px 32px 24px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.5);
  background: linear-gradient(
    135deg,
    rgba(249, 250, 251, 0.8) 0%,
    rgba(243, 244, 246, 0.5) 100%
  );
  backdrop-filter: blur(10px);
}

.portal-modal .modal-header h3 {
  margin: 0;
  font-size: 22px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.025em;
}

.portal-modal .modal-close {
  background: rgba(248, 250, 252, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.5);
  padding: 8px;
  cursor: pointer;
  color: #64748b;
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  backdrop-filter: blur(10px);
}

.portal-modal .modal-close:hover {
  background: rgba(239, 246, 255, 0.9);
  border-color: rgba(147, 197, 253, 0.3);
  color: #3b82f6;
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
}

.portal-modal .modal-content {
  padding: 32px;
  max-height: calc(95vh - 80px);
  overflow-y: auto;
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

/* Vue Transition动画 */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .portal-modal,
.modal-leave-to .portal-modal {
  transform: translateY(-20px) scale(0.95);
}

.modal-enter-to .portal-modal,
.modal-leave-from .portal-modal {
  transform: translateY(0) scale(1);
}

/* 保留原有的Portal对话框样式用于其他对话框 */
.portal-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000;
}

.portal-dialog {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  min-width: 300px;
  max-width: 400px;
}

.portal-dialog h3 {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
  text-align: center;
}

.dialog-description {
  margin: 0 0 20px 0;
  color: #666;
  font-size: 14px;
  line-height: 1.5;
  text-align: center;
}

.input-with-button {
  display: flex;
  gap: 8px;
  align-items: center;
}

.input-with-button input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  background-color: #f9f9f9;
}

.input-with-button input:focus {
  outline: none;
  border-color: #007acc;
  box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.1);
}

.btn.small {
  padding: 8px 12px;
  font-size: 12px;
  min-width: auto;
}

.dialog-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dialog-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border: 2px solid transparent;
  border-radius: 8px;
  background: #f8f9fa;
  color: #495057;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  justify-content: center;
  position: relative;
  overflow: hidden;
}

.dialog-btn::before {
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

.dialog-btn:hover::before {
  left: 100%;
}

.dialog-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.dialog-btn.vscode {
  background: #e3f2fd;
  color: #1976d2;
  border-color: #90caf9;
}

.dialog-btn.vscode:hover {
  background: #bbdefb;
  border-color: #64b5f6;
}

.dialog-btn.jetbrains {
  background: #fff3e0;
  color: #f57c00;
  border-color: #ffcc02;
}

.dialog-btn.jetbrains:hover {
  background: #ffe0b2;
  border-color: #ffb74d;
}

.dialog-btn.cancel {
  background: #fce4ec;
  color: #c2185b;
  border-color: #f8bbd9;
}

.dialog-btn.cancel:hover {
  background: #f8bbd9;
  border-color: #f48fb1;
}

.dialog-btn.delete {
  background: #ffebee;
  color: #d32f2f;
  border-color: #ffcdd2;
}

.dialog-btn.delete:hover {
  background: #ffcdd2;
  border-color: #ef9a9a;
}

/* 删除确认对话框特定样式 */
.portal-dialog.delete-confirm p {
  margin: 0 0 20px 0;
  color: #666;
  text-align: center;
  line-height: 1.5;
}

.delete-confirm .dialog-buttons {
  flex-direction: row;
  gap: 12px;
}

.delete-confirm .dialog-btn {
  flex: 1;
}

/* Portal弹窗响应式样式 */
.portal-modal {
  width: 95%;
  margin: 16px;
  max-height: 90vh;
}

.portal-modal .modal-header {
  padding: 16px 20px 12px;
}

.portal-modal .modal-header h3 {
  font-size: 18px;
}

.portal-modal .modal-content {
  padding: 16px 20px 20px;
}

.field-container {
  margin-bottom: 16px;
}

.field-container label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #475569;
  font-size: 14px;
  letter-spacing: 0.025em;
}

.field-input {
  width: 100%;
  padding: 14px 18px;
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  font-size: 15px;
  font-weight: 400;
  line-height: 1.5;
  color: #1e293b;
  background: rgba(255, 255, 255, 0.8);
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  resize: none;
}

.field-input:focus {
  outline: none;
  border-color: #6366f1;
  background: #ffffff;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.field-input:hover:not(:disabled):not(:read-only) {
  border-color: rgba(99, 102, 241, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.field-input::placeholder {
  color: #94a3b8;
  font-size: 14px;
}

/* 移除了重复的状态指示器样式，现在在 TokenList.vue 中 */

/* 小窗口模式专用样式 (760px) - 白色主题 */
@media (max-width: 760px) {
  .app-content {
    flex-direction: column;
  }

  .sidebar {
    width: 100% !important;
    height: 80px !important;
    flex-direction: row !important;
    overflow-x: auto !important;
    background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%) !important;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 1px 3px rgba(0, 0, 0, 0.06) !important;
    border-right: none !important;
    border-bottom: 1px solid rgba(226, 232, 240, 0.6) !important;
    backdrop-filter: blur(10px) !important;
    /* 禁止y轴滚动 */
    overflow-y: no-scroll !important;
  }

  .sidebar::before {
    display: none; /* 移除深色主题的装饰背景 */
  }

  .sidebar-header {
    display: none; /* 隐藏logo */
  }

  .sidebar-nav {
    display: flex !important;
    flex-direction: row !important;
    justify-content: space-between !important;
    align-items: center !important;
    padding: 10px 18px 10px !important;
    width: 100% !important;
    gap: 0 !important;
  }

  .nav-item {
    min-width: auto !important;
    width: auto !important;
    padding: 14px 38px !important;
    font-size: 13px !important;
    font-weight: 500 !important;
    white-space: nowrap !important;
    border-radius: 12px !important;
    background: rgba(248, 250, 252, 0.8) !important;
    color: #475569 !important;
    border: 1px solid rgba(209, 213, 218, 0.701) !important;

    box-shadow: none !important;
    /* backdrop-filter: blur(8px) !important; */
    transition: all 0.2s ease !important;
    position: relative !important;
    overflow: hidden !important;
    flex-shrink: 0 !important;
    justify-content: center !important;
  }

  .nav-item:hover {
    /* background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%) !important; */
    color: #1e293b !important;
    border-color: rgba(148, 163, 184, 0.3) !important;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1) !important;
    transform: translateY(-1px) !important;
  }

  .nav-item.active {
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%) !important;
    color: white !important;
    border-color: transparent !important;
    box-shadow: 0 6px 20px rgba(59, 130, 246, 0.3) !important;
    transform: translateY(-1px) !important;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  @keyframes borderFlowMobile {
    0% {
      transform: translateX(-100%) rotate(0deg);
    }
    100% {
      transform: translateX(100%) rotate(0deg);
    }
  }

  .sidebar-header {
    padding: 12px 16px;
    border-right: 1px solid rgba(6, 182, 212, 0.15);
    border-bottom: none;
    flex-shrink: 0;
    background: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(10px);
  }

  .app-logo {
    transition: all 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    position: relative;
  }

  .app-logo::before {
    top: -5px;
    left: -5px;
    right: -5px;
    bottom: -5px;
    animation: logoRotate 4s linear infinite;
  }

  .app-logo:hover {
    transform: translateY(-2px) scale(1.02);
  }

  .logo-text {
    font-size: 18px;
  }

  .sidebar-nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 20px 10px;
    overflow-x: auto;
    flex: 1;
  }

  .nav-item {
    padding: 14px 38px;
    border-radius: 16px;
    width: auto;
    min-width: 130px;
    flex-shrink: 0;
    justify-content: center;
    font-size: 14px;
    gap: 12px;
    margin: 0;
    transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    border: 1px solid transparent;
  }

  /* 移动端波纹效果 */
  .nav-item::before {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  }

  .nav-item:hover::before {
    width: 250px;
    height: 250px;
  }

  /* 移动端霓虹边框 */
  .nav-item::after {
    border-radius: 16px;
    animation: gradientMove 3s ease infinite;
    opacity: 0;
    transition: opacity 0.4s ease;
  }

  /* 移动端点击动画 */
  .nav-item:active {
    transform: translateY(-2px) scale(0.95);
  }

  .nav-item:active::before {
    width: 300px;
    height: 300px;
    transition: all 0.1s ease;
  }

  /* 移动端文字效果 */
  .nav-item:hover span {
    letter-spacing: 0.8px;
  }

  .nav-item.active span {
    font-weight: 600;
    letter-spacing: 0.6px;
  }

  .main-content {
    padding: 0;
    flex: 1;
  }

  .token-generator-header,
  .view-header-bar {
    padding: 12px 20px;
  }

  .header-left h2 {
    font-size: 20px;
  }

  .plugin-card {
    flex-direction: column;
    text-align: center;
    gap: 12px;
  }

  .plugin-actions {
    justify-content: center;
  }

  .status-messages-container {
    left: 16px;
    right: 16px;
  }

  .status-toast {
    left: 16px !important;
    width: auto;
    min-width: 150px;
    max-width: calc(100vw - 32px);
  }
}

/* Info hint styles */
.info-hint {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  margin-top: 12px;
  padding: 12px 16px;
  background: rgba(239, 246, 255, 0.5);
  border: 1px solid rgba(191, 219, 254, 0.5);
  border-radius: 10px;
  color: #2563eb;
  font-size: 13px;
  line-height: 1.5;
  backdrop-filter: blur(8px);
}

.info-hint svg {
  flex-shrink: 0;
  opacity: 0.8;
  margin-top: 1px;
}

/* Generate section styles */
.generate-section,
.generated-section {
  margin-top: 16px;
  min-height: 120px; /* 确保两个状态高度一致，防止布局跳动 */
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
}

.generate-input {
  background: rgba(248, 250, 252, 0.8) !important;
  color: #94a3b8;
}

.input-with-button .btn {
  white-space: nowrap;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
}

/* Integrated button styles */
.input-with-integrated-button {
  position: relative;
  display: flex;
  align-items: stretch;
  border: 1px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  overflow: hidden;
  background: rgba(255, 255, 255, 0.8);
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.input-with-integrated-button input {
  flex: 1;
  padding: 14px 18px;
  border: none;
  outline: none;
  font-size: 15px;
  background: transparent;
  border-radius: 0;
  color: #1e293b;
}

.input-with-integrated-button input:focus {
  outline: none;
  box-shadow: none;
}

.input-with-integrated-button:focus-within {
  border-color: #6366f1;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
  background: #ffffff;
}

.input-with-integrated-button:hover {
  border-color: rgba(99, 102, 241, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.integrated-btn {
  border: none;
  padding: 14px 24px;
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 0;
  flex-shrink: 0;
  border-left: 1px solid rgba(226, 232, 240, 0.3);
  position: relative;
  overflow: hidden;
}

.integrated-btn::before {
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

.integrated-btn:hover:not(:disabled)::before {
  left: 100%;
}

.integrated-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.1);
}

.integrated-btn:active:not(:disabled) {
  transform: scale(0.98);
}

.integrated-btn:disabled {
  background: #e2e8f0;
  color: #94a3b8;
  cursor: not-allowed;
}

.integrated-btn.loading {
  opacity: 0.8;
}

.integrated-btn.primary {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
}

.integrated-btn.secondary {
  background: linear-gradient(135deg, #64748b 0%, #475569 100%);
}

/* Section header styles - 已废弃，使用新的step-header */

/* Reset button styles */
.reset-btn {
  background: rgba(241, 245, 249, 0.8);
  border: 1px solid rgba(226, 232, 240, 0.6);
  border-radius: 10px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  color: #64748b;
  padding: 0;
  position: relative;
  overflow: hidden;
}

.reset-btn::before {
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

.reset-btn:hover::before {
  left: 100%;
}

.reset-btn:hover {
  background: rgba(226, 232, 240, 0.9);
  border-color: rgba(203, 213, 225, 0.8);
  color: #475569;
  transform: rotate(180deg);
}

.reset-btn:active {
  transform: rotate(180deg) scale(0.95);
}

/* 防封插件弹窗内容样式 */
.plugin-modal-description {
  margin-bottom: 24px;
  color: #666;
  text-align: center;
  line-height: 1.5;
}

/* 删除确认弹窗样式 */
.dialog-buttons {
  display: flex;
  flex-direction: row;
  gap: 12px;
}

/* 删除确认弹窗中的 dialog-btn 特殊样式 */
.delete-confirm .dialog-btn {
  flex: 1;
}

/* 赞赏弹窗样式 */
.appreciation-content {
  text-align: center;
}

.appreciation-qr-container {
  display: flex;
  justify-content: center;
  margin: 24px 0;
  padding: 20px;
  background: rgba(248, 250, 252, 0.5);
  border-radius: 12px;
  border: 1px solid rgba(226, 232, 240, 0.3);
}

.appreciation-qr {
  width: 200px;
  height: 200px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.appreciation-text {
  margin-top: 20px;
}

.appreciation-text p {
  margin: 8px 0;
  color: #666;
  font-size: 14px;
  line-height: 1.5;
}

/* 删除确认弹窗样式 - 与邮箱管理样式统一 */
.delete-confirm-content {
  text-align: center;
  padding: 10px;
}

.confirm-icon {
  color: #dc2626;
  margin: 0 20px;
  filter: drop-shadow(0 2px 4px rgba(220, 38, 38, 0.1));
  animation: iconPulse 2s ease-in-out infinite;
}

@keyframes iconPulse {
  0%,
  100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

.delete-confirm-content h4 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.delete-confirm-content p {
  margin: 0 0 12px 0;
  color: #64748b;
  line-height: 1.6;
}

.delete-confirm-content strong {
  color: #dc2626;
  font-weight: 600;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 6px;
}

/* 强制同步确认弹窗样式 */
.force-confirm-content {
  text-align: center;
}

.warning-icon-container {
  display: flex;
  justify-content: center;
  margin-bottom: 20px;
}

.warning-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.1),
    rgba(239, 68, 68, 0.1)
  );
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #f59e0b;
}

.warning-details {
  margin: 24px 0;
  padding: 16px;
  background: linear-gradient(
    135deg,
    rgba(254, 243, 199, 0.4) 0%,
    rgba(254, 215, 170, 0.2) 100%
  );
  border-radius: 12px;
  border: 1px solid rgba(245, 158, 11, 0.3);
  box-shadow: 0 2px 8px rgba(245, 158, 11, 0.1);
}

.warning-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 8px 0;
  color: #92400e;
  font-size: 14px;
  line-height: 1.5;
}

.warning-item svg {
  color: #f59e0b;
  flex-shrink: 0;
  width: 18px;
  height: 18px;
}

.force-confirm-footer {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.btn-cancel,
.btn-confirm {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  flex: 1;
  justify-content: center;
  position: relative;
  overflow: hidden;
}

.btn-cancel {
  background: rgba(241, 245, 249, 0.8);
  color: #475569;
  border: 1px solid rgba(226, 232, 240, 0.6);
}

.btn-cancel::before {
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

.btn-cancel:hover {
  background: rgba(226, 232, 240, 0.9);
  border-color: rgba(203, 213, 225, 0.8);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.btn-cancel:hover::before {
  left: 100%;
}

.btn-confirm {
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
  box-shadow: 0 4px 14px rgba(239, 68, 68, 0.25);
}

.btn-confirm::before {
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

.btn-confirm:hover {
  background: linear-gradient(135deg, #dc2626, #b91c1c);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(239, 68, 68, 0.35);
}

.btn-confirm:hover::before {
  left: 100%;
}

/* 同步历史弹窗样式 */
.sync-history-content {
  max-height: 60vh;
  overflow-y: auto;
}

.btn-clear-history {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 6px;
  padding: 8px 16px;
  background: #fef2f2;
  color: #dc2626;
  border: 1px solid #fecaca;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  margin-bottom: 16px;
  position: relative;
  overflow: hidden;
}

.btn-clear-history::before {
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

.btn-clear-history:hover::before {
  left: 100%;
}

.btn-clear-history:hover {
  background: #fecaca;
  border-color: #f87171;
  transform: translateY(-1px);
}

/* 平台选择弹窗样式 */
.platform-selector-content {
  max-height: 60vh;
  overflow-y: auto;
}

.plugin-modal-content p {
  margin: 0 0 24px 0;
  color: #64748b;
  font-size: 15px;
  line-height: 1.6;
}

.plugin-options-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  max-height: 56vh;
}

.plugin-option-card {
  background: rgba(255, 255, 255, 0.8);
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 16px;
  padding: 24px;
  text-align: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.plugin-option-card:hover {
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
  transform: translateY(-2px);
}

.plugin-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px auto;
}

.plugin-icon.vscode {
  background: linear-gradient(135deg, #007acc 0%, #005a9e 100%);
  color: white;
}

.plugin-icon.jetbrains {
  background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
  color: white;
}

.plugin-option-card h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}

.plugin-option-card p {
  margin: 0 0 20px 0;
  font-size: 14px;
  color: #64748b;
  line-height: 1.5;
}

.plugin-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.vscode-btn {
  background: linear-gradient(135deg, #007acc 0%, #005a9e 100%);
  color: white;
  position: relative;
  overflow: hidden;
}

.vscode-btn::before {
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

.vscode-btn:hover {
  box-shadow: 0 4px 12px rgba(0, 122, 204, 0.3);
  transform: translateY(-1px);
}

.vscode-btn:hover::before {
  left: 100%;
}

.jetbrains-btn {
  background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
  color: white;
  position: relative;
  overflow: hidden;
}

.jetbrains-btn::before {
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

.jetbrains-btn:hover {
  box-shadow: 0 4px 12px rgba(255, 107, 53, 0.3);
  transform: translateY(-1px);
}

.jetbrains-btn:hover::before {
  left: 100%;
}



/* 设置页面样式  */
.settings-section:first-child {
  padding-top: 20px;
}
.settings-section {
  padding: 0 20px;
}
.settings-section:last-child {
  padding-bottom: 20px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid rgba(59, 130, 246, 0.1);
}

.section-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.1) 0%,
    rgba(139, 92, 246, 0.1) 100%
  );
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.section-icon svg {
  color: #3b82f6;
  width: 20px;
  height: 20px;
}

.section-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.section-description {
  color: #64748b;
  font-size: 14px;
  line-height: 1.6;
  margin: 0 0 24px 0;
}

.setting-item {
  margin-bottom: 32px;
  padding: 24px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 16px;
  border: 1px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(10px);
  transition: all 0.3s ease;
}

.setting-item:hover {
  border-color: rgba(59, 130, 246, 0.2);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1);
}
.setting-item-create-tag {
  margin-top: 20px;
}

.threshold-config-wrapper {
  display: flex;
  flex-direction: row;
  align-items: stretch;
  gap: 16px;
}

.btn-config-threshold {
  padding: 12px 20px;
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
}

.btn-config-threshold::before {
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

.btn-config-threshold:hover::before {
  left: 100%;
}

.btn-config-threshold:hover {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.btn-config-threshold svg {
  flex-shrink: 0;
}

.btn-config-threshold.secondary {
  background: linear-gradient(135deg, rgba(226, 232, 240, 0.8) 0%, rgba(203, 213, 225, 0.8) 100%);
  color: #475569;
  box-shadow: 0 2px 8px rgba(100, 116, 139, 0.15);
}

.btn-config-threshold.secondary:hover {
  background: linear-gradient(135deg, rgba(203, 213, 225, 0.9) 0%, rgba(148, 163, 184, 0.9) 100%);
  box-shadow: 0 4px 12px rgba(100, 116, 139, 0.25);
}

.threshold-config-wrapper {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.threshold-display {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1;
}

.threshold-section {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
  white-space: nowrap;
}

.threshold-label {
  font-size: 12px;
  font-weight: 600;
  color: #475569;
  flex-shrink: 0;
}

.threshold-rules {
  display: flex;
  flex-direction: row;
  gap: 4px;
  flex-wrap: nowrap;
  flex: 1;
}

.threshold-rules .rule-item {
  font-size: 12px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 4px;
  white-space: nowrap;
  color: #475569;
}

.threshold-rules .color-dot {
  width: 10px;
  height: 10px;
  border-radius: 2px;
  flex-shrink: 0;
}

.threshold-rules .rule-item.red .color-dot {
  background: #b91c1c;
}

.threshold-rules .rule-item.yellow .color-dot {
  background: #a16207;
}

.threshold-rules .rule-item.green .color-dot {
  background: #15803d;
}

.setting-label {
  margin-bottom: 16px;
}

.setting-label label {
  display: block;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
  margin-bottom: 4px;
}

.setting-help {
  display: block;
  font-size: 13px;
  color: #64748b;
  line-height: 1.4;
}

.setting-control {
  flex: 1;
}

.path-input-group {
  display: flex;
  gap: 12px;
  align-items: center;
  margin-bottom: 12px;
}

.path-input {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 10px;
  background: #f8fafc;
  color: #475569;
  font-size: 14px;
  font-family: "SF Mono", "Monaco", "Inconsolata", "Roboto Mono", "Consolas",
    monospace;
  transition: all 0.3s ease;
  min-height: 44px;
  box-sizing: border-box;
}

.path-input:focus {
  outline: none;
  border-color: #3b82f6;
  background: #ffffff;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.path-input.loading {
  background: linear-gradient(90deg, #f8fafc 25%, #e2e8f0 50%, #f8fafc 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  color: #94a3b8;
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

.btn-select-path {
  padding: 12px 20px;
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  position: relative;
  overflow: hidden;
}

.btn-select-path::before {
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

.btn-select-path:hover::before {
  left: 100%;
}

.btn-select-path:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.btn-select-path:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.btn-open-folder {
  padding: 12px;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 44px;
  height: 44px;
  box-shadow: 0 4px 14px rgba(16, 185, 129, 0.25);
  position: relative;
  overflow: hidden;
}

.btn-open-folder::before {
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

.btn-open-folder:hover::before {
  left: 100%;
}

.btn-open-folder:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(16, 185, 129, 0.35);
}

.btn-open-folder:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.setting-note {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 12px 16px;
  background: rgba(59, 130, 246, 0.05);
  border: 1px solid rgba(59, 130, 246, 0.1);
  border-radius: 10px;
  font-size: 13px;
  color: #475569;
  line-height: 1.5;
}

.setting-note svg {
  color: #3b82f6;
  margin-top: 1px;
  flex-shrink: 0;
}

.current-files {
  background: #f1f5f9;
  border-radius: 12px;
  padding: 16px;
  border: 1px solid #e2e8f0;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-name {
  font-weight: 600;
  color: #1e293b;
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-name svg {
  color: #3b82f6;
  flex-shrink: 0;
}

.file-path {
  font-family: "SF Mono", "Monaco", "Inconsolata", "Roboto Mono", "Consolas",
    monospace;
  font-size: 12px;
  color: #64748b;
  word-break: break-all;
  line-height: 1.4;
  margin-top: 2px;
}

.setting-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
  flex-wrap: wrap;
}

.btn-reset,
.btn-open {
  padding: 12px 20px;
  border: 2px solid transparent;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  text-decoration: none;
}

.btn-reset {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
}

.btn-reset:hover {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

.btn-open {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
}

.btn-open:hover {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.app-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 12px;
  border: 1px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(5px);
  flex-wrap: nowrap !important;
  min-height: 56px;
}

.info-label {
  font-weight: 500;
  color: #475569;
  font-size: 14px;
}

.info-value {
  font-weight: 600;
  color: #1e293b;
  font-size: 14px;
}

.info-value-with-action {
  display: flex;
  align-items: center;
  gap: 8px;
}

.info-action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background: #4f46e5;
  color: white;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
  position: relative;
  overflow: hidden;
}

.info-action-btn::before {
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

.info-action-btn:hover::before {
  left: 100%;
}

.info-action-btn:hover:not(:disabled) {
  background: #4338ca;
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.info-action-btn.primary {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
}

.info-action-btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}

.info-action-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* 仓库信息特殊样式 */
/* GitHub 信息项样式 */
.github-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.github-repo {
  display: flex;
  align-items: center;
}

.github-tip-line {
  display: flex;
  align-items: center;
  gap: 8px;
}

.github-icon {
  color: #333;
  flex-shrink: 0;
}

.github-tip {
  margin: 0;
  font-size: 12px;
  line-height: 1.4;
  color: #666;
}

.app-actions {
  display: flex;
  gap: 12px;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid rgba(226, 232, 240, 0.5);
}

.app-action-btn {
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s ease;
  flex: 1;
  justify-content: center;
}

.app-action-btn.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
}

.app-action-btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.app-action-btn.primary:disabled {
  background: #9ca3af;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.app-action-btn.secondary {
  background: rgba(255, 255, 255, 0.8);
  color: #475569;
  border: 1px solid rgba(226, 232, 240, 0.8);
}

.app-action-btn.secondary:hover {
  background: rgba(255, 255, 255, 0.95);
  border-color: rgba(226, 232, 240, 1);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}



/* 移除小屏幕时的垂直布局，始终保持一行显示 */



/* 顶部标签页导航（小窗口时显示） */
.top-tabs-nav {
  display: none;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(226, 232, 240, 0.5);
  padding: 0 8px;
  gap: 2px;
  overflow-x: auto;
  overflow-y: hidden;
  flex-shrink: 0;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.top-tabs-nav::-webkit-scrollbar {
  display: none;
}

.top-tab-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 10px 12px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: #64748b;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  flex-shrink: 0;
}

.top-tab-item svg {
  width: 14px;
  height: 14px;
  opacity: 0.7;
  transition: opacity 0.2s ease;
  flex-shrink: 0;
}

.top-tab-item span {
  flex-shrink: 0;
}

.top-tab-item:hover {
  color: #4f46e5;
  background: rgba(79, 70, 229, 0.05);
}

.top-tab-item:hover svg {
  opacity: 1;
}

.top-tab-item.active {
  color: #4f46e5;
  border-bottom-color: #4f46e5;
  background: rgba(79, 70, 229, 0.08);
}

.top-tab-item.active svg {
  opacity: 1;
}

/* 小窗口时隐藏侧边栏，显示顶部标签页 */
@media (max-width: 900px) {
  .sidebar {
    display: none;
  }

  .top-tabs-nav {
    display: flex;
  }

  .main-content {
    margin-left: 0;
  }

  .modal-container-overlay {
    left: 0;
  }
}

/* WebDAV云同步样式 */
/* 平台和服务器地址布局 */
.platform-group {
  flex: 1;
  min-width: 200px;
}

.server-group {
  flex: 1.5;
  min-width: 250px;
}

/* 紧凑型平台选择按钮样式 */
.platform-selector-btn-compact {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: rgba(255, 255, 255, 0.8);
  border: 2px solid rgba(226, 232, 240, 0.6);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(8px);
  font-size: 14px;
  color: #1e293b;
  height: 47px;
}

.platform-selector-btn-compact:hover {
  background: rgba(239, 246, 255, 0.9);
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.1);
  transform: translateY(-1px);
}

.selected-platform-icon-compact {
  font-size: 18px;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.selected-platform-name-compact {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
  flex: 1;
  text-align: left;
  margin-left: 8px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.dropdown-icon-compact {
  color: #94a3b8;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.platform-selector-btn-compact:hover .dropdown-icon-compact {
  color: #3b82f6;
  transform: scale(1.1);
}

/* 平台选择弹窗样式 */
.platform-selector-modal {
  background: linear-gradient(145deg, #ffffff 0%, #fafbfc 100%);
  border-radius: 20px;
  padding: 0;
  max-width: 700px;
  width: 90vw;
  max-height: 80vh;
  overflow: hidden;
  box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.25),
    0 8px 16px -8px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  animation: modalSlideUp 0.3s ease-out;
}

.platform-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  background: rgba(248, 250, 252, 0.5);
}

.platform-modal-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
}

.platform-modal-body {
  padding: 24px 32px 32px;
  max-height: 60vh;
  overflow-y: auto;
}

.platform-modal-body p {
  margin: 0 0 20px 0;
  color: #64748b;
  font-size: 14px;
  line-height: 1.6;
}

.platform-modal-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 12px;
}

.platform-modal-card {
  position: relative;
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: rgba(248, 250, 252, 0.8);
  border: 2px solid rgba(226, 232, 240, 0.5);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(8px);
}

.platform-modal-card:hover {
  background: rgba(239, 246, 255, 0.9);
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.1);
  transform: translateY(-1px);
}

.platform-modal-card.active {
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.12),
    rgba(37, 99, 235, 0.08)
  );
  border-color: #3b82f6;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.15);
}

.platform-modal-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  font-size: 24px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  border: 1px solid rgba(226, 232, 240, 0.5);
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.platform-modal-card:hover .platform-modal-icon {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.3);
  transform: scale(1.05);
}

.platform-modal-card.active .platform-modal-icon {
  background: rgba(59, 130, 246, 0.15);
  border-color: #3b82f6;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.platform-modal-info {
  flex: 1;
  min-width: 0;
}

.platform-modal-name {
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
  margin-bottom: 4px;
  line-height: 1.3;
}

.platform-modal-desc {
  font-size: 13px;
  color: #64748b;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.platform-modal-card.active .platform-modal-name {
  color: #3b82f6;
}

.platform-modal-card.active .platform-modal-desc {
  color: #1e40af;
}

.platform-selected-mark {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 28px;
  height: 28px;
  background: #3b82f6;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
}

.webdav-config-group {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.input-row {
  display: flex;
  gap: 16px;
}

.input-group {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-label {
  font-size: 14px;
  font-weight: 500;
  color: #475569;
}

.webdav-input {
  padding: 12px 16px;
  border: 2px solid rgba(226, 232, 240, 0.6);
  border-radius: 10px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(5px);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  height: 47px;
}

.webdav-input:focus {
  outline: none;
  border-color: #3b82f6;
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* 密码输入框容器 - 提供相对定位上下文 */
.password-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

/* 为密码输入框内的按钮留出空间 */
.password-input-wrapper .password-input {
  padding-right: 50px !important;
}

/* 密码输入框 - 确保与其他输入框样式一致，为按钮留出空间 */
.password-input {
  /* 完全继承webdav-input的基础样式，确保高度一致 */
  padding: 12px 16px !important;
  border: 2px solid rgba(226, 232, 240, 0.6) !important;
  border-radius: 10px !important;
  font-size: 14px !important;
  background: rgba(255, 255, 255, 0.8) !important;
  backdrop-filter: blur(5px) !important;
  width: 100%;
  box-sizing: border-box !important;

  /* 只对边框和背景应用过渡效果，避免type切换时的动画 */
  transition: border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important;

  /* 强制统一样式，避免type切换时的样式差异 */
  font-family: inherit !important;
  line-height: normal !important;
  letter-spacing: normal !important;
  text-align: left !important;
  color: inherit !important;
  -webkit-text-security: none !important;

  /* 禁用浏览器默认的type切换动画 */
  -webkit-transition: border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important;
  -moz-transition: border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important;
}

/* 当type为password时，确保样式一致 */
.password-input[type="password"] {
  font-family: inherit !important;
  -webkit-text-security: disc !important;
  /* 禁用密码字段的默认动画 */
  animation: none !important;
  -webkit-animation: none !important;
  -moz-animation: none !important;
  -o-animation: none !important;
  -ms-animation: none !important;
}

/* 当type为text时，确保样式一致 */
.password-input[type="text"] {
  font-family: inherit !important;
  -webkit-text-security: none !important;
  /* 禁用文本字段的默认动画 */
  animation: none !important;
  -webkit-animation: none !important;
  -moz-animation: none !important;
  -o-animation: none !important;
  -ms-animation: none !important;
}

/* 密码输入框焦点状态 */
.password-input:focus {
  outline: none !important;
  border-color: #3b82f6 !important;
  background: rgba(255, 255, 255, 0.95) !important;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1) !important;
}

/* 密码切换按钮 - 绝对定位在输入框内部右侧 */
.password-toggle-btn {
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  color: #6b7280;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  width: 24px;
  height: 24px;
}

.password-toggle-btn:hover {
  color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
  transform: translateY(-50%) scale(1.1);
}

.password-toggle-btn:active {
  transform: translateY(-50%) scale(0.9);
}

.password-toggle-btn svg {
  transition: all 0.2s ease;
}

.webdav-actions {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}

.btn-test-connection,
.btn-save-config {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-test-connection {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
}

.btn-test-connection:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.btn-save-config {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
}

.btn-save-config:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.btn-test-connection:disabled,
.btn-save-config:disabled {
  background: linear-gradient(135deg, #94a3b8 0%, #64748b 100%);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.sync-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.btn-sync,
.btn-upload,
.btn-download {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 18px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.btn-sync::before,
.btn-upload::before,
.btn-download::before {
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

.btn-sync:hover:not(:disabled)::before,
.btn-upload:hover:not(:disabled)::before,
.btn-download:hover:not(:disabled)::before {
  left: 100%;
}

.btn-sync {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  color: white;
}

.btn-sync:hover:not(:disabled) {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);
}

.btn-upload {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
}

.btn-upload:hover:not(:disabled) {
  background: linear-gradient(135deg, #d97706 0%, #b45309 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
}

.btn-download {
  background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);
  color: white;
}

.btn-download:hover:not(:disabled) {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(6, 182, 212, 0.3);
}

.btn-sync:disabled,
.btn-upload:disabled,
.btn-download:disabled {
  background: linear-gradient(135deg, #94a3b8 0%, #64748b 100%);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

/* 自动上传和自动下载开关样式 */
.auto-sync-toggle {
  margin-bottom: 20px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.6);
  border-radius: 10px;
  border: 1px solid rgba(226, 232, 240, 0.5);
}

.auto-sync-row {
  display: flex;
  gap: 24px;
  align-items: flex-start;
}

.auto-sync-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* 开关容器 - 只有开关可点击 */
.toggle-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 只包裹开关的 label - 只有这部分可点击 */
.toggle-label-switch {
  display: inline-flex;
  cursor: pointer;
  user-select: none;
}

/* 旧的 toggle-label 保持兼容性（如果其他地方还在用） */
.toggle-label {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  user-select: none;
}

.toggle-checkbox {
  display: none;
}

.toggle-slider {
  position: relative;
  width: 48px;
  height: 24px;
  background: #cbd5e1;
  border-radius: 24px;
  transition: background 0.3s ease;
  flex-shrink: 0;
}

.toggle-slider::before {
  content: "";
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: transform 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.toggle-checkbox:checked + .toggle-slider {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}

.toggle-checkbox:checked + .toggle-slider::before {
  transform: translateX(24px);
}

.toggle-text {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  user-select: none;
  pointer-events: none; /* 文字不可点击 */
}

.toggle-help {
  font-size: 12px;
  color: #64748b;
  margin-top: 12px;
}

/* 同步状态指示器样式 */
.sync-status-indicator {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.08),
    rgba(147, 51, 234, 0.08)
  );
  border: 1px solid rgba(59, 130, 246, 0.15);
  border-radius: 12px;
  margin-bottom: 16px;
  animation: pulse-border 2s ease-in-out infinite;
}

.sync-status-indicator .status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  color: #3b82f6;
}

.sync-status-indicator .status-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sync-status-indicator .status-message {
  font-weight: 500;
  color: #1e293b;
  font-size: 14px;
}

.sync-status-indicator .status-duration {
  font-size: 12px;
  color: #64748b;
  font-weight: 400;
}

@keyframes pulse-border {
  0%,
  100% {
    border-color: rgba(59, 130, 246, 0.15);
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.1);
  }
  50% {
    border-color: rgba(59, 130, 246, 0.25);
    box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.05);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* 同步历史按钮样式 */
.btn-history {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: linear-gradient(135deg, #f8fafc, #e2e8f0);
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  color: #475569;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  text-decoration: none;
}

.btn-history:hover {
  background: linear-gradient(135deg, #e2e8f0, #cbd5e1);
  border-color: #94a3b8;
  color: #334155;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.btn-history.active {
  background: linear-gradient(135deg, #3b82f6, #2563eb);
  border-color: #2563eb;
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
}

.btn-history.active:hover {
  background: linear-gradient(135deg, #2563eb, #1d4ed8);
  border-color: #1d4ed8;
}

/* 同步历史记录样式 */
.sync-history {
  margin-top: 16px;
  padding: 20px;
  background: rgba(248, 250, 252, 0.8);
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  max-height: 400px;
  overflow: hidden;
}

.history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e2e8f0;
}

.history-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-clear-history {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 6px;
  color: #dc2626;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-clear-history:hover {
  background: rgba(239, 68, 68, 0.15);
  border-color: rgba(239, 68, 68, 0.3);
  color: #b91c1c;
}

.history-list {
  max-height: 300px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #cbd5e1 transparent;
}

.history-list::-webkit-scrollbar {
  width: 6px;
}

.history-list::-webkit-scrollbar-track {
  background: transparent;
}

.history-list::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.history-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  margin-bottom: 8px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  transition: all 0.2s ease;
  position: relative;
}

.history-item:hover {
  background: #f8fafc;
  border-color: #cbd5e1;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.history-item.success {
  border-left: 4px solid #10b981;
}

.history-item.error {
  border-left: 4px solid #ef4444;
}

.history-item.warning {
  border-left: 4px solid #f59e0b;
}

.history-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  background: rgba(148, 163, 184, 0.1);
  color: #64748b;
  flex-shrink: 0;
}

.history-item.success .history-icon {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.history-item.error .history-icon {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.history-item.warning .history-icon {
  background: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.history-content {
  flex: 1;
  min-width: 0;
}

.history-message {
  font-size: 13px;
  font-weight: 500;
  color: #1e293b;
  margin-bottom: 4px;
  word-break: break-word;
}

.history-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 11px;
  color: #64748b;
}

.history-duration {
  padding: 2px 6px;
  background: rgba(148, 163, 184, 0.1);
  border-radius: 4px;
  font-weight: 500;
}

.history-status {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  flex-shrink: 0;
}

.history-status.success {
  color: #10b981;
  background: rgba(16, 185, 129, 0.1);
}

.history-status.error {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

.history-status.warning {
  color: #f59e0b;
  background: rgba(245, 158, 11, 0.1);
}

.history-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  text-align: center;
}

.history-empty .empty-icon {
  color: #cbd5e1;
  margin-bottom: 12px;
}

.history-empty p {
  margin: 0;
  color: #64748b;
  font-size: 14px;
}

/* WebDAV配置头部样式 */
.webdav-config-header {
  display: flex;
  justify-content: between;
  align-items: center;
  gap: 12px;
}

.webdav-config-header label {
  margin: 0;
  flex: 1;
}

.connection-status-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border: 1px solid;
  transition: all 0.3s ease;
}

.connection-status-badge.status-connected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #059669;
}

.connection-status-badge.status-error {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.3);
  color: #dc2626;
}

.connection-status-badge.status-disconnected {
  background: rgba(148, 163, 184, 0.1);
  border-color: rgba(148, 163, 184, 0.3);
  color: #64748b;
}

/* 操作状态指示器样式（覆盖式） */
.operation-status-indicator {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 16px;
  background: rgba(248, 250, 252, 0.95);
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  margin: 12px 0;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot-badge {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot-badge.success {
  background: #10b981;
}

.status-dot-badge.error {
  background: #ef4444;
}

.status-dot-badge.warning {
  background: #f59e0b;
}

.status-dot-badge.info {
  background: #3b82f6;
}

.status-text {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
}

/* 统一状态指示器样式（保留用于兼容） */
.unified-status-indicator {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(248, 250, 252, 0.8);
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  margin: 12px 0;
  transition: all 0.3s ease;
}

.status-dot {
  position: relative;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.status-dot.success {
  background: #10b981;
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

.status-dot.error {
  background: #ef4444;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.status-dot.warning {
  background: #f59e0b;
  box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
}

.status-dot.active {
  background: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.status-dot.idle {
  background: #94a3b8;
  box-shadow: 0 0 0 3px rgba(148, 163, 184, 0.1);
}

.status-dot.active .dot-inner,
.status-dot.warning .dot-inner {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 6px;
  height: 6px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 0.8;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 0.4;
    transform: translate(-50%, -50%) scale(1.2);
  }
}

.status-content-area {
  flex: 1;
  min-width: 0;
}

.status-main-message {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  margin-bottom: 4px;
  line-height: 1.4;
}

.status-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 6px;
}

.progress-bar-mini {
  flex: 1;
  height: 3px;
  background: rgba(226, 232, 240, 0.6);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill-mini {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6 0%, #60a5fa 50%, #3b82f6 100%);
  background-size: 200px 100%;
  border-radius: 2px;
  transition: width 0.3s ease;
  animation: shimmer 2s ease-in-out infinite;
}

@keyframes shimmer {
  0% {
    background-position: -200px 0;
  }
  100% {
    background-position: calc(200px + 100%) 0;
  }
}

.progress-text-mini {
  font-size: 11px;
  color: #64748b;
  font-weight: 500;
  white-space: nowrap;
}

/* 统一按钮行样式 */
.webdav-unified-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
  margin: 16px 0;
  padding: 16px;
  background: rgba(248, 250, 252, 0.6);
  border: 1px solid #e2e8f0;
  border-radius: 12px;
}

.webdav-unified-actions .divider {
  width: 1px;
  height: 24px;
  background: #cbd5e1;
  margin: 0 4px;
}

.webdav-unified-actions button {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  text-decoration: none;
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.webdav-unified-actions button::before {
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

.webdav-unified-actions button:hover::before {
  left: 100%;
}

.webdav-unified-actions .btn-test-connection {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(16, 185, 129, 0.25);
}

.webdav-unified-actions .btn-test-connection:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(16, 185, 129, 0.35);
}

.webdav-unified-actions .btn-save-config {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(59, 130, 246, 0.25);
}

.webdav-unified-actions .btn-save-config:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.35);
}

.webdav-unified-actions .btn-upload {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(245, 158, 11, 0.25);
}

.webdav-unified-actions .btn-upload:hover:not(:disabled) {
  background: linear-gradient(135deg, #d97706 0%, #b45309 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(245, 158, 11, 0.35);
}

.webdav-unified-actions .btn-download {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(139, 92, 246, 0.25);
}

.webdav-unified-actions .btn-download:hover:not(:disabled) {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(139, 92, 246, 0.35);
}

.webdav-unified-actions .btn-history {
  background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(107, 114, 128, 0.25);
}

.webdav-unified-actions .btn-history:hover:not(:disabled) {
  background: linear-gradient(135deg, #4b5563 0%, #374151 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(107, 114, 128, 0.35);
}

.webdav-unified-actions button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.webdav-unified-actions button svg {
  transition: transform 0.25s ease;
}

.webdav-unified-actions button:hover:not(:disabled) svg {
  transform: scale(1.1);
}

/* 赞赏弹窗样式 */
.appreciation-modal {
  background: linear-gradient(145deg, #ffffff 0%, #fefefe 100%);
  border-radius: 20px;
  padding: 0;
  max-width: 400px;
  width: 90vw;
  overflow: hidden;
  box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.25),
    0 8px 16px -8px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  animation: modalSlideUp 0.3s ease-out;
}

.appreciation-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px 16px;
  background: linear-gradient(
    135deg,
    rgba(236, 72, 153, 0.05),
    rgba(219, 39, 119, 0.05)
  );
  border-bottom: 1px solid rgba(236, 72, 153, 0.1);
}

.appreciation-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
  display: flex;
  align-items: center;
  gap: 8px;
}

.appreciation-body {
  padding: 24px 32px 32px;
  text-align: center;
}

.appreciation-body p {
  margin: 0 0 20px 0;
  font-size: 15px;
  color: #475569;
  line-height: 1.6;
}

.appreciation-qr-container {
  display: flex;
  justify-content: center;
  margin: 20px 0;
  padding: 20px;
  background: rgba(248, 250, 252, 0.8);
  border-radius: 16px;
  border: 1px solid rgba(226, 232, 240, 0.5);
}

.appreciation-qr {
  max-width: 240px;
  width: 100%;
  height: auto;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease;
}

.appreciation-qr:hover {
  transform: scale(1.02);
}

.appreciation-text {
  margin-top: 16px;
}

.appreciation-text p {
  margin: 8px 0;
  font-size: 14px;
  color: #64748b;
}

.appreciation-text p:last-child {
  font-weight: 500;
  color: #1e293b;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .appreciation-modal {
    max-width: 95vw;
    margin: 20px;
  }

  .appreciation-header,
  .appreciation-body {
    padding-left: 20px;
    padding-right: 20px;
  }

  .appreciation-qr-container {
    margin: 10px 0;
    padding: 16px;
  }

  .appreciation-qr {
    max-width: 200px;
  }
}

/* 强制同步确认弹窗样式 */
.force-confirm-dialog {
  background: linear-gradient(145deg, #ffffff 0%, #fafbfc 100%);
  border-radius: 20px;
  padding: 0;
  max-width: 500px;
  width: 90vw;
  overflow: hidden;
  box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.25),
    0 8px 16px -8px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  animation: modalSlideUp 0.3s ease-out;
}

.force-confirm-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 24px 32px 16px;
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.05),
    rgba(239, 68, 68, 0.05)
  );
  border-bottom: 1px solid rgba(245, 158, 11, 0.1);
}

.warning-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  border-radius: 12px;
  color: white;
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
}

.force-confirm-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1e293b;
  flex: 1;
}

.force-confirm-body {
  padding: 24px 32px;
}

.force-confirm-body p {
  margin: 0 0 20px 0;
  font-size: 16px;
  color: #475569;
  line-height: 1.6;
}

.warning-details {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.warning-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(245, 158, 11, 0.05);
  border: 1px solid rgba(245, 158, 11, 0.1);
  border-radius: 8px;
  font-size: 14px;
  color: #475569;
}

.warning-item svg {
  color: #f59e0b;
  flex-shrink: 0;
}

.force-confirm-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 20px 32px 24px;
  background: rgba(248, 250, 252, 0.5);
  border-top: 1px solid rgba(226, 232, 240, 0.3);
}

/* 这些样式已在前面定义，此处保留以避免重复 */

/* 响应式设计 */
@media (max-width: 768px) {
  .force-confirm-dialog {
    max-width: 95vw;
    margin: 20px;
  }

  .force-confirm-header,
  .force-confirm-body,
  .force-confirm-footer {
    padding-left: 20px;
    padding-right: 20px;
  }

  .btn-cancel,
  .btn-confirm {
    justify-content: center;
  }
}

/* 统一模态框样式 - 现代化设计 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(15, 23, 42, 0.4);
  backdrop-filter: blur(12px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  animation: modalFadeIn 0.3s ease-out;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0px);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(12px);
  }
}

.modal-content {
  background: rgba(255, 255, 255, 0.98);
  border-radius: 24px;
  box-shadow: 0 32px 64px rgba(0, 0, 0, 0.12),
    0 0 0 1px rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(226, 232, 240, 0.3);
  backdrop-filter: blur(20px);
  max-width: 600px;
  width: 90vw;
  max-height: 85vh;
  overflow: hidden;
  /* animation: modalSlideUp 0.4s cubic-bezier(0.34, 1.56, 0.64, 1); */
}

@keyframes modalSlideUp {
  from {
    opacity: 0;
    transform: translateY(30px) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid #e2e8f0;
  background: linear-gradient(135deg, #f8fafc, #f1f5f9);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: rgba(148, 163, 184, 0.1);
  color: #64748b;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.modal-close:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.modal-body {
  padding: 24px;
  max-height: 60vh;
  overflow-y: auto;
}

.history-actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 20px;
}

.history-actions .btn-clear-history {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 8px;
  color: #dc2626;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.history-actions .btn-clear-history:hover {
  background: rgba(239, 68, 68, 0.15);
  border-color: rgba(239, 68, 68, 0.3);
  color: #b91c1c;
}

.history-list-modal {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item-modal {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  transition: all 0.2s ease;
  position: relative;
}

.history-item-modal:hover {
  background: #f1f5f9;
  border-color: #cbd5e1;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.history-item-modal.success {
  border-left: 4px solid #10b981;
}

.history-item-modal.error {
  border-left: 4px solid #ef4444;
}

.history-item-modal.warning {
  border-left: 4px solid #f59e0b;
}

.history-item-modal .history-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: rgba(148, 163, 184, 0.1);
  color: #64748b;
  flex-shrink: 0;
}

.history-item-modal.success .history-icon {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.history-item-modal.error .history-icon {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.history-item-modal.warning .history-icon {
  background: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.history-item-modal .history-content {
  flex: 1;
  min-width: 0;
}

.history-item-modal .history-message {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  margin-bottom: 6px;
  word-break: break-word;
}

.history-item-modal .history-meta {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: #64748b;
}

.history-item-modal .history-duration {
  padding: 2px 8px;
  background: rgba(148, 163, 184, 0.1);
  border-radius: 6px;
  font-weight: 500;
}

.history-item-modal .history-status {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  flex-shrink: 0;
}

.history-item-modal .history-status.success {
  color: #10b981;
  background: rgba(16, 185, 129, 0.1);
}

.history-item-modal .history-status.error {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

.history-item-modal .history-status.warning {
  color: #f59e0b;
  background: rgba(245, 158, 11, 0.1);
}

.history-empty-modal {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  text-align: center;
}

.history-empty-modal .empty-icon {
  color: #cbd5e1;
  margin-bottom: 16px;
}

.history-empty-modal h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #374151;
}

.history-empty-modal p {
  margin: 0;
  color: #64748b;
  font-size: 14px;
}

/* 响应式设计 - WebDAV */
@media (max-width: 768px) {
  /* 平台选择弹窗响应式 */
  .platform-modal-grid {
    grid-template-columns: 1fr;
    gap: 10px;
  }

  .platform-modal-card {
    padding: 12px;
    gap: 12px;
  }

  .platform-modal-icon {
    width: 40px;
    height: 40px;
    font-size: 20px;
  }

  .platform-modal-name {
    font-size: 15px;
  }

  .platform-modal-desc {
    font-size: 12px;
  }

  .platform-selector-modal {
    max-width: 95vw;
    margin: 20px;
  }

  .platform-modal-header,
  .platform-modal-body {
    padding-left: 20px;
    padding-right: 20px;
  }

  /* 移动端保持左右布局，但调整比例 */
  .platform-group {
    flex: 1.2;
    min-width: 140px;
  }

  .server-group {
    flex: 1.8;
    min-width: 180px;
  }

  .platform-selector-btn-compact {
    padding: 10px 12px;
    height: 44px;
  }

  .selected-platform-name-compact {
    font-size: 13px;
  }

  .selected-platform-icon-compact {
    font-size: 16px;
    width: 20px;
    height: 20px;
  }
}

/* 账号管理页面样式优化 */
.account-management-container {
  padding: 0;
  background: transparent;
}

/* 统一的头部按钮样式 */
.btn-header-unified {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  min-height: 40px;
  position: relative;
  overflow: hidden;
}

.btn-header-unified::before {
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

.btn-header-unified:hover::before {
  left: 100%;
}

.btn-header-unified.primary {
  background: #4f46e5;
  color: white;
  box-shadow: 0 2px 12px rgba(79, 70, 229, 0.25);
}

.btn-header-unified.primary:hover:not(:disabled) {
  background: #4338ca;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(79, 70, 229, 0.35);
}

.btn-header-unified.secondary {
  background: rgba(241, 245, 249, 0.9);
  color: #475569;
  border: 1px solid rgba(203, 213, 225, 0.6);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.btn-header-unified.secondary:hover:not(:disabled) {
  background: rgba(226, 232, 240, 0.95);
  border-color: rgba(148, 163, 184, 0.8);
  transform: translateY(-2px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1);
}

.btn-header-unified.purchase {
  background: #10b981;
  color: white;
  box-shadow: 0 2px 12px rgba(16, 185, 129, 0.25);
}

.btn-header-unified.purchase:hover:not(:disabled) {
  background: #059669;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(16, 185, 129, 0.35);
}

.btn-header-unified.danger {
  background: #ef4444;
  color: white;
  box-shadow: 0 2px 12px rgba(239, 68, 68, 0.25);
}

.btn-header-unified.danger:hover:not(:disabled) {
  background: #dc2626;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.35);
}

.btn-header-unified.special {
  background: #8b5cf6;
  color: white;
  box-shadow: 0 2px 12px rgba(139, 92, 246, 0.25);
}

.btn-header-unified.special:hover:not(:disabled) {
  background: #7c3aed;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(139, 92, 246, 0.35);
}

.btn-header-unified.appreciation {
  background: #ec4899;
  color: white;
  box-shadow: 0 2px 12px rgba(236, 72, 153, 0.25);
}

.btn-header-unified.appreciation:hover:not(:disabled) {
  background: #db2777;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(236, 72, 153, 0.35);
}

.btn-header-unified.community {
  background: #06b6d4;
  color: white;
  box-shadow: 0 2px 12px rgba(6, 182, 212, 0.25);
}

.btn-header-unified.community:hover:not(:disabled) {
  background: #0891b2;
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(6, 182, 212, 0.35);
}

.btn-header-unified:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.btn-header-unified.loading {
  opacity: 0.8;
  cursor: not-allowed;
  pointer-events: none;
}

.btn-header-unified svg {
  transition: transform 0.25s ease;
}

.btn-header-unified:hover:not(:disabled) svg {
  transform: scale(1.1);
}

/* Header按钮样式增强 */
.btn-header.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  border: none;
  padding: 10px 16px;
  border-radius: 10px;
  font-weight: 500;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn-header.primary:hover {
  transform: translateY(-2px);
}

.btn-header.primary svg {
  transition: all 0.2s ease;
}

.btn-header.primary:hover svg {
  transform: scale(1.1);
}

/* 邮箱管理按钮样式 */
.outlook-manager-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.btn-header-action {
  padding: 10px 16px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;

  transition: all 0.25s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
}

.btn-header-action.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
}

.btn-header-action.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
}

.btn-header-action.primary:hover::before {
  left: 100%;
}

.btn-header-action.secondary {
  background: linear-gradient(
    135deg,
    rgba(107, 114, 128, 0.08) 0%,
    rgba(75, 85, 99, 0.06) 100%
  );
  color: #4b5563;
  border: 1px solid rgba(107, 114, 128, 0.2);
}

.btn-header-action.secondary:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(107, 114, 128, 0.12) 0%,
    rgba(75, 85, 99, 0.08) 100%
  );
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(107, 114, 128, 0.15);
  border-color: rgba(107, 114, 128, 0.3);
}

.btn-header-action.secondary:hover::before {
  left: 100%;
}

.btn-header-action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

/* 账户数量徽章样式 - 学习账号管理的status-item */
.account-count-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px 8px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.account-count-badge::before {
  content: "●";
  font-size: 8px;
  line-height: 1;
}

/* 不同状态的颜色 */
.account-count-badge.status-active {
  background: #f0fdf4;
  color: #15803d;
  border-color: #bbf7d0;
}

.account-count-badge.status-active::before {
  color: #22c55e;
}

.account-count-badge.status-inactive {
  background: #fee2e2;
  color: #b91c1c;
  border-color: #fecaca;
}

.account-count-badge.status-inactive::before {
  color: #ef4444;
}

.account-count-badge.status-error {
  background: #fef3c7;
  color: #d97706;
  border-color: #fde68a;
}

.account-count-badge.status-error::before {
  color: #f59e0b;
}

.account-count-badge.status-unknown {
  background: #f3f4f6;
  color: #6b7280;
  border-color: #e5e7eb;
}

.account-count-badge.status-unknown::before {
  color: #9ca3af;
}

.account-count-badge.status-empty {
  background: #f3f4f6;
  color: #6b7280;
  border-color: #e5e7eb;
}

.account-count-badge.status-empty::before {
  color: #9ca3af;
}

@keyframes fadeInScale {
  from {
    opacity: 0;
    transform: scale(0.8);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.header-left {
  display: flex;

  align-items: center;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 40px;
}

/* 全局禁用右键菜单 */
* {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

/* 允许输入框和文本区域选择文本 */
input,
textarea,
[contenteditable="true"] {
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  user-select: text;
}

/* 禁用所有输入框的浏览器自动填充功能 */
input,
textarea,
select {
  /* 禁用浏览器自动填充背景色 */
  -webkit-box-shadow: 0 0 0 1000px transparent inset !important;
  box-shadow: 0 0 0 1000px transparent inset !important;
  /* 禁用浏览器自动填充文字颜色 */
  -webkit-text-fill-color: inherit !important;
}

/* 针对 Webkit 浏览器的自动填充样式重置 */
input:-webkit-autofill,
input:-webkit-autofill:hover,
input:-webkit-autofill:focus,
input:-webkit-autofill:active,
textarea:-webkit-autofill,
textarea:-webkit-autofill:hover,
textarea:-webkit-autofill:focus,
textarea:-webkit-autofill:active,
select:-webkit-autofill,
select:-webkit-autofill:hover,
select:-webkit-autofill:focus,
select:-webkit-autofill:active {
  /* 重置自动填充的背景色 */
  -webkit-box-shadow: 0 0 0 1000px transparent inset !important;
  /* 保持原有文字颜色 */
  -webkit-text-fill-color: inherit !important;
  /* 禁用过渡动画 */
  transition: background-color 5000s ease-in-out 0s !important;
}

/* 全局禁用输入框自动完成下拉列表 */
input::-webkit-contacts-auto-fill-button,
input::-webkit-credentials-auto-fill-button {
  visibility: hidden;
  display: none !important;
  pointer-events: none;
  height: 0;
  width: 0;
  margin: 0;
}

/* 禁用右键菜单的额外保护 */
body {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

/* 统一的弹窗按钮样式 */
.dialog-btn-unified {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  min-height: 44px;
  position: relative;
  overflow: hidden;
  flex: 1;
}

.dialog-btn-unified::before {
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

.dialog-btn-unified:hover::before {
  left: 100%;
}

.dialog-btn-unified.primary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(59, 130, 246, 0.25);
}

.dialog-btn-unified.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.35);
}

.dialog-btn-unified.secondary {
  background: linear-gradient(
    135deg,
    rgba(241, 245, 249, 0.9) 0%,
    rgba(226, 232, 240, 0.8) 100%
  );
  color: #475569;
  border: 1px solid rgba(203, 213, 225, 0.6);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.dialog-btn-unified.secondary:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(226, 232, 240, 0.95) 0%,
    rgba(203, 213, 225, 0.9) 100%
  );
  border-color: rgba(148, 163, 184, 0.8);
  transform: translateY(-2px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1);
}

.dialog-btn-unified.success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(16, 185, 129, 0.25);
}

.dialog-btn-unified.success:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(16, 185, 129, 0.35);
}

.dialog-btn-unified.warning {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(245, 158, 11, 0.25);
}

.dialog-btn-unified.warning:hover:not(:disabled) {
  background: linear-gradient(135deg, #d97706 0%, #b45309 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(245, 158, 11, 0.35);
}

.dialog-btn-unified.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 2px 12px rgba(239, 68, 68, 0.25);
}

.dialog-btn-unified.danger:hover:not(:disabled) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.35);
}

.dialog-btn-unified.cancel {
  background: linear-gradient(
    135deg,
    rgba(156, 163, 175, 0.1) 0%,
    rgba(107, 114, 128, 0.05) 100%
  );
  color: #6b7280;
  border: 1px solid rgba(156, 163, 175, 0.3);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.dialog-btn-unified.cancel:hover:not(:disabled) {
  background: linear-gradient(
    135deg,
    rgba(156, 163, 175, 0.15) 0%,
    rgba(107, 114, 128, 0.1) 100%
  );
  border-color: rgba(107, 114, 128, 0.4);
  transform: translateY(-2px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1);
}

.dialog-btn-unified:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.dialog-btn-unified svg {
  transition: transform 0.25s ease;
}

.dialog-btn-unified:hover:not(:disabled) svg {
  transform: scale(1.1);
}

/* 全局统一弹窗按钮样式 - 覆盖所有组件 */
.btn-modal-unified,
.btn {
  padding: 12px 20px !important;
  border-radius: 10px !important;
  font-size: 14px !important;
  font-weight: 500 !important;
  cursor: pointer !important;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1) !important;
  display: flex !important;
  align-items: center !important;
  gap: 8px !important;
  min-width: auto !important;
  justify-content: center !important;
  min-height: 44px !important;
  position: relative !important;
  overflow: hidden !important;
  letter-spacing: -0.01em !important;
}

.btn-modal-unified::before,
.btn::before {
  content: "" !important;
  position: absolute !important;
  top: 0 !important;
  left: -100% !important;
  width: 100% !important;
  height: 100% !important;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  ) !important;
  transition: left 0.5s !important;
}

.btn-modal-unified:hover::before,
.btn:hover::before {
  left: 100% !important;
}

.btn-modal-unified:disabled,
.btn:disabled {
  opacity: 0.6 !important;
  cursor: not-allowed !important;
  transform: none !important;
  box-shadow: none !important;
}

.btn-modal-unified svg,
.btn svg {
  transition: transform 0.25s ease !important;
}

.btn-modal-unified:hover:not(:disabled) svg,
.btn:hover:not(:disabled) svg {
  transform: scale(1.1) !important;
}
</style>
