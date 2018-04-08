// https://developer.github.com/v3/activity/events/types/

pub mod push;
pub mod issue_comment;

#[derive(Debug, Clone)]
pub enum GitHubEvent {
    Push(push::Push)
}