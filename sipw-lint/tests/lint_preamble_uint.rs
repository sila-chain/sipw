/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use pretty_assertions::assert_eq;
use sipw_lint::lints::preamble::Uint;
use sipw_lint::reporters::Text;
use sipw_lint::Linter;

#[tokio::test]
async fn valid() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
sip: 1234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-sip", Uint("sip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn invalid() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
sip: -1234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-sip", Uint("sip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-sip]: preamble header `sip` must be an unsigned integer
  |
6 | sip: -1234
  |     ^^^^^^ not a non-negative integer
  |
"#
    );
}

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
sip: 1é234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-sip", Uint("sip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-sip]: preamble header `sip` must be an unsigned integer
  |
6 | sip: 1é234
  |     ^^^^^^ not a non-negative integer
  |
"#
    );
}
