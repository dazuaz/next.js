use std::sync::Arc;

use anyhow::Result;
use turbo_tasks::{backend::CachedTaskType, TaskId};

use crate::{
    backend::{AnyOperation, TaskDataCategory},
    data::{CachedDataItem, CachedDataUpdate},
    utils::chunked_vec::ChunkedVec,
};

pub trait BackingStorage {
    fn startup(&self);
    fn next_free_task_id(&self) -> TaskId;
    fn uncompleted_operations(&self) -> Vec<AnyOperation>;
    fn save_snapshot(
        &self,
        operations: Vec<Arc<AnyOperation>>,
        task_cache_updates: Vec<ChunkedVec<(Arc<CachedTaskType>, TaskId)>>,
        meta_updates: Vec<ChunkedVec<CachedDataUpdate>>,
        data_updates: Vec<ChunkedVec<CachedDataUpdate>>,
    ) -> Result<()>;
    fn forward_lookup_task_cache(&self, key: &CachedTaskType) -> Option<TaskId>;
    fn reverse_lookup_task_cache(&self, task_id: TaskId) -> Option<Arc<CachedTaskType>>;
    fn lookup_data(&self, task_id: TaskId, category: TaskDataCategory) -> Vec<CachedDataItem>;
}
