# Ojos CLI API Docs

![Ojos Project header](https://ojosproject.org/images/header.png)

## Table of Contents

- [Ojos CLI API Docs](#ojos-cli-api-docs)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [`ojos frontend`](#ojos-frontend)
  - [`ojos newsletter`](#ojos-newsletter)
    - [`ojos newsletter config`](#ojos-newsletter-config)
    - [`ojos newsletter publish <NEWSLETTER>`](#ojos-newsletter-publish-newsletter)
  - [`ojos docs`](#ojos-docs)
    - [`ojos docs copy`](#ojos-docs-copy)

## Introduction

Welcome to the Ojos CLI API documentation. This document will expand as the CLI
is updated.

> [!TIP]
> You can get most of this information by running `ojos [COMMAND] --help`.

## `ojos frontend`

Introduced in v1.0.0

Creates necessary `.tsx`, `.module.css`, and `/components/` files for a new frontend page

Options:

| Option              | Description                                        |
| ------------------- | -------------------------------------------------- |
| `-n, --name <NAME>` | A page name, to skip the input portion             |
| `-d, --dir <DIR>`   | Choose where to generate files [default: src/app/] |
| `-y`                | Assume yes, skips the verification                 |

## `ojos newsletter`

Manages the configuration and publication of our email newsletter.

### `ojos newsletter config`

Manage your configuration to interact with the Mailgun API.

| Option                    | Description                                                                                   |
| ------------------------- | --------------------------------------------------------------------------------------------- |
| `-p, --path <PATH>`       | Where to save the `Newsletter` folder [default: /Users/user]                                  |
| `-d, --domain <DOMAIN>`   | Domain of the email. For Ojos, it's `mail.ojosproject.org`                                    |
| `-a, --api-key <API_KEY>` | Our Mailgun API key                                                                           |
| `-e, --email <EMAIL>`     | The email to send information. For Ojos, it's `Ojos Project <newsletter@mailojosproject.org>` |
| `-s, --show`              | Includes this to **only** show your configuration settings.                                   |

### `ojos newsletter publish <NEWSLETTER>`

Publish to the newsletter.

Arguments:

| Argument       | Description                                                                                        |
| -------------- | -------------------------------------------------------------------------------------------------- |
| `<NEWSLETTER>` | The newsletter to publish. For Ojos, we use `newsletter`, `newsletter-es`, or `newsletter-testing` |

## `ojos docs`

### `ojos docs copy`

Introduced in v1.3.0.

Copy a /docs/ folder to the current directory from another. Useful for
publishing docs on the website.

| Option                | Description                          |
| --------------------- | ------------------------------------ |
| `-i, --input <INPUT>` | The path to search for `/docs/`      |
| `-r, --readme`        | Include the README at `../README.md` |
