# TBD

## Deploying contract

```bash
near login # to create local keys in near-cli for desired account
near deploy <account-id> <path-to-wasm-file>
near call <account-id> new '{"owner":"<owner-account-id>"}' --accountId <account-id>
```

