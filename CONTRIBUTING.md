# Contributing to Telluric Deckay projects

These contributing guidelines apply to all projects in the [Telluric
Deckay](https://github.com/TelluricDeckay/) organization.

To view open issues on all projects in the TD organization, you can use
[this
link](https://github.com/issues?q=is%3Aissue+is%3Aopen+org%3ATelluricDeckay).

You may join our [Zulip](https://zulip.com/) chat room at
[https://telluric-deckay.zulipchat.com/](telluric-deckay.zulipchat.com)

## Bug Reports and Feature Requests

Anyone my open an issue.

## Coding Standards

[Rust Code Style
Guidelines](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html?highlight=formatting#automatic-formatting-with-rustfmt)

## Patches and Pull Requests

To prevent work-overlap, please post on a ticket if you'll be working
on a specific issue (or create a ticket if there's not one open yet.
**Note**: If more than one person submits a patch for the same thing,
your patch may get rejected.

**Note**: If you agreed to work to work on a ticket but later find that
you're unable to work on it, or if you changed your mind, please post
again on the ticket to let everyone know it's up for grabs.

Use [The GitHub flow](https://guides.github.com/introduction/flow/),
which mostly just involves creating a separate branch for each patch
you're working on. Using that method helps prevent merge conflicts
later. *Note* that you should never need to work on the trunk (main)
branch or merge your patches into the trunk branch (See "syncing"
below).

Source code patches should only contain changes related to a single
issue. This helps speed up the review and discussion process. However,
if you're helping fix typos and grammar errors in documentation,
multiple changes in one PR is fine. General rule of thumb for
documentation patches on this project is 5 unrelated changes or fewer
to a PR. But if they are only one-word or letter changes, I can be
flexible and more than 5 will still be gratefully accepted for review.

## Documentation

Lines in markdown documents should be less than 79 characters long when
possible.

## Syncing

Periodically, you'll need to sync your repo with the upstream.
GitHub has instructions for doing this

* [Configuring a remote for a fork](https://help.github.com/articles/configuring-a-remote-for-a-fork/)
  * For step 3 on that page, use https://github.com/TelluricDeckay/<repo-name> for the URL.
* [Syncing a Fork](https://help.github.com/articles/syncing-a-fork/)
  * On that page, it shows how to merge the **trunk** branch (steps 4 & 5).
