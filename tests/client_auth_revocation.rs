// Copyright 2023 Daniel McCarney.
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

#![cfg(feature = "ring")]

use webpki::{KeyUsage, RevocationCheckDepth, RevocationOptionsBuilder};

fn check_cert(
    ee: &[u8],
    intermediates: &[&[u8]],
    ca: &[u8],
    crls: &[&dyn webpki::CertRevocationList],
) -> Result<(), webpki::Error> {
    let anchors = &[webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];
    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    let time = webpki::Time::from_seconds_since_unix_epoch(0x1fed_f00d);
    // TODO(XXX): Allow configuring depth and revocation status requirements per-test.
    let revocation = match crls.len() {
        0 => None,
        _ => Some(
            RevocationOptionsBuilder::new(crls)
                .unwrap()
                .with_depth(RevocationCheckDepth::Chain)
                .allow_unknown_status()
                .build(),
        ),
    };
    cert.verify_for_usage(
        &[webpki::ECDSA_P256_SHA256],
        anchors,
        intermediates,
        time,
        KeyUsage::client_auth(),
        revocation,
    )
}

// DO NOT EDIT BELOW: generated by tests/generate.py

#[test]
fn no_crls_test_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[cfg(feature = "alloc")]
#[test]
fn no_crls_test_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn no_relevant_crl_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/no_relevant_crl_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[cfg(feature = "alloc")]
#[test]
fn no_relevant_crl_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/no_relevant_crl_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn ee_not_revoked_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_not_revoked_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[cfg(feature = "alloc")]
#[test]
fn ee_not_revoked_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_not_revoked_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn ee_revoked_badsig_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_badsig_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::InvalidCrlSignatureForPublicKey)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_revoked_badsig_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_badsig_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::InvalidCrlSignatureForPublicKey)
    );
}

#[test]
fn ee_revoked_wrong_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_wrong_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_revoked_wrong_ku_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_wrong_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[test]
fn ee_not_revoked_wrong_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_not_revoked_wrong_ku_ee_depth.crl.der")
            .as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_not_revoked_wrong_ku_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_not_revoked_wrong_ku_ee_depth.crl.der")
            .as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[test]
fn ee_revoked_no_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_no_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_revoked_no_ku_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_no_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[test]
fn ee_revoked_crl_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_crl_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_revoked_crl_ku_ee_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_crl_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[test]
fn no_crls_test_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn no_relevant_crl_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/no_relevant_crl_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[cfg(feature = "alloc")]
#[test]
fn no_relevant_crl_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/no_relevant_crl_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn int_not_revoked_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_not_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[cfg(feature = "alloc")]
#[test]
fn int_not_revoked_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_not_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(check_cert(ee, intermediates, ca, crls), Ok(()));
}

#[test]
fn int_revoked_badsig_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_badsig_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::InvalidCrlSignatureForPublicKey)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn int_revoked_badsig_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_badsig_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::InvalidCrlSignatureForPublicKey)
    );
}

#[test]
fn int_revoked_wrong_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_wrong_ku_chain_depth.crl.der")
            .as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn int_revoked_wrong_ku_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_crl_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_wrong_ku_chain_depth.crl.der")
            .as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::IssuerNotCrlSigner)
    );
}

#[test]
fn ee_revoked_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_revoked_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[test]
fn int_revoked_no_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_no_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn int_revoked_no_ku_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/no_ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/no_ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_no_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[test]
fn int_revoked_crl_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_crl_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn int_revoked_crl_ku_chain_depth_owned() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/int_revoked_crl_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[test]
fn ee_with_top_bit_set_serial_revoked() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.topbit.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_with_top_bit_set_serial_revoked.crl.der")
            .as_slice(),
    )
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ee_with_top_bit_set_serial_revoked_owned() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.topbit.ee.der");
    let intermediates = &[
        include_bytes!("client_auth_revocation/ku_chain.int.a.ca.der").as_slice(),
        include_bytes!("client_auth_revocation/ku_chain.int.b.ca.der").as_slice(),
    ];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[&webpki::BorrowedCertRevocationList::from_der(
        include_bytes!("client_auth_revocation/ee_with_top_bit_set_serial_revoked.crl.der")
            .as_slice(),
    )
    .unwrap()
    .to_owned()
    .unwrap() as &dyn webpki::CertRevocationList];
    assert_eq!(
        check_cert(ee, intermediates, ca, crls),
        Err(webpki::Error::CertRevoked)
    );
}
