#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Push {
  #[serde(rename="ref")]
  push_ref: String, // SHA?
  before: String, // SHA?
  after: String, // SHA?
  created: bool,
  deleted: bool,
  forced: bool,
  base_ref: Option<String>,
  compare: String, // URL?
  commits: Vec<Commit>,
  head_commit: Commit,
  repository: Repository,
  pusher: GitUser,
  sender: DetailedGitHubUser,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    name: String,
    email: String,
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename="id")]
    commit_id: String, // SHA
    tree_id: String, //SHA
    distinct: bool,
    message: String,
    timestamp: String, // datetime
    url: String, // URL
    author: GitHubUser,
    committer: GitHubUser,
    added: Vec<String>,
    removed: Vec<String>,
    modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename="id")]
    repository_id: u64,
    name: String,
    full_name: String,

    // AJM(TODO): Sometimes this is a GitUser, sometimes a DetailedGitUser
    owner: GitUser,

    private: bool,
    html_url: String, // URL
    description: String,
    fork: bool,
    url: String, // URL
    forks_url: String, // URL
    keys_url: String, // URL
    collaborators_url: String, // URL
    teams_url: String, // URL
    hooks_url: String, // URL
    issue_events_url: String, // URL
    events_url: String, // URL
    assignees_url: String, // URL
    branches_url: String, // URL
    tags_url: String, // URL
    blobs_url: String, // URL
    git_tags_url: String, // URL
    git_refs_url: String, // URL
    trees_url: String, // URL
    statuses_url: String, // URL
    languages_url: String, // URL
    stargazers_url: String, // URL
    contributors_url: String, // URL
    subscribers_url: String, // URL
    subscription_url: String, // URL
    commits_url: String, // URL
    git_commits_url: String, // URL
    comments_url: String, // URL
    issue_comment_url: String, // URL
    contents_url: String, // URL
    compare_url: String, // URL
    merges_url: String, // URL
    archive_url: String, // URL
    downloads_url: String, // URL
    issues_url: String, // URL
    pulls_url: String, // URL
    milestones_url: String, // URL
    notifications_url: String, // URL
    labels_url: String, // URL
    releases_url: String, // URL

    // TODO(AJM): This is sometimes a unix timestamp, and sometimes
    // a 8601 date. Ignore for now.
    created_at: String, // Date Time

    updated_at: String, // Date Time

    // TODO(AJM): This is sometimes a unix timestamp, and sometimes
    // a 8601 date. Ignore for now.
    pushed_at: u64, // Unix Timestamp?

    git_url: String, // git@URL
    ssh_url: String, // ssh@URL
    clone_url: String, // URL
    svn_url: String, // URL
    homepage: Option<String>, // URL
    size: u64,
    stargazers_count: u64,
    watchers_count: u64,
    language: Option<String>,
    has_issues: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: u64,
    mirror_url: Option<String>,
    open_issues_count: u64,
    forks: u64,
    open_issues: u64,
    watchers: u64,
    default_branch: String, // branch
    stargazers: Option<u64>,
    master_branch: String, // branch
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedGitHubUser {
    login: String,
    #[serde(rename="id")]
    sender_id: u64,
    avatar_url: String, // URL? "https://avatars.githubusercontent.com/u/6752317?v=3",
    gravatar_id: String, // URL? "",
    url: String, // URL? "https://api.github.com/users/baxterthehacker",
    html_url: String, // URL? "https://github.com/baxterthehacker",
    followers_url: String, // URL? "https://api.github.com/users/baxterthehacker/followers",
    following_url: String, // URL? "https://api.github.com/users/baxterthehacker/following{/other_user}",
    gists_url: String, // URL? "https://api.github.com/users/baxterthehacker/gists{/gist_id}",
    starred_url: String, // URL? "https://api.github.com/users/baxterthehacker/starred{/owner}{/repo}",
    subscriptions_url: String, // URL? "https://api.github.com/users/baxterthehacker/subscriptions",
    organizations_url: String, // URL? "https://api.github.com/users/baxterthehacker/orgs",
    repos_url: String, // URL? "https://api.github.com/users/baxterthehacker/repos",
    events_url: String, // URL? "https://api.github.com/users/baxterthehacker/events{/privacy}",
    received_events_url: String, // URL?
    #[serde(rename="type")]
    sender_type: String,
    site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUser {
    name: String,
    email: String,
}

