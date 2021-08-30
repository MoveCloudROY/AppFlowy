use crate::{
    entities::{app::RepeatedApp, workspace::parser::*},
    errors::*,
    impl_def_and_def_mut,
};
use flowy_derive::ProtoBuf;
use std::convert::TryInto;

#[derive(ProtoBuf, Default)]
pub struct CreateWorkspaceRequest {
    #[pb(index = 1)]
    pub name: String,

    #[pb(index = 2)]
    pub desc: String,

    #[pb(index = 3)]
    pub user_id: String,
}

#[derive(ProtoBuf, Default)]
pub struct CreateWorkspaceParams {
    #[pb(index = 1)]
    pub name: String,

    #[pb(index = 2)]
    pub desc: String,

    #[pb(index = 3)]
    pub user_id: String,
}

impl TryInto<CreateWorkspaceParams> for CreateWorkspaceRequest {
    type Error = WorkspaceError;

    fn try_into(self) -> Result<CreateWorkspaceParams, Self::Error> {
        let name = WorkspaceName::parse(self.name).map_err(|e| {
            ErrorBuilder::new(ErrorCode::WorkspaceNameInvalid)
                .msg(e)
                .build()
        })?;

        let desc = WorkspaceDesc::parse(self.desc).map_err(|e| {
            ErrorBuilder::new(ErrorCode::WorkspaceDescInvalid)
                .msg(e)
                .build()
        })?;

        if self.user_id.is_empty() {
            return Err(ErrorBuilder::new(ErrorCode::UserIdIsEmpty)
                .msg("Create workspace failed. UserId is empty")
                .build());
        }

        Ok(CreateWorkspaceParams {
            name: name.0,
            desc: desc.0,
            user_id: self.user_id,
        })
    }
}

#[derive(PartialEq, ProtoBuf, Default, Debug)]
pub struct Workspace {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub desc: String,

    #[pb(index = 4)]
    pub apps: RepeatedApp,
}

impl Workspace {
    pub fn new(id: String, name: String, desc: String) -> Self {
        Self {
            id,
            name,
            desc,
            apps: RepeatedApp::default(),
        }
    }
}

#[derive(PartialEq, Debug, Default, ProtoBuf)]
pub struct RepeatedWorkspace {
    #[pb(index = 1)]
    pub items: Vec<Workspace>,
}

impl_def_and_def_mut!(RepeatedWorkspace, Workspace);
