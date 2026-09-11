/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file. You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use pretty_assertions::assert_eq;
use sipw_lint::lints::markdown::RelativeLinks;
use sipw_lint::reporters::Text;
use sipw_lint::Linter;

#[tokio::test]
async fn inline_link_to_consensus_specs() {
    let src = r#"---
header: value1
---

[hi](https://github.com/sila-chain/consensus-specs/blob/6c2b46ae3248760e0f6e52d61077d8b31e43ad1d/specs/sip4844/validator.md#compute_aggregated_poly_and_commitment)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: vec![
                    "^https://(www\\.)?github\\.com/sila-chain/consensus-specs/blob/[a-f0-9]{40}/.+$",
                ],
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
async fn inline_link_with_scheme() {
    let src = r#"---
header: value1
---

[hi](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hi](https://example.com/)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn inline_link_with_scheme_to_sips_sila_org() {
    let src = r#"---
header: value1
---

[hello](https://sips.sila.org/SIPS/sip-1234)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hello](https://sips.sila.org/SIPS/sip-1234)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn inline_link_with_scheme_to_srcs_sila_org() {
    let src = r#"---
header: value1
---

[hello](https://srcs.sila.org/SRCS/src-1234)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hello](https://srcs.sila.org/SRCS/src-1234)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn inline_link_with_scheme_to_creativecommons_copyright() {
    let src = r#"---
header: value1
---

[copyright](https://creativecommons.org/publicdomain/zero/1.0/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [copyright](https://creativecommons.org/publicdomain/zero/1.0/)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../LICENSE.md` instead
"#
    );
}

#[tokio::test]
async fn inline_link_with_scheme_and_numbers() {
    let src = r#"---
header: value1
---

[hi](https://example.com/4444)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hi](https://example.com/4444)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn inline_link_protocol_relative() {
    let src = r#"---
header: value1
---

[hi](//example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hi](//example.com/)
  | ^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn inline_link_root_relative() {
    let src = r#"---
header: value1
---

Hello [hi](/foo)!
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi](/foo)!
  |       ^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn inline_link_relative() {
    let src = r#"---
header: value1
---

Hello [hi](./foo/bar)!
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
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
async fn reference_link_with_scheme() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: https://example.com
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi][hello]!
  |       ^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn reference_link_relative() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: ./hello-world
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
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
async fn reference_link_with_scheme_to_sips_sila_org() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: https://sips.sila.org/SIPS/sip-1234
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi][hello]!
  |       ^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn reference_link_with_scheme_to_srcs_sila_org() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: https://srcs.sila.org/SRCS/src-1234
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi][hello]!
  |       ^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn inline_autolink() {
    let src = r#"---
header: value1
---

https://example.com/

hello world
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | https://example.com/
  | ^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn anchor_link() {
    let src = r#"---
header: value1
---

<a href="https://example.com/">example</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <a href="https://example.com/">example</a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn anchor_link_protocol_relative() {
    let src = r#"---
header: value1
---

<a href="//example.com/">example</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <a href="//example.com/">example</a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn anchor_link_protocol_relative_to_sips_sila_org() {
    let src = r#"---
header: value1
---

<a href="//sips.sila.org/SIPS/sip-1234">example</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <a href="//sips.sila.org/SIPS/sip-1234">example</a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn anchor_link_protocol_relative_to_srcs_sila_org() {
    let src = r#"---
header: value1
---

<a href="//srcs.sila.org/SRCS/src-1234">example</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <a href="//srcs.sila.org/SRCS/src-1234">example</a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `./sip-1234.md` instead
"#
    );
}

#[tokio::test]
async fn anchor_link_relative_double_slash() {
    let src = r#"---
header: value1
---

<a href="foo//example">example</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
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
async fn anchor_link_protocol_relative_to_creativecommons_copyright() {
    let src = r#"---
header: value1
---

<a href="//creativecommons.org/publicdomain/zero/1.0/">copyright</a>
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <a href="//creativecommons.org/publicdomain/zero/1.0/">copyright</a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../LICENSE.md` instead
"#
    );
}

#[tokio::test]
async fn img_relative_double_slash() {
    let src = r#"---
header: value1
---

<img src="foo//example">
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
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
async fn img_protocol_relative() {
    let src = r#"---
header: value1
---

<img src="//example.com/foo.jpg">
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <img src="//example.com/foo.jpg">
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
"#
    );
}

#[tokio::test]
async fn img_protocol_relative_to_sips_sila_org() {
    let src = r#"---
header: value1
---

<img src="//sips.sila.org/assets/sip-712/eth_sign.png">
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <img src="//sips.sila.org/assets/sip-712/eth_sign.png">
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../assets/sip-712/eth_sign.png` instead
"#
    );
}

#[tokio::test]
async fn img_protocol_relative_to_srcs_sila_org() {
    let src = r#"---
header: value1
---

<img src="//srcs.sila.org/assets/src-712/eth_sign.png">
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <img src="//srcs.sila.org/assets/src-712/eth_sign.png">
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../assets/src-712/eth_sign.png` instead
"#
    );
}

#[tokio::test]
async fn img_with_scheme_to_sips_sila_org() {
    let src = r#"---
header: value1
---

<img src="https://sips.sila.org/assets/sip-712/eth_sign.png">!
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <img src="https://sips.sila.org/assets/sip-712/eth_sign.png">!
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../assets/sip-712/eth_sign.png` instead
"#
    );
}

#[tokio::test]
async fn img_with_scheme_to_srcs_sila_org() {
    let src = r#"---
header: value1
---

<img src="https://srcs.sila.org/assets/src-712/eth_sign.png">
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-rel",
            RelativeLinks {
                exceptions: Vec::<&str>::new(),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | <img src="https://srcs.sila.org/assets/src-712/eth_sign.png">
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ used here
  |
  = help: use `../assets/src-712/eth_sign.png` instead
"#
    );
}
