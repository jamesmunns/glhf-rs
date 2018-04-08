#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Push {
  #[serde(rename="ref")]
  pub push_ref: String, // SHA?
  pub before: String, // SHA?
  pub after: String, // SHA?
  pub created: bool,
  pub deleted: bool,
  pub forced: bool,
  pub base_ref: Option<String>,
  pub compare: String, // URL?
  pub commits: Vec<Commit>,
  pub head_commit: Commit,
  pub repository: Repository,
  pub pusher: GitUser,
  pub sender: DetailedGitHubUser,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename="id")]
    pub commit_id: String, // SHA
    pub tree_id: String, //SHA
    pub distinct: bool,
    pub message: String,
    pub timestamp: String, // datetime
    pub url: String, // URL
    pub author: GitHubUser,
    pub committer: GitHubUser,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename="id")]
    pub repository_id: u64,
    pub name: String,
    pub full_name: String,

    // AJM(TODO): Sometimes this is a GitUser, sometimes a DetailedGitUser
    pub owner: GitUser,

    pub private: bool,
    pub html_url: String, // URL
    pub description: String,
    pub fork: bool,
    pub url: String, // URL
    pub forks_url: String, // URL
    pub keys_url: String, // URL
    pub collaborators_url: String, // URL
    pub teams_url: String, // URL
    pub hooks_url: String, // URL
    pub issue_events_url: String, // URL
    pub events_url: String, // URL
    pub assignees_url: String, // URL
    pub branches_url: String, // URL
    pub tags_url: String, // URL
    pub blobs_url: String, // URL
    pub git_tags_url: String, // URL
    pub git_refs_url: String, // URL
    pub trees_url: String, // URL
    pub statuses_url: String, // URL
    pub languages_url: String, // URL
    pub stargazers_url: String, // URL
    pub contributors_url: String, // URL
    pub subscribers_url: String, // URL
    pub subscription_url: String, // URL
    pub commits_url: String, // URL
    pub git_commits_url: String, // URL
    pub comments_url: String, // URL
    pub issue_comment_url: String, // URL
    pub contents_url: String, // URL
    pub compare_url: String, // URL
    pub merges_url: String, // URL
    pub archive_url: String, // URL
    pub downloads_url: String, // URL
    pub issues_url: String, // URL
    pub pulls_url: String, // URL
    pub milestones_url: String, // URL
    pub notifications_url: String, // URL
    pub labels_url: String, // URL
    pub releases_url: String, // URL

    // TODO(AJM): This is sometimes a unix timestamp, and sometimes
    // a 8601 date. Ignore for now.
    pub created_at: u64, // Date Time

    pub updated_at: String, // Date Time

    // TODO(AJM): This is sometimes a unix timestamp, and sometimes
    // a 8601 date. Ignore for now.
    pub pushed_at: u64, // Unix Timestamp?

    pub git_url: String, // git@URL
    pub ssh_url: String, // ssh@URL
    pub clone_url: String, // URL
    pub svn_url: String, // URL
    pub homepage: Option<String>, // URL
    pub size: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u64,
    pub mirror_url: Option<String>,
    pub open_issues_count: u64,
    pub forks: u64,
    pub open_issues: u64,
    pub watchers: u64,
    pub default_branch: String, // branch
    pub stargazers: Option<u64>,
    pub master_branch: String, // branch
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedGitHubUser {
    pub login: String,
    #[serde(rename="id")]
    pub sender_id: u64,
    pub avatar_url: String, // URL? "https://avatars.githubusercontent.com/u/6752317?v=3",
    pub gravatar_id: String, // URL? "",
    pub url: String, // URL? "https://api.github.com/users/baxterthehacker",
    pub html_url: String, // URL? "https://github.com/baxterthehacker",
    pub followers_url: String, // URL? "https://api.github.com/users/baxterthehacker/followers",
    pub following_url: String, // URL? "https://api.github.com/users/baxterthehacker/following{/other_user}",
    pub gists_url: String, // URL? "https://api.github.com/users/baxterthehacker/gists{/gist_id}",
    pub starred_url: String, // URL? "https://api.github.com/users/baxterthehacker/starred{/owner}{/repo}",
    pub subscriptions_url: String, // URL? "https://api.github.com/users/baxterthehacker/subscriptions",
    pub organizations_url: String, // URL? "https://api.github.com/users/baxterthehacker/orgs",
    pub repos_url: String, // URL? "https://api.github.com/users/baxterthehacker/repos",
    pub events_url: String, // URL? "https://api.github.com/users/baxterthehacker/events{/privacy}",
    pub received_events_url: String, // URL?

    #[serde(rename="type")]
    pub sender_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
}

