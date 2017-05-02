pub mod types;
pub mod request;
pub mod context;

use juniper::RootNode;
use self::types::*;

pub fn schema() -> RootNode<'static, QueryRoot, MutationRoot> {
    RootNode::new(
        QueryRoot {},
        MutationRoot {}
    )
}
