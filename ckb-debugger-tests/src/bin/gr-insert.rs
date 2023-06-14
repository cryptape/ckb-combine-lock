use ckb_crypto::secp::Privkey;
use ckb_debugger_tests::combine_lock_mol::{ChildScriptConfigOpt, CombineLockWitness, Uint16};
use ckb_debugger_tests::{create_child_script_config, generate_sighash_all};
use ckb_debugger_tests::{
    hash::{blake160, hash},
    read_tx_template,
};
use ckb_jsonrpc_types::JsonBytes;
use ckb_types::packed::{BytesVec, Script, WitnessArgs};
use ckb_types::prelude::Pack;
use ckb_types::H256;
use clap::Parser;
use molecule::prelude::{Builder, Entity};

const G_PRIVKEY_BUF: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
];

#[derive(Parser)]
struct Args {
    #[arg(long)]
    has_config_cell: bool,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    drop(env_logger::init());
    let mut repr_tx = read_tx_template("../ckb-debugger-tests/templates/gr-insert.json")?;

    let child_script_private_key = Privkey::from(H256::from(G_PRIVKEY_BUF));
    let child_script_pubkey = child_script_private_key.pubkey().expect("pubkey");
    let child_script_pubkey_hash = blake160(&child_script_pubkey.serialize());
    let mut auth = vec![0u8; 21];
    auth[0] = 0; // CKB
    auth[1..].copy_from_slice(&child_script_pubkey_hash);

    let child_script_config = create_child_script_config(&repr_tx, &[2], &[auth.into()], &[&[0]], false)?;

    // the second input cell's type script is global registry
    let global_registry_id = {
        let type_ = repr_tx.mock_info.inputs[1].output.type_.as_ref().unwrap();
        let type_: Script = type_.clone().into();
        hash(type_.as_slice())
    };
    // set script args for the first cell
    let mut args = vec![1u8]; // use global registry
    args.extend(global_registry_id.to_vec());
    let current_hash = hash(child_script_config.as_slice());
    args.extend(current_hash);
    // set both first input and output cell
    repr_tx.mock_info.inputs[0].output.lock.args = JsonBytes::from_vec(args.clone());
    repr_tx.tx.outputs[0].lock.args = JsonBytes::from_vec(args);

    // set script args for the second cell
    let current_hash2 = vec![0; 32]; // a fake current hash
    let mut args2 = vec![1u8];
    args2.extend(global_registry_id.to_vec());
    args2.extend(current_hash2);

    // set both second input and output cell
    repr_tx.mock_info.inputs[1].output.lock.args = JsonBytes::from_vec(args2.clone());
    repr_tx.tx.outputs[1].lock.args = JsonBytes::from_vec(args2);

    let mut cell_data = vec![0xFF; 32]; // next hash
                                        // actually, we should put the new ChildScriptConfig. For testing purpose,
                                        // we just put arbitrary data
    cell_data.extend([1, 1]);
    repr_tx.tx.outputs_data[0] = JsonBytes::from_vec(cell_data);

    let mut cell_data2 = current_hash.to_vec();
    // actually, we should put the ChildScriptConfig. For testing purpose, we
    // just put arbitrary data
    cell_data2.extend([0, 0]);
    repr_tx.tx.outputs_data[1] = JsonBytes::from_vec(cell_data2);

    // signing part
    let inner_witness = BytesVec::new_builder().push(vec![0u8; 65].pack()).build();
    let config: ChildScriptConfigOpt = Some(child_script_config).pack();

    let combine_lock_witness = CombineLockWitness::new_builder()
        .index(Uint16::new_unchecked(0u16.to_le_bytes().to_vec().into()))
        .inner_witness(inner_witness)
        .script_config(config.clone())
        .build();

    let witness_args = WitnessArgs::new_builder()
        .lock(Some(combine_lock_witness.as_bytes()).pack())
        .build();
    repr_tx.tx.witnesses[0] = JsonBytes::from(witness_args.as_bytes().pack());

    let message = generate_sighash_all(&repr_tx, 0)?;
    let sig = child_script_private_key
        .sign_recoverable(&H256::from(message))
        .expect("sign")
        .serialize();
    let inner_witness = BytesVec::new_builder().push(sig.pack()).build();
    let combine_lock_witness = combine_lock_witness
        .as_builder()
        .inner_witness(inner_witness)
        .build();

    let witness_args = WitnessArgs::new_builder()
        .lock(Some(combine_lock_witness.as_bytes()).pack())
        .build();
    repr_tx.tx.witnesses[0] = JsonBytes::from(witness_args.as_bytes().pack());

    let json = serde_json::to_string_pretty(&repr_tx).unwrap();
    println!("{}", json);
    Ok(())
}
