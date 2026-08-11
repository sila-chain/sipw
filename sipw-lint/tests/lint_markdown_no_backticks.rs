/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use pretty_assertions::assert_eq;
use sipw_lint::lints::markdown::NoBackticks;
use sipw_lint::reporters::Text;
use sipw_lint::Linter;

#[tokio::test]
async fn sip_in_backticks() {
    let src = r#"---
header: value1
---

hello

`SIP-1234`
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-no-backticks]: proposal references should not be in backticks
  |
7 | `SIP-1234`
  | ^^^^^^^^^^
  |
  = info: the pattern in question: `SIP-[0-9]+`
"#
    );
}

#[tokio::test]
async fn valid_code_in_backticks() {
    let src = r#"---
header: value1
---

hello

`SRC20` and `ISRC7777`
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn multiple_sip_references() {
    let src = r#"---
header: value1
---

This document references `SIP-1234` and `SIP-5678` which should both be flagged.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("SIP-1234"));
    assert!(reports.contains("SIP-5678"));
}

#[tokio::test]
async fn sip_in_code_block() {
    let src = r#"---
header: value1
---

Here's some code:

```solidity
// This is fine because it's in a code block
function implementSIP1234() {
    // SIP-1234 implementation
}
```

But this `SIP-1234` should be flagged.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("SIP-1234"));
    assert_eq!(reports.matches("SIP-1234").count(), 1); // Only one instance should be flagged
}

#[tokio::test]
async fn sip_in_mixed_context() {
    let src = r#"---
header: value1
---

| SIP | Description |
|-----|-------------|
| `SIP-1234` | Some description |

- Item 1: `SIP-5678`
- Item 2: Some `code` and `SIP-9012`

The function `doSomething()` implements `SIP-3456`.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("SIP-1234"));
    assert!(reports.contains("SIP-5678"));
    assert!(reports.contains("SIP-9012"));
    assert!(reports.contains("SIP-3456"));
}

#[tokio::test]
async fn mixed_code_and_sip_references() {
    let src = r#"---
header: value1
---

The function `implementSRC20()` follows `SIP-20` standard.
The class `MyToken` implements `SIP-721` for NFTs.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should flag SIP references in backticks
    assert!(reports.contains("SIP-20"));
    assert!(reports.contains("SIP-721"));

    // The error message should mention backticks
    assert!(reports.contains("proposal references should not be in backticks"));
}

#[tokio::test]
async fn sip_in_image_alt_text() {
    let src = r#"---
header: value1
---

![This is `SIP-1234` in alt text](image.png)
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should still flag SIP references in backticks, even in alt text
    assert!(reports.contains("SIP-1234"));
}

#[tokio::test]
async fn self_reference_sip() {
    let src = r#"---
sip: 1234
title: Test SIP
---

This is `SIP-1234` which is the current SIP.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"SIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should flag SIP references in backticks, even for self-references
    assert!(reports.contains("SIP-1234"));
}
