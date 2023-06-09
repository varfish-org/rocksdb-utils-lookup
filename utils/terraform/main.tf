# Management of the GitHub project.

resource "github_repository" "rocksdb-utils-lookup" {
  name        = "rocksdb-utils-lookup"
  description = "Utilities for using RocksDB as read-only lookup table (after initial bulk import)"

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
