# Management of the GitHub project.

resource "github_repository" "tpl-rs" {
  name        = "tpl-rs"
  description = "This is a template repository"

  has_issues = true
  visibility = "public"

  allow_rebase_merge = false
  allow_merge_commit = false
  delete_branch_on_merge = true

  template {
    owner                = "bihealth"
    repository           = "tpl-rs"
    include_all_branches = true
  }
}
