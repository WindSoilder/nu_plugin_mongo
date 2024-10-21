# nu_plugin_mongo

A nushell plugin to interactive with [mongodb](https://www.mongodb.com).

Sometimes I want to explore mongodb data, but I don't want to open `mongo` shell or `python` to query mongodb data.

I want to have more lite weight tool to explore data.  So here is the plugin.

## Demo

<p align="center">
  <img width="600" src="demo.svg">
</p>

## Note

- All I want to do is query and explore data, so the plugin includes query commands only.
- You might note that the command prefix is `mongoc`, not `mongo`.  Because I don't want to make the name conflict with official lagacy [mongo](https://www.mongodb.com/docs/manual/reference/mongo/) cli.

## Full help

```nushell
> mongoc

Operate with mongodb

You must use one of the following subcommands. Using this command as-is will only produce this help message.

Usage:
  > mongoc

Subcommands:
  mongoc find - find mongodb documents
  mongoc find-one - find mongodb documents
  mongoc list - list mongodb connections
  mongoc open - open mongodb connection
  mongoc remove - remove mongodb handles
  mongoc select - select current mongodb handle

Flags:
  -h, --help: Display the help message for this command

Input/output types:
  ╭───┬─────────┬────────╮
  │ # │  input  │ output │
  ├───┼─────────┼────────┤
  │ 0 │ nothing │ string │
  ╰───┴─────────┴────────╯
```
