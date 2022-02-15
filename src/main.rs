use openssl::{
    pkey::PKey,
    sign::{Signer, Verifier},
};
use parity_wasm::elements::{CustomSection, Module, Section, Serialize};

fn main() {
    sign();
    verify();
}

fn sign() {
    let module_bytes = include_bytes!("../sample.wasm");
    let mut module = parity_wasm::deserialize_buffer::<Module>(module_bytes).unwrap();

    let private_key_bytes = include_bytes!("../keys/private.pem");
    let private_key = PKey::private_key_from_pem(private_key_bytes).unwrap();

    let mut signer = Signer::new_without_digest(&private_key).unwrap();
    let mut signature = signer.sign_oneshot_to_vec(module_bytes).unwrap();

    let mut custom = CustomSection::default();
    custom.name_mut().push_str("signature");
    assert!(signature.len() < 256);
    custom.payload_mut().push(signature.len() as u8); // section length
    custom.payload_mut().append(&mut signature); // section length

    // TODO add padding to max Signature length

    module.sections_mut().push(Section::Custom(custom));
    parity_wasm::serialize_to_file("sample-signed.wasm", module).unwrap();
}

fn verify() {
    let mut module = parity_wasm::deserialize_file("sample-signed.wasm").unwrap();

    let public_key_bytes = include_bytes!("../keys/public.pem");
    let public_key = PKey::public_key_from_pem(public_key_bytes).unwrap();

    let mut signature_section = module.clear_custom_section("signature").unwrap();
    let payload = signature_section.payload_mut();
    let size = payload[0] as usize;
    let signature = &signature_section.payload_mut()[1..=size];
    println!("{}", signature.len());

    let mut verifier = Verifier::new_without_digest(&public_key).unwrap();

    let mut buf = vec![];
    module.serialize(&mut buf).unwrap();
    assert!(verifier.verify_oneshot(&signature, &buf).unwrap());
}
