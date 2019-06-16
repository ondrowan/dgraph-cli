# Dgraph CLI

Dgraph CLI is a tool for communication with [Dgraph database](https://dgraph.io/).
It supports both insecure and secure connection. API will most likely change in
future releases.

## Installation

So far, it is possible to install this tool only via `cargo`:

```bash
$ cargo install dgraph-cli
```

## Usage

### Secure connection

If you want to connect to TLS secured server, you have to provide paths to all
required certificates:

```bash
$ dgraph-cli --url secure-dgraph-server:1443 --root_ca root_ca.crt --cert cert.crt --private_key private.key schema
```

### List schema

```bash
$ dgraph-cli schema

Predicate                          Type      Index   Reverse   Tokenizers       List    Count   Upsert   Lang
-----------------------------------------------------------------------------------------------------------------
_predicate_                        string    false   false                      true    false   false    false
dgraph.group.acl                   string    false   false                      false   false   false    false
dgraph.password                    password  false   false                      false   false   false    false
dgraph.user.group                  uid       false   true                       false   false   false    false
dgraph.xid                         string    true    false     exact            false   false   false    false
name                               string    false   false                      false   false   false    false
```

If you don't need all columns, filter them using `--fields` flag:

```bash
$ dgraph-cli schema --fields predicate type

Predicate                          Type
---------------------------------------------
_predicate_                        string
dgraph.group.acl                   string
dgraph.password                    password
dgraph.user.group                  uid
dgraph.xid                         string
name                               string
```

### Alter predicates

```bash
$ dgraph-cli alter "name: int ."
```