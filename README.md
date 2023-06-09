[![Crates.io](https://img.shields.io/crates/d/rocksdb-utils-lookup.svg)](https://crates.io/crates/rocksdb-utils-lookup)
[![Crates.io](https://img.shields.io/crates/v/rocksdb-utils-lookup.svg)](https://crates.io/crates/rocksdb-utils-lookup)
[![Crates.io](https://img.shields.io/crates/l/rocksdb-utils-lookup.svg)](https://crates.io/crates/rocksdb-utils-lookup)
[![CI](https://github.com/bihealth/rocksdb-utils-lookup/actions/workflows/rust.yml/badge.svg)](https://github.com/bihealth/rocksdb-utils-lookup/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/bihealth/rocksdb-utils-lookup/branch/main/graph/badge.svg?token=aZchhLWdzt)](https://codecov.io/gh/bihealth/rocksdb-utils-lookup)

# Helpers for using RocksDB as Lookup Tables

<img src="https://raw.githubusercontent.com/bihealth/rocksdb-utils-lookup/utils/dalle-utility-workers-rocking.png" width="512px" height="512px" align="right">

This repository contains helper code that is useful when using RocksDB as a static lookup table (after initial creation).

## Managing Project with Terraform

```
# export GITHUB_OWNER=bihealth
# export GITHUB_TOKEN=ghp_TOKEN

# cd utils/terraform
# terraform init
# terraform import github_repository.rocksdb-utils-lookup rocksdb-utils-lookup
# terraform validate
# terraform fmt
# terraform plan
# terraform apply
```
