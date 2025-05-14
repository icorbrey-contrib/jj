// Copyright 2025 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{any::Any, io::Read, time::SystemTime};

use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::backend::Backend;
use crate::backend::BackendResult;
use crate::backend::ChangeId;
use crate::backend::Commit;
use crate::backend::CommitId;
use crate::backend::Conflict;
use crate::backend::ConflictId;
use crate::backend::CopyRecord;
use crate::backend::FileId;
use crate::backend::SigningFn;
use crate::backend::SymlinkId;
use crate::backend::Tree;
use crate::backend::TreeId;
use crate::index::Index;
use crate::repo_path::{RepoPath, RepoPathBuf};

#[derive(Debug)]
pub struct TfvcBackend {}

impl TfvcBackend {
    pub fn name() -> &'static str {
        "tfvc"
    }
}

#[async_trait]
impl Backend for TfvcBackend {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        Self::name()
    }

    fn commit_id_length(&self) -> usize {
        todo!()
    }

    fn change_id_length(&self) -> usize {
        todo!()
    }

    fn root_commit_id(&self) -> &CommitId {
        todo!()
    }

    fn root_change_id(&self) -> &ChangeId {
        todo!()
    }

    fn empty_tree_id(&self) -> &TreeId {
        todo!()
    }

    fn concurrency(&self) -> usize {
        todo!()
    }

    async fn read_file(&self, _path: &RepoPath, id: &FileId) -> BackendResult<Box<dyn Read>> {
        todo!()
    }

    async fn write_file(
        &self,
        _path: &RepoPath,
        contents: &mut (dyn Read + Send),
    ) -> BackendResult<FileId> {
        todo!()
    }

    async fn read_symlink(&self, _path: &RepoPath, id: &SymlinkId) -> BackendResult<String> {
        todo!()
    }

    async fn write_symlink(&self, _path: &RepoPath, target: &str) -> BackendResult<SymlinkId> {
        todo!()
    }

    async fn read_tree(&self, _path: &RepoPath, id: &TreeId) -> BackendResult<Tree> {
        todo!()
    }

    async fn write_tree(&self, _path: &RepoPath, contents: &Tree) -> BackendResult<TreeId> {
        todo!()
    }

    fn read_conflict(&self, _path: &RepoPath, id: &ConflictId) -> BackendResult<Conflict> {
        todo!()
    }

    fn write_conflict(&self, _path: &RepoPath, conflict: &Conflict) -> BackendResult<ConflictId> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn read_commit(&self, id: &CommitId) -> BackendResult<Commit> {
        todo!()
    }

    async fn write_commit(
        &self,
        mut contents: Commit,
        mut sign_with: Option<&mut SigningFn>,
    ) -> BackendResult<(CommitId, Commit)> {
        todo!()
    }

    fn get_copy_records(
        &self,
        paths: Option<&[RepoPathBuf]>,
        root_id: &CommitId,
        head_id: &CommitId,
    ) -> BackendResult<BoxStream<BackendResult<CopyRecord>>> {
        todo!()
    }

    #[tracing::instrument(skip(self, index))]
    fn gc(&self, index: &dyn Index, keep_newer: SystemTime) -> BackendResult<()> {
        todo!()
    }
}
