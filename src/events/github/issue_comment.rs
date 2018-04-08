// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Label {
//     url: String,
//     name: String,
//     color: String, // Color Hex
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Issue {
//     url: String, // URL
//     labels_url: String, // URL
//     comments_url: String, // URL
//     events_url: String, // URL
//     html_url: String, // URL
//     #[serde(rename="id")]
//     issue_id: u64,
//     number: u64,
//     title: String,
//     user: DetailedGitHubUser,
//     labels: Vec<Label>,
//     state: String,
//     locked: bool,
//     assignee: Option<String>,
//     milestone: Option<String>,
//     comments: u64,
//     created_at: String, // Date Time
//     updated_at: String, // Date Time
//     closed_at: Option<String>, // Date Time
//     body: String
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Comment {
//     url: String, // URL
//     html_url: String, // URL
//     issue_url: String, // URL
//     #[serde(rename="id")]
//     comment_id: u64,
//     user: DetailedGitHubUser,
//     created_at: String, // Date Time
//     updated_at: String, // Date Time
//     body: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct IssueComment {
//   action: String,
//   issue: Issue,
//   comment: Comment,
//   repository: Repository,
//   sender: DetailedGitHubUser
// }