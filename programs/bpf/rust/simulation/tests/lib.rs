#![cfg(feature = "test-bpf")]

use {
    solana_bpf_rust_simulation::process_instruction,
    solana_program_test::{processor, tokio, ProgramTest},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Signer,
        sysvar,
        transaction::Transaction,
    },
};

#[tokio::test]
async fn no_panic() {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "solana_bpf_rust_simulation",
        program_id,
        processor!(process_instruction),
    );

    let mut context = program_test.start_with_context().await;
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(sysvar::slot_history::id(), false)],
            data: vec![],
        }],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction_with_preflight(transaction)
        .await
        .unwrap();
}
