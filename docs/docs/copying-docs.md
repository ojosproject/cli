# Copying Docs from One Repo to Another

The Ojos CLI can make it so that you copy an entire folder's content to your
current directory. You can do so like this:

Structure:

```plaintext
ojosproject/
  website/
    docs/
      ojos-cli/

  ojos-cli/
    docs/
```

I want to copy the documents inside of `ojosproject/cli/docs/` to
`/ojosproject/website/docs/ojos-cli/`. I can do so with the following commands:

```shell
cd ojosproject/website/docs/ojos-cli/

ojos docs copy -i ../../../ojos-cli/docs/ -r
```

`-r`, short for `--readme`, copies the directory at the parent folder of your
`-i`, short for `--input`. The input is relative to your current directory.
