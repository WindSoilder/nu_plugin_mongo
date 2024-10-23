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

## Usage

1. First, you need to have latest nushell installed(Of cause it's a nushell plugin).
2. Then you can install from `cargo`
```
cargo install nu_plugin_mongo
```
3. Add and use the plugin: `plugin add <nu_plugin_mongo_path>; plugin use mongo`
4. Then you will gain some `mongoc` commands to play with.


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
  mongoc list-colls - list all available collection names
  mongoc list-indexes - find mongodb documents
  mongoc open - open mongodb connection, the url must contains default databse
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
