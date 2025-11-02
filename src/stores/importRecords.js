import { ref, computed } from 'vue';

/**
 * 导入记录状态
 */
export const ImportStatus = {
  PENDING: 'pending',     // 进行中
  SUCCESS: 'success',     // 成功
  FAILED: 'failed'        // 失败
};

/**
 * 导入记录数据结构
 */
class ImportRecord {
  constructor(session, status = ImportStatus.PENDING) {
    this.id = Date.now() + Math.random(); // 唯一ID
    this.session = session;
    this.status = status;
    this.importTime = new Date();
    this.error = null;
    this.message = null;
  }
}

/**
 * 导入记录管理器（单例模式）
 */
class ImportRecordsManager {
  constructor() {
    // 所有导入记录（内存存储）
    this.records = ref([]);
  }

  /**
   * 添加导入记录（去重：如果已存在相同session，则更新状态而不是新增）
   */
  addRecord(session, status = ImportStatus.PENDING) {
    // 检查是否已存在相同的 session
    const existingRecord = this.records.value.find(r => r.session === session);

    if (existingRecord) {
      // 如果已存在，更新状态为 PENDING（重新开始导入）
      existingRecord.status = status;
      existingRecord.importTime = new Date();
      existingRecord.error = null;
      existingRecord.message = null;
      return existingRecord;
    }

    // 不存在则创建新记录
    const record = new ImportRecord(session, status);
    this.records.value.unshift(record); // 新记录放在最前面
    return record;
  }

  /**
   * 批量添加导入记录（去重：如果已存在相同session，则更新状态而不是新增）
   */
  addRecords(sessions) {
    const newRecords = [];

    sessions.forEach(session => {
      // 检查是否已存在相同的 session
      const existingRecord = this.records.value.find(r => r.session === session);

      if (existingRecord) {
        // 如果已存在，更新状态为 PENDING（重新开始导入）
        existingRecord.status = ImportStatus.PENDING;
        existingRecord.importTime = new Date();
        existingRecord.error = null;
        existingRecord.message = null;
        newRecords.push(existingRecord);
      } else {
        // 不存在则创建新记录
        const record = new ImportRecord(session, ImportStatus.PENDING);
        newRecords.push(record);
      }
    });

    // 只添加真正新创建的记录
    const recordsToAdd = newRecords.filter(r => !this.records.value.includes(r));
    this.records.value.unshift(...recordsToAdd);

    return newRecords;
  }

  /**
   * 更新记录状态
   */
  updateRecord(recordId, status, error = null, message = null) {
    const record = this.records.value.find(r => r.id === recordId);
    if (record) {
      record.status = status;
      record.error = error;
      record.message = message;
    }
  }

  /**
   * 删除记录
   */
  deleteRecord(recordId) {
    const index = this.records.value.findIndex(r => r.id === recordId);
    if (index !== -1) {
      this.records.value.splice(index, 1);
    }
  }

  /**
   * 清空所有记录
   */
  clearAll() {
    this.records.value = [];
  }

  /**
   * 获取所有记录
   */
  getAllRecords() {
    return this.records.value;
  }

  /**
   * 获取失败的记录
   */
  getFailedRecords() {
    return this.records.value.filter(r => r.status === ImportStatus.FAILED);
  }

  /**
   * 获取成功的记录
   */
  getSuccessRecords() {
    return this.records.value.filter(r => r.status === ImportStatus.SUCCESS);
  }

  /**
   * 获取进行中的记录
   */
  getPendingRecords() {
    return this.records.value.filter(r => r.status === ImportStatus.PENDING);
  }

  /**
   * 统计信息
   */
  getStats() {
    return computed(() => ({
      total: this.records.value.length,
      success: this.getSuccessRecords().length,
      failed: this.getFailedRecords().length,
      pending: this.getPendingRecords().length
    }));
  }
}

// 导出单例实例
export const importRecordsManager = new ImportRecordsManager();

// 导出响应式数据供组件使用
export const useImportRecords = () => {
  return {
    records: importRecordsManager.records,
    stats: importRecordsManager.getStats(),
    addRecord: (session, status) => importRecordsManager.addRecord(session, status),
    addRecords: (sessions) => importRecordsManager.addRecords(sessions),
    updateRecord: (recordId, status, error, message) => importRecordsManager.updateRecord(recordId, status, error, message),
    deleteRecord: (recordId) => importRecordsManager.deleteRecord(recordId),
    clearAll: () => importRecordsManager.clearAll(),
    getFailedRecords: () => importRecordsManager.getFailedRecords(),
    getSuccessRecords: () => importRecordsManager.getSuccessRecords(),
    getPendingRecords: () => importRecordsManager.getPendingRecords()
  };
};

