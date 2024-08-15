# Publishing emails using the Ojos CLI

This guide will help you figure out how to use the Ojos CLI to publish email to
the Ojos Project newsletter.

## Configure your settings

Before you are allowed to send email, you need to set some settings. Settings
such as...

1. The domain (it's `mail.ojosproject.org`)
2. The API Key (Carlos has this)
3. The email (it's `Ojos Project <newsletter@mail.ojosproject.org>`)

You can set all of these by running the following command:

```shell
ojos newsletter config -d mail.ojosproject.org -e "Ojos Project <newsletter@mail.ojosproject.org>" -a <YOUR API KEY>
```

That commend will do a few things. First, it'll create a new `Newsletter/`
folder inside of your home folder. Inside of that folder, a `mail/`
folder and a `.env` file. Inside of the `mail/` folder, you will find two text
files: `content_en.txt` and `content_es.txt`.

> [!CRITICAL]
> The content inside of `.env` includes information that should be **SECRET**.
> NEVER share the content of this folder.

If you want to check if your settings are correct, you can run:

```shell
ojos newsletter config --show
```

The output should look something like this:

```text
EMAIL = Ojos Project <newsletter@mail.ojosproject.org>
DOMAIN = mail.ojosproject.org
API_KEY = REDACTED.

Check the full configuration in "/Users/<your username>/Newsletter/.env".
```

Congrats, you're ready to write an email!

## Writing an email

As explained in the last section, a new folder called `Newsletter` was created
in your home folder. Inside of that folder, there's a `mail` folder. Go inside
and you will find a `content_en.txt` file. Write your full email in that file!

When you're ready to publish (ensure to add a sender signature!), you need to
choose which newsletter you're sending to:

- `newsletter`: Production mailing list
- `newsletter-testing`: Testing emailing list

Then you run this command:

```shell
ojos newsletter publish <chosen newsletter>
```

You will be asked questions, such as confirmation of settings and previewing
your email content. Follow the instructions and your email should be sent in!
