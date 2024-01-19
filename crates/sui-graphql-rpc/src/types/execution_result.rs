// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::transaction_block_effects::TransactionBlockEffects;
use async_graphql::*;

/// The result of an execution, including errors that occurred during said execution.
#[derive(SimpleObject, Clone)]
pub(crate) struct ExecutionResult {
    /// The errors field captures any errors that occurred during execution
    pub errors: Option<Vec<String>>,

    /// The effects of the executed transaction.
    pub effects: TransactionBlockEffects,
}
