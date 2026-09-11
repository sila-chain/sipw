/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use pretty_assertions::assert_eq;
use sipw_lint::lints::preamble::RequireReferenced;
use sipw_lint::reporters::Text;
use sipw_lint::Linter;

#[tokio::test]
async fn valid() {
    let src = r#"---
header: Extension of SIP-44
other: 1234, 44, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn valid_erc() {
    let src = r#"---
header: Extension of SRC-44
other: 1234, 44, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: Exténsion of SIP-9999 ánd SIP-44
other: 1234, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-req-ref]: proposals mentioned in preamble header `header` must appear in `other`
  |
2 | header: Exténsion of SIP-9999 ánd SIP-44
  |                      ^^^^^^^^     ^^^^^^ mentioned here
  |                      |
  |                      mentioned here
  |
"#
    );
}

#[tokio::test]
async fn one_missing() {
    let src = r#"---
header: Extension of SIP-9999 and SIP-44
other: 1234, 44, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-req-ref]: proposals mentioned in preamble header `header` must appear in `other`
  |
2 | header: Extension of SIP-9999 and SIP-44
  |                      ^^^^^^^^ mentioned here
  |
"#
    );
}

#[tokio::test]
async fn two_missing() {
    let src = r#"---
header: Extension of SIP-9999 and SIP-45
other: 1234, 44, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-req-ref]: proposals mentioned in preamble header `header` must appear in `other`
  |
2 | header: Extension of SIP-9999 and SIP-45
  |                      ^^^^^^^^     ^^^^^^ mentioned here
  |                      |
  |                      mentioned here
  |
"#
    );
}

#[tokio::test]
async fn missing_sip_erc() {
    let src = r#"---
header: Extension of SIP-9999 and SRC-45
other: 1234, 44, 55
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-req-ref",
            RequireReferenced {
                name: "header",
                requires: "other",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-req-ref]: proposals mentioned in preamble header `header` must appear in `other`
  |
2 | header: Extension of SIP-9999 and SRC-45
  |                      ^^^^^^^^     ^^^^^^ mentioned here
  |                      |
  |                      mentioned here
  |
"#
    );
}
