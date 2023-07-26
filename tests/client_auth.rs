// Copyright 2023 Joseph Birr-Pixton.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#[cfg(feature = "alloc")]
use webpki::KeyUsage;

#[cfg(feature = "alloc")]
static ALL_SIGALGS: &[&dyn webpki::SignatureVerificationAlgorithm] = &[
    webpki::ECDSA_P256_SHA256,
    webpki::ECDSA_P256_SHA384,
    webpki::ECDSA_P384_SHA256,
    webpki::ECDSA_P384_SHA384,
    webpki::ED25519,
    webpki::RSA_PKCS1_2048_8192_SHA256,
    webpki::RSA_PKCS1_2048_8192_SHA384,
    webpki::RSA_PKCS1_2048_8192_SHA512,
    webpki::RSA_PKCS1_3072_8192_SHA384,
];

#[cfg(feature = "alloc")]
fn check_cert(ee: &[u8], ca: &[u8]) -> Result<(), webpki::Error> {
    let anchors = &[webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(0x1fed_f00d);
    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    cert.verify_for_usage(
        ALL_SIGALGS,
        anchors,
        &[],
        time,
        KeyUsage::client_auth(),
        None,
    )
}

// DO NOT EDIT BELOW: generated by tests/generate.py

#[test]
#[cfg(feature = "alloc")]
fn cert_with_no_eku_accepted_for_client_auth() {
    let ee = include_bytes!("client_auth/cert_with_no_eku_accepted_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_no_eku_accepted_for_client_auth.ca.der");
    assert_eq!(check_cert(ee, ca), Ok(()));
}

#[test]
#[cfg(feature = "alloc")]
fn cert_with_clientauth_eku_accepted_for_client_auth() {
    let ee = include_bytes!("client_auth/cert_with_clientauth_eku_accepted_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_clientauth_eku_accepted_for_client_auth.ca.der");
    assert_eq!(check_cert(ee, ca), Ok(()));
}

#[test]
#[cfg(feature = "alloc")]
fn cert_with_both_ekus_accepted_for_client_auth() {
    let ee = include_bytes!("client_auth/cert_with_both_ekus_accepted_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_both_ekus_accepted_for_client_auth.ca.der");
    assert_eq!(check_cert(ee, ca), Ok(()));
}

#[test]
#[cfg(feature = "alloc")]
fn cert_with_serverauth_eku_rejected_for_client_auth() {
    let ee = include_bytes!("client_auth/cert_with_serverauth_eku_rejected_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_serverauth_eku_rejected_for_client_auth.ca.der");
    assert_eq!(check_cert(ee, ca), Err(webpki::Error::RequiredEkuNotFound));
}
